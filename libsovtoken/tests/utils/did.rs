extern crate rust_indy_sdk as indy;

use indy::ErrorCode;

pub fn add_new_trustee_did(wallet_handle: i32, did_trustee: &str, pool_handle: i32) -> Result<(String, String), ErrorCode> {
    let (did, verkey) = indy::did::Did::new(wallet_handle, "{}").unwrap();
    let req_nym = indy::ledger::Ledger::build_nym_request(did_trustee, &did, Some(&verkey), None, Some("TRUSTEE"))?;
    indy::ledger::Ledger::sign_and_submit_request(pool_handle, wallet_handle, &did_trustee, &req_nym)?;
    Ok((did, verkey))
}

pub fn add_multiple_trustee_dids(num_trustees: u8, wallet_handle: i32, pool_handle: i32) -> Result<Vec<(String, String)>, ErrorCode> {
    let mut v: Vec<(String, String)> = Vec::new();
    let (did_trustee, vk) = indy::did::Did::new(wallet_handle, &json!({"seed":"000000000000000000000000Trustee1"}).to_string()).unwrap();
    v.push((did_trustee.clone(), vk.clone()));
    for _ in 1..num_trustees {
        let (did, verkey) = indy::did::Did::new(wallet_handle, "{}").unwrap();
        let req_nym = indy::ledger::Ledger::build_nym_request(&did_trustee, &did, Some(&verkey), None, Some("TRUSTEE"))?;
        indy::ledger::Ledger::sign_and_submit_request(pool_handle, wallet_handle, &did_trustee, &req_nym)?;
        v.push((did, verkey));
    }
    Ok(v)
}