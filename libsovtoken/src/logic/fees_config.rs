#![warn(unused_imports)]
#[allow(unused_imports)]

use serde::{Serialize, Deserialize};
use serde::lib::HashMap;

//type Fee =  (String, u32, String);

#[derive(Serialize, Deserialize)]
pub struct Fees<'a> {
   pub fees: &'a HashMap<&'static str, u32>,
}

#[derive(Serialize, Deserialize)]
pub struct Signatures {
    signatures: HashMap<String,String>,
}

#[derive(Serialize, Deserialize)]
pub struct Operation {
    type_op: &'static str,
    fees: HashMap <&'static str, u32>,
}

#[derive(Serialize, Deserialize)]
pub struct SetFeesRequest {
    type_txn: &'static str,
    signatures: Signatures,
    protocolVersion: u32,
    operation: Operation,
}



#[cfg(test)]
mod fees_config_test {

    use super::*;
    use std::ffi::CString;
    use utils::ffi_support::{str_from_char_ptr, cstring_from_str};
    use utils::json_conversion::{JsonSerialize};
    static test_op_json : &'static str = r#"{"fees":[["ThisIsomeBizzareDIdsgivenTOme",1001]]}"#;
    #[test]
    fn valid_request () {
        let fee :Fees = Fees {
            fees : HashMap![(String::from("ThisIsomeBizzareDIdsgivenTOme"), 1001)],
            };
        assert_eq!(fee.to_json().unwrap(), test_op_json);
    }
}