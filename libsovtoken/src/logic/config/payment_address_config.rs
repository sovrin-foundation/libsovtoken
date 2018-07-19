//!  defines structure and implementation for PaymentAddressConfig which is used
//! for generating payment addresses

#![warn(unused_imports)]
#[allow(unused_imports)]

use serde_json;
use std::ffi::CString;
use utils::ffi_support::cstring_from_str;
use utils::json_conversion::JsonSerialize;

/**
     The config structure maps to the config json structure
     used to serialize input via serde and use the data in our logic

     The seed should be 32 bytes, thats what libsodium requires. Seed can be optional, in that case libsodium generates a random 32 byte seed

*/
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentAddressConfig {
    pub seed : String,
}

impl PaymentAddressConfig {

    /**
        converts PaymentAddressConfig json encoded string (CString)
    */
    pub fn serialize_to_cstring(&self) -> Result<CString, serde_json::Error> {
        let serialized = JsonSerialize::to_json(&self)?;
        return Ok(cstring_from_str(serialized));
    }
}
