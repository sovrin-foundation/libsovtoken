#![warn(unused_imports)]
#[allow(unused_imports)]

use serde::{Serialize, Deserialize};

type Fee =  (String, u32, String);

#[derive(Serialize, Deserialize)]
pub struct FeesConfig<'a> {
    fees: Vec<Fee>,
}

#[cfg(test)]
mod fees_config_test {

    use super::*;
    use std::ffi::CString;
    use utils::ffi_support::{str_from_char_ptr, cstring_from_str};
    use utils::json_conversion::{JsonSerialize};
    const test_op_json = r#"{"outputs":[["ThisIsomeBizzareDIdsgivenTOme",1001,"MoreBigBitNums"]]}"#;
    #[test]
    fn valid_request () {
        let fee :FeesConfig = FeesConfig{ vec![Sring::from("ThisIsomeBizzareDIdsgivenTOme"), 1001, String::from("MoreBigBitNums")]};
        assert_eq!(fee.to_json().unwrap(), test_op_json);
    }
};