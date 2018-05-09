use std::ffi::CString;
use utils::ffi_support::cstring_from_str;
use serde::Serialize;
use serde_json;
use utils::json_conversion::JsonSerialize;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

static COUNTER : AtomicUsize = ATOMIC_USIZE_INIT;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request<T>
    where T: Serialize
{
    pub operation: T,
    pub identifier: String,
    pub req_id: u64,
    pub protocol_version: u32,
    pub signatures: HashMap<String, String>,
}
impl<T> Request<T> 
    where T: Serialize
{
    pub fn new(operation: T, did: String) -> Self {
        return Request {
            operation,
            identifier: did,
            req_id: COUNTER.fetch_add(1, Ordering::SeqCst) as u64,
            protocol_version: 1,
            signatures: HashMap::new(),
        }
    }

    pub fn sign(&mut self, did: &str, key: &str) -> Result<(), ()>  {
        self.signatures.insert(String::from(did), format!("000{}", key));
        return Ok(())
    }

    pub fn serialize_to_cstring(&self) -> Result<CString, serde_json::Error> {
        let serialized = JsonSerialize::to_json(&self)?;
        return Ok(cstring_from_str(serialized));
    }
}