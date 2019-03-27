//!
//! this module contains functions that assist with std::ffi related behaviors
//! such as: converting const char * to str

use libc::c_char;
use std::ffi::{CString, CStr};
use ErrorCode;
use utils::json_conversion::JsonDeserialize;

/**
    utility method for converting *const c_char a &str.  Returns None
    if the input is invalid
*/
pub fn str_from_char_ptr<'a>(str_ptr: *const c_char) -> Option<&'a str> {
    if str_ptr.is_null() {
        return None;
    }

    let c_str: &CStr = unsafe { CStr::from_ptr(str_ptr)};
    let str_slice: &str = c_str.to_str().unwrap();
    return Some(str_slice);
}

/**
    utility method for converting *const c_char a String.  Returns None if the input is invalid
*/
pub fn string_from_char_ptr(str_ptr: *const c_char) -> Option<String> {
    str_from_char_ptr(str_ptr).map(|s| s.to_string())
}

/**
    method for converting String to CString with no error checking.
*/
pub fn cstring_from_str(string: String) -> CString {
    return CString::new(string).unwrap();
}

/**
    method for converting String to *const c_char
*/
pub fn c_pointer_from_string(string: String) -> *const c_char {
    return c_pointer_from_str(&string);
}

/**
    method for converting &str to *const c_char
*/
    pub fn c_pointer_from_str(string: &str) -> *const c_char {
    let cstring = CString::new(string).unwrap();
    return Box::new(cstring).into_raw();
}


/**
    Deserialize a char ptr to a struct.
*/
pub fn deserialize_from_char_ptr<'a, S: JsonDeserialize<'a>>(str_ptr: *const c_char) -> Result<S, ErrorCode> {
    let json_string = str_from_char_ptr(str_ptr).ok_or(ErrorCode::CommonInvalidStructure)?;
    println!("deserializing = {:?}",json_string);

    let result = S::from_json(json_string).map_err(|_| ErrorCode::CommonInvalidStructure);
    return result;
}

/**
    Creates a closure which calls a callback on `Ok`.

    Returns an `ErrorCode`
*/

macro_rules ! api_result_handler {
    ( <$value_type:ty>, $command_handle:ident, $cb:ident ) => {
        move |result: Result<$value_type, ErrorCode>| {
            let result_error_code = result.and(Ok(ErrorCode::Success)).ok_or_err();
            if let (Some(cb), Ok(value)) = ($cb, result) {
                cb($command_handle, result_error_code as i32, value);
            }
            return result_error_code as i32;
        }
    }
}

/**
    Checks that a callback is not null.  Copied from indy-sdk
*/
macro_rules! check_useful_c_callback {
    ($x:ident, $e:expr) => {
        let $x = match $x {
            Some($x) => $x,
            None => return $e
        };
    }
}

/**
    Checks that a pointer is not null.  Copied from indy-sdk
*/
macro_rules! check_useful_c_ptr {
    ($ptr:ident, $err1:expr) => {
        if $ptr.is_null() {
            return $err1
        }
    }
}


#[cfg(test)]
mod ffi_support_tests {

    use std::ptr;
    use std::ffi::CString;
    use serde_json::Value;
    use utils::general::ResultExtension;
    use utils::ffi_support::{str_from_char_ptr, cstring_from_str, deserialize_from_char_ptr, c_pointer_from_string, string_from_char_ptr};
    use ErrorCode;

    static VALID_DUMMY_JSON: &'static str = r#"{"field1":"data"}"#;

    // this test makes sure our conversion from char * to str works by returning a valid
    // str object
    #[test]
    fn convert_char_ptr_to_str_test() {
        let json_str = CString::new(VALID_DUMMY_JSON).unwrap();
        let json_str_ptr = json_str.as_ptr();

        let json: &str = str_from_char_ptr(json_str_ptr).unwrap();

        assert_eq!(VALID_DUMMY_JSON, json, "str_from_char_ptr didn't convert the inputs");
    }

    // this test makes sure our conversion from null char * to str correctly returns None
    #[test]
    fn convert_null_char_ptr_to_str_test() {
        let json: Option<&str> = str_from_char_ptr(ptr::null());

        assert_eq!(None, json, "str_from_char_ptr didn't return None as expected");
    }

    #[test]
    fn test_c_pointer_from_string() {
        let string = String::from("test1234");
        let pointer = c_pointer_from_string(string.clone());
        let string2 = string_from_char_ptr(pointer).unwrap();
        assert_eq!(string2, string);
    }

    #[test]
    fn deserialize_error_with_null_pointer() {
        let pointer = ptr::null();
        let error = deserialize_from_char_ptr::<Value>(pointer).unwrap_err();
        assert_eq!(error, ErrorCode::CommonInvalidStructure);
    }

    #[test]
    fn deserialize_error_with_bad_json() {
        let bad_json = cstring_from_str(String::from(r#"{fein:"aewf",}"#));
        let pointer = bad_json.as_ptr();
        let error = deserialize_from_char_ptr::<Value>(pointer).unwrap_err();
        assert_eq!(error, ErrorCode::CommonInvalidStructure);
    }

    #[test]
    fn deserialize_good_json() {
        let good_json = cstring_from_str(String::from(r#"{"a": 1}"#));
        let pointer = good_json.as_ptr();
        let json_value = deserialize_from_char_ptr::<Value>(pointer).unwrap();
        assert_eq!(json_value.get("a").unwrap(), 1);
    }

    #[test]
    fn api_result_handler_callback_on_ok() {
        static mut CALLBACK_CALLED: bool = false;
        extern fn callback(_ch: i32, _ec: i32, val: u32) {
            assert_eq!(val, 2);
            unsafe { CALLBACK_CALLED = true }
        }

        let ch = 1242;
        let cb = Some(callback);
        let result_handler = api_result_handler!(<u32>, ch, cb);
        let result = result_handler(Ok(2));
        assert_eq!(result, ErrorCode::Success as i32);
        assert!(unsafe { CALLBACK_CALLED });

        unsafe { CALLBACK_CALLED = false }
        let result = result_handler(Err(ErrorCode::CommonInvalidStructure));
        assert_eq!(result, ErrorCode::CommonInvalidStructure as i32);
        assert!(! unsafe { CALLBACK_CALLED });
    }

}
