
#[allow(unused_variables)]
#[allow(dead_code)]


extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

extern crate sovtoken;

use std::ptr;
use std::ffi::CString;

use serde::{Serialize, Deserialize};
use sovtoken::utils::json_conversion::{JsonDeserialize, JsonSerialize};
use sovtoken::utils::ffi_support::str_from_char_ptr;

// helper structures and data
#[derive(Debug, Serialize, Deserialize)]
pub struct DummyJsonStruct {
    pub field1: String,
}

static FIELD1_VALUE: &'static str = "data";
static VALID_DUMMY_JSON: &'static str = r#"{"field1":"data"}"#;    // do not pretty this up with spaces or tests fail

// This test creates valid json and calls the utils method for deseralizing json into
// a test type defined above
#[test]
fn convert_json_str_to_type_test() {

    let result: DummyJsonStruct = DummyJsonStruct::from_json(VALID_DUMMY_JSON).unwrap();

    assert_eq!(FIELD1_VALUE, result.field1, "decoding default json failed");
}

// this test creates an oject with the Serialize attribute and expects to get a propertly
// formatted json str returned
#[test]
fn convert_obj_to_json_str_test() {

    let field1: String = FIELD1_VALUE.to_string();
    let instance: DummyJsonStruct = DummyJsonStruct { field1, };

    let result: String = instance.to_json().unwrap();

    assert_ne!(true, result.is_empty(), "convert_obj_to_json_str_test failed to serialize to json string");
    assert_eq!(VALID_DUMMY_JSON, result, "convert_obj_to_json_str_test failed to serialized json did not match result");
}


// this test covers converting an object to json and the resulting json back into an object.  the
// two objects should contain same values.  they will not be the same object however
#[test]
fn convert_obj_to_json_and_back_to_obj_test() {

    let field1: String = FIELD1_VALUE.to_string();
    let instance: DummyJsonStruct = DummyJsonStruct { field1, };

    let result_str: String = instance.to_json().unwrap();

    let result: DummyJsonStruct = DummyJsonStruct::from_json(result_str.as_str()).unwrap();

    assert_eq!(FIELD1_VALUE, result.field1, "decoding default json failed");
    assert_eq!(instance.field1, result.field1, "comparison of objects failed");
}

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