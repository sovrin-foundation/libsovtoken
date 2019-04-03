extern crate indyrs as indy;

use sovtoken::logic::config::set_fees_config::SetFees;
use sovtoken::logic::request::Request;
use sovtoken::utils::constants::general::PAYMENT_METHOD_NAME;
use utils::wallet::Wallet;

use indy::future::Future;

pub fn set_fees(pool_handle: i32, wallet_handle: i32, payment_method: &str, fees: &str, dids: &Vec<&str>, submitter_did: Option<&str>) -> String {
    let set_fees_req = indy::payments::build_set_txn_fees_req(wallet_handle, submitter_did, payment_method, &fees).wait().unwrap();

    let set_fees_req = Request::<SetFees>::multi_sign_request(wallet_handle, &set_fees_req, dids.to_vec()).unwrap();

    indy::ledger::submit_request(pool_handle, &set_fees_req).wait().unwrap()
}

pub fn get_fees(wallet: &Wallet, pool_handle: i32, submitter_did: Option<&str>) -> String {
    let get_fees_req = indy::payments::build_get_txn_fees_req(
        wallet.handle,
        submitter_did,
        PAYMENT_METHOD_NAME
    ).wait().unwrap();
    let result = indy::ledger::submit_request(pool_handle, &get_fees_req).wait().unwrap();
    indy::payments::parse_get_txn_fees_response(PAYMENT_METHOD_NAME, &result).wait().unwrap()
}