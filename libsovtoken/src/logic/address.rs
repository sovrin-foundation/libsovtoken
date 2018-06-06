/// Methods for dealing with addresses, pub keys and private keys

use rust_base58::ToBase58;
use indy::ErrorCode;

use utils::general::StringUtils;


// ------------------------------------------------------------------
// statics that make up parts of the payment address
// ------------------------------------------------------------------
/// = "pay"
pub static PAY_INDICATOR: &'static str = "pay";
/// = "sov"
pub static SOVRIN_INDICATOR: &'static str = "sov";
/// = ":"
pub static PAYMENT_ADDRESS_FIELD_SEP: &'static str = ":";

pub const CHECKSUM_LEN: usize = 4;

pub const VERKEY_LEN: usize = 44;

// 8 is for the pay:sov:
pub const ADDRESS_LEN: usize = VERKEY_LEN + CHECKSUM_LEN + 8;

/**
    Prefixes a verkey with "pay:sov" using the format and static data defined in this module. it does
    not check for, nor add, checksum

    Note:  this method is similar to [`verkey_checksum_from_address`] but not the same since it does
    not add the checksum

    returns fully formatted address
*/
pub fn verkey_to_address(verkey : &String) -> String {
    let indicator = sovrin_indicator();
    return format!("{}{}", indicator, verkey);
}

/**
    Extracts the verkey from an address.
    Removes the "pay:sov:" indiator and the checksum.

    ```
    use sovtoken::logic::address::verkey_from_address;
    let address = String::from("pay:sov:XrVf57oUam71eOOY1vjL1ZUm2czNV8UPekhTst9kJYLXj2yZ");
    let verkey = verkey_from_address(address).unwrap();
    assert_eq!(verkey, String::from("XrVf57oUam71eOOY1vjL1ZUm2czNV8UPekhTst9kJYLX"));
    ```
*/
pub fn verkey_from_address(address: String) -> Result<String, ErrorCode> {
    let address = validate_address(address)?;
    let indicator_length = ADDRESS_LEN - VERKEY_LEN - CHECKSUM_LEN;
    let verkey = &address[indicator_length..VERKEY_LEN + indicator_length];
    return Ok(String::from(verkey));
}

/**
    Removes the "pay:sov:".
    Leaves the verkey with the checksum.

    ```
    use sovtoken::logic::address::verkey_checksum_from_address;
    let address = String::from("pay:sov:XrVf57oUam71eOOY1vjL1ZUm2czNV8UPekhTst9kJYLXj2yZ");
    let verkey_checksum = verkey_checksum_from_address(address).unwrap();
    assert_eq!(verkey_checksum, String::from("XrVf57oUam71eOOY1vjL1ZUm2czNV8UPekhTst9kJYLXj2yZ"));
    ```
*/
pub fn verkey_checksum_from_address(address: String) -> Result<String, ErrorCode> {
    let address = validate_address(address)?;
    let indicator_length = ADDRESS_LEN - VERKEY_LEN - CHECKSUM_LEN;
    let verkey_with_checksum = &address[indicator_length..VERKEY_LEN + indicator_length + CHECKSUM_LEN];
    return Ok(String::from(verkey_with_checksum));
}

/** computes a checksum based on an address */
pub fn compute_address_checksum(address: String) -> String {
    let address_bytes = address.into_bytes();
    let with_checksum = address_bytes.to_base58_check();
    let check_sum =  with_checksum.as_str().from_right(CHECKSUM_LEN);
    return check_sum;
}

/** creates the fully formatted payment address string */
pub fn create_formatted_address_with_checksum(verkey: String) -> String {
    let indicator = sovrin_indicator();
    let checksum = compute_address_checksum(verkey.clone());
    return format!(
        "{}{}{}", indicator, verkey, checksum
    );
}

/**
    returns checksum field from address.  address must be a valid sovrin address
*/
pub fn get_checksum(address: &str) -> Result<String, ErrorCode> {
    validate_address(String::from(address))?;
    let checksum = address.from_right(CHECKSUM_LEN);
    return Ok(String::from(checksum));
}

/**
   `validate_address` simply checks that an address is formatted
   as the following pay:sov:<address><checksum>
*/
pub fn validate_address(address: String) -> Result<String, ErrorCode> {
    let indicator = sovrin_indicator();
    if !address.starts_with(&indicator) {
        return Err(ErrorCode::CommonInvalidStructure);
    }

    if address.len() != ADDRESS_LEN {
        return Err(ErrorCode::CommonInvalidStructure)
    }

    return Ok(address);
}

/*
    Methods "private" (aka not exported from this module)

    KEEP all public methods above
*/

fn sovrin_indicator() -> String {
    return format!(
        "{}{separator}{}{separator}",
        PAY_INDICATOR,
        SOVRIN_INDICATOR,
        separator = PAYMENT_ADDRESS_FIELD_SEP,
    );
}

