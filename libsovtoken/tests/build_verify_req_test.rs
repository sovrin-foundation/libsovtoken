#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate sovtoken;
extern crate indyrs as indy;

use std::{thread, time};

use indy::future::Future;

use sovtoken::ErrorCode;
use sovtoken::logic::parsers::common::TXO;

mod utils;
use utils::wallet::Wallet;
use utils::setup::{Setup, SetupConfig};


fn sleep(msec: u64) {
    let ms = time::Duration::from_millis(msec);
    thread::sleep(ms);
}

#[test]
pub fn build_and_submit_verify_on_mint() {
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: Some(vec![10]),
        fees: None
    });
    let payment_addresses = &setup.addresses;
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();
    let txo = utils::payment::get_utxo::get_first_utxo_txo_for_payment_address(&wallet, pool_handle, dids[0], &payment_addresses[0]);

    //We need to wait a little before trying to verify txn
    sleep(1000);

    let (get_utxo_req, payment_method) = indy::payments::build_verify_payment_req(wallet.handle, Some(dids[0]), &txo).wait().unwrap();
    let res = indy::ledger::sign_and_submit_request(pool_handle, wallet.handle, dids[0], &get_utxo_req).wait().unwrap();
    let res = indy::payments::parse_verify_payment_response(&payment_method, &res).wait().unwrap();

    let res_parsed: serde_json::Value = serde_json::from_str(&res).unwrap();
    assert!(res_parsed.as_object().unwrap().get("sources").unwrap().as_array().unwrap().is_empty());
    assert_eq!(res_parsed.as_object().unwrap().get("receipts").unwrap().as_array().unwrap().get(0).unwrap().as_object().unwrap().get("receipt").unwrap().as_str().unwrap(), txo);
}

#[test]
pub fn build_and_submit_verify_on_mint_with_empty_did() {
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: Some(vec![10]),
        fees: None
    });
    let payment_addresses = &setup.addresses;
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();
    let txo = utils::payment::get_utxo::get_first_utxo_txo_for_payment_address(&wallet, pool_handle, dids[0], &payment_addresses[0]);

    //We need to wait a little before trying to verify txn
    sleep(1000);

    let (get_utxo_req, payment_method) = indy::payments::build_verify_payment_req(wallet.handle, None, &txo).wait().unwrap();
    let res = indy::ledger::sign_and_submit_request(pool_handle, wallet.handle, dids[0], &get_utxo_req).wait().unwrap();
    let res = indy::payments::parse_verify_payment_response(&payment_method, &res).wait().unwrap();

    let res_parsed: serde_json::Value = serde_json::from_str(&res).unwrap();
    assert!(res_parsed.as_object().unwrap().get("sources").unwrap().as_array().unwrap().is_empty());
    assert_eq!(res_parsed.as_object().unwrap().get("receipts").unwrap().as_array().unwrap().get(0).unwrap().as_object().unwrap().get("receipt").unwrap().as_str().unwrap(), txo);
}

#[test]
pub fn build_and_submit_verify_on_xfer() {
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 2,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: Some(vec![10]),
        fees: None
    });
    let payment_addresses = &setup.addresses;
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();
    let txo = utils::payment::get_utxo::get_first_utxo_txo_for_payment_address(&wallet, pool_handle, dids[0], &payment_addresses[0]);

    let inputs = json!([txo]).to_string();
    let outputs = json!([
        {
            "recipient": payment_addresses[1],
            "amount": 10
        }
    ]).to_string();
    let (req, method) = indy::payments::build_payment_req(wallet.handle, Some(dids[0]), &inputs, &outputs, None).wait().unwrap();
    let res = indy::ledger::submit_request(pool_handle, &req).wait().unwrap();
    let res = indy::payments::parse_payment_response(&method, &res).wait().unwrap();

    let res_parsed: serde_json::Value = serde_json::from_str(&res).unwrap();
    let utxos = res_parsed.as_array().unwrap();
    let value = utxos.get(0).unwrap().as_object().unwrap();
    let new_utxo = value.get("receipt").unwrap().as_str().unwrap();

    //We need to wait a little before trying to verify txn
    sleep(1000);

    let (get_utxo_req, payment_method) = indy::payments::build_verify_payment_req(wallet.handle, Some(dids[0]), &new_utxo).wait().unwrap();
    let res = indy::ledger::sign_and_submit_request(pool_handle, wallet.handle, dids[0], &get_utxo_req).wait().unwrap();
    let res = indy::payments::parse_verify_payment_response(&payment_method, &res).wait().unwrap();

    let res_parsed: serde_json::Value = serde_json::from_str(&res).unwrap();
    assert_eq!(res_parsed.as_object().unwrap().get("sources").unwrap().as_array().unwrap().get(0).unwrap().as_str().unwrap(), txo);
    assert_eq!(res_parsed.as_object().unwrap().get("receipts").unwrap().as_array().unwrap().get(0).unwrap().as_object().unwrap().get("receipt").unwrap().as_str().unwrap(), new_utxo);
}

