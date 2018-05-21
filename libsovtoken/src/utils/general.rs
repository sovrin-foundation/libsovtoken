//! general helper methods.   Dont go nuts and add everything under the sun here.
//! As a general rule if you are adding multiple functions of similar behavior (or data)
//! it should be in its own module.  keep these organized too please.



pub trait ResultExtension<A> {
    /**
     * Unwraps `Ok` or `Err` for a `Result<A, A>`
     * 
     * The `Result`'s `Ok` and `Err` need to be the same type.
    */
    fn ok_or_err(self) -> A;
}

impl<A> ResultExtension<A> for Result<A, A> {
    fn ok_or_err(self) -> A {
        return match self {
            Ok(a) => a,
            Err(a) => a,
        }
    }
}

///    Given an u8 array of 0 to any length, convert it to an Option type where
///    a zero length array becomes Option<None>
///
///    # Parmeters
///    inputs:  &[u8]
///    # Return
///    outputs: Option<&[u8]>
pub fn some_or_none_option_u8(data : &[u8]) -> Option<&[u8]> {
    if 0 == data.len() {
        return None;
    }

    return Some(data);
}


///  for a str, this trait adds string manipulation functions
///  to ease the work of dealing with strings
pub trait StringUtils {
    fn from_right(&self, count : usize) -> String;
}

/// this impl adds StringUtils to any str
impl<'a> StringUtils for &'a str {
    fn from_right(&self, count : usize) -> String {
        let len = self.chars().count();

        if len < count {
            let complete: String = self.chars().collect::<String>();
            return complete.to_owned();
        }

        let right_most: String = self.chars()
                                    .skip(len - count).take(count)
                                    .collect::<String>();

        return right_most.to_owned();
    }

}


pub mod base58 {
    use indy::api::ErrorCode;
    use rust_base58::{FromBase58};

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
}


/*
         UNIT TESTS BELOW
         (and only unit tests---do not add more functions below this mod)
*/

#[cfg(test)]
mod general_tests {

    use indy::api::ErrorCode;
    use utils::general::StringUtils;
    use utils::general::some_or_none_option_u8;
    use super::base58;

    #[test]
    fn success_empty_u8_array_becomes_option_none() {
        let data = String::new();

        let results = some_or_none_option_u8(data.as_bytes());

        assert_eq!(true, results.is_none(), "some_or_none_option_u8 failed to return None");
    }

    #[test]
    fn success_u8_array_becomes_option_some() {
        let data = "1234567890".to_string();

        let results = some_or_none_option_u8(data.as_bytes());

        assert_eq!(true, results.is_some(), "some_or_none_option_u8 failed to return Some");
    }

    #[test]
    fn success_get_right_4_chars() {
        let data = "1234567890".to_string();

        let len = data.len();
        let get_four = 4;
        let computed_result: String = data.chars().skip(len - get_four).take(get_four).collect();
        let copy = data.as_str();
        let result = copy.from_right(get_four);

        assert_eq!(computed_result, result, "from_right test failed");
    }

    #[test]
    fn success_from_right_full_string_when_exceeding_len() {
        let data = "1234567890".to_string();

        let len = data.len();
        let copy = data.as_str();
        let result = copy.from_right(75);

        assert_eq!(data, result, "from_right test failed");
    }

    fn deserialize_base58_string(serialized: &str, expected: Result<&str, ErrorCode>) {
        let serialized = String::from(serialized);
        let expected = expected.map(|deserialized| String::from(deserialized));
        assert_eq!(base58::deserialize_string(serialized), expected);
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