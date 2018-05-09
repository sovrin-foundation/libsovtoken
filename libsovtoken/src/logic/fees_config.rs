#![warn(unused_imports)]
#[allow(unused_imports)]

use serde::{Serialize, Deserialize};
use serde_json::{Value, Error};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Fees {
   pub  fees: HashMap<String, u64>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct SetFeesRequest {
    #[serde(rename = "type")]
    txn_type: &'static str,
    fees:  HashMap<String, u64>,
}

impl SetFeesRequest {
    pub fn new(outputs: HashMap<String>, did: String) -> Request<MintRequest> {
        let mint = MintRequest {
            txn_type: "20000",
            outputs: outputs,
        };
    }

}


#[cfg(test)]
mod fees_config_test {

    use super::*;
    use std::ffi::CString;
    use utils::ffi_support::{str_from_char_ptr, cstring_from_str};
    use serde_json;
    use serde_json::{Value, Error};
    use utils::json_conversion::{JsonSerialize};

    // TESTING GLOBAL VARS
    static TEST_OP_JSON: &'static str = r#"{"fees":{"ThisIsomeBizzareDIdsgivenTOme":1001}}"#;
    static TEST_SIGS_JSON: &'static str = r#"{"signatures":{"one":"two","three":"four"}}"#;
    static TEST_OPS_JSON: &'static str = r#"{"type":"FEE","fees":{"ThisIsomeBizzareDIdsgivenTOme":1001,"ThisIsomeBizzareDIdsgivenTOme1":1001}}"#;

    // fees_txn_handler requires that a valid fees transaction is serialized. This tests that
    // the serializing structure for fees works correctly


    #[test]
    fn valid_fees () {
        let mut fees_map = HashMap::new();
        fees_map.insert(String::from("ThisIsomeBizzareDIdsgivenTOme"), 1001 as u64);
        let fee :Fees = Fees {
            fees: fees_map,
            };
        assert_eq!(fee.to_json().unwrap(), TEST_OP_JSON);
    }

    // fees_txn_handler requires that a valid signature in the txn is serialized. This tests that
    // the serializing structure for signature works correctly
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

    // fees_txn_handler requires that a valid operation is serialized. This tests that
    // the serializing of the operation structure works correctly
    #[test]
    fn valid_ops () {
        let mut fees = HashMap::new();
        fees.insert(String::from("ThisIsomeBizzareDIdsgivenTOme"), 1001 as u64);
        fees.insert(String::from("ThisIsomeBizzareDIdsgivenTOme1"), 1001 as u64);

        let fee_test :Fees = Fees {
            fees,
        };
        let op : Operation = Operation {
            type_op: String::from("FEE"),
            fees: fee_test,
        };
        assert_eq!(op.to_json().unwrap(), TEST_OPS_JSON);
    }

    // fees_txn_handler requires that a valid fees fees_txn is serialized. This tests that
    // the serializing structure for a request works correctly
    #[test]
    fn valid_request () {

        let request_id : u64 = 1525718269097278;
        let protocol_version: u64 = 1001;
        let req_test = r#"{'reqId':1525718269097278,'signatures':{'CA4bVFDU4GLbX8xZju811o':'2NozDUZYJ2sxuGWmPG67j2tecyJRiDzaZSvgUp7Pkc9qzBCSeJQCgrfcX7Bs3JvTSFjYCTHGHU7XUj6DQ2wKx4ZZ', 'M9BJDuS24bqbJNvBRsoGg3':'dCPMnEYKESrPJFcGwBHwtWY9PmtB7tJYg35JLQz7jDGzfrPvTMfm462tsUC57iPkYRFmDnAhKeWigqZPFvr2hei'},'protocolVersion': 1,'operation':{'type':'20000','fees':{'10001':8,'1':4}}}"#;
        let mut fees = HashMap::new();
        let mut sig_map = HashMap::new();

        sig_map.insert(String::from("CA4bVFDU4GLbX8xZju811o"),
                       String::from("2NozDUZYJ2sxuGWmPG67j2tecyJRiDzaZSvgUp7Pkc9qzBCSeJQCgrfcX7Bs3JvTSFjYCTHGHU7XUj6DQ2wKx4ZZ"));
        sig_map.insert(String::from("M9BJDuS24bqbJNvBRsoGg3"),
                       String::from("dCPMnEYKESrPJFcGwBHwtWY9PmtB7tJYg35JLQz7jDGzfrPvTMfm462tsUC57iPkYRFmDnAhKeWigqZPFvr2hei"));

        fees.insert(String::from("10001"), 8 as u64);
        fees.insert(String::from("1"), 4 as u64);

        let signatures : Signatures = Signatures {
            signatures: sig_map,
        };

        let fee_test: Fees = Fees {
            fees,
        };

        let operation : Operation = Operation {
            type_op: String::from("FEE"),
            fees: fee_test,
        };
        let req : SetFeesRequest = SetFeesRequest {
            request_id,
            signatures,
            protocol_version,
            operation,
        };

        // 4,294,967,295
        // 18,446,744,073,709,551,615
        // 1,525,718,269,097,278
        let json_req : Result<String, Error> = serde_json::from_str(req_test);
        println!("{:?}", u64::max_value());
        assert_eq!(req.to_json().unwrap(),req_test.to_string(), "Expecting a correct Json string")
    }
}
