/// Methods for dealing with addresses, pub keys and private keys

use rust_base58::{ToBase58, FromBase58};
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

pub static PAYMENT_ADDRESS_QUALIFIER: &'static str = "pay:sov:";

// Following lengths are in bytes
pub const CHECKSUM_LEN: usize = 4;

pub const VERKEY_LEN: usize = 32;

// ASSUMPTION: Qualifier is always considered as ASCII, when this assumption becomes false,
// the following logic will break since byte size changes.
// TODO: It is better to have a lazy_static
pub const ADDR_QUAL_LEN: usize = 8;

pub const ADDRESS_LEN: usize = VERKEY_LEN + CHECKSUM_LEN + ADDR_QUAL_LEN;

/**
    Prefixes a verkey with "pay:sov" using the format and static data defined in this module. it does
    not check for, nor add, checksum

    Note:  this method is similar to [`verkey_checksum_from_address`] but not the same since it does
    not add the checksum

    returns fully formatted address
*/
// QUESTION: Why is this needed? Address without a checksum is not an address
pub fn verkey_to_address(verkey : &String) -> String {
    return format!("{}{}", PAYMENT_ADDRESS_QUALIFIER, verkey);
}

/**
Takes a fully qualified address and returns the unqualified address (qualifier is stripped)
*/
pub fn strip_qualifier_from_address(address : &str) -> String {
    return address.clone()[ADDR_QUAL_LEN..].to_string();
}

