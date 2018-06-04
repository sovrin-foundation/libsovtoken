/*!
    Payment Input
*/

use serde::{de, ser, ser::{SerializeTuple}, Deserialize, Serialize};
use std::fmt;

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
    ```
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
    let signature = String::from("239asdkj3298uadkljasd98u234ijasdlkj");
    let input = Input::new(address, 30, Some(signature));

    let json = Input::to_json(&input).unwrap();
    assert_eq!(json, r#"["pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja",30,"239asdkj3298uadkljasd98u234ijasdlkj"]"#);
    ```

*/
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Input {
    pub address: String,
    pub seq_no: u32,
    pub signature: Option<String>,
    pub extra: Option<String>,
}

impl Input {
    pub fn new(address: String, seq_no: u32, signature: Option<String>) -> Input {
        return Input { address, seq_no, signature, extra: None};
    }

    pub fn new_with_extra(
        address: String,
        seq_no: u32,
        signature: Option<String>,
        extra: Option<String>
    ) -> Input
    {
        return Input { address, seq_no, signature, extra }
    }

    pub fn sign_with(self, signature: String) -> Self {
        return Input::new(self.address, self.seq_no, Some(signature));
    }
}

impl Serialize for Input {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if self.signature.is_none() {
            return Err(ser::Error::custom("Expected Input to have a signature."))
        }

        let mut seq = serializer.serialize_tuple(2)?;
        seq.serialize_element(&self.address)?;
        seq.serialize_element(&self.seq_no)?;
        seq.serialize_element(&self.signature)?;
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

            fn visit_seq<V: de::SeqAccess<'de>>(self, mut seq: V) -> Result<Input, V::Error> {
                let address = seq
                    .next_element()?
                    .ok_or(de::Error::invalid_length(0, &"3"))?;

                let seq_no = seq
                    .next_element()?
                    .ok_or(de::Error::invalid_length(1, &"3"))?;

                let signature = seq
                    .next_element()?
                    .ok_or(de::Error::invalid_length(2, &"3"))?;

                return Ok(Input::new(address, seq_no, signature));
            }

            fn visit_map<V: de::MapAccess<'de>>(self, mut map: V) -> Result<Input, V::Error> {
                let mut address = None;
                let mut seq_no = None;
                let mut signature = None;
                let mut extra = None;


                while let Some(key) = map.next_key()? {
                    match key {
                        "address" => { address = map.next_value()?; },
                        "seqNo" => { seq_no =  map.next_value()?; },
                        "signature" => { signature = map.next_value()?; },
                        "extra" => { extra = map.next_value()?; },
                        x => { return Err(de::Error::unknown_field(x, FIELDS)) }
                    }
                }

                let address = address.ok_or(de::Error::missing_field("address"))?;
                let seq_no = seq_no.ok_or( de::Error::missing_field("seqNo"))?;

                return Ok(Input::new_with_extra(address, seq_no, signature, extra));
            }
        }

        const FIELDS: &'static [&'static str] = &["address", "seqNo", "signature"];
        return deserializer.deserialize_struct("Input", FIELDS, InputVisitor);
    }
}


#[cfg(test)]
mod input_tests {
    use super::Input;
    use serde_json;
    use utils::json_conversion::{JsonDeserialize, JsonSerialize};

    fn json_value_to_string(json: serde_json::Value) -> String {
        return serde_json::to_string(&json).unwrap();
    }

