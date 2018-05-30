use std::ffi::CString;
use utils::ffi_support::cstring_from_str;
use utils::random::rand_req_id;
use serde::Serialize;
use serde_json;
use utils::json_conversion::JsonSerialize;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request<T>
    where T: Serialize
{
    pub operation: T,
    pub req_id: u32,
    pub protocol_version: u32,
    pub identifier : String,
}

impl<T> Request<T> 
    where T: Serialize
{
    pub fn new(operation: T, identifier : String) -> Self {
        let req_id = rand_req_id();
        return Request {
            operation,
            protocol_version: 1,
            req_id,
            identifier
        }
    }

    pub fn serialize_to_cstring(&self) -> Result<CString, serde_json::Error> {
        let serialized = JsonSerialize::to_json(&self)?;
        return Ok(cstring_from_str(serialized));
    }
}