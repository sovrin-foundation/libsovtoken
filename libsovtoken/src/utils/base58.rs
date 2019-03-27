//!  Base58 helper functions


use ErrorCode;
use bs58;
use bs58::decode::DecodeError;
/**
    Deserializes a base58 String object with checksum.

    Errors: `ErrorCode::CommonInvalidStructure`.
*/
// Question: Why dont we use this?
pub fn deserialize_b58_check_string(s: String) -> Result<String, ErrorCode> {
    let deserialized_bytes = bs58::decode(&s)
        .with_check(None)
        .into_vec()
        .map_err(|_| ErrorCode::CommonInvalidStructure)?;

    return String::from_utf8(deserialized_bytes)
        .map_err(|_| ErrorCode::CommonInvalidStructure);
}

/**
    converts a u8 array (bytes) into String
*/
pub fn serialize_bytes(bytes: &[u8]) -> String {
    bs58::encode(bytes).into_string()
}

pub trait FromBase58 {
    fn from_base58(&self) -> Result<Vec<u8>, DecodeError>;
    fn from_base58_check(&self) -> Result<Vec<u8>, DecodeError>;
}

impl<I: AsRef<[u8]>> FromBase58 for I {

    fn from_base58(&self) -> Result<Vec<u8>, DecodeError> {
        bs58::decode(self).into_vec()
    }

    fn from_base58_check(&self) -> Result<Vec<u8>, DecodeError> {
        bs58::decode(self).with_check(None).into_vec()
    }
}


pub trait IntoBase58 {
    fn into_base58(&self) -> String;
    fn into_base58_check(&self) -> String;
}

impl<I: AsRef<[u8]>> IntoBase58 for I {
    fn into_base58(&self) -> String {
        bs58::encode(self).into_string()
    }

    fn into_base58_check(&self) -> String {
        bs58::encode(self).with_check().into_string()
    }
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
        assert_eq!(deserialize_b58_check_string(serialized), expected);
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