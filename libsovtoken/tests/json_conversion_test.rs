
#[allow(unused_variables)]
#[allow(dead_code)]


extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

extern crate sovtoken;

use serde::{Serialize, Deserialize};
use sovtoken::utils::json_conversion::JsonDeserialize;

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

    assert_eq!("data", result.field1, "decoding default json failed")
}