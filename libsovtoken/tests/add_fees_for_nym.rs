#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate rust_libindy_wrapper as indy;
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
        "recipient": addresses[0],
        "amount": 9
    }]).to_string();

    let (did_new, verkey_new) = indy::did::Did::new(wallet.handle, "{}").unwrap();

    let nym_req = indy::ledger::Ledger::build_nym_request(dids[0], &did_new,  Some(&verkey_new), None, None).unwrap();
    let nym_req_signed = indy::ledger::Ledger::sign_request(wallet.handle, dids[0], &nym_req).unwrap();
    let (nym_req_with_fees, pm) = indy::payments::Payment::add_request_fees(wallet.handle, dids[0], &nym_req_signed, &inputs, &outputs, None).unwrap();
    let nym_resp = indy::ledger::Ledger::submit_request(pool_handle, &nym_req_with_fees).unwrap();
    let parsed_resp = indy::payments::Payment::parse_response_with_fees(&pm, &nym_resp).unwrap();

    let parsed_resp_json: Vec<HashMap<String, serde_json::Value>> = serde_json::from_str(&parsed_resp).unwrap();
    assert_eq!(parsed_resp_json.len(), 1);
    assert_eq!(parsed_resp_json[0].get("amount").unwrap().as_u64().unwrap(), 9);
    assert_eq!(parsed_resp_json[0].get("recipient").unwrap().as_str().unwrap(), addresses[0]);

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
        "recipient": addresses[0],
        "amount": 9
    }]).to_string();

    let (did_new, verkey_new) = indy::did::Did::new(wallet.handle, "{}").unwrap();

    let nym_req = indy::ledger::Ledger::build_nym_request(dids[0], &did_new,  Some(&verkey_new), None, None).unwrap();
    let nym_req_signed = indy::ledger::Ledger::sign_request(wallet.handle, dids[0], &nym_req).unwrap();
    let (nym_req_with_fees, pm) = indy::payments::Payment::add_request_fees(wallet.handle, dids[0], &nym_req_signed, &inputs, &outputs, None).unwrap();
    let nym_resp = indy::ledger::Ledger::submit_request(pool_handle, &nym_req_with_fees).unwrap();
    let err = indy::payments::Payment::parse_response_with_fees(&pm, &nym_resp).unwrap_err();
    assert_eq!(err, ErrorCode::PaymentInsufficientFundsError);
}

#[test]
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
        "recipient": addresses[0],
        "amount": 9
    }]).to_string();

    let (did_new, verkey_new) = indy::did::Did::new(wallet.handle, "{}").unwrap();

    let nym_req = indy::ledger::Ledger::build_nym_request(dids[0], &did_new,  Some(&verkey_new), None, None).unwrap();
    let nym_req_signed = indy::ledger::Ledger::sign_request(wallet.handle, dids[0], &nym_req).unwrap();
    let (nym_req_with_fees, pm) = indy::payments::Payment::add_request_fees(wallet.handle, dids[0], &nym_req_signed, &inputs, &outputs, None).unwrap();
    let nym_resp = indy::ledger::Ledger::submit_request(pool_handle, &nym_req_with_fees).unwrap();
    indy::payments::Payment::parse_response_with_fees(&pm, &nym_resp).unwrap();

    let (did_new_2, verkey_new_2) = indy::did::Did::new(wallet.handle, "{}").unwrap();

    let nym_req = indy::ledger::Ledger::build_nym_request(dids[0], &did_new_2,  Some(&verkey_new_2), None, None).unwrap();
    let nym_req_signed = indy::ledger::Ledger::sign_request(wallet.handle, dids[0], &nym_req).unwrap();
    let (nym_req_with_fees, pm) = indy::payments::Payment::add_request_fees(wallet.handle, dids[0], &nym_req_signed, &inputs, &outputs, None).unwrap();
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
        "recipient": addresses[0],
        "amount": 9
    }]).to_string();

    let (did_new, verkey_new) = indy::did::Did::new(wallet.handle, "{}").unwrap();

    let nym_req = indy::ledger::Ledger::build_nym_request(dids[0], &did_new,  Some(&verkey_new), None, None).unwrap();
    let nym_req_signed = indy::ledger::Ledger::sign_request(wallet.handle, dids[0], &nym_req).unwrap();
    let (nym_req_with_fees, pm) = indy::payments::Payment::add_request_fees(wallet.handle, dids[0], &nym_req_signed, &inputs, &outputs, None).unwrap();
    let nym_resp = indy::ledger::Ledger::submit_request(pool_handle, &nym_req_with_fees).unwrap();
    let parsed_resp = indy::payments::Payment::parse_response_with_fees(&pm, &nym_resp).unwrap();

    let parsed_resp_json: Vec<HashMap<String, serde_json::Value>> = serde_json::from_str(&parsed_resp).unwrap();
    assert_eq!(parsed_resp_json.len(), 1);
    assert_eq!(parsed_resp_json[0].get("amount").unwrap().as_u64().unwrap(), 9);
    assert_eq!(parsed_resp_json[0].get("recipient").unwrap().as_str().unwrap(), addresses[0]);

    let utxos = get_utxo::send_get_utxo_request(&wallet, pool_handle, dids[0], &addresses[0]);
    let utxo = &utxos[0];

    assert_eq!(utxos.len(), 1);
    assert_eq!(utxo.payment_address, addresses[0]);
    assert_eq!(utxo.amount, 9);
}

