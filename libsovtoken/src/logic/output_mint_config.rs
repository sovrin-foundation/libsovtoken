/*!
    Defines structure and implementation for OutputMintConfig and MintRequest
    these are the structures for the 'build_mint_txn_handler'
 */


use logic::request::Request;

type Output = (String, u32);

#[derive(Serialize, Deserialize)]
pub struct OutputMintConfig {
    pub outputs: Vec<Output>,
}

#[derive(Serialize, Debug)]
struct MintRequest<'a> {
    #[serde(rename = "type")]
    txn_type: &'static str,
    outputs: &'a Vec<(String, u32)>,
    signatures: Vec<String>,
}
 
impl<'a> MintRequest<'a> {
    fn new(outputs: &'a Vec<Output>) -> Self {
        return MintRequest {
            txn_type: "1001",
            outputs: &outputs,
            signatures: Vec::new(),
        };
    }
}

impl<'a> Request for MintRequest<'a> {
    fn sign(&mut self, key: &str) -> bool {
        self.signatures.push(format!("000{}", key));
        return true;
    }
}

// this test ensures that the deserialized JSON is serialized correctly
#[cfg(test)]
mod output_mint_config_test {
    use super::OutputMintConfig;
    use utils::json_conversion::JsonSerialize;
    #[test]
    fn serializing_mint_struct_config() {
        let mint : OutputMintConfig = OutputMintConfig { 
            outputs: vec![(String::from("AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja"), 10)],
        };
        assert_eq!(mint.to_json().unwrap(), r#"{"outputs":[["AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja",10]]}"#);
    }
}

#[cfg(test)]
mod mint_request_test {
    use super::*;
    use std::ffi::CString;
    use utils::ffi_support::{str_from_char_ptr, cstring_from_str};
    use utils::json_conversion::{JsonSerialize};


    #[test]
    fn invalid_outputs() {
        unimplemented!();
    }

    #[test]
    fn unsigned_request() {
        unimplemented!();
    }

    // TODO: Use wallet key.
    #[test]
    fn valid_request() {
        let outputs = vec![(String::from("AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja"), 10)];
        let mut mint = MintRequest::new(&outputs);
        mint.sign("my_totally_random_key");
        let cstring = mint.serialize_to_cstring();
        let result = str_from_char_ptr(cstring.as_ptr()).unwrap();

        let expected = r#"{"type":"1001","outputs":[["AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja",10]],"signatures":["000my_totally_random_key"]}"#;

        assert_eq!(result, expected);
    }
}