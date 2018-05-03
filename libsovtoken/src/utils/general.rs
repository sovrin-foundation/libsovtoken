//! general helper methods.   Dont go nuts and add everything under the sun here.
//! As a general rule if you are adding multiple functions of similar behavior (or data)
//! it should be in its own module.  keep these organized too please.


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


/*
         UNIT TESTS BELOW
         (and only unit tests---do not add more functions below this mod)
*/

#[cfg(test)]
mod general_tests {

    use utils::general::StringUtils;
    use utils::general::some_or_none_option_u8;

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
}