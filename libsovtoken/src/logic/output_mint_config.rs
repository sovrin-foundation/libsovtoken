//
// defines structure and implementation for OutputMintConfig which is used
// for minting tokens 
// these are the structures for the outputs taken in by 'build_mint_txn_handler'
// in the JSON format {'output': [['address', 10]]}
#[derive(Serialize, Deserialize)]
pub struct Output(String, u32);

#[derive(Serialize, Deserialize)]
pub struct OutputMintConfig {
    pub outputs: Vec<Output>,
}

// this test ensures that the deserialized JSON is serialized correctly
#[cfg(test)]
mod output_mint_config_test {
    use super::OutputMintConfig;
    use super::Output;
    use utils::json_conversion::JsonSerialize;
    #[test]
    fn serializing_mint_struct_config() {
        let mint : OutputMintConfig = OutputMintConfig { 
            outputs: vec![Output(String::from("AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja"), 10)],
        };
        assert_eq!(mint.to_json().unwrap(), r#"{"outputs":[["AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja",10]]}"#);
    }
}