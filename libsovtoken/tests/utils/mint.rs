extern crate serde_json;

use std::collections::HashMap;
use std::str::FromStr;

use sovtoken::utils::ErrorCode;
use sovtoken::utils::json_conversion::JsonDeserialize;
use sovtoken::logic::request::Request;
use sovtoken::logic::config::output_mint_config::MintRequest;
use utils;
use sovtoken::utils::callbacks::ClosureHandler;

pub fn mint_tokens(cfg: HashMap<String, u64>, pool_handle: i32, wallet_handle: i32, trustee_dids: &Vec<&str>) -> Result<utils::parse_mint_response::ParseMintResponse, ErrorCode> {
    let vec_outputs:Vec<HashMap<&str, serde_json::Value>> = cfg.iter().map(|(pa, am)| {
        let mut map = HashMap::new();
        map.insert("recipient", serde_json::Value::String(pa.clone()));
        map.insert("amount", serde_json::Value::Number(serde_json::Number::from_str(&am.to_string()).unwrap()));
        map
    }).collect();

    let did = trustee_dids[0];

    let json = serde_json::to_string(&vec_outputs).unwrap();

    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let submitter_did_str = opt_c_str!(submitter_did);
    let outputs_json = c_str!(outputs_json);
    let extra_str = opt_c_str!(extra);

    let err = ErrorCode::from(unsafe { indy_sys::indy_build_mint_req(command_handle, wallet_handle, opt_c_ptr!(submitter_did, submitter_did_str), outputs_json.as_ptr(), opt_c_ptr!(extra, extra_str), cb) });

    err.try_err()?;

    let (err, val, val2) = receiver.recv()?;

    err.try_err()?;

    let (mint_req, _) = Ok((val, val2))?;

    let mint_req = Request::<MintRequest>::multi_sign_request(wallet_handle, &mint_req, trustee_dids.to_vec())?;

    //let result = indy::ledger::Ledger::sign_and_submit_request(pool_handle, wallet_handle, did, &mint_req)?;
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let submitter_did = c_str!(did);
    let request_json = c_str!(&mint_req);

    let result = unsafe {
        indy_sys::indy_sign_and_submit_request(command_handle,
                                               pool_handle,
                                               wallet_handle,
                                               submitter_did.as_ptr(),
                                               request_json.as_ptr(),
                                               cb)
    };

    utils::parse_mint_response::ParseMintResponse::from_json(&result).map_err(|_| ErrorCode::CommonInvalidStructure)
}


