use serde::{Serialize, Deserialize};
use utils::json_conversion::JsonDeserialize;

#[derive(Serialize, Deserialize)]
pub struct OutputMintConfig {
    pub txn_type : i32,
    pub addresses : Vec<String>,
}


