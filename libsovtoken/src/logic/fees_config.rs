#![warn(unused_imports)]
#[allow(unused_imports)]

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

//type Fee =  (String, u32, String);

#[derive(Serialize, Deserialize)]
pub struct Fees {
   pub fees: HashMap<String, u32>,
}

#[derive(Serialize, Deserialize)]
pub struct Signatures {
    signatures: HashMap<String,String>,
}

#[derive(Serialize, Deserialize)]
pub struct Operation {
    #[serde(rename = "type")]
    type_op: String,
    #[serde(flatten)]
    fees: Fees,
}

#[derive(Serialize, Deserialize)]
pub struct SetFeesRequest {
    #[serde(rename = "type")]
    request_id: u32,
    signatures: Signatures,
    protocol_version: u32,
    operation: Operation,
}



#[cfg(test)]
mod fees_config_test {

    use super::*;
    use std::ffi::CString;
    use utils::ffi_support::{str_from_char_ptr, cstring_from_str};
    use utils::json_conversion::{JsonSerialize};
    static TEST_OP_JSON: &'static str = r#"{"fees":{"ThisIsomeBizzareDIdsgivenTOme":1001}}"#;
    static TEST_SIGS_JSON: &'static str = r#"{"signatures":{"one":"two","three":"four"}}"#;
    static TEST_OPS_JSON: &'static str = r#"{"type":"FEE","fees":{"ThisIsomeBizzareDIdsgivenTOme":1001,"ThisIsomeBizzareDIdsgivenTOme1":1001}}"#;
    #[test]
    fn valid_fees () {
        let mut fees_map = HashMap::new();
        fees_map.insert(String::from("ThisIsomeBizzareDIdsgivenTOme"), 1001 as u32);
        let fee :Fees = Fees {
            fees: fees_map,
            };
        assert_eq!(fee.to_json().unwrap(), TEST_OP_JSON);
    }
    #[test]
    fn valid_signatures () {
        let mut sig_map = HashMap::new();
        sig_map.insert(String::from("one"), String::from("two"));
        sig_map.insert(String::from("three"), String::from("four"));

        let sig : Signatures = Signatures {
            signatures: sig_map,
        };
        assert_eq!(sig.to_json().unwrap(), TEST_SIGS_JSON);
    }
    #[test]
    fn valid_ops () {
        let mut fees = HashMap::new();
        fees.insert(String::from("ThisIsomeBizzareDIdsgivenTOme"), 1001 as u32);
        fees.insert(String::from("ThisIsomeBizzareDIdsgivenTOme1"), 1001 as u32);

        let fee_test :Fees = Fees {
            fees,
        };
        let op : Operation = Operation {
            type_op: String::from("FEE"),
            fees: fee_test,
        };
        assert_eq!(op.to_json().unwrap(), TEST_OPS_JSON);
    }

    #[test]
    fn valid_request () {
        let mut fees = HashMap::new();

        let mut sig_map = HashMap::new();
        sig_map.insert(String::from("one"), String::from("two"));
        sig_map.insert(String::from("three"), String::from("four"));

        let signatures : Signatures = Signatures {
            signatures: sig_map,
        };

        fees.insert(String::from("ThisIsomeBizzareDIdsgivenTOme"), 1001 as u32);
        fees.insert(String::from("ThisIsomeBizzareDIdsgivenTOme1"), 1001 as u32);
        let fee_test :Fees = Fees {
            fees,
        };

        let operation : Operation = Operation {
            type_op: String::from("FEE"),
            fees: fee_test,
        };

        let request_id : u32 = 101010299102190291029;
        let protocol_version: u32 = 1001;
        let req : SetFeesRequest = Request {
            type_txn,
            signatures,
            protocol_version,
            operation,
        };
        
    }

}