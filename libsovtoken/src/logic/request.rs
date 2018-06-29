//!
//!
use serde::Serialize;
use serde_json;
use std::ffi::CString;
use libc::c_char;

use utils::ffi_support::{cstring_from_str, c_pointer_from_string};
use utils::random::rand_req_id;
use utils::json_conversion::JsonSerialize;
use utils::constants::general::PROTOCOL_VERSION;
use logic::types::{ProtocolVersion, ReqId};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request<T>
    where T: Serialize
{
    pub operation: T,
    pub req_id: ReqId,
    pub protocol_version: ProtocolVersion,
    pub identifier : Option<String>,
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
            identifier
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
}