use std::ffi::CString;
use utils::ffi_support::cstring_from_str;
use serde::Serialize;
use utils::json_conversion::JsonSerialize;
use std::collections::HashMap;
// use std::time::Instant;


// static TIMER : Instant = Instant::now();

// fn generate_elapsed_counter() -> u64 {
//     let elapsed = TIMER.elapsed();
//     return elapsed.as_secs() * 1_000_000 + (elapsed.subsec_nanos() / 1000) as u64;
// }

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request<T>
    where T: Serialize
{
    pub operation: T,
    pub identifier: String,
    pub req_id: u64,
    pub protocol_version: u32,
    pub signatures: HashMap<String, String>
}

impl<T> Request<T> 
    where T: Serialize
{
    pub fn new(operation: T, did: String) -> Self {
        return Request {
            operation: operation,
            identifier: did,
            // req_id: generate_elapsed_counter(),
            req_id: 31631531513,
            protocol_version: 1,
            signatures: HashMap::new()
        }
    }

    pub fn sign(&mut self, did: &str, key: &str) -> Result<(),()>  {
        self.signatures.insert(String::from(did), format!("000{}", key));
        return Ok(())
    }

    pub fn serialize_to_cstring(&self) -> CString {
        let serialized = JsonSerialize::to_json(&self).unwrap();
        return cstring_from_str(serialized);
    }
}