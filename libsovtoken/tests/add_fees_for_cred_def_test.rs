#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate indyrs as indy;
extern crate sovtoken;

use std::{thread, time};
use std::collections::HashMap;
use sovtoken::utils::random::rand_string;

mod utils;

use utils::setup::{Setup, SetupConfig};
use utils::wallet::Wallet;

use indy::future::Future;

fn send_cred_def_with_fees(did: &str,
                           name: &str,
                           version: &str,
                           attrs: &str,
                           wallet_handle: i32,
                           pool_handle: i32,
                           inputs_json: &str,
                           outputs_json: &str,
                           extra: Option<&str>,
                           schema: Option<String>) -> Result<(String, String, String), indy::IndyError> {
    let schema = schema.unwrap_or_else(|| create_schema_json(did, name, version, attrs, wallet_handle, pool_handle));

    let tag = rand_string(7);
    let (cred_def_id, cred_def_json) = indy::anoncreds::issuer_create_and_store_credential_def(
        wallet_handle,
        did,
        &schema,
        &tag,
        None,
        &json!({"support_revocation": false}).to_string()
    ).wait().unwrap();

    let cred_def_req = indy::ledger::build_cred_def_request(did, &cred_def_json).wait().unwrap();
    let cred_def_req_signed = indy::ledger::sign_request(wallet_handle, did, &cred_def_req).wait().unwrap();
    let (cred_def_req_with_fees, pm) = indy::payments::add_request_fees(
        wallet_handle,
        Some(did),
        &cred_def_req_signed,
        inputs_json,
        outputs_json,
        extra
    ).wait().unwrap();
    let cred_def_response_with_fees = indy::ledger::submit_request(pool_handle, &cred_def_req_with_fees).wait().unwrap();

    indy::payments::parse_response_with_fees(&pm, &cred_def_response_with_fees).map(|s| (s, cred_def_id, cred_def_json)).wait()
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

pub const SCHEMA_VERSION: &'static str = "1.0";
pub const GVT_SCHEMA_ATTRIBUTES: &'static str = r#"["name", "age", "sex", "height"]"#;

#[test]
pub fn build_and_submit_cred_def_with_fees() {
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: Some(vec![10]),
        fees: Some(json!({
            "102": 1
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

    let (parsed_resp, cred_def_id, _) = send_cred_def_with_fees(dids[0], rand_string(5).as_str(), SCHEMA_VERSION, GVT_SCHEMA_ATTRIBUTES, wallet.handle, pool_handle, &inputs, &outputs, None, None).unwrap();

    let parsed_resp_json: Vec<HashMap<String, serde_json::Value>> = serde_json::from_str(&parsed_resp).unwrap();
    assert_eq!(parsed_resp_json.len(), 1);
    assert_eq!(parsed_resp_json[0].get("amount").unwrap().as_u64().unwrap(), 9);
    assert_eq!(parsed_resp_json[0].get("recipient").unwrap().as_str().unwrap(), addresses[0]);

    thread::sleep(time::Duration::from_millis(100));

    let get_cred_def_req = indy::ledger::build_get_cred_def_request(Some(dids[0]), &cred_def_id).wait().unwrap();
    let get_cred_def_resp = indy::ledger::sign_and_submit_request(pool_handle, wallet.handle, dids[0], &get_cred_def_req).wait().unwrap();
    let (cred_def_id_get, _) = indy::ledger::parse_get_cred_def_response(&get_cred_def_resp).wait().unwrap();
    assert_eq!(cred_def_id, cred_def_id_get);
}

#[test]
pub fn build_and_submit_cred_def_with_fees_insufficient_funds() {
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: Some(vec![9]),
        fees: Some(json!({
            "102": 1
        })),
    });
    let addresses = &setup.addresses;
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();

    let utxo = utils::payment::get_utxo::get_first_utxo_txo_for_payment_address(&wallet, pool_handle, dids[0], &addresses[0]);

    let inputs = json!([utxo]).to_string();
    let outputs_1 = json!([{
        "recipient": addresses[0],
        "amount": 9
    }]).to_string();

    let parsed_err = send_cred_def_with_fees(dids[0], rand_string(3).as_str(), SCHEMA_VERSION, GVT_SCHEMA_ATTRIBUTES, wallet.handle, pool_handle, &inputs, &outputs_1, None, None).unwrap_err();
    assert_eq!(parsed_err.error_code, indy::ErrorCode::PaymentInsufficientFundsError);

    let outputs_2 = json!([{
        "recipient": addresses[0],
        "amount": 1
    }]).to_string();

    let parsed_err = send_cred_def_with_fees(dids[0], rand_string(3).as_str(), SCHEMA_VERSION, GVT_SCHEMA_ATTRIBUTES, wallet.handle, pool_handle, &inputs, &outputs_2, None, None).unwrap_err();
    assert_eq!(parsed_err.error_code, indy::ErrorCode::PaymentExtraFundsError);
}

#[test]
pub fn build_and_submit_cred_def_with_fees_double_spend() {
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: Some(vec![10]),
        fees: Some(json!({
            "102": 1
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

    send_cred_def_with_fees(dids[0], rand_string(3).as_str(), SCHEMA_VERSION, GVT_SCHEMA_ATTRIBUTES, wallet.handle, pool_handle, &inputs, &outputs, None, None).unwrap();

    let parsed_err = send_cred_def_with_fees(dids[0], rand_string(3).as_str(), SCHEMA_VERSION, GVT_SCHEMA_ATTRIBUTES, wallet.handle, pool_handle, &inputs, &outputs, None, None).unwrap_err();
    assert_eq!(parsed_err.error_code, indy::ErrorCode::PaymentSourceDoesNotExistError);
}

#[test]
pub fn build_and_submit_cred_def_with_fees_twice_and_check_utxo_remain_unspent() {
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: Some(vec![10]),
        fees: Some(json!({
            "102": 1
        })),
    });
    let addresses = &setup.addresses;
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();
    let (did_new, _) = indy::did::create_and_store_my_did(wallet.handle, "{}").wait().unwrap();

    let utxo = utils::payment::get_utxo::get_first_utxo_txo_for_payment_address(&wallet, pool_handle, dids[0], &addresses[0]);

    let inputs = json!([utxo]).to_string();
    let outputs = json!([{
        "recipient": addresses[0],
        "amount": 9
    }]).to_string();

    let schema = Some(create_schema_json(dids[0], rand_string(3).as_str(), SCHEMA_VERSION, GVT_SCHEMA_ATTRIBUTES, wallet.handle, pool_handle));
    let parsed_err = send_cred_def_with_fees(&did_new, rand_string(3).as_str(), SCHEMA_VERSION, GVT_SCHEMA_ATTRIBUTES, wallet.handle, pool_handle, &inputs, &outputs, None, schema).unwrap_err();
    assert_eq!(parsed_err.error_code, indy::ErrorCode::CommonInvalidStructure);

    let utxo_2 = utils::payment::get_utxo::get_first_utxo_txo_for_payment_address(&wallet, pool_handle, dids[0], &addresses[0]);
    assert_eq!(utxo, utxo_2);
}