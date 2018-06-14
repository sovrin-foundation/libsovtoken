use libc::c_char;
use utils::ffi_support::str_from_char_ptr;
use std::char;
use std::error::Error;
use std::fmt;

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
    * Validate did
    *
    * Validates that the did is a length of 20 to 21 and that it only contains
    * alphanumeric characters.
    */
    pub fn validate(self) -> Result<Self, DidErrors> {
        let Did(did_string) = self;
        let did_length = did_string.len();

        if did_length < Did::LENGTH_LOWER_BOUND || did_length > Did::LENGTH_HIGHER_BOUND {
            return Err(DidErrors::InvalidLength(did_length));
        }

        if let Some(c) = did_string.chars().find(|c| ! char::is_alphanumeric(*c)) {
            return Err(DidErrors::InvalidChar(c));
        }

        return Ok(self);
    }
}

impl<'a> Into<String> for Did<'a> {
    fn into(self) -> String {
        return String::from(self.0);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum DidErrors {
    InvalidLength(usize),
    InvalidChar(char),
}

impl fmt::Display for DidErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self);
    }
}

impl Error for DidErrors {
    fn description(&self) -> &str {
        match self {
            DidErrors::InvalidLength(_) => "Invalid did length.",
            DidErrors::InvalidChar(_) => "Invalid char in did.",
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
        assert_eq!(Err(DidErrors::InvalidLength(19)), Did::new("0123456789abcdefghi").validate());
    }

    #[test]
    fn did_invalid_length_long() {
        assert_eq!(Err(DidErrors::InvalidLength(22)), Did::new("0123456789abcdefghijkl").validate());
    }

    #[test]
    fn did_invalid_char() {
        assert_eq!(Err(DidErrors::InvalidChar('!')), Did::new("0123456789abcd!efghij").validate());
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
        assert_eq!(Err(DidErrors::InvalidChar('!')), Did::from_pointer(pointer).unwrap().validate());
    }

    #[test]
    fn did_valid_deserialize() {
        let pointer = c_pointer_from_str("0123456789abcdefghijk");
        assert!(Did::from_pointer(pointer).unwrap().validate().is_ok());
    }
}