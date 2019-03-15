//!
//!
use serde::Serialize;
use serde_json;
use std::ffi::CString;
use libc::c_char;
use utils::{IndyHandle, ErrorCode};

use utils::ffi_support::{cstring_from_str, c_pointer_from_string};
use utils::random::rand_req_id;
use utils::json_conversion::JsonSerialize;
use utils::constants::general::PROTOCOL_VERSION;
use logic::type_aliases::{ProtocolVersion, ReqId};

use indy_sys::ledger::indy_multi_sign_request;
use utils::callbacks::ClosureHandler;

pub const DEFAULT_LIBSOVTOKEN_DID: &'static str = "LibsovtokenDid11111111";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request<T>
    where T: Serialize
{
    pub operation: T,
    pub req_id: ReqId,
    pub protocol_version: ProtocolVersion,
    pub identifier : String,
}

impl<T> Request<T> 
    where T: Serialize
{
    pub fn new(operation: T, identifier : Option<String>) -> Self {
        let req_id = rand_req_id();
        return Request {
            operation,
            protocol_version: PROTOCOL_VERSION,
            req_id,
            identifier: identifier.unwrap_or(DEFAULT_LIBSOVTOKEN_DID.to_string())
        }
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
        let signed_req: String = req.to_string();
        for did in dids {
            // TODO:  allocating a receiver we don't use.  change how command handle and cb are allocated.
            let (_receiver, cmd_handle, cb) = ClosureHandler::cb_ec_string();
            ErrorCode::from(
                unsafe
                {
                    indy_multi_sign_request(cmd_handle, wallet_handle, did.as_ptr() as *const _, signed_req.as_ptr() as *const _, cb)
                }
            );
        }
        Ok(signed_req)
    }
}