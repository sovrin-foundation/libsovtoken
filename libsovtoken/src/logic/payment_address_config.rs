//
//
//

use serde::{Serialize, Deserialize};
use utils::json_conversion::JsonDeserialize;

// The config structure maps to the config json structure
// used to serialize input via serde and use the data in our logic
// TODO: do we need to make the data private and add acccessors?
#[derive(Serialize, Deserialize)]
pub struct PaymentAddressConfig {
    pub seed : String,
}

