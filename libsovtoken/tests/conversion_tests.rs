
#[allow(unused_variables)]
#[allow(dead_code)]


extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

extern crate sovtoken;

use std::ptr;
use std::ffi::CString;

use serde::{Serialize, Deserialize};
use sovtoken::utils::json_conversion::JsonDeserialize;
use sovtoken::utils::ffi_support::str_from_char_ptr;

// helper structures and data
#[derive(Debug, Serialize, Deserialize)]
pub struct DummyJsonStruct {
    pub field1: String,
}

static VALID_DUMMY_JSON: &'static str = r#"{ "field1" : "data" }"#;

// This test creates valid json and calls the utils method for deseralizing json into
// a test type defined above
#[test]
fn convert_json_str_to_type_test() {

    let result: DummyJsonStruct = DummyJsonStruct::from_json(VALID_DUMMY_JSON).unwrap();

    assert_eq!("data", result.field1, "decoding default json failed");
}

// this test makes sure our conversion from char * to str works.
#[test]
fn convert_char_ptr_to_str_test() {
    let json_str = CString::new(VALID_DUMMY_JSON).unwrap();
    let json_str_ptr = json_str.as_ptr();

    let json : &str = str_from_char_ptr(json_str_ptr).unwrap();

    assert_eq!(VALID_DUMMY_JSON, json, "str_from_char_ptr didn't convert the inputs");
}

// this test makes sure our conversion from char * to str works.
#[test]
fn convert_null_char_ptr_to_str_test() {

    let json : Option<&str> = str_from_char_ptr(ptr::null());

    assert_eq!(None, json, "str_from_char_ptr didn't return None as expected");
}