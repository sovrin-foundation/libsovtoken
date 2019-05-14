#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate sovtoken;
extern crate indyrs as indy;

use std::{thread, time};
use std::collections::HashMap;

use indy::future::Future;

use sovtoken::ErrorCode;
use sovtoken::utils::random::rand_string;
use utils::wallet::Wallet;
use utils::setup::{Setup, SetupConfig};
mod utils;

pub const SCHEMA_VERSION: &'static str = "1.0";
pub const GVT_SCHEMA_ATTRIBUTES: &'static str = r#"["name", "age", "sex", "height"]"#;

fn send_revoc_reg_def_with_fees(issuer_did: &str,
                                name: &str,
                                version: &str,
                                tag: &str,
                                attrs: &str,
                                inputs_json: &str,
                                outputs_json: &str,
                                extra: Option<&str>,
                                wallet_handle: i32,
                                pool_handle: i32,
                                cred_def_id: Option<String>) -> Result<String, indy::IndyError> {
    let cred_def_id = cred_def_id.unwrap_or_else(|| {
        let (_, id, _) = create_cred_def(issuer_did,
                                         name,
                                         version,
                                         attrs,
                                         wallet_handle,
                                         pool_handle,
                                         None);
        id
    });

    let tails_writer_config = utils::anoncreds::tails_writer_config();
    let tails_writer_handle = indy::blob_storage::open_writer("default", &tails_writer_config).wait().unwrap();

    let it: Option<String> = None;
    let (_rev_reg_id, revoc_reg_def_json, _rev_reg_entry_json) =
        indy::anoncreds::issuer_create_and_store_revoc_reg(wallet_handle,
                                                           issuer_did,
                                                           None,
                                                           tag,
                                                           &cred_def_id,
                                                           &json!({ "max_cred_num": Some(5), "issuance_type": it }).to_string(),
                                                           tails_writer_handle).wait().unwrap();

    let req = indy::ledger::build_revoc_reg_def_request(issuer_did, &revoc_reg_def_json).wait().unwrap();
    let (req_with_fees, pm) =
        indy::payments::add_request_fees(
            wallet_handle,
            Some(issuer_did),
            &req,
            inputs_json,
            outputs_json,
            extra).wait().unwrap();
    let response = indy::ledger::sign_and_submit_request(pool_handle, wallet_handle, issuer_did, &req_with_fees).wait().unwrap();
    indy::payments::parse_response_with_fees(&pm, &response).wait()
}

fn create_cred_def(did: &str,
                   name: &str,
                   version: &str,
                   attrs: &str,
                   wallet_handle: i32,
                   pool_handle: i32,
                   schema: Option<String>) -> (String, String, String) {
    let schema = schema.unwrap_or_else(|| create_schema_json(did, name, version, attrs, wallet_handle, pool_handle));

    let tag = rand_string(7);
    let (cred_def_id, cred_def_json) = indy::anoncreds::issuer_create_and_store_credential_def(
        wallet_handle,
        did,
        &schema,
        &tag,
        None,
        &json!({"support_revocation": true}).to_string()
    ).wait().unwrap();

    let cred_def_req = indy::ledger::build_cred_def_request(did, &cred_def_json).wait().unwrap();
    let cred_def_req_signed = indy::ledger::sign_request(wallet_handle, did, &cred_def_req).wait().unwrap();
    indy::ledger::submit_request(pool_handle, &cred_def_req_signed).wait().unwrap();

    (schema, cred_def_id, cred_def_json)
}

fn create_schema_json(did: &str,
                      name: &str,
                      version: &str,
                      attrs: &str,
                      wallet_handle: i32,
                      pool_handle: i32) -> String {
    let (schema_id, schema_json) = indy::anoncreds::issuer_create_schema(did, name, version, attrs).wait().unwrap();
    let schema_req = indy::ledger::build_schema_request(did, &schema_json).wait().unwrap();
    let schema_resp = indy::ledger::sign_and_submit_request(pool_handle, wallet_handle, did, &schema_req).wait().unwrap();
    thread::sleep(time::Duration::from_millis(100));
    let get_schema_req = indy::ledger::build_get_schema_request(Some(did), &schema_id).wait().unwrap();
    let get_schema_req_signed = indy::ledger::sign_request(wallet_handle, did, &get_schema_req).wait().unwrap();
    let get_schema_resp = utils::ledger::submit_request_with_retries(pool_handle, &get_schema_req_signed, &schema_resp).unwrap();
    let (_, schema_json) = indy::ledger::parse_get_schema_response(&get_schema_resp).wait().unwrap();
    schema_json
}

