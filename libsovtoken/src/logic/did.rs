/*!
    DID (Decentralized Identifier)
*/

use libc::c_char;
use utils::ffi_support::str_from_char_ptr;
use std::char;
use std::error::Error;
use std::fmt;


/**
    A struct which holds the did.

    The did needs to be between 20 and 21 characters and contain only
    alphanumeric characters.
*/
#[derive(Debug, PartialEq, Eq)]
pub struct Did<'a>(&'a str);

impl<'a> Did<'a> {
    const LENGTH_LOWER_BOUND: usize = 20;
    const LENGTH_HIGHER_BOUND: usize = 21;

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
            use sovtoken::logic::did::Did;
            use sovtoken::logic::did::DidError;
        
            let did_invalid = Did::new("123456789[01234567890");
            let error = did_invalid.validate().unwrap_err();
            assert_eq!(DidError::InvalidChar('['), error);
        ```
    */
    pub fn validate(self) -> Result<Self, DidError> {
        let Did(did_string) = self;
        let did_length = did_string.len();

        if did_length < Did::LENGTH_LOWER_BOUND || did_length > Did::LENGTH_HIGHER_BOUND {
            return Err(DidError::InvalidLength(did_length));
        }

        if let Some(c) = did_string.chars().find(|c| ! char::is_alphanumeric(*c)) {
            return Err(DidError::InvalidChar(c));
        }

        return Ok(self);
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


#[cfg(test)]
mod test_did_validation {
    use super::*;
    use utils::ffi_support::c_pointer_from_str;
    use std::ptr;

    #[test]
    fn did_invalid_length_short() {
        assert_eq!(Err(DidError::InvalidLength(19)), Did::new("0123456789abcdefghi").validate());
    }

    #[test]
    fn did_invalid_length_long() {
        assert_eq!(Err(DidError::InvalidLength(22)), Did::new("0123456789abcdefghijkl").validate());
    }

    #[test]
    fn did_invalid_char() {
        assert_eq!(Err(DidError::InvalidChar('!')), Did::new("0123456789abcd!efghij").validate());
    }

    #[test]
    fn did_valid_length_20() {
        assert!(Did::new("0123456789abcdefghij").validate().is_ok());
    }

    #[test]
    fn did_valid_length_21() {
        assert!(Did::new("0123456789abcdefghijk").validate().is_ok());
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
        let pointer = c_pointer_from_str("0123456789abcdefghijk");
        assert!(Did::from_pointer(pointer).unwrap().validate().is_ok());
    }
}