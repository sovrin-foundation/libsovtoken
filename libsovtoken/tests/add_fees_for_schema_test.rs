#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate rust_indy_sdk as indy;
extern crate sovtoken;


mod utils;

use std::{thread, time};
use std::collections::HashMap;
use indy::ErrorCode;

pub const GVT_SCHEMA_NAME: &'static str = "gvt";
pub const SCHEMA_VERSION: &'static str = "1.0";
pub const GVT_SCHEMA_ATTRIBUTES: &'static str = r#"["name", "age", "sex", "height"]"#;

#[test]
pub fn build_and_submit_schema_with_fees() {
    sovtoken::api::sovtoken_init();
    let payment_method = sovtoken::api::PAYMENT_METHOD_NAME;
    let pc_str = utils::pool::create_pool_config();
    let pool_config = Some(pc_str.as_str());
    indy::pool::Pool::set_protocol_version(2).unwrap();

    let pool_name = utils::pool::create_pool_ledger(pool_config);
    let pool_handle = indy::pool::Pool::open_ledger(&pool_name, None).unwrap();
    let wallet = utils::wallet::Wallet::new();

    let (did_trustee, _) = utils::did::initial_trustee(wallet.handle);

    let pa1 = utils::payment::address::generate(&wallet, None);

    let mut mint_cfg = HashMap::new();
    mint_cfg.insert(pa1.clone(), 10);

    utils::mint::mint_tokens(mint_cfg, pool_handle, wallet.handle, &did_trustee).unwrap();

    let (utxo, _, _) = utils::get_utxo::get_first_utxo_for_payment_address(wallet.handle, pool_handle, &did_trustee, &pa1);

    let inputs = json!([utxo]).to_string();
    let outputs = json!([{
        "paymentAddress": pa1,
        "amount": 9
    }]).to_string();

    let fees = json!({
        "101": 1
    }).to_string();

    let dids = vec![did_trustee.as_str()];

    utils::fees::set_fees(pool_handle, wallet.handle, payment_method, &fees, &dids);

    let (parsed_resp, schema_id, _) = _send_schema_with_fees(&did_trustee, GVT_SCHEMA_NAME, SCHEMA_VERSION, GVT_SCHEMA_ATTRIBUTES, wallet.handle, pool_handle, &inputs, &outputs).unwrap();

    let parsed_resp_json: Vec<HashMap<String, serde_json::Value>> = serde_json::from_str(&parsed_resp).unwrap();
    assert_eq!(parsed_resp_json.len(), 1);
    assert_eq!(parsed_resp_json[0].get("amount").unwrap().as_u64().unwrap(), 9);
    assert_eq!(parsed_resp_json[0].get("paymentAddress").unwrap().as_str().unwrap(), pa1);

    thread::sleep(time::Duration::from_millis(100));

    let get_schema_req = indy::ledger::Ledger::build_get_schema_request(&did_trustee, &schema_id).unwrap();
    let get_schema_resp = indy::ledger::Ledger::sign_and_submit_request(pool_handle, wallet.handle, &did_trustee, &get_schema_req).unwrap();
    let (schema_id_get, _) = indy::ledger::Ledger::parse_get_schema_response(&get_schema_resp).unwrap();
    assert_eq!(schema_id, schema_id_get);

    let fees = json!({
        "101": 0
    }).to_string();

    utils::fees::set_fees(pool_handle, wallet.handle, payment_method, &fees, &dids);
}

#[test]
#[ignore]
pub fn build_and_submit_schema_with_fees_insufficient_funds() {
    sovtoken::api::sovtoken_init();
    let payment_method = sovtoken::api::PAYMENT_METHOD_NAME;
    let pc_str = utils::pool::create_pool_config();
    let pool_config = Some(pc_str.as_str());
    indy::pool::Pool::set_protocol_version(2).unwrap();

    let pool_name = utils::pool::create_pool_ledger(pool_config);
    let pool_handle = indy::pool::Pool::open_ledger(&pool_name, None).unwrap();
    let wallet = utils::wallet::Wallet::new();

    let (did_trustee, _) = utils::did::initial_trustee(wallet.handle);

    let pa1 = utils::payment::address::generate(&wallet, None);

    let mut mint_cfg = HashMap::new();
    mint_cfg.insert(pa1.clone(), 9);

    utils::mint::mint_tokens(mint_cfg, pool_handle, wallet.handle, &did_trustee).unwrap();

    let (utxo, _, _) = utils::get_utxo::get_first_utxo_for_payment_address(wallet.handle, pool_handle, &did_trustee, &pa1);

    let inputs = json!([utxo]).to_string();
    let outputs = json!([{
        "paymentAddress": pa1,
        "amount": 9
    }]).to_string();

    let fees = json!({
        "101": 1
    }).to_string();

    let dids = vec![did_trustee.as_str()];

    utils::fees::set_fees(pool_handle, wallet.handle, payment_method, &fees, &dids);

    let parsed_err = _send_schema_with_fees(&did_trustee, GVT_SCHEMA_NAME, SCHEMA_VERSION, GVT_SCHEMA_ATTRIBUTES, wallet.handle, pool_handle, &inputs, &outputs).unwrap_err();
    assert_eq!(parsed_err, ErrorCode::PaymentInsufficientFundsError);

    let fees = json!({
        "101": 0
    }).to_string();

    utils::fees::set_fees(pool_handle, wallet.handle, payment_method, &fees, &dids);
}