#[test]
pub fn build_and_submit_nym_with_fees_from_invalid_did_and_check_utxo_remain_unspent() {
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
        "recipient": addresses[0],
        "amount": 9
    }]).to_string();

    let (did_new, verkey_new) = indy::did::Did::new(wallet.handle, "{}").unwrap();
    let (did_new_2, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();

    let nym_req = indy::ledger::Ledger::build_nym_request(&did_new_2, &did_new,  Some(&verkey_new), None, None).unwrap();
    let (nym_req_with_fees, pm) = indy::payments::Payment::add_request_fees(wallet.handle, dids[0], &nym_req, &inputs, &outputs, None).unwrap();
    let resp = indy::ledger::Ledger::sign_and_submit_request(pool_handle, wallet.handle, dids[0], &nym_req_with_fees).unwrap();
    let err = indy::payments::Payment::parse_response_with_fees(&pm, &resp).unwrap_err();
    assert_eq!(err, ErrorCode::CommonInvalidStructure);

    let utxo_2 = utils::payment::get_utxo::get_first_utxo_txo_for_payment_address(&wallet, pool_handle, dids[0], &addresses[0]);
    assert_eq!(utxo, utxo_2);
}

/// This test reproduces bug described in https://evernym.atlassian.net/browse/TOK-252
#[test]
pub fn build_and_submit_nym_with_fees_from_other_nym_txn() {
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
        "recipient": addresses[0],
        "amount": 9
    }]).to_string();

    // We have a user and a malicious node
    // Both of them generate a nym write request
    // User 1 generates fees for his nym and sends it to the ledger
    // Malicious node gets the request from user, fetches fees from it and sends its own nym with that fees
    // It should not be accepted by other nodes and nym from user should be written
    let (did_new_1, verkey_new_1) = indy::did::Did::new(wallet.handle, "{}").unwrap();
    let (did_new_2, verkey_new_2) = indy::did::Did::new(wallet.handle, "{}").unwrap();

    let nym_req_1 = indy::ledger::Ledger::build_nym_request(dids[0], &did_new_1,  Some(&verkey_new_1), None, None).unwrap();
    let nym_req_2 = indy::ledger::Ledger::build_nym_request(dids[1], &did_new_2,  Some(&verkey_new_2), None, None).unwrap();

    let (nym_req_with_fees_1, pm) = indy::payments::Payment::add_request_fees(wallet.handle, dids[0], &nym_req_1, &inputs, &outputs, None).unwrap();

    let nym_req_with_fees_1_parsed = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(&nym_req_with_fees_1).unwrap();
    let fees: &serde_json::Value = nym_req_with_fees_1_parsed.get("fees").unwrap();

    let mut nym_req_without_fees: serde_json::Map<String, serde_json::Value> = serde_json::from_str(&nym_req_2).unwrap();
    nym_req_without_fees.insert("fees".to_string(), fees.clone());
    let nym_req_with_fees_2 = serde_json::to_string(&nym_req_without_fees).unwrap();

    let resp = indy::ledger::Ledger::sign_and_submit_request(pool_handle, wallet.handle, dids[1], &nym_req_with_fees_2).unwrap();
    let err = indy::payments::Payment::parse_response_with_fees(&pm, &resp).unwrap_err();
    assert_eq!(err, ErrorCode::CommonInvalidStructure);

    let resp = indy::ledger::Ledger::sign_and_submit_request(pool_handle, wallet.handle, dids[0], &nym_req_with_fees_1).unwrap();
    indy::payments::Payment::parse_response_with_fees(&pm, &resp).unwrap();
}
