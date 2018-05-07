#![warn(unused_imports)]
#[allow(unused_imports)]

use serde::{Serialize, Deserialize};

type fee =  (String, u32, String)

#[derive(Serialize, Deserialize)]
pub struct Signatures {
    signatures: {did : String, sig : String},
}



#[derive(Serialize, Deserialize)]
pub struct FeesConfig {

    txn_type: u32,
    signatures: Signatures,
    fees: Vec<fee>
}