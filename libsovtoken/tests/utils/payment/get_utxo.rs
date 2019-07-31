extern crate indyrs as indy;
extern crate serde_json;
extern crate sovtoken;

use indy::future::Future;

use sovtoken::logic::parsers::parse_get_utxo_response::UTXO;
use utils::wallet::Wallet;

pub fn get_first_utxo_txo_for_payment_address(wallet: &Wallet, pool_handle: i32, did: &str, address: &str) -> String {
    let (mut utxos, _) = send_get_utxo_request(wallet, pool_handle, did, address, None);
    let utxo = utxos.remove(0);
    utxo.source
}

pub fn send_get_utxo_request(wallet: &Wallet, pool_handle: i32, did: &str, address: &str, from: Option<i64>) -> (Vec<UTXO>, Option<i64>) {
    let (req, method) = indy::payments::build_get_payment_sources_with_from_request(wallet.handle, Some(did), address, from).wait().unwrap();
    let res = indy::ledger::sign_and_submit_request(pool_handle, wallet.handle, did, &req).wait().unwrap();
    let (parsed_resp, next) = indy::payments::parse_get_payment_sources_with_from_response(&method, &res).wait().unwrap();
    (serde_json::from_str(&parsed_resp).unwrap(), next)
}