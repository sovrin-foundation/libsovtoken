//!
//!  Implementations for Serde Json serialization/deserialization
//!

use serde::{Serialize, Deserialize};
use serde_json::{Error, from_str, to_string};

//
// given a json formatted string, return object of given type
// any type with the Deserialize attribute will be supported
//
pub trait JsonDeserialize<'a>: Deserialize<'a> {
    fn from_json(json: &'a str) -> Result<Self, Error> {
        from_str(json)
    }
}

// this impl adds json deseralization to any object with Deserialize attribute
impl<'a, T: Deserialize<'a> > JsonDeserialize<'a> for T { }


// given a type with the attribute of Serialize, this trait
// will support serializing the public data members into properly
// formatted json
pub trait JsonSerialize : Serialize + Sized {
    fn to_json(&self) -> Result<String, Error> {
        to_string(self)
    }
}

// this impl adds json seralization to any object with Serialize attribute
impl<T:Serialize> JsonSerialize for T { }




/*
         UNIT TESTS BELOW
         (and only unit tests---do not add more functions below this mod)
*/

#[cfg(test)]
mod json_conversion_tests {

    use serde::{Serialize, Deserialize};
    use utils::json_conversion::{JsonDeserialize, JsonSerialize};

    // helper structures and data
    #[derive(Debug, Serialize, Deserialize)]
    struct DummyJsonStruct {
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

}