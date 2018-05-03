

#![warn(unused_imports)]
#[allow(unused_imports)]

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct OutputMintConfig {
    pub txn_type : i32,
    pub addresses : Vec<String>,
}


