use libc::c_char;
use std::ffi::CString;
use utils::ffi_support::cstring_from_str;
use serde::Serialize;
use utils::json_conversion::JsonSerialize;

pub trait Request: Serialize + Sized {
    fn sign(&mut self, key: &str) -> bool;

    fn serialize_to_cstring(&self) -> CString {
        let serialized = JsonSerialize::to_json(&self).unwrap();
        return cstring_from_str(serialized);
    }
}