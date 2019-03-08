extern crate serde_json;
extern crate sovtoken;

use sovtoken::logic::parsers::parse_get_utxo_response::UTXO;
use utils::wallet::Wallet;

pub fn get_first_utxo_txo_for_payment_address(wallet: &Wallet, pool_handle: i32, did: &str, address: &str) -> String {
    let mut utxos = send_get_utxo_request(wallet, pool_handle, did, address);
    let utxo = utxos.remove(0);
    utxo.source
}

pub fn send_get_utxo_request(wallet: &Wallet, pool_handle: i32, did: &str, address: &str) -> Vec<UTXO> {
    let (req, method) = indy::payments::Payment::build_get_payment_sources_request(wallet.handle, Some(did), address).unwrap();
    let res = indy::ledger::Ledger::sign_and_submit_request(pool_handle, wallet.handle, did, &req).unwrap();
    let parsed_resp = indy::payments::Payment::parse_get_payment_sources_response(&method, &res).unwrap();
    serde_json::from_str(&parsed_resp).unwrap()
}