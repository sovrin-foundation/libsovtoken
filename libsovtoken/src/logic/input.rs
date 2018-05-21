/*!
    Payment Input
*/

use serde::{de, ser, ser::{SerializeTuple}, Deserialize, Serialize};
use std::fmt;

/**
    Struct which holds a payment address, seq_no, signature

    ```text
    // (payment_address, seq_no, signature)
    ("pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja", 30, "239asdkj3298uadkljasd98u234ijasdlkj")
    ```

    # Deserialization
    Input can be deserialized from an array or an object. Both are valid:

    ## From Array
    ```
    use sovtoken::utils::json_conversion::JsonDeserialize;
    use sovtoken::logic::input::Input;
    let json = r#"["pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja", 30, "239asdkj3298uadkljasd98u234ijasdlkj"]"#;
    let input = Input::from_json(json);
    ```

    ## From Object
    ```
    use sovtoken::utils::json_conversion::JsonDeserialize;
    use sovtoken::logic::input::Input;
    let json = r#"{
        "paymentAddress": "pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja",
        "seq_no": 30,
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
    let input = Input::new(address, 30, signature);

    let json = Input::to_json(&input).unwrap();
    assert_eq!(json, r#"["pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja",30,"239asdkj3298uadkljasd98u234ijasdlkj"]"#);
    ```

*/
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Input {
    pub payment_address: String,
    pub seq_no: u32,
    pub signature: Option<String>
}

impl Input {
    pub fn new(payment_address: String, seq_no: u32, signature: Option<String>) -> Input {
        return Input { payment_address, seq_no, signature};
    }

    pub fn sign_with(self, signature: String) -> Self {
        return Input::new(self.payment_address, self.seq_no, Some(signature));
    }
}

impl Serialize for Input {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_tuple(2)?;
        seq.serialize_element(&self.payment_address)?;
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
                return formatter.write_str("Expected an Input with address and seq_no.");
            }

            fn visit_seq<V: de::SeqAccess<'de>>(self, mut seq: V) -> Result<Input, V::Error> {
                let payment_address = seq
                    .next_element()?
                    .ok_or(de::Error::invalid_length(0, &"2"))?;

                let seq_no = seq
                    .next_element()?
                    .ok_or(de::Error::invalid_length(1, &"2"))?;

                let signature = seq
                    .next_element()?
                    .ok_or(de::Error::invalid_length(2, &"2"))?;

                return Ok(Input::new(payment_address, seq_no, signature));
            }

            fn visit_map<V: de::MapAccess<'de>>(self, mut map: V) -> Result<Input, V::Error> {
                let mut payment_address = None;
                let mut seq_no = None;
                let mut signature = None;


                while let Some(key) = map.next_key()? {
                    match key {
                        "paymentAddress" => { payment_address = map.next_value()?; },
                        "seq_no" => { seq_no =  map.next_value()?; },
                        "signature" => { signature = map.next_value()?; },
                        x => { return Err(de::Error::unknown_field(x, FIELDS)) }
                    }
                }

                let payment_address = payment_address.ok_or(de::Error::missing_field("paymentAddress"))?;
                let seq_no = seq_no.ok_or_else(|| de::Error::missing_field("seq_no"))?;
                let signature = signature.ok_or_else(|| de::Error::missing_field("signature"))?;

                return Ok(Input::new(payment_address, seq_no, signature));
            }
        }

        const FIELDS: &'static [&'static str] = &["paymentAddress", "seq_no", "signature"];
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

    fn assert_valid_serialize(input: Input, json: serde_json::Value) {
        let json_string = json_value_to_string(json);
        let input_serialized = Input::to_json(&input).unwrap();
        assert_eq!(input_serialized, json_string);
    }

    fn input_with_extra() -> Input {
        let address = String::from("pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja");
        let signature = Some(String::from("239asdkj3298uadkljasd98u234ijasdlkj"));
        return Input::new(address, 30, signature);
        panic!("There is no extra.");
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
        assert_invalid_deserialize(json, "invalid length 1, expected 2");
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
            "paymentAddress": "pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja",
            "signature": "239asdkj3298uadkljasd98u234ijasdlkj",
        });
        assert_invalid_deserialize(json, "missing field `seq_no`");
    }

    #[test]
    fn deserialize_input_object_without_signature_or_extra() {
        let json = json!({
            "paymentAddress": "pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja",
            "seq_no": 30,
        });
        let input = input_without_extra_or_signature();
        assert_valid_deserialize(json, input);
    }


    #[test]
    fn deserialize_input_object_without_extra() {
        let json = json!({
            "paymentAddress": "pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja",
            "seq_no": 30,
            "signature": "239asdkj3298uadkljasd98u234ijasdlkj",
        });
        let input = input_without_extra();
        assert_valid_deserialize(json, input);
    }

    #[test]
    fn deserialize_input_object_with_extra() {
        let json = json!({
            "paymentAddress": "pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja",
            "seq_no": 30,
            "signature": "239asdkj3298uadkljasd98u234ijasdlkj",
        });
        let input = input_with_extra();
        assert_valid_deserialize(json, input);
    }

    #[test]
    fn serialize_input_without_extra_or_signature() {
        let json = json!(["pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja", 30]);
        let input = input_without_extra_or_signature();
        assert_valid_serialize(input, json);
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
        panic!("There is no extra.");
    }

    #[test]
    fn sign_input() {
        let input = Input::new(String::from("pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja"), 30, None);
        let signed_input = input.sign_with(String::from("3aRkv0kyRjCYu7SazNpbOzJPhKWlQDFBU7Judz16nx6CzAUsp06q2PaPWmKh"));
        assert_eq!(signed_input.payment_address, String::from("pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja"));
        assert_eq!(signed_input.seq_no, 30);
        assert_eq!(signed_input.signature.unwrap(), String::from("3aRkv0kyRjCYu7SazNpbOzJPhKWlQDFBU7Judz16nx6CzAUsp06q2PaPWmKh"));
    }
}
