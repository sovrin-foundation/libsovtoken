#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate rust_indy_sdk as indy;
extern crate sovtoken;


mod utils;
use sovtoken::utils::random::rand_string;
use std::{thread, time};
use std::collections::HashMap;
use indy::ErrorCode;

pub const SCHEMA_VERSION: &'static str = "1.0";
pub const GVT_SCHEMA_ATTRIBUTES: &'static str = r#"["name", "age", "sex", "height"]"#;

#[test]
pub fn build_and_submit_cred_def_with_fees() {
    sovtoken::api::sovtoken_init();
    let payment_method = sovtoken::utils::constants::general::PAYMENT_METHOD_NAME;
    let pc_str = utils::pool::create_pool_config();
    let pool_config = Some(pc_str.as_str());
    indy::pool::Pool::set_protocol_version(2).unwrap();

    let pool_name = utils::pool::create_pool_ledger(pool_config);
    let pool_handle = indy::pool::Pool::open_ledger(&pool_name, None).unwrap();
    let wallet = utils::wallet::Wallet::new();

    let trustees = utils::did::initial_trustees(4, wallet.handle, pool_handle).unwrap();
    let dids = utils::did::did_str_from_trustees(&trustees);

    let pa1 = utils::payment::address::generate(&wallet, None);

    let mut mint_cfg = HashMap::new();
    mint_cfg.insert(pa1.clone(), 10);

    utils::mint::mint_tokens(mint_cfg, pool_handle, wallet.handle, &dids).unwrap();

    let (utxo, _, _) = utils::get_utxo::get_first_utxo_for_payment_address(wallet.handle, pool_handle, dids[0], &pa1);

    let inputs = json!([utxo]).to_string();
    let outputs = json!([{
        "paymentAddress": pa1,
        "amount": 9
    }]).to_string();

    let fees = json!({
        "102": 1
    }).to_string();

    utils::fees::set_fees(pool_handle, wallet.handle, payment_method, &fees, &dids);

    let (parsed_resp, cred_def_id, _) = _send_cred_def_with_fees(dids[0], rand_string(5).as_str(), SCHEMA_VERSION, GVT_SCHEMA_ATTRIBUTES, wallet.handle, pool_handle, &inputs, &outputs).unwrap();

    let parsed_resp_json: Vec<HashMap<String, serde_json::Value>> = serde_json::from_str(&parsed_resp).unwrap();
    assert_eq!(parsed_resp_json.len(), 1);
    assert_eq!(parsed_resp_json[0].get("amount").unwrap().as_u64().unwrap(), 9);
    assert_eq!(parsed_resp_json[0].get("paymentAddress").unwrap().as_str().unwrap(), pa1);

    thread::sleep(time::Duration::from_millis(100));

    let get_cred_def_req = indy::ledger::Ledger::build_get_cred_def_request(dids[0], &cred_def_id).unwrap();
    let get_cred_def_resp = indy::ledger::Ledger::sign_and_submit_request(pool_handle, wallet.handle, dids[0], &get_cred_def_req).unwrap();
    let (cred_def_id_get, _) = indy::ledger::Ledger::parse_get_cred_def_response(&get_cred_def_resp).unwrap();
    assert_eq!(cred_def_id, cred_def_id_get);

    let fees = json!({
        "102": 0
    }).to_string();

    utils::fees::set_fees(pool_handle, wallet.handle, payment_method, &fees, &dids);
}

#[test]
#[ignore]
pub fn build_and_submit_cred_def_with_fees_insufficient_funds() {
    sovtoken::api::sovtoken_init();
    let payment_method = sovtoken::utils::constants::general::PAYMENT_METHOD_NAME;
    let pc_str = utils::pool::create_pool_config();
    let pool_config = Some(pc_str.as_str());
    indy::pool::Pool::set_protocol_version(2).unwrap();

    let pool_name = utils::pool::create_pool_ledger(pool_config);
    let pool_handle = indy::pool::Pool::open_ledger(&pool_name, None).unwrap();
    let wallet = utils::wallet::Wallet::new();

    let trustees = utils::did::initial_trustees(4, wallet.handle, pool_handle).unwrap();
    let dids = utils::did::did_str_from_trustees(&trustees);

    let pa1 = utils::payment::address::generate(&wallet, None);

    let mut mint_cfg = HashMap::new();
    mint_cfg.insert(pa1.clone(), 9);

    utils::mint::mint_tokens(mint_cfg, pool_handle, wallet.handle, &dids).unwrap();

    let (utxo, _, _) = utils::get_utxo::get_first_utxo_for_payment_address(wallet.handle, pool_handle, dids[0], &pa1);

    let inputs = json!([utxo]).to_string();
    let outputs = json!([{
        "paymentAddress": pa1,
        "amount": 9
    }]).to_string();

    let fees = json!({
        "102": 1
    }).to_string();

    utils::fees::set_fees(pool_handle, wallet.handle, payment_method, &fees, &dids);

    let parsed_err = _send_cred_def_with_fees(dids[0], rand_string(3).as_str(), SCHEMA_VERSION, GVT_SCHEMA_ATTRIBUTES, wallet.handle, pool_handle, &inputs, &outputs).unwrap_err();
    assert_eq!(parsed_err, ErrorCode::PaymentInsufficientFundsError);

    let fees = json!({
        "102": 0
    }).to_string();

    utils::fees::set_fees(pool_handle, wallet.handle, payment_method, &fees, &dids);
}