    fn assert_invalid_deserialize(json: serde_json::Value, error_message_starts_with: &str) {
        let json_string = json_value_to_string(json);
        let invalid = Input::from_json(&json_string).unwrap_err();
        println!("{}", invalid);
        assert!(format!("{}", invalid).starts_with(error_message_starts_with));
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

    fn input_with_extra() -> Input {
        let address = String::from("pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja");
        let seq_no = 30;
        let signature = Some(String::from("239asdkj3298uadkljasd98u234ijasdlkj"));
        let extra = Some(String::from("This is an extra string."));
        return Input::new_with_extra(address, seq_no, signature, extra);
    }

    fn input_without_extra() -> Input {
        let address = String::from("pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja");
        let signature= Some(String::from("239asdkj3298uadkljasd98u234ijasdlkj"));
        return Input::new(address, 30, signature);
    }

    fn input_without_extra_or_signature() -> Input {
        let address = String::from("pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja");
        return Input::new(address, 30, None);
    }

    #[test]
    fn deserialize_invalid_input_tuple() {
        let json = json!(["Avadsfesaafefsdfcv"]);
        assert_invalid_deserialize(json, "invalid length 1, expected 3");
    }

    #[test]
    fn deserialize_invalid_tuple_no_signature() {
        let json = json!(["pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja", 30,]);
        assert_invalid_deserialize(json, "invalid length 2, expected 3")
    }

    #[test]
    fn deserialize_input_tuple() {
        let json = json!(["pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja", 30, "239asdkj3298uadkljasd98u234ijasdlkj"]);
        let expected = Input::new(String::from("pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja"), 30, Some(String::from("239asdkj3298uadkljasd98u234ijasdlkj")));
        assert_valid_deserialize(json, expected);
    }

    #[test]
    fn deserialize_invalid_input_object() {
        let json = json!({
            "address": "pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja",
            "signature": "239asdkj3298uadkljasd98u234ijasdlkj",
        });
        assert_invalid_deserialize(json, "missing field `seqNo`");
    }

    #[test]
    fn deserialize_input_object_without_signature_or_extra() {
        let json = json!({
            "address": "pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja",
            "seqNo": 30,
        });
        let input = input_without_extra_or_signature();
        assert_valid_deserialize(json, input);
    }


    #[test]
    fn deserialize_input_object_without_extra() {
        let json = json!({
            "address": "pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja",
            "seqNo": 30,
            "signature": "239asdkj3298uadkljasd98u234ijasdlkj",
        });
        let input = input_without_extra();
        assert_valid_deserialize(json, input);
    }

    #[test]
    fn deserialize_input_object_with_extra() {
        let json = json!({
            "address": "pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja",
            "seqNo": 30,
            "signature": "239asdkj3298uadkljasd98u234ijasdlkj",
            "extra": "This is an extra string.",
        });
        let input = input_with_extra();
        assert_valid_deserialize(json, input);
    }

    #[test]
    fn serialize_invalid_without_signature() {
        let input = input_without_extra_or_signature();
        assert_invalid_serialize(input, "Expected Input to have a signature.");
    }

    #[test]
    fn serialize_input_without_extra() {
        let json = json!(["pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja", 30, "239asdkj3298uadkljasd98u234ijasdlkj"]);
        let input = input_without_extra();
        assert_valid_serialize(input, json);
    }

    #[test]
    fn serialize_input_with_extra() {
        let json = json!(["pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja", 30, "239asdkj3298uadkljasd98u234ijasdlkj"]);
        let input = input_with_extra();
        assert_valid_serialize(input, json);
    }

    #[test]
    fn sign_input() {
        let input = Input::new(String::from("pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja"), 30, None);
        let signed_input = input.sign_with(String::from("3aRkv0kyRjCYu7SazNpbOzJPhKWlQDFBU7Judz16nx6CzAUsp06q2PaPWmKh"));
        assert_eq!(signed_input.address, String::from("pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja"));
        assert_eq!(signed_input.seq_no, 30);
        assert_eq!(signed_input.signature.unwrap(), String::from("3aRkv0kyRjCYu7SazNpbOzJPhKWlQDFBU7Judz16nx6CzAUsp06q2PaPWmKh"));
    }
}

#[cfg(test)]
mod input_config_test {
    use logic::input::{Input, InputConfig};
    use utils::json_conversion::JsonSerialize;

    // this test ensures that the deserialized JSON is serialized correctly
    #[test]
    fn serializing_fee_struct_output_config() {

        let input = Input::new(String::from("dakjhe238yad"), 30, Some(String::from("239asdkj3298uadkljasd98u234ijasdlkj")));

        let fee: InputConfig = InputConfig {
            ver: 1,
            inputs: vec![input],
        };
        assert_eq!(fee.to_json().unwrap(), r#"{"ver":1,"inputs":[["dakjhe238yad",30,"239asdkj3298uadkljasd98u234ijasdlkj"]]}"#);
    }
}