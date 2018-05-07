
#[allow(unused_variables)]
#[allow(dead_code)]


extern crate sovtoken;

use std::ptr;
use std::ffi::CString;

use sovtoken::utils::ffi_support::str_from_char_ptr;

static VALID_DUMMY_JSON: &'static str = r#"{"field1":"data"}"#;    // do not pretty this up with spaces or tests fail


// this test makes sure our conversion from char * to str works by returning a valid
// str object
#[test]
fn convert_char_ptr_to_str_test() {
    let json_str = CString::new(VALID_DUMMY_JSON).unwrap();
    let json_str_ptr = json_str.as_ptr();

    let json : &str = str_from_char_ptr(json_str_ptr).unwrap();

    assert_eq!(VALID_DUMMY_JSON, json, "str_from_char_ptr didn't convert the inputs");
}

// this test makes sure our conversion from null char * to str correctly returns None
#[test]
fn convert_null_char_ptr_to_str_test() {

    let json : Option<&str> = str_from_char_ptr(ptr::null());

    assert_eq!(None, json, "str_from_char_ptr didn't return None as expected");
}