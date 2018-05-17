/// Methods for dealing with addresses, pub keys and private keys

use rust_base58::ToBase58;
use indy::api::ErrorCode;

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

pub static CHECKSUM_LEN: usize = 4;

pub static VALID_ADDRESS_LEN: usize = 32;


/**
    Extracts the verkey from an address.
    Removes the "pay:sov:" indiator and the checksum.

    ```
    use sovtoken::logic::address::verkey_from_address;
    let address = String::from("pay:sov:tsnhvjruaskqncfyeponHJdkeuxAejdijdeA");
    let verkey = verkey_from_address(address).unwrap();
    assert_eq!(verkey, String::from("tsnhvjruaskqncfyeponHJdkeuxAejdi"));
    ```
*/
pub fn verkey_from_address(address: String) -> Result<String, ErrorCode> {
    let address = validate_address(address)?;
    let verkey = &address[8..40];
    return Ok(String::from(verkey));
}

/** computes a checksum based on an address */
pub fn compute_address_checksum(address: String) -> String {
    let address_bytes = address.into_bytes();
    let with_checksum = address_bytes.to_base58();
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

pub fn get_checksum(address: &str) -> Result<String, ErrorCode> {
    validate_address(String::from(address))?;
    let checksum = address.from_right(CHECKSUM_LEN);
    return Ok(String::from(checksum));
}

fn sovrin_indicator() -> String {
    return format!(
        "{}{separator}{}{separator}",
        PAY_INDICATOR,
        SOVRIN_INDICATOR,
        separator = PAYMENT_ADDRESS_FIELD_SEP,
    );
}

fn validate_address(address: String) -> Result<String, ErrorCode> {
    let indicator = sovrin_indicator();
    if !address.starts_with(&indicator) {
        return Err(ErrorCode::CommonInvalidStructure);
    }

    // 44 is address length;
    if address.len() != VALID_ADDRESS_LEN + CHECKSUM_LEN + indicator.len() {
        return Err(ErrorCode::CommonInvalidStructure)
    }

    return Ok(address);
}


#[cfg(test)]
mod address_tests {
    use utils::random::rand_string;

    use super::*;

    fn verkey_invalid_address_length(length: usize) {
        assert!(length != VALID_ADDRESS_LEN);
        let verkey = rand_string(length);
        let checksum = rand_string(CHECKSUM_LEN);
        let invalid_address = format!("pay:sov:{}{}", verkey, checksum);
        let result = verkey_from_address(invalid_address);
        let error = result.unwrap_err();
        assert_eq!(ErrorCode::CommonInvalidStructure, error);
    }

    #[test]
    fn test_verkey_invalid_address_length_long_and_short() {
        verkey_invalid_address_length(30);
        verkey_invalid_address_length(40);
    }

    #[test]
    fn test_verkey_invalid_address_indicator() {
        let verkey = rand_string(VALID_ADDRESS_LEN);
        let checksum = rand_string(CHECKSUM_LEN);
        let invalid_address = format!("pat:sov:{}{}", verkey, checksum);
        let result = verkey_from_address(invalid_address);
        let error = result.unwrap_err();
        assert_eq!(ErrorCode::CommonInvalidStructure, error);
    }

    #[test]
    fn test_verkey_from_address() {
        let verkey = rand_string(VALID_ADDRESS_LEN);
        let checksum = rand_string(CHECKSUM_LEN);
        let valid_address = format!("pay:sov:{}{}", verkey, checksum);
        let result = verkey_from_address(valid_address);
        let verkey_extracted = result.unwrap();
        assert_eq!(verkey_extracted, verkey);
    }

    #[test]
    fn test_success_validate_create_formatted_address_with_checksum() {
        let address = create_formatted_address_with_checksum(rand_string(32));

        // got our result, if its correct, it will look something like this:
        // pay:sov:gzidfrdJtvgUh4jZTtGvTZGU5ebuGMoNCbofXGazFa91234
        // break it up into the individual parts we expect to find and
        // test the validity of the parts
        let pay_indicator = &address[0..3];
        let first_separator = &address[3..4];
        let sov_indicator = &address[4..7];
        let second_indicator = &address[7..8];
        let result_address = &address[8..40];

        let checksum: String = get_checksum(&address).unwrap();

        assert_eq!(PAY_INDICATOR, pay_indicator, "PAY_INDICATOR not found");
        assert_eq!(PAYMENT_ADDRESS_FIELD_SEP, first_separator, "first PAYMENT_ADDRESS_FIELD_SEP not found");
        assert_eq!(SOVRIN_INDICATOR, sov_indicator, "SOVRIN_INDICATOR not found");
        assert_eq!(PAYMENT_ADDRESS_FIELD_SEP, second_indicator, "second PAYMENT_ADDRESS_FIELD_SEP not found");
        assert_eq!(VALID_ADDRESS_LEN, result_address.chars().count(), "address is not 32 bytes");
        assert_eq!(CHECKSUM_LEN, checksum.len(), "checksum is not 4 bytes");
    }

    #[test]
    fn test_get_checksum_invalid() {
        let address = String::from("pay:sov:tsnhvjruaskqncfyeonHJdkeuxAejdijdeA");
        assert_eq!(get_checksum(&address).unwrap_err(), ErrorCode::CommonInvalidStructure);
    }

    #[test]
    fn test_get_checksum() {
        let address = String::from("pay:sov:tsnhvjruaskqncfyeponHJdkeuxAejdijdeA");
        assert_eq!(get_checksum(&address).unwrap(), String::from("jdeA"));
    }
}