#[test]
pub fn build_and_submit_verify_on_fees() {
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: Some(vec![10]),
        fees: Some(json!({
            "1": 1
        })),
    });
    let payment_addresses = &setup.addresses;
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();
    let txo = utils::payment::get_utxo::get_first_utxo_txo_for_payment_address(&wallet, pool_handle, dids[0], &payment_addresses[0]);

    let inputs = json!([txo]).to_string();
    let outputs = json!([{
        "recipient": payment_addresses[0],
        "amount": 9
    }]).to_string();

    let (did_new, verkey_new) = indy::did::create_and_store_my_did(wallet.handle, "{}").wait().unwrap();

    let nym_req = indy::ledger::build_nym_request(dids[0], &did_new,  Some(&verkey_new), None, None).wait().unwrap();
    let nym_req_signed = indy::ledger::sign_request(wallet.handle, dids[0], &nym_req).wait().unwrap();
    let (nym_req_with_fees, pm) = indy::payments::add_request_fees(wallet.handle, Some(dids[0]), &nym_req_signed, &inputs, &outputs, None).wait().unwrap();
    let nym_resp = indy::ledger::submit_request(pool_handle, &nym_req_with_fees).wait().unwrap();
    let res = indy::payments::parse_response_with_fees(&pm, &nym_resp).wait().unwrap();

    let res_parsed: serde_json::Value = serde_json::from_str(&res).unwrap();
    let utxos = res_parsed.as_array().unwrap();
    let value = utxos.get(0).unwrap().as_object().unwrap();
    let new_utxo = value.get("receipt").unwrap().as_str().unwrap();

    //We need to wait a little before trying to verify txn
    sleep(1000);

    let (get_utxo_req, payment_method) = indy::payments::build_verify_payment_req(wallet.handle, Some(dids[0]), &new_utxo).wait().unwrap();
    let res = indy::ledger::sign_and_submit_request(pool_handle, wallet.handle, dids[0], &get_utxo_req).wait().unwrap();
    let res = indy::payments::parse_verify_payment_response(&payment_method, &res).wait().unwrap();

    let res_parsed: serde_json::Value = serde_json::from_str(&res).unwrap();
    assert_eq!(res_parsed.as_object().unwrap().get("sources").unwrap().as_array().unwrap().get(0).unwrap().as_str().unwrap(), txo);
    assert_eq!(res_parsed.as_object().unwrap().get("receipts").unwrap().as_array().unwrap().get(0).unwrap().as_object().unwrap().get("receipt").unwrap().as_str().unwrap(), new_utxo);
}

#[test]
pub fn build_and_submit_verify_req_for_unexistant_utxo() {
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: None,
        fees: None,
    });

    let pool_handle = setup.pool_handle;
    let payment_addresses = &setup.addresses;
    let dids = setup.trustees.dids();
    let txo = TXO { address: payment_addresses[0].to_string(), seq_no: 999999 }.to_libindy_string().unwrap();

    //We need to wait a little before trying to verify txn
    sleep(1000);

    let (get_utxo_req, payment_method) = indy::payments::build_verify_payment_req(wallet.handle, Some(dids[0]), &txo).wait().unwrap();
    let res = indy::ledger::sign_and_submit_request(pool_handle, wallet.handle, dids[0], &get_utxo_req).wait().unwrap();
    let err = indy::payments::parse_verify_payment_response(&payment_method, &res).wait().unwrap_err();

    assert_eq!(err.error_code, ErrorCode::PaymentSourceDoesNotExistError);
}

#[test]
fn build_verify_req_works_for_invalid_utxo() {
    sovtoken::api::sovtoken_init();
    let wallet = Wallet::new();
    let (did, _) = indy::did::create_and_store_my_did(wallet.handle, &json!({"seed": "000000000000000000000000Trustee1"}).to_string()).wait().unwrap();

    let receipt = "txo:sov:1234";

    let err = indy::payments::build_verify_payment_req(wallet.handle, Some(&did), receipt).wait().unwrap_err();

    assert_eq!(err.error_code, ErrorCode::CommonInvalidStructure)
}