//! Payments module contains functions for working with payments.  :D

use std::str;
use log;

use super::payment_address_config::PaymentAddressConfig;
use libraries::sodium::{CryptoEngine, CryptoError};
use libraries::rust_base58::Base58;
use utils::general::some_or_none_option_u8;

// statics that make up parts of the payment address
pub static PAY_INDICATOR: &'static str = "pay";
pub static SOVRIN_INDICATOR: &'static str = "sov";
pub static PAYMENT_ADDRESS_FIELD_SEP: &'static str = ":";

// computes a check some based on an address
fn compute_address_checksum(address: String) -> String {
    return "1234".to_string();
}

// creates the fully formatted payment address string
fn create_formatted_address_with_checksum(address: String) -> String {

    let mut result: String = PAY_INDICATOR.to_owned();

    result.push_str(PAYMENT_ADDRESS_FIELD_SEP);
    result.push_str(SOVRIN_INDICATOR);
    result.push_str(PAYMENT_ADDRESS_FIELD_SEP);
    result.push_str(&address);
    result.push_str(&compute_address_checksum(address));

    return result;
}

// creates fully formatted address based on inputted seed.  If seed is empty
// then a randomly generated seed is used by libsodium
// the format of the return is:
//     pay:sov:{32 byte address}{4 byte checksum}
pub fn create_payment_address(config: PaymentAddressConfig) -> String {

    // TODO: how should we handle errors other than panic?
    let usable_seed = some_or_none_option_u8(config.seed.as_bytes());
    let (pub_address, private_key) = match CryptoEngine::create_key_pair_for_signature(usable_seed)
    {
        Ok(r) => r,
        Err(e) =>  {
            error!("error creating keys with seed '{}'", config.seed);
            panic!("crypto error: {:?}", e);
        },
    };

    let pub_address_str = Base58::encode(&pub_address);

    return create_formatted_address_with_checksum(pub_address_str.to_string());
}
