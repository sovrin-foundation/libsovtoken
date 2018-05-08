//
// this module contains functions that assist with std::ffi related behaviors
// such as: converting const char * to str
//
#![allow(unused_variables)]
#![allow(unused_imports)]
#[warn(unused_imports)]

use std::ffi::{CString, CStr};
use std::str::Utf8Error;
use libc::c_char;

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
    match str_from_char_ptr(str_ptr) {
        Some(s) => return Some(s.to_string()),
        None => return None,
    };
}



pub fn cstring_from_str(string: String) -> CString {
    return CString::new(string).unwrap();
}

/**
    Transforms a c_string into a &str or returns an error_code.

    ```
        let c_string = std::ffi::CString::from("My test str");
        let my_str: &str = unpack_c_string_or_error!(c_string, ErrorCode::CommonInvalidParam2);
        // assert_eq!(my_str, "My test str");
    ```
*/
macro_rules! unpack_c_string_or_error {
    ( $c_string:ident, $error_code:expr ) => {
        match $crate::utils::ffi_support::str_from_char_ptr($c_string) {
            Some(s) => s,
            None => return $error_code,
        }
    }
}

#[cfg(test)]
mod ffi_support_tests {
    use std::ptr;
    use std::ffi;
    use std::ffi::CString;
    use libc::c_char;

    use utils::ffi_support::str_from_char_ptr;

    static VALID_DUMMY_JSON: &'static str = r#"{"field1":"data"}"#;

    // do not pretty this up with spaces or tests fail
    fn unpack_c_string_wrapper(c_str: *const c_char) -> u32 {
        unpack_c_string_or_error!(c_str, 0);
        return 1;
    }

    #[test]
    fn unpack_c_string_or_error_error_on_null() {
        let status = self::unpack_c_string_wrapper(ptr::null());
        assert_eq!(status, 0);
    }

    #[test]
    fn unpack_c_string_success() {
        let c_string = ffi::CString::new("My test str").unwrap().as_ptr();
        let status = unpack_c_string_wrapper(c_string);
        assert_eq!(status, 1);
    }

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
}