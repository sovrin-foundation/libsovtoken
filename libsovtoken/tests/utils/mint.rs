extern crate rust_indy_sdk as indy;
extern crate serde_json;

use std::collections::HashMap;
use std::str::FromStr;

use indy::ErrorCode;
use sovtoken::utils::json_conversion::JsonDeserialize;
use sovtoken::logic::request::Request;
use sovtoken::logic::config::output_mint_config::MintRequest;
use utils;
use utils::did::NymRole;

pub fn mint_tokens(cfg: HashMap<String, u64>, pool_handle: i32, wallet_handle: i32, did_trustee: &str) -> Result<utils::parse_mint_response::ParseMintResponse, ErrorCode> {
    let trustees = utils::did::create_multiple_nym(wallet_handle, pool_handle, did_trustee, 3, NymRole::Trustee).unwrap();
    let mut dids = utils::did::did_str_from_trustees(&trustees);
    dids.insert(0, did_trustee);

    let vec_outputs:Vec<HashMap<&str, serde_json::Value>> = cfg.iter().map(|(pa, am)| {
        let mut map = HashMap::new();
        map.insert("paymentAddress", serde_json::Value::String(pa.clone()));
        map.insert("amount", serde_json::Value::Number(serde_json::Number::from_str(&am.to_string()).unwrap()));
        map
    }).collect();

    let json = serde_json::to_string(&vec_outputs).unwrap();

    let (mint_req, _) = indy::payments::Payment::build_mint_req(wallet_handle, did_trustee, &json)?;

    let mint_req = Request::<MintRequest>::multi_sign_request(wallet_handle, &mint_req, dids)?;

    let result = indy::ledger::Ledger::sign_and_submit_request(pool_handle, wallet_handle, &did_trustee, &mint_req)?;

    utils::parse_mint_response::ParseMintResponse::from_json(&result).map_err(|_| ErrorCode::CommonInvalidStructure)
}


