extern crate rust_indy_sdk as indy;
extern crate serde_json;

use std::collections::HashMap;
use indy::ErrorCode;
use utils;
use sovtoken::utils::json_conversion::JsonDeserialize;
use std::str::FromStr;

pub fn mint_tokens(cfg: HashMap<String, u64>, pool_handle: i32, wallet_handle: i32, did_trustee: &str) -> Result<utils::parse_mint_response::ParseMintResponse, ErrorCode> {

    let (did, verkey) = utils::did::add_new_trustee_did(wallet_handle, did_trustee, pool_handle)?;
    let (did_2, verkey_2) = utils::did::add_new_trustee_did(wallet_handle, did_trustee, pool_handle)?;
    let (did_3, verkey_3) = utils::did::add_new_trustee_did(wallet_handle, did_trustee, pool_handle)?;

    let vec_outputs:Vec<HashMap<&str, serde_json::Value>> = cfg.iter().map(|(pa, am)| {
        let mut map = HashMap::new();
        map.insert("paymentAddress", serde_json::Value::String(pa.clone()));
        map.insert("amount", serde_json::Value::Number(serde_json::Number::from_str(&am.to_string()).unwrap()));
        map
    }).collect();
    let json = serde_json::to_string(&vec_outputs).unwrap();

    let (mint_req, _) = indy::payments::Payment::build_mint_req(wallet_handle, did_trustee, &json)?;

    let mint_req = indy::ledger::Ledger::multi_sign_request(wallet_handle, did_trustee, &mint_req)?;
    let mint_req = indy::ledger::Ledger::multi_sign_request(wallet_handle, &did, &mint_req)?;
    let mint_req = indy::ledger::Ledger::multi_sign_request(wallet_handle, &did_2, &mint_req)?;
    let mint_req = indy::ledger::Ledger::multi_sign_request(wallet_handle, &did_3, &mint_req)?;

    let result = indy::ledger::Ledger::sign_and_submit_request(pool_handle, wallet_handle, &did_trustee, &mint_req)?;

    utils::parse_mint_response::ParseMintResponse::from_json(&result).map_err(|_| ErrorCode::CommonInvalidStructure)
}


