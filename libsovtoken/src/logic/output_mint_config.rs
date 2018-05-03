//
// defines structure and implementation for OutputMintConfig which is used
// for minting tokens 
//


use serde::{Serialize, Deserialize};
use utils::json_conversion::JsonDeserialize;

// TODO: do we need to account for nulls?
// ouputs consist of 
#[derive(Serialize, Deserialize)]
pub struct Output(String, u32);

#[derive(Serialize, Deserialize)]
pub struct OutputMintConfig {
    pub outputs: Vec<Output>,
}