#[test]
#[ignore]
pub fn build_and_submit_schema_with_fees_double_spend() {
    sovtoken::api::sovtoken_init();
    let payment_method = sovtoken::api::PAYMENT_METHOD_NAME;
    let pc_str = utils::pool::create_pool_config();
    let pool_config = Some(pc_str.as_str());
    indy::pool::Pool::set_protocol_version(2).unwrap();

    let pool_name = utils::pool::create_pool_ledger(pool_config);
    let pool_handle = indy::pool::Pool::open_ledger(&pool_name, None).unwrap();
    let wallet = utils::wallet::Wallet::new();

    let (did_trustee, _) = utils::did::initial_trustee(wallet.handle);

    let pa1 = utils::payment::address::generate(&wallet, None);

    let mut mint_cfg = HashMap::new();
    mint_cfg.insert(pa1.clone(), 10);

    utils::mint::mint_tokens(mint_cfg, pool_handle, wallet.handle, &did_trustee).unwrap();

    let (utxo, _, _) = utils::get_utxo::get_first_utxo_for_payment_address(wallet.handle, pool_handle, &did_trustee, &pa1);

    let inputs = json!([utxo]).to_string();
    let outputs = json!([{
        "paymentAddress": pa1,
        "amount": 9
    }]).to_string();

    let fees = json!({
        "101": 1
    }).to_string();

    let dids = vec![did_trustee.as_str()];

    utils::fees::set_fees(pool_handle, wallet.handle, payment_method, &fees, &dids);

    _send_schema_with_fees(&did_trustee, GVT_SCHEMA_NAME, SCHEMA_VERSION, GVT_SCHEMA_ATTRIBUTES, wallet.handle, pool_handle, &inputs, &outputs).unwrap();

    let _parsed_err = _send_schema_with_fees(&did_trustee, &(GVT_SCHEMA_NAME.to_owned() + "1"), SCHEMA_VERSION, GVT_SCHEMA_ATTRIBUTES, wallet.handle, pool_handle, &inputs, &outputs).unwrap_err();
    //assert_eq!(parsed_err, ErrorCode::PaymentUTXODoesNotExist);
    //TODO: this test should fail for a while until we get some vision on a ErrorCodes (both on parsing and new ones)
    assert!(false);

    let fees = json!({
        "100": 0
    }).to_string();

    utils::fees::set_fees(pool_handle, wallet.handle, payment_method, &fees, &dids);
}

fn _send_schema_with_fees(did: &str,
                          name: &str,
                          version: &str,
                          attrs: &str,
                          wallet_handle: i32,
                          pool_handle: i32,
                          inputs_json: &str,
                          outputs_json: &str) -> Result<(String, String, String), ErrorCode> {
    let (schema_id, schema_json) = indy::anoncreds::Issuer::create_schema(did, name, version, attrs).unwrap();
    let schema_req = indy::ledger::Ledger::build_schema_request(did, &schema_json).unwrap();
    let schema_req_signed = indy::ledger::Ledger::sign_request(wallet_handle, did, &schema_req).unwrap();
    let (schema_req_with_fees, pm) = indy::payments::Payment::add_request_fees(wallet_handle, did, &schema_req_signed, inputs_json, outputs_json).unwrap();
    let schema_resp = indy::ledger::Ledger::submit_request(pool_handle, &schema_req_with_fees).unwrap();
    indy::payments::Payment::parse_response_with_fees(&pm, &schema_resp).map(|s| (s, schema_id, schema_json))
}