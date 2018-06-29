/*!
    Payment Output
*/

use serde::{de, ser, ser::{SerializeTuple}, Deserialize, Serialize};
use std::fmt;
use logic::type_aliases::TokenAmount;

pub type Outputs = Vec<Output>;

/**
 * Config which holds a vec of [`Output`]s
 *
 * Also has a version for backward compatability.
 *
 * [`Outputs`]: Output
 */
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct OutputConfig {
    pub ver: u8,
    pub outputs: Outputs,
}

/**
    Struct which holds a payment address, token amount, and extra data.

    ```text
    // (address, token_amount)
    ("pay:sov:a8QAXMjRwEGoGLmMFEc5sTcntZxEF1BpqAs8GoKFa9Ck81fo7", 5)
    ```

    # Deserialization
    Output can be deseriazlized from an array or an object. Both are valid:

    ## From Array
    ```
    use sovtoken::utils::json_conversion::JsonDeserialize;
    use sovtoken::logic::output::Output;
    let json = r#"["pay:sov:a8QAXMjRwEGoGLmMFEc5sTcntZxEF1BpqAs8GoKFa9Ck81fo7", 5]"#;
    let output = Output::from_json(json);
    ```

    ## From Object
    ```
    use sovtoken::utils::json_conversion::JsonDeserialize;
    use sovtoken::logic::output::Output;
    let json = r#"{
        "address": "pay:sov:a8QAXMjRwEGoGLmMFEc5sTcntZxEF1BpqAs8GoKFa9Ck81fo7",
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
    let address = String::from("pay:sov:a8QAXMjRwEGoGLmMFEc5sTcntZxEF1BpqAs8GoKFa9Ck81fo7");
    let output = Output::new(address, 5, None);
    let json = Output::to_json(&output).unwrap();
    assert_eq!(json, r#"["pay:sov:a8QAXMjRwEGoGLmMFEc5sTcntZxEF1BpqAs8GoKFa9Ck81fo7",5]"#);
    ```

*/
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Output {
    pub address: String,
    pub amount: TokenAmount,
    // This is wrong, there should be no extra in output
    pub extra: Option<String>,
}

impl Output {
    pub fn new(address: String, amount: TokenAmount, extra: Option<String>) -> Output {
        return Output { address, amount, extra };
    }
}

impl Serialize for Output {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_tuple(2)?;
        seq.serialize_element(&self.address)?;
        seq.serialize_element(&self.amount)?;
        return seq.end();
    }
}

impl<'de> Deserialize<'de> for Output {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Output, D::Error> {
        struct OutputVisitor;

        impl<'de> de::Visitor<'de> for OutputVisitor {
            type Value = Output;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                return formatter.write_str("Expected an Output with address and tokens.");
            }

            fn visit_seq<V: de::SeqAccess<'de>>(self, mut seq: V) -> Result<Output, V::Error> {
                let address = seq
                    .next_element()?
                    .ok_or(de::Error::invalid_length(0, &"2"))?;

                let amount = seq
                    .next_element()?
                    .ok_or(de::Error::invalid_length(1, &"2"))?;

                let extra = None;

                return Ok(Output::new(address, amount, extra));
            }

            fn visit_map<V: de::MapAccess<'de>>(self, mut map: V) -> Result<Output, V::Error> {
                let mut address = None;
                let mut amount = None;
                let mut extra = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "paymentAddress" => { address = map.next_value()?; },
                        "amount" => { amount =  map.next_value()?; },
                        "extra" => { extra = map.next_value()?; },
                        x => { return Err(de::Error::unknown_field(x, FIELDS)) }
                    }
                }

                let address = address.ok_or(de::Error::missing_field("paymentAddress"))?;
                let amount = amount.ok_or_else(|| de::Error::missing_field("amount"))?;

                return Ok(Output::new(address, amount, extra));
            }
        }

        const FIELDS: &'static [&'static str] = &["paymentAddress", "amount", "extra"];
        return deserializer.deserialize_struct("Output", FIELDS, OutputVisitor);
    }
}


#[cfg(test)]
mod output_tests {
    use super::*;
    use serde_json;
    use utils::json_conversion::{JsonDeserialize, JsonSerialize};


    fn assert_invalid_deserialize(json: serde_json::Value, error_message_starts_with: &str) {
        let json_string = json_value_to_string(json);
        let invalid = Output::from_json(&json_string).unwrap_err();
        assert!(format!("{}", invalid).starts_with(error_message_starts_with));
    }

