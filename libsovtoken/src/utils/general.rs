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

/**
    `validate_did_len` expects a did and then validates that
    it is the correct length
*/
pub fn validate_did_len(submitter_did : &str) -> bool {
    let did_len = submitter_did.len();
    if did_len != 22 && did_len != 21 {
        return false;
    }
    true
}


/*
         UNIT TESTS BELOW
         (and only unit tests---do not add more functions below this mod)
*/

#[cfg(test)]
mod general_tests {

    use super::validate_did_len;
    use utils::general::{StringUtils, some_or_none_option_u8};
    use utils::random::rand_string;


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

    #[test]
    fn success_validate_did_len_22() {
        let did: String = rand_string(22);

        assert_eq!(true, validate_did_len(&did), "DID of len 22 should have passed");
    }

    #[test]
    fn success_validate_did_len_21() {
        let did: String = rand_string(21);

        assert_eq!(true, validate_did_len(&did), "DID of len 21 should have passed");
    }

    #[test]
    fn fails_validate_did_len_23() {
        let did: String = rand_string(23);

        assert_eq!(false, validate_did_len(&did), "DID of len 23 should have failed");
    }

    #[test]
    fn fails_validate_did_len_18() {
        let did: String = rand_string(18);

        assert_eq!(false, validate_did_len(&did), "DID of len 18 should have failed");
    }

    #[test]
    fn fails_validate_did_len_1() {
        let did: String = rand_string(1);

        assert_eq!(false, validate_did_len(&did), "DID of len 1 should have failed");
    }

    #[test]
    fn fails_validate_did_len_0() {
        let did: String = "".to_string();

        assert_eq!(false, validate_did_len(&did), "DID of len 0 should have failed");
    }
}