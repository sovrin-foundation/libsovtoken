/*!
    Payment Input
*/

use serde::{de, ser, Deserialize, ser::SerializeTuple, Serialize};
use std::fmt;
use logic::parsers::common::TXO;

pub type Inputs = Vec<Input>;

/**
 * Config which holds a vec of [`Input`]s
 * 
 * Also has a version for backward compatability.
 * 
 * [`Inputs`]: Input
 */
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct InputConfig {
    pub ver: u8,
    pub inputs: Inputs,
}


/**
    Struct which holds an address, seq_no, signature, and extra data.

    # Deserialization
    Input can be deserialized from an array or an object. Both are valid:

    ## From Array
    An array with the format of `[address, seq_no, signature]`.
    When deserializing from an array, the signature is required.
    ```ignore
    use sovtoken::utils::json_conversion::JsonDeserialize;
    use sovtoken::logic::input::Input;
    let json = r#"["pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja", 30, "239asdkj3298uadkljasd98u234ijasdlkj"]"#;
    let input = Input::from_json(json);
    ```

    ## From Object
    ### Required Fields
    * address
    * seq_no

    ### Optional Fields
    * signature
    * extra
    
    ```
    use sovtoken::utils::json_conversion::JsonDeserialize;
    use sovtoken::logic::input::Input;
    let json = r#"{
        "address": "pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja",
        "seqNo": 30,
        "signature": "239asdkj3298uadkljasd98u234ijasdlkj"
    }"#;
    let input = Input::from_json(json);
    ```

    # Serialization
    When Input is serialized, it is always serialized as an array:

    ```
    use sovtoken::utils::json_conversion::JsonSerialize;
    use sovtoken::logic::input::Input;
    let address = String::from("pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja");
    let input = Input::new(address, 30);

    let json = Input::to_json(&input).unwrap();
    assert_eq!(json, r#"["pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja",30]"#);
    ```

*/
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Input {
    pub address: String,
    pub seq_no: i32
}

impl ToString for Input {
    fn to_string(&self) -> String {
        format!("{}{}", self.seq_no, self.address)
    }
}

impl Input {
    pub fn new(address: String, seq_no: i32) -> Input {
        return Input { address, seq_no};
    }

    /*pub fn sign_with(self, signature: String) -> Self {
        return Input::new(self.address, self.seq_no);
    }*/
}

impl Serialize for Input {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_tuple(2)?;
        seq.serialize_element(&self.address)?;
        seq.serialize_element(&self.seq_no)?;
        return seq.end();
    }
}

impl<'de> Deserialize<'de> for Input {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Input, D::Error> {
        struct InputVisitor;

        impl<'de> de::Visitor<'de> for InputVisitor {
            type Value = Input;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                return formatter.write_str("Expected an Input with address and seqNo.");
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                let txo = TXO::from_libindy_string(v)
                    .map_err(|ec| de::Error::custom(format!("Error when deserializing txo: error code {:?}", ec)))?;
                Ok(Input{ address: txo.address, seq_no: txo.seq_no })
            }

            fn visit_seq<V: de::SeqAccess<'de>>(self, mut seq: V) -> Result<Input, V::Error> {
                let address = seq
                    .next_element()?
                    .ok_or(de::Error::invalid_length(0, &"2"))?;

                let seq_no = seq
                    .next_element()?
                    .ok_or(de::Error::invalid_length(1, &"2"))?;

                return Ok(Input::new(address, seq_no));
            }

            fn visit_map<V: de::MapAccess<'de>>(self, mut map: V) -> Result<Input, V::Error> {
                let mut address = None;
                let mut seq_no = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "address" => { address = map.next_value()?; },
                        "seqNo" => { seq_no =  map.next_value()?; },
                        x => { return Err(de::Error::unknown_field(x, FIELDS)) }
                    }
                }

                let address = address.ok_or(de::Error::missing_field("address"))?;
                let seq_no = seq_no.ok_or( de::Error::missing_field("seqNo"))?;

                return Ok(Input::new(address, seq_no));
            }
        }

        const FIELDS: &'static [&'static str] = &["address", "seqNo"];
        return deserializer.deserialize_any(InputVisitor);
    }
}


#[cfg(test)]
mod input_tests {
    use super::Input;
    use serde_json;
    use utils::json_conversion::{JsonDeserialize, JsonSerialize};
    use logic::parsers::common::TXO;
    use rust_base58::ToBase58;