#[cfg(test)]
mod address_tests {
    use utils::random::rand_string;

    use super::*;

    fn verkey_invalid_address_verkey_length(length: usize) {
        assert!(length != VERKEY_LEN);
        let verkey = rand_string(length);
        let checksum = rand_string(CHECKSUM_LEN);
        let invalid_address = format!("pay:sov:{}{}", verkey, checksum);
        let result = verkey_from_address(invalid_address);
        let error = result.unwrap_err();
        assert_eq!(ErrorCode::CommonInvalidStructure, error);
    }

    #[test]
    fn test_verkey_to_address_success() {
        let verkey = rand_string(VERKEY_LEN);
        let address = verkey_to_address(&verkey);

        let verkey_len = verkey.chars().count();
        let address_len = address.chars().count();

        assert_eq!(8, address_len - verkey_len);
    }

    #[test]
    fn test_verkey_invalid_address_length_long_and_short() {
        verkey_invalid_address_verkey_length(40);
        verkey_invalid_address_verkey_length(50);
    }

    #[test]
    fn test_verkey_invalid_address_indicator() {
        let verkey = rand_string(VERKEY_LEN);
        let checksum = rand_string(CHECKSUM_LEN);
        let invalid_address = format!("pat:sov:{}{}", verkey, checksum);
        let result = verkey_from_address(invalid_address);
        let error = result.unwrap_err();
        assert_eq!(ErrorCode::CommonInvalidStructure, error);
    }

    #[test]
    fn test_verkey_from_address() {
        let verkey = rand_string(VERKEY_LEN);
        let checksum = rand_string(CHECKSUM_LEN);
        let valid_address = format!("pay:sov:{}{}", verkey, checksum);
        let result = verkey_from_address(valid_address);
        let verkey_extracted = result.unwrap();
        assert_eq!(verkey_extracted, verkey);
    }

    #[test]
    fn test_verkey_checksum_from_address() {
        let verkey = rand_string(VERKEY_LEN);
        let checksum = rand_string(CHECKSUM_LEN);
        let valid_address = format!("pay:sov:{}{}", verkey, checksum);
        let verkey_checksum = verkey_checksum_from_address(valid_address).unwrap();
        assert_eq!(verkey_checksum, format!("{}{}", verkey, checksum));
    }

    #[test]
    fn test_verkey_checksum_invalid_address() {
        let verkey = rand_string(VERKEY_LEN);
        let checksum = rand_string(CHECKSUM_LEN);
        let invalid_address = format!("pat:sov:{}{}", verkey, checksum);
        let error = verkey_checksum_from_address(invalid_address).unwrap_err();
        assert_eq!(ErrorCode::CommonInvalidStructure, error);
    }

    #[test]
    fn test_success_validate_create_formatted_address_with_checksum() {
        let address = create_formatted_address_with_checksum(rand_string(VERKEY_LEN));

        // got our result, if its correct, it will look something like this:
        // pay:sov:gzidfrdJtvgUh4jZTtGvTZGU5ebuGMoNCbofXGazFa91234
        // break it up into the individual parts we expect to find and
        // test the validity of the parts
        let pay_indicator = &address[0..3];
        let first_separator = &address[3..4];
        let sov_indicator = &address[4..7];
        let second_indicator = &address[7..8];
        let result_address = &address[8..52];

        let checksum: String = get_checksum(&address).unwrap();

        assert_eq!(PAY_INDICATOR, pay_indicator, "PAY_INDICATOR not found");
        assert_eq!(PAYMENT_ADDRESS_FIELD_SEP, first_separator, "first PAYMENT_ADDRESS_FIELD_SEP not found");
        assert_eq!(SOVRIN_INDICATOR, sov_indicator, "SOVRIN_INDICATOR not found");
        assert_eq!(PAYMENT_ADDRESS_FIELD_SEP, second_indicator, "second PAYMENT_ADDRESS_FIELD_SEP not found");
        assert_eq!(VERKEY_LEN, result_address.chars().count(), "address is not 32 bytes");
        assert_eq!(CHECKSUM_LEN, checksum.len(), "checksum is not 4 bytes");
    }

    #[test]
    fn test_get_checksum_invalid() {
        let address = String::from("pay:sov:r3JT61jXZf0jwlq0K10SVRMj5bIA0tkF5bvP3pFpso7q8Ha");
        assert_eq!(get_checksum(&address).unwrap_err(), ErrorCode::CommonInvalidStructure);
    }

    #[test]
    fn test_get_checksum() {
        let address = String::from("pay:sov:r3JT61jXZf0jwlq0K10SMVRMj5bIA0tkF5bvP3pFpso7q8Ha");
        assert_eq!(get_checksum(&address).unwrap(), String::from("q8Ha"));
    }
}
