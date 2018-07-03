extern crate rust_indy_sdk as indy;

use indy::ErrorCode;

pub fn add_new_trustee_did(wallet_handle: i32, did_trustee: &str, pool_handle: i32) -> Result<(String, String), ErrorCode> {
    let (did, verkey) = indy::did::Did::new(wallet_handle, "{}").unwrap();
    let req_nym = indy::ledger::Ledger::build_nym_request(did_trustee, &did, Some(&verkey), None, Some("TRUSTEE"))?;
    indy::ledger::Ledger::sign_and_submit_request(pool_handle, wallet_handle, &did_trustee, &req_nym)?;
    Ok((did, verkey))
}