    fn json_value_to_string(json: serde_json::Value) -> String {
        return serde_json::to_string(&json).unwrap();
    }

    fn assert_invalid_deserialize(json: serde_json::Value, error_message_starts_with: &str) {
        let json_string = json_value_to_string(json);
        let invalid = Input::from_json(&json_string).unwrap_err();
        println!("{}", invalid);
        assert!(format!("{}", invalid).contains(error_message_starts_with));
    }

    fn assert_valid_deserialize(json: serde_json::Value, expected_input: Input) {
        let json_string = json_value_to_string(json);
        let input = Input::from_json(&json_string).unwrap();
        assert_eq!(input, expected_input);
    }

    fn assert_invalid_serialize(input: Input, error_message_starts_with: &str) {
        let invalid = Input::to_json(&input).unwrap_err();
        assert!(format!("{}", invalid).starts_with(error_message_starts_with));
    }

    fn assert_valid_serialize(input: Input, json: serde_json::Value) {
        let json_string = json_value_to_string(json);
        let input_serialized = Input::to_json(&input).unwrap();
        assert_eq!(input_serialized, json_string);
    }

    fn valid_input() -> Input {
        let address = String::from("pay:sov:a8QAXMjRwEGoGLmMFEc5sTcntZxEF1BpqAs8GoKFa9Ck81fo7");
        return Input::new(address, 30);
    }

    #[test]
    fn deserialize_invalid_input_tuple() {
        let json = json!(["Avadsfesaafefsdfcv"]);
        assert_invalid_deserialize(json, "invalid length 1, expected 2");
    }

    #[test]
    fn deserialize_invalid_tuple_invalid_seq_no() {
        let json = json!(["pay:sov:a8QAXMjRwEGoGLmMFEc5sTcntZxEF1BpqAs8GoKFa9Ck81f", 2.5]);
        assert_invalid_deserialize(json, "invalid type: floating point")
    }

    #[test]
    fn deserialize_input_tuple() {
        let json = json!(["pay:sov:a8QAXMjRwEGoGLmMFEc5sTcntZxEF1BpqAs8GoKFa9Ck81fo7", 30]);
        let expected = Input::new(String::from("pay:sov:a8QAXMjRwEGoGLmMFEc5sTcntZxEF1BpqAs8GoKFa9Ck81fo7"), 30);
        assert_valid_deserialize(json, expected);
    }

    #[test]
    fn deserialize_invalid_input_object_without_seq_no() {
        let json = json!("txo:sov:".to_string() + &json!({
            "address": "pay:sov:a8QAXMjRwEGoGLmMFEc5sTcntZxEF1BpqAs8GoKFa9Ck81fo7",
        }).to_string().as_bytes().to_base58_check());
        assert_invalid_deserialize(json, "missing field `seqNo`");
    }

    #[test]
    fn deserialize_input_object_without_address() {
        let json = json!("txo:sov:".to_string() + &json!({
            "seqNo": 30,
        }).to_string().as_bytes().to_base58_check());
        assert_invalid_deserialize(json, "missing field `address`");
    }

    #[test]
    fn deserialize_input_object_with_keys() {
        let json = json!(
            TXO {
                address: "pay:sov:a8QAXMjRwEGoGLmMFEc5sTcntZxEF1BpqAs8GoKFa9Ck81fo7".to_string(),
                seq_no: 30
            }.to_libindy_string().unwrap()
        );
        let input = valid_input();
        assert_valid_deserialize(json, input);
    }
}

#[cfg(test)]
mod input_config_test {
    use logic::input::{Input, InputConfig};
    use utils::json_conversion::JsonSerialize;

    // this test ensures that the deserialized JSON is serialized correctly
    #[test]
    fn serializing_payload_struct_output_config() {

        let input = Input::new(String::from("a8QAXMjRwEGoGLmMFEc5sTcntZxEF1BpqAs8GoKFa9Ck81fo7"), 30);

        let fee: InputConfig = InputConfig {
            ver: 1,
            inputs: vec![input],
        };
        assert_eq!(fee.to_json().unwrap(), r#"{"ver":1,"inputs":[["a8QAXMjRwEGoGLmMFEc5sTcntZxEF1BpqAs8GoKFa9Ck81fo7",30]]}"#);
    }
}