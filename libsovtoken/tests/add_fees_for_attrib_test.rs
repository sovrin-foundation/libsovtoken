#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;
extern crate indyrs as indy;
extern crate sovtoken;

use indy::future::Future;

mod utils;
use utils::setup::{Setup, SetupConfig};
use utils::wallet::Wallet;

use sovtoken::ErrorCode;
use sovtoken::logic::parsers::common::UTXO;
use sovtoken::utils::constants::txn_types::ATTRIB;


pub const ATTRIB_RAW_DATA_2: &'static str = r#"{"endpoint":{"ha":"127.0.0.1:5555"}}"#;
pub const ATTRIB_RAW_DATA: &'static str = r#"{"endpoint":{"ha":"127.0.0.1:5555"}}"#;

#[test]
pub fn build_and_submit_attrib_with_fees() {
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: Some(vec![10]),
        fees: Some(json!({
            "100": 1
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

    let parsed_resp = _send_attrib_with_fees(dids[0], Some(ATTRIB_RAW_DATA), wallet.handle, pool_handle, &inputs, &outputs).unwrap();

    let parsed_utxos: Vec<UTXO> = serde_json::from_str(&parsed_resp).unwrap();
    assert_eq!(parsed_utxos.len(), 1);
    assert_eq!(parsed_utxos[0].amount, 9);
    assert_eq!(parsed_utxos[0].recipient, addresses[0]);

    let get_attrib_resp = send_get_attrib_req(&wallet, pool_handle, dids[0], dids[0], Some("endpoint"));
    let data = get_data_from_attrib_reply(get_attrib_resp);
    assert_eq!(ATTRIB_RAW_DATA, data);
}

#[test]
pub fn build_and_submit_attrib_with_fees_and_no_change() {
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: Some(vec![10]),
        fees: Some(json!({
            "100": 10
        })),
    });
    let addresses = &setup.addresses;
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();

    let utxo = utils::payment::get_utxo::get_first_utxo_txo_for_payment_address(&wallet, pool_handle, dids[0], &addresses[0]);

    let inputs = json!([utxo]).to_string();
    let outputs = json!([]).to_string();

    let parsed_resp = _send_attrib_with_fees(dids[0], Some(ATTRIB_RAW_DATA_2), wallet.handle, pool_handle, &inputs, &outputs).unwrap();

    let parsed_utxos: Vec<UTXO> = serde_json::from_str(&parsed_resp).unwrap();
    assert_eq!(parsed_utxos.len(), 0);

    let get_attrib_resp = send_get_attrib_req(&wallet, pool_handle, dids[0], dids[0], Some("endpoint"));
    let data = get_data_from_attrib_reply(get_attrib_resp);
    assert_eq!(ATTRIB_RAW_DATA_2, data);
}

#[test]
pub fn build_and_submit_attrib_with_fees_incorrect_funds() {
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: Some(vec![9]),
        fees: Some(json!({
            ATTRIB: 1
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

    let parsed_err = _send_attrib_with_fees(dids[0], Some(ATTRIB_RAW_DATA), wallet.handle, pool_handle, &inputs, &outputs_1).unwrap_err();
    assert_eq!(parsed_err.error_code, ErrorCode::PaymentInsufficientFundsError);

    let outputs_2 = json!([{
        "recipient": addresses[0],
        "amount": 1
    }]).to_string();

    let parsed_err = _send_attrib_with_fees(dids[0], Some(ATTRIB_RAW_DATA), wallet.handle, pool_handle, &inputs, &outputs_2).unwrap_err();
    assert_eq!(parsed_err.error_code, ErrorCode::PaymentExtraFundsError);
}

#[test]
pub fn build_and_submit_attrib_with_fees_from_invalid_did_and_check_utxo_remain_unspent() {
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: Some(vec![9]),
        fees: Some(json!({
            ATTRIB: 1
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

    let parsed_err = _send_attrib_with_fees(&did_new, Some(ATTRIB_RAW_DATA), wallet.handle, pool_handle, &inputs, &outputs).unwrap_err();
    assert_eq!(parsed_err.error_code, ErrorCode::CommonInvalidStructure);

    let utxo_2 = utils::payment::get_utxo::get_first_utxo_txo_for_payment_address(&wallet, pool_handle, dids[0], &addresses[0]);
    assert_eq!(utxo, utxo_2);
}

#[test]
pub fn build_and_submit_attrib_with_fees_double_spend() {
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: Some(vec![10]),
        fees: Some(json!({
            ATTRIB: 1
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

    let parsed_resp = _send_attrib_with_fees(dids[0], Some(ATTRIB_RAW_DATA), wallet.handle, pool_handle, &inputs, &outputs).unwrap();

    let parsed_utxos: Vec<UTXO> = serde_json::from_str(&parsed_resp).unwrap();
    assert_eq!(parsed_utxos.len(), 1);
    assert_eq!(parsed_utxos[0].amount, 9);
    assert_eq!(parsed_utxos[0].recipient, addresses[0]);

    let get_attrib_resp = send_get_attrib_req(&wallet, pool_handle, dids[0], dids[0], Some("endpoint"));
    let data = get_data_from_attrib_reply(get_attrib_resp);
    assert_eq!(ATTRIB_RAW_DATA, data);

    let parsed_err = _send_attrib_with_fees(dids[0], Some(ATTRIB_RAW_DATA_2), wallet.handle, pool_handle, &inputs, &outputs).unwrap_err();
    assert_eq!(parsed_err.error_code, ErrorCode::PaymentSourceDoesNotExistError);
}

fn _send_attrib_with_fees(did: &str, data: Option<&str>, wallet_handle: i32, pool_handle: i32, inputs: &str, outputs: &str) -> Result<String, indy::IndyError> {
    let attrib_req = indy::ledger::build_attrib_request(did, did,  None, data, None).wait().unwrap();
    let attrib_req_signed = indy::ledger::sign_request(wallet_handle, did, &attrib_req).wait().unwrap();
    let (attrib_req_with_fees, pm) = indy::payments::add_request_fees(wallet_handle, Some(did), &attrib_req_signed, inputs, outputs, None).wait().unwrap();
    let attrib_resp = indy::ledger::submit_request(pool_handle, &attrib_req_with_fees).wait().unwrap();
    indy::payments::parse_response_with_fees(&pm, &attrib_resp).wait()
}

fn send_get_attrib_req(wallet: &Wallet, pool_handle: i32, did: &str, target: &str, attribute: Option<&str>) -> String {
    let get_attrib_req = indy::ledger::build_get_attrib_request(Some(did), target, attribute, None, None).wait().unwrap();
    indy::ledger::sign_and_submit_request(pool_handle, wallet.handle, did, &get_attrib_req).wait().unwrap()
}

fn get_data_from_attrib_reply(reply: String) -> String {
    let reply_value: serde_json::Value = serde_json::from_str(&reply).unwrap();
    reply_value
        .get("result").unwrap()
        .get("data").unwrap()
        .as_str().unwrap()
        .to_owned()
}