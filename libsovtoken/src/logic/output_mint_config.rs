//
// defines structure and implementation for OutputMintConfig which is used
// for minting tokens 
// these are the structures for the outputs taken in by 'build_mint_txn_handler'
// in the JSON format {'output': [['address', 10]]}


use logic::request::Request;

type Output = (String, u32);

#[derive(Serialize, Deserialize)]
pub struct OutputMintConfig {
    pub outputs: Vec<Output>,
}

#[derive(Serialize)]
struct MintRequest<'a> {
    ty: &'static str,
    outputs: &'a Vec<(String, u32)>,
    signatures: Vec<String>,
}

impl<'a> MintRequest<'a> {
    fn new(outputs: &'a Vec<Output>) -> Self {
        return MintRequest {
            ty: "1001",
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
    use utils::ffi_support::{char_ptr_from_str};

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
        let result = str_from_char_ptr(mint.serialize_to_c_char()).unwrap();

        let expected = r#"
            "type": "101",
            "outputs": [("AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja", 10)],
            "signatures": ["000my_totally_random_key"]
        "#;

        assert_eq!(result, expected);
    }
}