#[test]
pub fn build_and_submit_revoc_reg_def_works_with_fees() {
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: Some(vec![10]),
        fees: Some(json!({
            "113": 1
        })),
    });
    let addresses = &setup.addresses;
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();

    let utxo = utils::payment::get_utxo::get_first_utxo_txo_for_payment_address(&wallet, pool_handle, dids[0], &addresses[0]);

    let inputs = json!([utxo]).to_string();
    let outputs = json!([{
        "recipient": addresses[0],
        "amount": 9
    }]).to_string();

    let parsed_resp = send_revoc_reg_def_with_fees(dids[0],
                                                   rand_string(5).as_str(),
                                                   SCHEMA_VERSION,
                                                   rand_string(7).as_str(),
                                                   GVT_SCHEMA_ATTRIBUTES,
                                                   &inputs,
                                                   &outputs,
                                                   None,
                                                   wallet.handle,
                                                   pool_handle,
                                                   None).unwrap();

    let parsed_resp_json: Vec<HashMap<String, serde_json::Value>> = serde_json::from_str(&parsed_resp).unwrap();
    assert_eq!(parsed_resp_json.len(), 1);
    assert_eq!(parsed_resp_json[0].get("amount").unwrap().as_u64().unwrap(), 9);
    assert_eq!(parsed_resp_json[0].get("recipient").unwrap().as_str().unwrap(), addresses[0]);
}

#[test]
pub fn build_and_submit_revoc_reg_def_works_with_fees_and_spent_utxo() {
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: Some(vec![10]),
        fees: Some(json!({
            "113": 1
        })),
    });
    let addresses = &setup.addresses;
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();

    let utxo = utils::payment::get_utxo::get_first_utxo_txo_for_payment_address(&wallet, pool_handle, dids[0], &addresses[0]);

    let inputs = json!([utxo]).to_string();
    let outputs = json!([{
        "recipient": addresses[0],
        "amount": 10
    }]).to_string();

    let (req, _) = indy::payments::build_payment_req(wallet.handle, Some(dids[0]), &inputs, &outputs, None).wait().unwrap();
    indy::ledger::submit_request(pool_handle, &req).wait().unwrap();

    let outputs_2 = json!([{
        "recipient": addresses[0],
        "amount": 9
    }]).to_string();

    let parsed_err = send_revoc_reg_def_with_fees(dids[0],
                                                  rand_string(5).as_str(),
                                                  SCHEMA_VERSION,
                                                  rand_string(7).as_str(),
                                                  GVT_SCHEMA_ATTRIBUTES,
                                                  &inputs,
                                                  &outputs_2,
                                                  None,
                                                  wallet.handle,
                                                  pool_handle,
                                                  None).unwrap_err();

    assert_eq!(parsed_err.error_code, ErrorCode::PaymentSourceDoesNotExistError);
}

#[test]
pub fn build_and_submit_revoc_reg_def_works_with_fees_and_insufficient_funds() {
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: Some(vec![10]),
        fees: Some(json!({
            "113": 11
        })),
    });
    let addresses = &setup.addresses;
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();

    let utxo = utils::payment::get_utxo::get_first_utxo_txo_for_payment_address(&wallet, pool_handle, dids[0], &addresses[0]);

    let inputs = json!([utxo]).to_string();
    let outputs = json!([{
        "recipient": addresses[0],
        "amount": 9
    }]).to_string();

    let parsed_err = send_revoc_reg_def_with_fees(dids[0],
                                                  rand_string(5).as_str(),
                                                  SCHEMA_VERSION,
                                                  rand_string(7).as_str(),
                                                  GVT_SCHEMA_ATTRIBUTES,
                                                  &inputs,
                                                  &outputs,
                                                  None,
                                                  wallet.handle,
                                                  pool_handle,
                                                  None).unwrap_err();

    assert_eq!(parsed_err.error_code, ErrorCode::PaymentInsufficientFundsError);
}
