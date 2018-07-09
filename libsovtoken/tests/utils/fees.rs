extern crate rust_indy_sdk as indy;

use sovtoken::logic::config::set_fees_config::SetFees;
use sovtoken::logic::request::Request;

pub fn set_fees(pool_handle: i32, wallet_handle: i32, payment_method: &str, fees: &str, dids: &Vec<&str>) -> String {
    let set_fees_req = indy::payments::Payment::build_set_txn_fees_req(wallet_handle, dids[0], payment_method, &fees).unwrap();

    let set_fees_req = Request::<SetFees>::multi_sign_request(wallet_handle, &set_fees_req, dids.to_vec()).unwrap();

    indy::ledger::Ledger::submit_request(pool_handle, &set_fees_req).unwrap()
}