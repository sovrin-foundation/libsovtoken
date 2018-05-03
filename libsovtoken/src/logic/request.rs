use libc::c_char;
use utils::ffi_support::char_ptr_from_str;
use serde::Serialize;
use utils::json_conversion::JsonSerialize;

pub trait Request: Serialize + Sized {
    fn sign(&mut self, key: &str) -> bool;
    fn serialize_to_c_char(&self) -> *const c_char {
        let serialized = JsonSerialize::to_json(self).unwrap();
        return char_ptr_from_str(&serialized).unwrap();
    }
}