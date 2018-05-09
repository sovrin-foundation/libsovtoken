use std::ffi::CString;
use utils::ffi_support::cstring_from_str;
use serde::Serialize;
use serde_json;
use utils::json_conversion::JsonSerialize;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request<T>
    where T: Serialize
{
    pub operation: T,
    pub protocol_version: u32,
}

impl<T> Request<T> 
    where T: Serialize
{
    pub fn new(operation: T) -> Self {
        return Request {
            operation,
            protocol_version: 1,
        }
    }

    pub fn serialize_to_cstring(&self) -> Result<CString, serde_json::Error> {
        let serialized = JsonSerialize::to_json(&self)?;
        return Ok(cstring_from_str(serialized));
    }
}