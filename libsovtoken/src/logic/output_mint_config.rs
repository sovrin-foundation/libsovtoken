/*!
 *  Defines structure and implementation for OutputMintConfig and MintRequest
 *  these are the structures for the [`build_mint_txn_handler`]
 * 
 *  [`build_mint_txn_handler`]: ../../api/fn.build_mint_txn_handler.html
 */


use logic::request::Request;

type Output = (String, u32);

/**
 *  Json config to customize [`build_mint_txn_handler`]
 *  
 *  [`build_mint_txn_handler`]: ../../api/fn.build_mint_txn_handler.html
 */
#[derive(Serialize, Deserialize)]
pub struct OutputMintConfig {
    pub outputs: Vec<Output>,
}

#[derive(Serialize, Debug)]
pub struct MintRequest {
    #[serde(rename = "type")]
    txn_type: &'static str,
    outputs: Vec<(String, u32)>,
    signatures: Vec<String>,
}

/**
 * A struct that can be transformed into a Mint JSON object.
 */
impl MintRequest {

    /**
     * Creates a new `MintRequest` with `outputs`
     */
    pub fn new(outputs: Vec<Output>) -> Self {
        return MintRequest {
            txn_type: "1001",
            outputs: outputs,
            signatures: Vec::new(),
        };
    }
}

impl From<OutputMintConfig> for MintRequest {

    /**
     * Creates a `MintRequest` from an [`OutputMintConfig`].
     */
    fn from(mint_config: OutputMintConfig) -> Self {
        return MintRequest::new(mint_config.outputs);
    }
}

// TODO: Actually sign the Request
impl Request for MintRequest {
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
    use serde_json;
    use utils::ffi_support::str_from_char_ptr;

    fn initial_mint_request() -> MintRequest {
        let outputs = vec![(String::from("AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja"), 10)];
        return MintRequest::new(outputs);
    }

    fn assert_mint_request<F>(expected: serde_json::Value, f: F)
        where F: Fn(&mut MintRequest) -> ()
    {
        let mut mint_req = initial_mint_request();
        f(&mut mint_req);
        let mint_req_c_string = mint_req.serialize_to_cstring();
        let mint_req_json_str = str_from_char_ptr(mint_req_c_string.as_ptr()).unwrap();
        let mint_req_json_value : serde_json::Value = serde_json::from_str(mint_req_json_str).unwrap();
        assert_eq!(mint_req_json_value, expected);
    }

    #[test]
    fn unsigned_request() {
        assert_mint_request(
            json!({
                "type": "1001",
                "outputs": [["AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja",10]],
                "signatures": []
            }),
            |_mint_req| {}
        );
    }

    #[test]
    fn create_request_with_mint_config() {
        let outputs = vec![(String::from("AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja"), 10)];
        let mint_config = OutputMintConfig {
            outputs: outputs.clone()
        };
        let request = MintRequest::from(mint_config);
        assert_eq!(request.outputs, outputs);
    }

    #[test]
    fn valid_request() {
        assert_mint_request(
            json!({
                "type": "1001",
                "outputs": [["AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja",10]],
                "signatures": ["000my_totally_random_key"]
            }),
            |mint_req| { mint_req.sign("my_totally_random_key"); }
        );
    }
}