/**
    Extracts the verkey from an address.
    Removes the "pay:sov:" indicator and the checksum.

    ```
    use sovtoken::logic::address::verkey_from_address;
    let address = String::from("pay:sov:XrVf57oUam71eOOY1vjL1ZUm2czNV8UPekhTst9kJYLXj2yZ");
    let verkey = verkey_from_address(address).unwrap();
    assert_eq!(verkey, String::from("XrVf57oUam71eOOY1vjL1ZUm2czNV8UPekhTst9kJYLX"));
    ```
*/
// QUESTION: Why is this needed?
pub fn verkey_from_address(address: String) -> Result<String, ErrorCode> {
    validate_address(&address)
    /*let indicator_length = ADDRESS_LEN - VERKEY_LEN - CHECKSUM_LEN;
    let verkey = &address[indicator_length..VERKEY_LEN + indicator_length];
    return Ok(String::from(verkey));*/
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
// TODO Fix function name
pub fn verkey_checksum_from_address(fq_address: String) -> Result<String, ErrorCode> {
    validate_address(&fq_address)?;
    /*let indicator_length = ADDRESS_LEN - VERKEY_LEN - CHECKSUM_LEN;
    let verkey_with_checksum = &address[indicator_length..VERKEY_LEN + indicator_length + CHECKSUM_LEN];*/
    return Ok(strip_qualifier_from_address(&fq_address).to_string());
}

/** computes an unqualified (verkey+checksum) based from a verkey */
pub fn compute_unqual_address_from_verkey(verkey: &str) -> String {
    // TODO: Make function return ErrorCode
    let verkey_bytes = verkey.from_base58().unwrap();
    verkey_bytes.to_base58_check()
}

/** creates the fully formatted payment address string */
pub fn create_formatted_address_with_checksum(verkey: &str) -> String {
    let address = compute_unqual_address_from_verkey(verkey);
    return format!(
        "{}{}", PAYMENT_ADDRESS_QUALIFIER, address
    );
}

/**
    returns checksum field from address.  address must be a valid sovrin address
*/
// QUESTION: Why is this needed?
// TODO: Remove this method
/*pub fn get_checksum(address: &str) -> Result<String, ErrorCode> {
    validate_address(&String::from(address))?;
    let checksum = address.from_right(CHECKSUM_LEN);
    return Ok(String::from(checksum));
}*/

/**
   `validate_address` checks that an address is formatted
   as the following pay:sov:<verkey><checksum>, that the verkey is valid (lengthwise) and return the verkey
*/
pub fn validate_address(fully_qualified_address: &str) -> Result<String, ErrorCode> {
    if !fully_qualified_address.starts_with(&PAYMENT_ADDRESS_QUALIFIER) {
        return Err(ErrorCode::CommonInvalidStructure);
    }

    let address = strip_qualifier_from_address(&fully_qualified_address);
    match address.from_base58_check() {
        Ok(vk) => {
          if vk.len() != VERKEY_LEN {
              // TODO: Assumes checksum is 4 bytes but the base58 lib should declare a constant
              // for checksum size and this code should import that constant
              return Err(ErrorCode::CommonInvalidStructure)
          } else {
              return Ok(vk.to_base58());
          }
        },
        Err(_) => return Err(ErrorCode::CommonInvalidStructure)
    }
}

/*
    Methods "private" (aka not exported from this module)

    KEEP all public methods above
*/


#[cfg(test)]
pub mod address_tests {
    use utils::random::{rand_string, rand_bytes};

    use super::*;

    fn verkey_invalid_address_verkey_length(length: usize) {
        assert_ne!(length, VERKEY_LEN);
        let verkey = rand_string(length);
        let checksum = rand_string(CHECKSUM_LEN);
        let invalid_address = format!("pay:sov:{}{}", verkey, checksum);
        let result = verkey_from_address(invalid_address);
        let error = result.unwrap_err();
        assert_eq!(ErrorCode::CommonInvalidStructure, error);
    }

    pub fn gen_random_base58_verkey() -> String {
        let vk_bytes = rand_bytes(VERKEY_LEN);
        vk_bytes.to_base58()
    }

    pub fn gen_random_base58_address() -> String {
        let verkey = gen_random_base58_verkey();
        compute_unqual_address_from_verkey(&verkey)
    }

    #[test]
    fn test_verkey_to_address_success() {
        let vk_bytes = rand_bytes(VERKEY_LEN);
        let verkey = vk_bytes.to_base58();
        let address = compute_unqual_address_from_verkey(&verkey);
        let address_bytes = address.from_base58().unwrap();

        assert!(address.len() > verkey.len());
        assert_eq!(address_bytes.len() - vk_bytes.len(), CHECKSUM_LEN);
    }

    #[test]
    fn test_verkey_invalid_address_length_long_and_short() {
        verkey_invalid_address_verkey_length(40);
        verkey_invalid_address_verkey_length(55);
    }

    #[test]
    fn test_verkey_invalid_address_indicator() {
        let address = gen_random_base58_address();
        let invalid_address = format!("pat:sov:{}", address);
        let result = verkey_from_address(invalid_address);
        let error = result.unwrap_err();
        assert_eq!(ErrorCode::CommonInvalidStructure, error);
    }

    #[test]
    fn test_verkey_from_address() {
        let verkey = gen_random_base58_verkey();
        let address = compute_unqual_address_from_verkey(&verkey);
        let valid_fq_address = format!("pay:sov:{}", address);
        let result = verkey_from_address(valid_fq_address);
        let verkey_extracted = result.unwrap();
        assert_eq!(verkey_extracted, verkey);
    }

    #[test]
    fn test_invalid_length_verkey() {
        let vk_bytes = rand_bytes(VERKEY_LEN+1);
        let verkey = vk_bytes.to_base58();
        let address = compute_unqual_address_from_verkey(&verkey);
        let fq_address = format!("pay:sov:{}", address);
        let result = verkey_from_address(fq_address);
        let error = result.unwrap_err();
        assert_eq!(ErrorCode::CommonInvalidStructure, error);
    }

    #[test]
    fn test_strip_qualifier() {
        let address = gen_random_base58_address();
        let valid_fq_address = format!("pay:sov:{}", address);
        assert_eq!(strip_qualifier_from_address(&valid_fq_address), address);
    }

    /*#[test]
    fn test_verkey_checksum_from_address() {
        let verkey = rand_string(VERKEY_LEN);
        let checksum = rand_string(CHECKSUM_LEN);
        let valid_address = format!("pay:sov:{}{}", verkey, checksum);
        let verkey_checksum = verkey_checksum_from_address(valid_address).unwrap();
        assert_eq!(verkey_checksum, format!("{}{}", verkey, checksum));
    }*/

    #[test]
    fn test_verkey_checksum_invalid_address() {
        let address = gen_random_base58_address();
        let invalid_address = format!("pat:sov:{}", address);
        let error = verkey_checksum_from_address(invalid_address).unwrap_err();
        assert_eq!(ErrorCode::CommonInvalidStructure, error);
    }

    #[test]
    fn test_success_validate_create_formatted_address_with_checksum() {
        let address = create_formatted_address_with_checksum(&gen_random_base58_verkey());

        // got our result, if its correct, it will look something like this:
        // pay:sov:gzidfrdJtvgUh4jZTtGvTZGU5ebuGMoNCbofXGazFa91234
        // break it up into the individual parts we expect to find and
        // test the validity of the parts
        let pay_indicator = &address[0..3];
        let first_separator = &address[3..4];
        let sov_indicator = &address[4..7];
        let second_indicator = &address[7..8];
        let result_address = &address[8..];

        assert_eq!(PAY_INDICATOR, pay_indicator, "PAY_INDICATOR not found");
        assert_eq!(PAYMENT_ADDRESS_FIELD_SEP, first_separator, "first PAYMENT_ADDRESS_FIELD_SEP not found");
        assert_eq!(SOVRIN_INDICATOR, sov_indicator, "SOVRIN_INDICATOR not found");
        assert_eq!(PAYMENT_ADDRESS_FIELD_SEP, second_indicator, "second PAYMENT_ADDRESS_FIELD_SEP not found");
        assert_eq!(VERKEY_LEN + CHECKSUM_LEN, result_address.from_base58().unwrap().len(), "address is not 36 bytes");
        assert_eq!(VERKEY_LEN, result_address.from_base58_check().unwrap().len(), "verkey is not 32 bytes");
    }

    #[test]
    fn test_to_and_fro() {
        let addresses = vec!["2Viu9qrpqM48PSw3vdoQoFKP5AvYTChUZhwWtCydfW9iu7ftRt",
                                        "C1iM7fr4cT32J3DuwKDQDPLbNhN7NaEk9ex2ictk86Lg1ZKC9",
                                        "zivqx63btpvxCM2Aj7hqVMBkbB84v7aJ5xDC6MNQj7MSPFJN1",
                                        "28dLM4uKiPa2cyLuUsEpKDa8HyvcTMTmg6ji5X23eLA8jZCJAv",
                                        "TKe9eXtchV71J2qXX5HwP8rbkTBStnEEkMwQkHie265VtRSbs"];
        let verkeys = vec!["EFfodscoymgdJDuM885uEWmgCcA25P6VR6TjVqsYZLW3",
                                    "2gcGb3qbTGNc5zkdcBq9Kq4nQutptt7ofoFVRTmxAnJc",
                                    "9pdZM4dWas2WsQkiD1H57yT8qwME6T38fS2M6AwmDR2v",
                                    "B2gfDbd9EBh7Acs3x3cqgWebTApqZvuSKKhSocKzM4Cq",
                                    "52JU5iD4ryAkjpYLb58qwY48sGQZGYq3gQs1uqY3o1oz"];
        for i in 0..5 {
            let a = compute_unqual_address_from_verkey(verkeys[i]);
            assert_eq!(&a, &addresses[i]);
            let fa = format!("pay:sov:{}", &addresses[i]);
            assert_eq!(verkey_from_address(fa).unwrap(), verkeys[i])
        }
    }

    /*#[test]
    fn test_get_checksum_invalid() {
        let address = String::from("pay:sov:r3JT61jXZf0jwlq0K10SVRMj5bIA0tkF5bvP3pFpso7q8Ha");
        assert_eq!(get_checksum(&address).unwrap_err(), ErrorCode::CommonInvalidStructure);
    }

    #[test]
    fn test_get_checksum() {
        let address = String::from("pay:sov:r3JT61jXZf0jwlq0K10SMVRMj5bIA0tkF5bvP3pFpso7q8Ha");
        assert_eq!(get_checksum(&address).unwrap(), String::from("q8Ha"));
    }*/
}