#[test]
#[ignore]
pub fn build_and_submit_cred_def_with_fees_double_spend() {
    sovtoken::api::sovtoken_init();
    let payment_method = sovtoken::utils::constants::general::PAYMENT_METHOD_NAME;
    let pc_str = utils::pool::create_pool_config();
    let pool_config = Some(pc_str.as_str());
    indy::pool::Pool::set_protocol_version(2).unwrap();

    let pool_name = utils::pool::create_pool_ledger(pool_config);
    let pool_handle = indy::pool::Pool::open_ledger(&pool_name, None).unwrap();
    let wallet = utils::wallet::Wallet::new();

    let trustees = utils::did::initial_trustees(4, wallet.handle, pool_handle).unwrap();
    let dids = utils::did::did_str_from_trustees(&trustees);

    let pa1 = utils::payment::address::generate(&wallet, None);

    let mut mint_cfg = HashMap::new();
    mint_cfg.insert(pa1.clone(), 10);

    utils::mint::mint_tokens(mint_cfg, pool_handle, wallet.handle, &dids).unwrap();

    let (utxo, _, _) = utils::get_utxo::get_first_utxo_for_payment_address(wallet.handle, pool_handle, dids[0], &pa1);

    let inputs = json!([utxo]).to_string();
    let outputs = json!([{
        "paymentAddress": pa1,
        "amount": 9
    }]).to_string();

    let fees = json!({
        "102": 1
    }).to_string();

    utils::fees::set_fees(pool_handle, wallet.handle, payment_method, &fees, &dids);

    _send_cred_def_with_fees(dids[0], rand_string(3).as_str(), SCHEMA_VERSION, GVT_SCHEMA_ATTRIBUTES, wallet.handle, pool_handle, &inputs, &outputs).unwrap();

    let _parsed_err = _send_cred_def_with_fees(dids[0], rand_string(3).as_str(), SCHEMA_VERSION, GVT_SCHEMA_ATTRIBUTES, wallet.handle, pool_handle, &inputs, &outputs).unwrap_err();
    //assert_eq!(parsed_err, ErrorCode::PaymentUTXODoesNotExist);
    //TODO: this test should fail for a while until we get some vision on a ErrorCodes (both on parsing and new ones)
    assert!(false);

    let fees = json!({
        "100": 0
    }).to_string();

    utils::fees::set_fees(pool_handle, wallet.handle, payment_method, &fees, &dids);
}

fn _send_cred_def_with_fees(did: &str,
                            name: &str,
                            version: &str,
                            attrs: &str,
                            wallet_handle: i32,
                            pool_handle: i32,
                            inputs_json: &str,
                            outputs_json: &str) -> Result<(String, String, String), ErrorCode> {
    let (schema_id, schema_json) = indy::anoncreds::Issuer::create_schema(did, name, version, attrs).unwrap();
    let schema_req = indy::ledger::Ledger::build_schema_request(did, &schema_json).unwrap();
    let schema_resp = indy::ledger::Ledger::sign_and_submit_request(pool_handle, wallet_handle, did, &schema_req).unwrap();
    thread::sleep(time::Duration::from_millis(100));
    let get_schema_req = indy::ledger::Ledger::build_get_schema_request(did, &schema_id).unwrap();
    let get_schema_req_signed = indy::ledger::Ledger::sign_request(wallet_handle, did, &get_schema_req).unwrap();
    let get_schema_resp = utils::ledger::submit_request_with_retries(pool_handle, &get_schema_req_signed, &schema_resp).unwrap();
    let (_, schema_json) = indy::ledger::Ledger::parse_get_schema_response(&get_schema_resp).unwrap();

    let tag = rand_string(7);
    let (cred_def_id, cred_def_json) = indy::anoncreds::Issuer::create_and_store_credential_def(
        wallet_handle,
        did,
        &schema_json,
        &tag,
        None,
        &json!({"support_revocation": false}).to_string()
    ).unwrap();

    let cred_def_req = indy::ledger::Ledger::build_cred_def_request(did, &cred_def_json).unwrap();
    let cred_def_req_signed = indy::ledger::Ledger::sign_request(wallet_handle, did, &cred_def_req).unwrap();
    let (cred_def_req_with_fees, pm) = indy::payments::Payment::add_request_fees(
        wallet_handle,
        did,
        &cred_def_req_signed,
        inputs_json,
        outputs_json
    ).unwrap();
    let cred_def_response_with_fees = indy::ledger::Ledger::submit_request(pool_handle, &cred_def_req_with_fees).unwrap();

    indy::payments::Payment::parse_response_with_fees(&pm, &cred_def_response_with_fees).map(|s| (s, cred_def_id, cred_def_json))
}