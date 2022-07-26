extern crate indyrs as indy;
extern crate serde_json;

use std::collections::HashMap;
use std::str::FromStr;

use indy::future::Future;

use sovtoken::ErrorCode;
use sovtoken::logic::config::output_mint_config::MintRequest;
use sovtoken::logic::request::Request;
use sovtoken::utils::json_conversion::JsonDeserialize;
use utils;

pub fn mint_tokens(cfg: HashMap<String, u64>, pool_handle: i32, wallet_handle: i32, trustee_dids: &Vec<&str>) -> Result<utils::parse_mint_response::ParseMintResponse, ErrorCode> {
    let vec_outputs: Vec<HashMap<&str, serde_json::Value>> = cfg.iter().map(|(pa, am)| {
        let mut map = HashMap::new();
        map.insert("recipient", serde_json::Value::String(pa.clone()));
        map.insert("amount", serde_json::Value::Number(serde_json::Number::from_str(&am.to_string()).unwrap()));
        map
    }).collect();

    let did = trustee_dids[0];

    let json = serde_json::to_string(&vec_outputs).unwrap();

    let (mint_req, _) = indy::payments::build_mint_req(wallet_handle, Some(did), &json, None).wait().unwrap();

    let mint_req = Request::<MintRequest>::multi_sign_request(wallet_handle, &mint_req, trustee_dids.to_vec()).unwrap();

    info!("Mint request: {}", mint_req);
    let result = indy::ledger::submit_request(pool_handle, &mint_req).wait().unwrap();
    info!("Mint result: {}", result);
    utils::parse_mint_response::ParseMintResponse::from_json(&result).map_err(|_| ErrorCode::CommonInvalidStructure)
}


