/*!
    DID (Decentralized Identifier)
*/

use libc::c_char;
use std::char;
use regex::Regex;

use bs58::decode::DecodeError;
use utils::ffi_support::str_from_char_ptr;
use utils::base58::{FromBase58};

// Fully qualified DID pattern
lazy_static! {
    pub static ref REGEX: Regex = Regex::new("^did:([a-z0-9]+:)([a-zA-Z0-9:.-_]*)").unwrap();
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

/**
    A struct which holds the did.

    The did needs to be between 20 and 21 characters and contain only
    alphanumeric characters.
*/
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Did(String);

impl Did {
    const METHOD: &'static str = "sov";

    pub fn new(did_string: String) -> Self {
        return Did(did_string);
    }

    pub fn from_pointer(pointer: *const c_char) -> Option<Self> {
        return str_from_char_ptr(pointer).map(|st| st.to_string()).map(Self::new);
    }

    pub fn unqualify(self) -> Did {
        match REGEX.captures(&self.0) {
            None => self,
            Some(caps) => {
                let method = caps.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
                if method.starts_with(Did::METHOD) {
                    caps.get(2).map(|m| Did::new(m.as_str().to_string())).unwrap_or(self)
                }
                else {
                    self
                }
            }
        }
    }

    pub fn is_fully_qualified(&self) -> bool {
        REGEX.is_match(&self.0)
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
            
                let did_invalid = Did::new("123456789[11234567891".to_string());
                let error = did_invalid.validate().unwrap_err();
                assert_eq!(DidError::InvalidChar('['), error);
            # }
        ```
    */
    pub fn validate(self) -> Result<Self, DidError> {
        if self.is_fully_qualified(){
            return Ok(self)
        }

        let (res_did, len) = {
            let did_string = &self.0;
            (did_string.from_base58().map_err(map_err_err!()), did_string.len())
        };

        match res_did {
            Ok(ref vec) if vec.len() == 32 || vec.len() == 16 => Ok(self),
            Ok(ref vec) => Err(DidError::InvalidLength(vec.len())),
            Err(DecodeError::InvalidCharacter {character: b, index: _}) => Err(DidError::InvalidChar(b as char)),
            Err(DecodeError::NonAsciiCharacter {index: _}) => Err(DidError::InvalidChar(0 as char)),
            Err(_) => Err(DidError::InvalidLength(len))
        }
    }
}

impl From<Did> for String {
    fn from(did: Did) -> String {
        return String::from(did.0);
    }
}


#[cfg(test)]
mod test_did_validation {

    use std::error::Error;
    use std::fmt;
    use std::ptr;

    use utils::base58::IntoBase58;
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
        assert_eq!(Err(DidError::InvalidLength(17)), Did::new("1123456789abcdef1".as_bytes().into_base58()).validate());
    }

    #[test]
    fn did_invalid_char() {
        assert_eq!(Err(DidError::InvalidChar('!')), Did::new("123456789abcd!efghij".to_string()).validate());
    }

    #[test]
    fn did_valid_length_16() {
        assert!(Did::new("1123456789abcdef".as_bytes().into_base58()).validate().is_ok());
    }

    #[test]
    fn did_valid_length_32() {
        assert!(Did::new("1123456789abcdef1123456789abcdef".as_bytes().into_base58()).validate().is_ok());
    }

    #[test]
    fn did_invalid_deserialize_null_ptr() {
        let pointer = ptr::null();
        assert!(Did::from_pointer(pointer).is_none());
    }

    #[test]
    fn did_invalid_deserialize() {
        let pointer = c_pointer_from_str("123456789abcd!efghij");
        assert_eq!(Err(DidError::InvalidChar('!')), Did::from_pointer(pointer).unwrap().validate());
    }

    #[test]
    fn did_valid_deserialize() {
        let pointer = c_pointer_from_str("1123456789abcdefghijk");
        assert!(Did::from_pointer(pointer).unwrap().validate().is_ok());
    }
}