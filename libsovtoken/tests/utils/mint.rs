extern crate rust_indy_sdk as indy;
extern crate serde_json;

use std::collections::HashMap;
use indy::ErrorCode;
use utils;
use sovtoken::utils::json_conversion::JsonDeserialize;
use sovtoken::logic::request::Request;
use std::str::FromStr;
use sovtoken::logic::config::output_mint_config::MintRequest;

pub fn mint_tokens(cfg: HashMap<String, u64>, pool_handle: i32, wallet_handle: i32, did_trustee: &str) -> Result<utils::parse_mint_response::ParseMintResponse, ErrorCode> {

    let (did, _) = utils::did::add_new_trustee_did(wallet_handle, did_trustee, pool_handle)?;
    let (did_2, _) = utils::did::add_new_trustee_did(wallet_handle, did_trustee, pool_handle)?;
    let (did_3, _) = utils::did::add_new_trustee_did(wallet_handle, did_trustee, pool_handle)?;

    let vec_outputs:Vec<HashMap<&str, serde_json::Value>> = cfg.iter().map(|(pa, am)| {
        let mut map = HashMap::new();
        map.insert("paymentAddress", serde_json::Value::String(pa.clone()));
        map.insert("amount", serde_json::Value::Number(serde_json::Number::from_str(&am.to_string()).unwrap()));
        map
    }).collect();
    let json = serde_json::to_string(&vec_outputs).unwrap();

    let (mint_req, _) = indy::payments::Payment::build_mint_req(wallet_handle, did_trustee, &json)?;

    let mint_req = Request::<MintRequest>::multi_sign_request(wallet_handle, &mint_req, vec![&did_trustee, &did, &did_2, &did_3])?;

    let result = indy::ledger::Ledger::sign_and_submit_request(pool_handle, wallet_handle, &did_trustee, &mint_req)?;

    utils::parse_mint_response::ParseMintResponse::from_json(&result).map_err(|_| ErrorCode::CommonInvalidStructure)
}


