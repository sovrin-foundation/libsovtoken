#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate sovtoken;
extern crate rust_libindy_wrapper as indy;

mod utils;
use utils::wallet::Wallet;
use utils::setup::{Setup, SetupConfig};
use sovtoken::logic::address::strip_qualifier_from_address;
use sovtoken::logic::address::verkey_from_unqualified_address;

#[test]
pub fn build_and_submit_get_utxo_request() {
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

    let (get_utxo_req, payment_method) = indy::payments::Payment::build_get_payment_sources_request(wallet.handle, dids[0], &payment_addresses[0]).unwrap();
    let res = indy::ledger::Ledger::sign_and_submit_request(pool_handle, wallet.handle, dids[0], &get_utxo_req).unwrap();
    let res = indy::payments::Payment::parse_get_payment_sources_response(&payment_method, &res).unwrap();

    let res_parsed: Vec<serde_json::Value> = serde_json::from_str(&res).unwrap();
    assert_eq!(res_parsed.len(), 1);
    let utxo = res_parsed.get(0).unwrap().as_object().unwrap();
    assert_eq!(utxo.get("paymentAddress").unwrap().as_str().unwrap(), payment_addresses[0]);
    assert_eq!(utxo.get("amount").unwrap().as_u64().unwrap(), 10);
}

#[test]
pub fn build_and_submit_get_utxo_request_no_utxo() {
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: None,
        fees: None,
    });
    let payment_addresses = &setup.addresses;
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();

    let (get_utxo_req, payment_method) = indy::payments::Payment::build_get_payment_sources_request(wallet.handle, dids[0], &payment_addresses[0]).unwrap();
    let res = indy::ledger::Ledger::sign_and_submit_request(pool_handle, wallet.handle, dids[0], &get_utxo_req).unwrap();
    let res = indy::payments::Payment::parse_get_payment_sources_response(&payment_method, &res).unwrap();

    let res_parsed: Vec<serde_json::Value> = serde_json::from_str(&res).unwrap();
    assert_eq!(res_parsed.len(), 0);
}

#[test]
pub fn payment_address_is_identifier() {
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 1,
        num_users: 0,
        mint_tokens: None,
        fees: None,
    });
    let payment_addresses = &setup.addresses;
    let dids = setup.trustees.dids();

    let (get_utxo_req, _) = indy::payments::Payment::build_get_payment_sources_request(wallet.handle, dids[0], &payment_addresses[0]).unwrap();
    let req : serde_json::Value = serde_json::from_str(&get_utxo_req).unwrap();
    let identifier = req.as_object().unwrap().get("identifier").unwrap().as_str().unwrap();
    let unqualified_addr = strip_qualifier_from_address(&payment_addresses[0]);
    let unqualified_addr = verkey_from_unqualified_address(&unqualified_addr).unwrap();
    assert_eq!(identifier, unqualified_addr);
    assert_ne!(identifier, dids[0]);
}