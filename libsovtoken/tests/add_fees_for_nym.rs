#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate rust_indy_sdk as indy;
extern crate sovtoken;

mod utils;

use indy::ErrorCode;
use std::collections::HashMap;
use utils::payment::get_utxo;
use utils::setup::{Setup, SetupConfig};
use utils::wallet::Wallet;

#[test]
pub fn build_and_submit_nym_with_fees() {
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
    let addresses = &setup.addresses;
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();


    let utxo = utils::payment::get_utxo::get_first_utxo_txo_for_payment_address(&wallet, pool_handle, dids[0], &addresses[0]);

    let inputs = json!([utxo]).to_string();
    let outputs = json!([{
        "paymentAddress": addresses[0],
        "amount": 9
    }]).to_string();

    let (did_new, verkey_new) = indy::did::Did::new(wallet.handle, "{}").unwrap();

    let nym_req = indy::ledger::Ledger::build_nym_request(dids[0], &did_new,  Some(&verkey_new), None, None).unwrap();
    let nym_req_signed = indy::ledger::Ledger::sign_request(wallet.handle, dids[0], &nym_req).unwrap();
    let (nym_req_with_fees, pm) = indy::payments::Payment::add_request_fees(wallet.handle, dids[0], &nym_req_signed, &inputs, &outputs).unwrap();
    let nym_resp = indy::ledger::Ledger::submit_request(pool_handle, &nym_req_with_fees).unwrap();
    let parsed_resp = indy::payments::Payment::parse_response_with_fees(&pm, &nym_resp).unwrap();

    let parsed_resp_json: Vec<HashMap<String, serde_json::Value>> = serde_json::from_str(&parsed_resp).unwrap();
    assert_eq!(parsed_resp_json.len(), 1);
    assert_eq!(parsed_resp_json[0].get("amount").unwrap().as_u64().unwrap(), 9);
    assert_eq!(parsed_resp_json[0].get("paymentAddress").unwrap().as_str().unwrap(), addresses[0]);

    let get_nym_req = indy::ledger::Ledger::build_get_nym_request(dids[0], &did_new).unwrap();
    let get_nym_resp = indy::ledger::Ledger::sign_and_submit_request(pool_handle, wallet.handle, dids[0], &get_nym_req).unwrap();
    let get_nym_resp_json: serde_json::Value = serde_json::from_str(&get_nym_resp).unwrap();
    assert!(get_nym_resp_json.as_object().unwrap().get("result").unwrap().as_object().unwrap().get("data").is_some());
}

#[test]
pub fn build_and_submit_nym_with_fees_insufficient_funds() {
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: Some(vec![10]),
        fees: Some(json!({
            "1": 2
        })),
    });
    let addresses = &setup.addresses;
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();


    let utxo = utils::payment::get_utxo::get_first_utxo_txo_for_payment_address(&wallet, pool_handle, dids[0], &addresses[0]);

    let inputs = json!([utxo]).to_string();
    let outputs = json!([{
        "paymentAddress": addresses[0],
        "amount": 9
    }]).to_string();

    let (did_new, verkey_new) = indy::did::Did::new(wallet.handle, "{}").unwrap();

    let nym_req = indy::ledger::Ledger::build_nym_request(dids[0], &did_new,  Some(&verkey_new), None, None).unwrap();
    let nym_req_signed = indy::ledger::Ledger::sign_request(wallet.handle, dids[0], &nym_req).unwrap();
    let (nym_req_with_fees, pm) = indy::payments::Payment::add_request_fees(wallet.handle, dids[0], &nym_req_signed, &inputs, &outputs).unwrap();
    let nym_resp = indy::ledger::Ledger::submit_request(pool_handle, &nym_req_with_fees).unwrap();
    let err = indy::payments::Payment::parse_response_with_fees(&pm, &nym_resp).unwrap_err();
    assert_eq!(err, ErrorCode::PaymentInsufficientFundsError);
}

