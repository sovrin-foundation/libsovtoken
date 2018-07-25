#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate sovtoken;
extern crate rust_indy_sdk as indy;

mod utils;
use indy::ErrorCode;
use utils::wallet::Wallet;
use utils::setup::{Setup, SetupConfig};
use sovtoken::logic::parsers::common::TXO;

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

    let (get_utxo_req, payment_method) = indy::payments::Payment::build_verify_req(wallet.handle, dids[0], &txo).unwrap();
    let res = indy::ledger::Ledger::sign_and_submit_request(pool_handle, wallet.handle, dids[0], &get_utxo_req).unwrap();
    let res = indy::payments::Payment::parse_verify_response(&payment_method, &res).unwrap();

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
    let (req, method) = indy::payments::Payment::build_payment_req(wallet.handle, dids[0], &inputs, &outputs, None).unwrap();
    let res = indy::ledger::Ledger::submit_request(pool_handle, &req).unwrap();
    let res = indy::payments::Payment::parse_payment_response(&method, &res).unwrap();

    let res_parsed: serde_json::Value = serde_json::from_str(&res).unwrap();
    let utxos = res_parsed.as_array().unwrap();
    let value = utxos.get(0).unwrap().as_object().unwrap();
    let new_utxo = value.get("receipt").unwrap().as_str().unwrap();

    let (get_utxo_req, payment_method) = indy::payments::Payment::build_verify_req(wallet.handle, dids[0], &new_utxo).unwrap();
    let res = indy::ledger::Ledger::sign_and_submit_request(pool_handle, wallet.handle, dids[0], &get_utxo_req).unwrap();
    let res = indy::payments::Payment::parse_verify_response(&payment_method, &res).unwrap();

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

    let (did_new, verkey_new) = indy::did::Did::new(wallet.handle, "{}").unwrap();

    let nym_req = indy::ledger::Ledger::build_nym_request(dids[0], &did_new,  Some(&verkey_new), None, None).unwrap();
    let nym_req_signed = indy::ledger::Ledger::sign_request(wallet.handle, dids[0], &nym_req).unwrap();
    let (nym_req_with_fees, pm) = indy::payments::Payment::add_request_fees(wallet.handle, dids[0], &nym_req_signed, &inputs, &outputs, None).unwrap();
    let nym_resp = indy::ledger::Ledger::submit_request(pool_handle, &nym_req_with_fees).unwrap();
    let res = indy::payments::Payment::parse_response_with_fees(&pm, &nym_resp).unwrap();

    let res_parsed: serde_json::Value = serde_json::from_str(&res).unwrap();
    let utxos = res_parsed.as_array().unwrap();
    let value = utxos.get(0).unwrap().as_object().unwrap();
    let new_utxo = value.get("receipt").unwrap().as_str().unwrap();

    let (get_utxo_req, payment_method) = indy::payments::Payment::build_verify_req(wallet.handle, dids[0], &new_utxo).unwrap();
    let res = indy::ledger::Ledger::sign_and_submit_request(pool_handle, wallet.handle, dids[0], &get_utxo_req).unwrap();
    let res = indy::payments::Payment::parse_verify_response(&payment_method, &res).unwrap();

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

    let (get_utxo_req, payment_method) = indy::payments::Payment::build_verify_req(wallet.handle, dids[0], &txo).unwrap();
    let res = indy::ledger::Ledger::sign_and_submit_request(pool_handle, wallet.handle, dids[0], &get_utxo_req).unwrap();
    let err = indy::payments::Payment::parse_verify_response(&payment_method, &res).unwrap_err();

    assert_eq!(err, ErrorCode::PaymentSourceDoesNotExistError);
}
