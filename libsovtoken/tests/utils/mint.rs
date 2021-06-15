extern crate indyrs as indy;
extern crate serde_json;
extern crate indy_sys;

use std::collections::HashMap;
use std::str::FromStr;


use sovtoken::ErrorCode;
use sovtoken::utils::json_conversion::JsonDeserialize;
use sovtoken::logic::request::Request;
use sovtoken::logic::config::output_mint_config::MintRequest;
use utils;

use indy::future::Future;

pub fn mint_tokens(cfg: HashMap<String, u64>, pool_handle: i32, wallet_handle: indy_sys::WalletHandle, trustee_dids: &Vec<&str>) -> Result<utils::parse_mint_response::ParseMintResponse, ErrorCode> {
    let vec_outputs:Vec<HashMap<&str, serde_json::Value>> = cfg.iter().map(|(pa, am)| {
        let mut map = HashMap::new();
        map.insert("recipient", serde_json::Value::String(pa.clone()));
        map.insert("amount", serde_json::Value::Number(serde_json::Number::from_str(&am.to_string()).unwrap()));
        map
    }).collect();

    let did = trustee_dids[0];

    let json = serde_json::to_string(&vec_outputs).unwrap();

    let (mint_req, _) = indy::payments::build_mint_req(wallet_handle, Some(did), &json, None).wait().unwrap();

    let mint_req = Request::<MintRequest>::multi_sign_request(wallet_handle, &mint_req, trustee_dids.to_vec()).unwrap();

    let result = indy::ledger::submit_request(pool_handle, &mint_req).wait().unwrap();

    utils::parse_mint_response::ParseMintResponse::from_json(&result).map_err(|_| ErrorCode::CommonInvalidStructure)
}


