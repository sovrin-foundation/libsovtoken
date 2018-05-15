/*!
    Payment Input
*/

use serde::{de, ser, ser::{SerializeTuple}, Deserialize, Serialize};
use std::fmt;

/**
    Struct which holds a payment address, token amount, and extra data.

    ```text
    // (payment_address, token_amount)
    ("pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja", 5)
    ```

    # Deserialization
    Output can be deseriazlized from an array or an object. Both are valid:

    ## From Array
    ```
    use sovtoken::utils::json_conversion::JsonDeserialize;
    use sovtoken::logic::output::Output;
    let json = r#"["pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja", 5]"#;
    let output = Output::from_json(json);
    ```

    ## From Object
    ```
    use sovtoken::utils::json_conversion::JsonDeserialize;
    use sovtoken::logic::output::Output;
    let json = r#"{
        "paymentAddress": "pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja",
        "amount": 5,
        "extra": None
    }"#;
    let output = Output::from_json(json);
    ```

    # Serialization
    When Output is serialized, it is always serialized as an array:

    ```
    use sovtoken::utils::json_conversion::JsonSerialize;
    use sovtoken::logic::output::Output;
    let address = String::from("pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja");
    let output = Output::new(address, 5, None);
    let json = Output::to_json(&output).unwrap();
    assert_eq!(json, r#"["pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja",5]"#);
    ```

*/
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Input {
    payment_address: String,
    amount: u32,
    signature: String,
    extra: Option<String>,
}

impl Input {
    pub fn new(payment_address: String, amount: u32, signature: String, extra: Option<String>) -> Input {
        return Input { payment_address, amount, signature, extra };
    }
}

impl Serialize for Input {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_tuple(2)?;
        seq.serialize_element(&self.payment_address)?;
        seq.serialize_element(&self.amount)?;
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
                return formatter.write_str("Expected an Input with address and amount.");
            }

            fn visit_seq<V: de::SeqAccess<'de>>(self, mut seq: V) -> Result<Input, V::Error> {
                let payment_address = seq
                    .next_element()?
                    .ok_or(de::Error::invalid_length(0, &"2"))?;

                let amount = seq
                    .next_element()?
                    .ok_or(de::Error::invalid_length(1, &"2"))?;

                let signature = seq
                    .next_element()?
                    .ok_or(de::Error::invalid_length(2, &"2"))?;

                let extra = None;

                return Ok(Input::new(payment_address, amount, signature, extra));
            }

            fn visit_map<V: de::MapAccess<'de>>(self, mut map: V) -> Result<Input, V::Error> {
                let mut payment_address = None;
                let mut amount = None;
                let mut signature = None;
                let mut extra = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "paymentAddress" => { payment_address = map.next_value()?; },
                        "amount" => { amount =  map.next_value()?; },
                        "signature" => { signature = map.next_value()?; },
                        "extra" => { extra = map.next_value()?; },
                        x => { return Err(de::Error::unknown_field(x, FIELDS)) }
                    }
                }

                let payment_address = payment_address.ok_or(de::Error::missing_field("paymentAddress"))?;
                let amount = amount.ok_or_else(|| de::Error::missing_field("amount"))?;
                let signature = signature.ok_or_else(|| de::Error::missing_field("signature"))?;

                return Ok(Input::new(payment_address, amount,signature, extra));
            }
        }

        const FIELDS: &'static [&'static str] = &["paymentAddress", "amount", "signature", "extra"];
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
        let signature = String::from("239asdkj3298uadkljasd98u234ijasdlkj");
        let extra = Some(String::from("ewt3eioSSDziqDGehdJLSEwanzZNsgaawqp"));
        return Input::new(address, 10, signature, extra);
    }

    fn input_without_extra() -> Input {
        let address = String::from("pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja");
        let signature= String::from("239asdkj3298uadkljasd98u234ijasdlkj");
        return Input::new(address, 10, signature, None);
    }

    #[test]
    fn deserialize_invalid_input_tuple() {
        let json = json!(["Avadsfesaafefsdfcv"]);
        assert_invalid_deserialize(json, "invalid length 1, expected 2");
    }

    #[test]
    fn deserialize_input_tuple() {
        let json = json!(["pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja", 10, "239asdkj3298uadkljasd98u234ijasdlkj"]);
        let expected = Input::new(String::from("pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja"), 10, String::from("239asdkj3298uadkljasd98u234ijasdlkj"), None);
        assert_valid_deserialize(json, expected);
    }

    #[test]
    fn deserialize_invalid_input_object() {
        let json = json!({
            "paymentAddress": "pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja",
            "amount": 10,
            "extra": "eifjoaiandvskasn",
        });
        assert_invalid_deserialize(json, "missing field `signature`");
    }

    #[test]
    fn deserialize_input_object_without_extra() {
        let json = json!({
            "paymentAddress": "pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja",
            "amount": 10,
            "signature": "239asdkj3298uadkljasd98u234ijasdlkj",
        });
        let input = input_without_extra();
        assert_valid_deserialize(json, input);
    }

    #[test]
    fn deserialize_input_object_with_extra() {
        let json = json!({
            "paymentAddress": "pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja",
            "amount": 10,
            "signature": "239asdkj3298uadkljasd98u234ijasdlkj",
            "extra": "ewt3eioSSDziqDGehdJLSEwanzZNsgaawqp",
        });
        let input = input_with_extra();
        assert_valid_deserialize(json, input);
    }

    #[test]
    fn serialize_input_without_extra() {
        let json = json!(["pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja", 10, "239asdkj3298uadkljasd98u234ijasdlkj"]);
        let input = input_without_extra();
        assert_valid_serialize(input, json);
    }

    #[test]
    fn serialize_input_with_extra() {
        let json = json!(["pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja", 10, "239asdkj3298uadkljasd98u234ijasdlkj"]);
        let input = input_with_extra();
        assert_valid_serialize(input, json);
    }

}
