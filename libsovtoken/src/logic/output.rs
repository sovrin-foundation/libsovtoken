/*!
    Payment Output
*/

use serde::{de, Deserialize};
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
    let output = Output::new(address, 5);
    let json = Output::to_json(&output).unwrap();
    assert_eq!(json, r#"{"address":"pay:sov:a8QAXMjRwEGoGLmMFEc5sTcntZxEF1BpqAs8GoKFa9Ck81fo7","amount":5}"#);
    ```

*/
#[derive(Debug, Eq, PartialEq, Clone, Serialize)]
pub struct Output {
    #[serde(rename = "address")]
    pub recipient: String,
    pub amount: TokenAmount
}

impl Output {
    pub fn new(address: String, amount: TokenAmount) -> Output {
        return Output { recipient: address, amount };
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

            fn visit_map<V: de::MapAccess<'de>>(self, mut map: V) -> Result<Output, V::Error> {
                let mut address = None;
                let mut amount = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "recipient" => { address = map.next_value()?; },
                        "address" => { address = map.next_value()?; },
                        "amount" => { amount =  map.next_value()?; },
                        x => { return Err(de::Error::unknown_field(x, FIELDS)) }
                    }
                }

                let address = address.ok_or(de::Error::missing_field("recipient or address"))?;
                let amount = amount.ok_or_else(|| de::Error::missing_field("amount"))?;

                return Ok(Output::new(address, amount));
            }
        }

        const FIELDS: &'static [&'static str] = &["recipient", "amount", "address"];
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

    fn output() -> Output {
        let address = String::from("a8QAXMjRwEGoGLmMFEc5sTcntZxEF1BpqAs8GoKFa9Ck81fo7");
        return Output::new(address, 10);
    }

    #[test]
    fn deserialize_output() {
        let json = json!(
            {
                 "address":"a8QAXMjRwEGoGLmMFEc5sTcntZxEF1BpqAs8GoKFa9Ck81fo7",
                 "amount":10
            }
        );
        let expected = output();
        assert_valid_deserialize(json, expected);
    }

    #[test]
    fn deserialize_invalid_output_object() {
        let json = json!({
            "address": "pay:sov:a8QAXMjRwEGoGLmMFEc5sTcntZxEF1BpqAs8GoKFa9Ck81fo7"
        });
        assert_invalid_deserialize(json, "missing field `amount`");
    }

    #[test]
    fn serialize_valid_output_object() {
        let output = output();
        let json = json!({
                 "address":"a8QAXMjRwEGoGLmMFEc5sTcntZxEF1BpqAs8GoKFa9Ck81fo7",
                 "amount":10
            });
        assert_valid_serialize(output, json);
    }

    #[test]
    fn serializing_fee_struct_output_config() {
        let output = output();

        let fee: OutputConfig = OutputConfig {
            ver: 1,
            outputs: vec![output],
        };
        assert_eq!(fee.to_json().unwrap(), r#"{"ver":1,"outputs":[{"address":"a8QAXMjRwEGoGLmMFEc5sTcntZxEF1BpqAs8GoKFa9Ck81fo7","amount":10}]}"#);
    }
}
