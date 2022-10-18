//!
//!  Implementations for Serde Json serialization/deserialization
//!

use serde::{Serialize, Deserialize};
use serde_json::{Error, from_str, to_string};

use ErrorCode;

//
// given a json formatted string, return object of given type
// any type with the Deserialize attribute will be supported
//
pub trait JsonDeserialize<'a>: Deserialize<'a> {
    fn from_json(json: &'a str) -> Result<Self, Error> {
        from_str(json)
    }

    /**
     *  Deserializes and maps the serde error to `ErrorCode::CommonInvalidStructure`.
     */
    fn from_json_error_code(json: &'a str) -> Result<Self, ErrorCode> {
        Self::from_json(json)
            .map_err(|_| ErrorCode::CommonInvalidStructure)
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


/**
Wrap the json! macro and outputs a c pointer.

Call it like you would call the json! macro, and
it returns a c pointer instead of `serde_json::value::Value`.

*/

macro_rules! json_c_pointer {
    ($json:tt) => {{
        let json = json!($json);
        let json_string = json.to_string();
        $crate::utils::ffi_support::c_pointer_from_string(json_string)
    }}
}

/*
## Example
```
    use sovtoken::utils::ffi_support::string_from_char_ptr;

    let c_string_pointer = json_c_pointer!({
        "nums": [1, 2, 3, 5, 8, 13, 21, 34, 55],
        "extra": {
            "info": "You can see this is a Fibonacci sequence."
        }
    });

    assert!(string_from_char_ptr(c_string_pointer).is_some());

```
*/


/*
         UNIT TESTS BELOW
         (and only unit tests---do not add more functions below this mod)
*/

#[cfg(test)]
mod json_conversion_tests {
    
    use utils::json_conversion::{JsonDeserialize, JsonSerialize};
    use ErrorCode;

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

    #[test]
    fn invalid_from_json_error_code() {
        let bad_json = "{abc}";
        let error_code = DummyJsonStruct::from_json_error_code(bad_json).unwrap_err();
        assert_eq!(ErrorCode::CommonInvalidStructure, error_code);
    }

    #[test]
    fn valid_from_json_error_code() {
        let dummy_struct = DummyJsonStruct::from_json(VALID_DUMMY_JSON).unwrap();
        assert_eq!("data", dummy_struct.field1);
    }

}

#[cfg(test)]
mod test_json_c_pointer {
    use utils::ffi_support::string_from_char_ptr;
    use serde_json;

    #[test]
    fn test_array() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        let j_ptr = json_c_pointer!(arr);
        let string = string_from_char_ptr(j_ptr).unwrap();
        let vec_nums: Vec<u32> = serde_json::from_str(&string).unwrap();
        assert_eq!(arr.to_vec(), vec_nums);
    }

    #[test]
    fn test_object() {
        #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
        struct T(u32);
        let t = T(1);
        let j_ptr = json_c_pointer!(t);
        let string = string_from_char_ptr(j_ptr).unwrap();
        let result: T = serde_json::from_str(&string).unwrap();
        assert_eq!(t, result);
    }

    #[test]
    fn test_static_object() {
        let j_ptr = json_c_pointer!({
            "key1": 123,
            "key2": 234,
            "Within": "the broken stone, the shattered glass, the silent cry",
            "sub_obj": {
                "aco": "Some more",
                "list": ["Finally", "We", "Arrived"]
            }
        });

        let string = string_from_char_ptr(j_ptr).unwrap();
        let json_value: serde_json::value::Value = serde_json::from_str(&string).unwrap();
        let list = json_value.get("sub_obj").unwrap().get("list").unwrap();
        assert_eq!("We", list.get(1).unwrap());
    }
}