#[test]
#[ignore]
pub fn build_and_submit_nym_with_fees_utxo_already_spent() {
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
    let addresses = &setup.addresses;
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();


    let utxo = utils::payment::get_utxo::get_first_utxo_txo_for_payment_address(&wallet, pool_handle, dids[0], &addresses[0]);

    let inputs = json!([utxo]).to_string();
    let outputs = json!([{
        "paymentAddress": addresses[0],
        "amount": 9
    }]).to_string();

    let (did_new, verkey_new) = indy::did::Did::new(wallet.handle, "{}").unwrap();

    let nym_req = indy::ledger::Ledger::build_nym_request(dids[0], &did_new,  Some(&verkey_new), None, None).unwrap();
    let nym_req_signed = indy::ledger::Ledger::sign_request(wallet.handle, dids[0], &nym_req).unwrap();
    let (nym_req_with_fees, pm) = indy::payments::Payment::add_request_fees(wallet.handle, dids[0], &nym_req_signed, &inputs, &outputs).unwrap();
    let nym_resp = indy::ledger::Ledger::submit_request(pool_handle, &nym_req_with_fees).unwrap();
    indy::payments::Payment::parse_response_with_fees(&pm, &nym_resp).unwrap();

    let (did_new_2, verkey_new_2) = indy::did::Did::new(wallet.handle, "{}").unwrap();

    let nym_req = indy::ledger::Ledger::build_nym_request(dids[0], &did_new_2,  Some(&verkey_new_2), None, None).unwrap();
    let nym_req_signed = indy::ledger::Ledger::sign_request(wallet.handle, dids[0], &nym_req).unwrap();
    let (nym_req_with_fees, pm) = indy::payments::Payment::add_request_fees(wallet.handle, dids[0], &nym_req_signed, &inputs, &outputs).unwrap();
    let nym_resp = indy::ledger::Ledger::submit_request(pool_handle, &nym_req_with_fees).unwrap();
    let err = indy::payments::Payment::parse_response_with_fees(&pm, &nym_resp).unwrap_err();
    assert_eq!(err, ErrorCode::PaymentSourceDoesNotExistError);
}

#[test]
pub fn build_and_submit_nym_with_fees_and_get_utxo() {
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
    let addresses = &setup.addresses;
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();


    let utxo = utils::payment::get_utxo::get_first_utxo_txo_for_payment_address(&wallet, pool_handle, dids[0], &addresses[0]);

    let inputs = json!([utxo]).to_string();
    let outputs = json!([{
        "paymentAddress": addresses[0],
        "amount": 9
    }]).to_string();

    let (did_new, verkey_new) = indy::did::Did::new(wallet.handle, "{}").unwrap();

    let nym_req = indy::ledger::Ledger::build_nym_request(dids[0], &did_new,  Some(&verkey_new), None, None).unwrap();
    let nym_req_signed = indy::ledger::Ledger::sign_request(wallet.handle, dids[0], &nym_req).unwrap();
    let (nym_req_with_fees, pm) = indy::payments::Payment::add_request_fees(wallet.handle, dids[0], &nym_req_signed, &inputs, &outputs).unwrap();
    let nym_resp = indy::ledger::Ledger::submit_request(pool_handle, &nym_req_with_fees).unwrap();
    let parsed_resp = indy::payments::Payment::parse_response_with_fees(&pm, &nym_resp).unwrap();

    let parsed_resp_json: Vec<HashMap<String, serde_json::Value>> = serde_json::from_str(&parsed_resp).unwrap();
    assert_eq!(parsed_resp_json.len(), 1);
    assert_eq!(parsed_resp_json[0].get("amount").unwrap().as_u64().unwrap(), 9);
    assert_eq!(parsed_resp_json[0].get("paymentAddress").unwrap().as_str().unwrap(), addresses[0]);

    let utxos = get_utxo::send_get_utxo_request(&wallet, pool_handle, dids[0], &addresses[0]);
    let utxo = &utxos[0];

    assert_eq!(utxos.len(), 1);
    assert_eq!(utxo.payment_address, addresses[0]);
    assert_eq!(utxo.amount, 9);
}