//
// defines structure and implementation for PaymentAddressConfig which is used
// for generating payment addresses
//

#![warn(unused_imports)]
#[allow(unused_imports)]


use serde::{Serialize, Deserialize};

// The config structure maps to the config json structure
// used to serialize input via serde and use the data in our logic
//
// The seed should be 32 bytes, thats what libsodium requires. Seed can be optional, in that case libsodium generates a random 32 byte seed
//
// TODO: do we need to make the data private and add acccessors?
#[derive(Serialize, Deserialize)]
pub struct PaymentAddressConfig {
    pub seed : String,
}

