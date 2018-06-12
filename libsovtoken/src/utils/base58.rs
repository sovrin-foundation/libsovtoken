//!  Base58 helper functions


use indy::ErrorCode;
use rust_base58::{FromBase58, ToBase58};

/**
    Deserializes a base58 String object with checksum.

    Errors: `ErrorCode::CommonInvalidStructure`.
*/
pub fn deserialize_string(s: String) -> Result<String, ErrorCode> {
    let deserialized_bytes = s
        .into_bytes()
        .from_base58_check()
        .map_err(|_| ErrorCode::CommonInvalidStructure)?;
    return String::from_utf8(deserialized_bytes)
        .map_err(|_| ErrorCode::CommonInvalidStructure);
}

/**
    converts a u8 array (bytes) into String
*/
pub fn serialize_bytes(bytes: &[u8]) -> String {
    return bytes.to_base58()
}


/*
         UNIT TESTS BELOW
         (and only unit tests---do not add more functions below this mod)
*/

#[cfg(test)]
mod base58_tests {

    use super::*;

    // this is a helper method used by the tests below
    fn deserialize_base58_string(serialized: &str, expected: Result<&str, ErrorCode>) {
        let serialized = String::from(serialized);
        let expected = expected.map(|deserialized| String::from(deserialized));
        assert_eq!(deserialize_string(serialized), expected);
    }

    #[test]
    fn deserialize_invalid_base58_string() {
        deserialize_base58_string("3NbSEAfMyPeDTppHLeehRonkVwi537H9YFCvV", Err(ErrorCode::CommonInvalidStructure));
    }

    #[test]
    fn deserialize_valid_base58_string_invalid_checksum() {
        deserialize_base58_string("3NbSEAfMyPeDeKn6mTppHLkVwi537H9YFdeV", Err(ErrorCode::CommonInvalidStructure));
    }

    #[test]
    fn deserialize_valid_base58_string_valid_checksum() {
        deserialize_base58_string("3NbSEAfMyPeDeKn6mTppHLkVwi537H9YFCvV", Ok("My base58 test string."));
    }

}