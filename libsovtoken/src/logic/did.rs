/*!
    DID (Decentralized Identifier)
*/

use libc::c_char;
use std::char;
use rust_base58::{FromBase58Error, FromBase58};
use utils::ffi_support::str_from_char_ptr;


/**
    A struct which holds the did.

    The did needs to be between 20 and 21 characters and contain only
    alphanumeric characters.
*/
#[derive(Debug, PartialEq, Eq)]
pub struct Did<'a>(&'a str);

impl<'a> Did<'a> {

    pub fn new(did_string: &'a str) -> Self {
        return Did(did_string);
    }

    pub fn from_pointer(pointer: *const c_char) -> Option<Self> {
        return str_from_char_ptr(pointer).map(Self::new);
    }

    /**
         Validate the did

         Validates that the did is a length of 20 to 21 and that it only contains
         alphanumeric characters.

        ```
            # extern crate sovtoken;
            # fn main() {
                use sovtoken::logic::did::Did;
                use sovtoken::logic::did::DidError;
            
                let did_invalid = Did::new("123456789[11234567891");
                let error = did_invalid.validate().unwrap_err();
                assert_eq!(DidError::InvalidChar('['), error);
            # }
        ```
    */
    pub fn validate(self) -> Result<Self, DidError> {
        let Did(did_string) = self;
        let res_did = did_string.from_base58().map_err(map_err_err!());

        match res_did {
            Ok(ref vec) if vec.len() == 32 || vec.len() == 16 => Ok(self),
            Ok(ref vec) => Err(DidError::InvalidLength(vec.len())),
            Err(FromBase58Error::InvalidBase58Byte(b, _)) => Err(DidError::InvalidChar(b as char)),
            Err(_) => Err(DidError::InvalidLength(did_string.len()))
        }
    }
}

impl<'a> From<Did<'a>> for String {
    fn from(did: Did<'a>) -> String {
        return String::from(did.0);
    }
}


/**
    Enum which holds possible errors with the did.

    The possible errors include:
    - `DidError::InvalidLength<usize>`
    - `DidError::InvalidChar<char>`
*/
#[derive(Debug, PartialEq, Eq)]
pub enum DidError {
    InvalidLength(usize),
    InvalidChar(char),
}


#[cfg(test)]
mod test_did_validation {

    use rust_base58::ToBase58;
    use std::error::Error;
    use std::fmt;
    use std::ptr;

    use utils::ffi_support::c_pointer_from_str;
    use super::*;

    impl fmt::Display for DidError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            return write!(f, "{}", self);
        }
    }

    impl Error for DidError {
        fn description(&self) -> &str {
            match self {
                &DidError::InvalidLength(_) => "Invalid did length.",
                &DidError::InvalidChar(_) => "Invalid char in did.",
            }
        }
    }

    #[test]
    fn did_invalid_length() {
        assert_eq!(Err(DidError::InvalidLength(17)), Did::new(&"1123456789abcdef1".as_bytes().to_base58()).validate());
    }

    #[test]
    fn did_invalid_char() {
        assert_eq!(Err(DidError::InvalidChar('!')), Did::new("0123456789abcd!efghij").validate());
    }

    #[test]
    fn did_valid_length_16() {
        assert!(Did::new(&"1123456789abcdef".as_bytes().to_base58()).validate().is_ok());
    }

    #[test]
    fn did_valid_length_32() {
        assert!(Did::new(&"1123456789abcdef1123456789abcdef".as_bytes().to_base58()).validate().is_ok());
    }

    #[test]
    fn did_invalid_deserialize_null_ptr() {
        let pointer = ptr::null();
        assert!(Did::from_pointer(pointer).is_none());
    }

    #[test]
    fn did_invalid_deserialize() {
        let pointer = c_pointer_from_str("0123456789abcd!efghij");
        assert_eq!(Err(DidError::InvalidChar('!')), Did::from_pointer(pointer).unwrap().validate());
    }

    #[test]
    fn did_valid_deserialize() {
        let pointer = c_pointer_from_str("1123456789abcdefghijk");
        assert!(Did::from_pointer(pointer).unwrap().validate().is_ok());
    }
}