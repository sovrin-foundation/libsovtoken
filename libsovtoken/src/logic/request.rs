//!
//!
use libc::c_char;
use serde::Serialize;
use serde_json;
use std::ffi::CString;

use logic::type_aliases::{ProtocolVersion, ReqId};
use {IndyHandle, ErrorCode};
use utils::constants::general::PROTOCOL_VERSION;
use utils::ffi_support::{cstring_from_str, c_pointer_from_string};
use utils::json_conversion::JsonSerialize;
use utils::random::rand_req_id;

use logic::indy_sdk_api::ledger;

pub const DEFAULT_LIBSOVTOKEN_DID: &'static str = "LibsovtokenDid11111111";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request<T>
    where T: Serialize
{
    pub operation: T,
    pub req_id: ReqId,
    pub protocol_version: ProtocolVersion,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier : Option<String>,
}

impl<T> Request<T>
    where T: Serialize
{
    pub fn new(operation: T, identifier: Option<String>) -> Self {
        let req_id = rand_req_id();
        return Request {
            operation,
            protocol_version: PROTOCOL_VERSION,
            req_id,
            identifier
        };
    }

    pub fn serialize_to_cstring(&self) -> Result<CString, serde_json::Error> {
        return self.serialize_to_string().map_err(map_err_err!())
            .map(|string| cstring_from_str(string));
    }

    pub fn serialize_to_string(&self) -> Result<String, serde_json::Error> {
        return JsonSerialize::to_json(&self).map_err(map_err_err!());
    }

    pub fn serialize_to_pointer(&self) -> Result<*const c_char, serde_json::Error> {
        return self.serialize_to_string()
            .map(|string| c_pointer_from_string(string));
    }

    pub fn multi_sign_request(wallet_handle: IndyHandle, req: &str, dids: Vec<&str>) -> Result<String, ErrorCode> {
        let mut signed_req: String = req.to_string();

        for did in dids {
            signed_req = ledger::Ledger::multi_sign_request(wallet_handle, did, &signed_req)?;
        }
        Ok(signed_req)
    }
}