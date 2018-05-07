#![warn(unused_imports)]
#[allow(unused_imports)]

use serde::{Serialize, Deserialize};

type Fee =  (String, u32, String);

#[derive(Serialize, Deserialize)]
pub struct FeesConfig {
   pub fees: Vec<Fee>,
}

#[cfg(test)]
mod fees_config_test {

    use super::*;
    use std::ffi::CString;
    use utils::ffi_support::{str_from_char_ptr, cstring_from_str};
    use utils::json_conversion::{JsonSerialize};
    static test_op_json : &'static str = r#"{"fees":[["ThisIsomeBizzareDIdsgivenTOme",1001,"MoreBigBitNums"]]}"#;
    #[test]
    fn valid_request () {
        let fee :FeesConfig = FeesConfig{
            fees : vec![(String::from("ThisIsomeBizzareDIdsgivenTOme"), 1001, String::from("MoreBigBitNums"))],
            };
        assert_eq!(fee.to_json().unwrap(), test_op_json);
    }
}