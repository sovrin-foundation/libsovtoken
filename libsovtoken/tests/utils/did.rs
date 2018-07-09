extern crate rust_indy_sdk as indy;

use indy::ErrorCode;

type DidAndVerKey = (String, String);

pub fn add_new_trustee_did(wallet_handle: i32, did_trustee: &str, pool_handle: i32) -> Result<DidAndVerKey, ErrorCode> {
    let (did, verkey) = indy::did::Did::new(wallet_handle, "{}").unwrap();
    let req_nym = indy::ledger::Ledger::build_nym_request(did_trustee, &did, Some(&verkey), None, Some("TRUSTEE"))?;
    indy::ledger::Ledger::sign_and_submit_request(pool_handle, wallet_handle, &did_trustee, &req_nym)?;
    Ok((did, verkey))
}

pub fn add_multiple_trustee_dids(num_trustees: u8, wallet_handle: i32, pool_handle: i32, did_trustee: &str) -> Result<Vec<DidAndVerKey>, ErrorCode> {
    let mut v: Vec<(String, String)> = Vec::new();
    for _ in 0..num_trustees {
        let new_did = add_new_trustee_did(wallet_handle, did_trustee, pool_handle)?;
        v.push(new_did);
    }
    Ok(v)
}

/**
Create and store the initial dids of trustees.

Includes the initial trustee.
*/
pub fn initial_trustees(num_trustees: u8, wallet_handle: i32, pool_handle: i32) -> Result<Vec<DidAndVerKey>, ErrorCode> {
    let first = initial_trustee(wallet_handle);

    let mut trustees = add_multiple_trustee_dids(
        num_trustees - 1,
        wallet_handle,
        pool_handle,
        &first.0
    )?;
    trustees.insert(0, first);

    Ok(trustees)
}

/**
Store the did of the intial trustee
*/
pub fn initial_trustee(wallet_handle: i32) -> DidAndVerKey {
    let first_json_seed = json!({
        "seed":"000000000000000000000000Trustee1"
    }).to_string();

    indy::did::Did::new(wallet_handle, &first_json_seed).unwrap()
}

/**
Discard the verkey and return the did from a `Vec<DidAndVerKey`.
*/
pub fn did_str_from_trustees<'a>(trustees: &'a Vec<DidAndVerKey>) -> Vec<&'a str> {
    trustees
        .iter()
        .map(|(ref did, _)| did.as_str())
        .collect()
}