//! Payments module contains functions for working with payments.  :D

use super::payment_address_config::PaymentAddressConfig;


pub static PAY_INDICATOR: &'static str = "pay";
pub static SOVRIN_INDICATOR: &'static str = "sov";
pub static PAYMENT_ADDRESS_FIELD_SEP: &'static str = ":";

const ADDRESS_FORMAT_STR: &'static str = "{}:{}:{}";

// computes a check some based on an address
fn compute_address_checksum(address: String) -> String {
    return "1234".to_string();
}

// creates the fully formatted payment address string
fn create_formatted_address(address: String) -> String {

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

    // TODO:  generate this through libsoduim
    let address = config.seed;

    return create_formatted_address(address);
}