    fn assert_valid_deserialize(json: serde_json::Value, expected_output: Output) {
        let json_string = json_value_to_string(json);
        let output = Output::from_json(&json_string).unwrap();
        assert_eq!(output, expected_output);
    }

    fn assert_valid_serialize(output: Output, json: serde_json::Value) {
        let json_string = json_value_to_string(json);
        let output_serialized = Output::to_json(&output).unwrap();
        assert_eq!(output_serialized, json_string);
    }

    fn json_value_to_string(json: serde_json::Value) -> String {
        return serde_json::to_string(&json).unwrap();
    }

    fn output_with_extra() -> Output {
        let address = String::from("pay:sov:a8QAXMjRwEGoGLmMFEc5sTcntZxEF1BpqAs8GoKFa9Ck81fo7");
        let extra = Some(String::from("ewt3eioSSDziqDGehdJLSEwanzZNsgaawqp"));
        return Output::new(address, 10, extra);
    }

    fn output_without_extra() -> Output {
        let address = String::from("pay:sov:a8QAXMjRwEGoGLmMFEc5sTcntZxEF1BpqAs8GoKFa9Ck81fo7");
        return Output::new(address, 10, None);
    }

    #[test]
    fn deserialize_invalid_output_tuple() {
        let json = json!(["Avadsfesaafefsdfcv"]);
        assert_invalid_deserialize(json, "invalid length 1, expected 2");
    }

    #[test]
    fn deserialize_output_tuple() {
        let json = json!(["pay:sov:a8QAXMjRwEGoGLmMFEc5sTcntZxEF1BpqAs8GoKFa9Ck81fo7", 10]);
        let expected = Output::new(String::from("pay:sov:a8QAXMjRwEGoGLmMFEc5sTcntZxEF1BpqAs8GoKFa9Ck81fo7"), 10, None);
        assert_valid_deserialize(json, expected);
    }

    #[test]
    fn deserialize_invalid_output_object() {
        let json = json!({
            "paymentAddress": "pay:sov:a8QAXMjRwEGoGLmMFEc5sTcntZxEF1BpqAs8GoKFa9Ck81fo7",
            "extra": "eifjoaiandvskasn",
        });
        assert_invalid_deserialize(json, "missing field `amount`");
    }

    #[test]
    fn deserialize_output_object_without_extra() {
        let json = json!({
            "paymentAddress": "pay:sov:a8QAXMjRwEGoGLmMFEc5sTcntZxEF1BpqAs8GoKFa9Ck81fo7",
            "amount": 10,
        });
        let output = output_without_extra();
        assert_valid_deserialize(json, output);
    }

    #[test]
    fn deserialize_output_object_with_extra() {
        let json = json!({
            "paymentAddress": "pay:sov:a8QAXMjRwEGoGLmMFEc5sTcntZxEF1BpqAs8GoKFa9Ck81fo7",
            "amount": 10,
            "extra": "ewt3eioSSDziqDGehdJLSEwanzZNsgaawqp",
        });
        let output = output_with_extra();
        assert_valid_deserialize(json, output);
    }

    #[test]
    fn serialize_output_without_extra() {
        let json = json!(["pay:sov:a8QAXMjRwEGoGLmMFEc5sTcntZxEF1BpqAs8GoKFa9Ck81fo7", 10]);
        let output = output_without_extra();
        assert_valid_serialize(output, json);
    }

    #[test]
    fn serialize_output_with_extra() {
        let json = json!(["pay:sov:a8QAXMjRwEGoGLmMFEc5sTcntZxEF1BpqAs8GoKFa9Ck81fo7", 10]);
        let output = output_with_extra();
        assert_valid_serialize(output, json);
    }

    // this test ensures that the deserialized JSON is serialized correctly
    #[test]
    fn serializing_fee_struct_output_config() {
        let output = Output::new(String::from("a8QAXMjRwEGoGLmMFEc5sTcntZxEF1BpqAs8GoKFa9Ck81fo7"), 10, None);

        let fee: OutputConfig = OutputConfig {
            ver: 1,
            outputs: vec![output],
        };
        assert_eq!(fee.to_json().unwrap(), r#"{"ver":1,"outputs":[["a8QAXMjRwEGoGLmMFEc5sTcntZxEF1BpqAs8GoKFa9Ck81fo7",10]]}"#);
    }
}
