extern crate indy;

use sovtoken::logic::config::set_fees_config::SetFees;
use sovtoken::logic::request::Request;
use sovtoken::utils::constants::general::PAYMENT_METHOD_NAME;
use utils::wallet::Wallet;

pub fn set_fees(pool_handle: i32, wallet_handle: i32, payment_method: &str, fees: &str, dids: &Vec<&str>) -> String {
    let set_fees_req = indy::payments::Payment::build_set_txn_fees_req(wallet_handle, Some(dids[0]), payment_method, &fees).unwrap();

    let set_fees_req = Request::<SetFees>::multi_sign_request(wallet_handle, &set_fees_req, dids.to_vec()).unwrap();

    indy::ledger::Ledger::submit_request(pool_handle, &set_fees_req).unwrap()
}

pub fn get_fees(wallet: &Wallet, pool_handle: i32, did: &str) -> String {
    let get_fees_req = indy::payments::Payment::build_get_txn_fees_req(
        wallet.handle,
        Some(did),
        PAYMENT_METHOD_NAME
    ).unwrap();
    let result = indy::ledger::Ledger::submit_request(pool_handle, &get_fees_req).unwrap();
    indy::payments::Payment::parse_get_txn_fees_response(PAYMENT_METHOD_NAME, &result).unwrap()
}