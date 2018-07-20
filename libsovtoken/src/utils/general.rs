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
    fn replace_char_at(&self, position: usize, c: char) -> String;
}

/// this impl adds StringUtils to any str
impl<'a> StringUtils for &'a str {
    /**
        returns right most portion of a string
    */
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

    /**
        returns a string with the character at position changed to replaced

        position: zero based index of element to be replace
        replace: char which will be placed in position
    */
    fn replace_char_at(&self, position: usize, replace: char) -> String {
        // Taken from https://stackoverflow.com/a/27320653
        let mut replacable : String = String::with_capacity(self.len());
        for (index, keep) in self.char_indices() {
            replacable.push(if index == position { replace } else { keep });
        }

        return replacable;
    }
}

/*
         UNIT TESTS BELOW
         (and only unit tests---do not add more functions below this mod)
*/

#[cfg(test)]
mod general_tests {

    use utils::general::{StringUtils, some_or_none_option_u8};

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
    fn success_replace_2nd() {
        let source : String = "abc".to_string();
        let result : String = "adc".to_string();

        assert_eq!(result, source.as_str().replace_char_at(1, 'd'));
    }

    #[test]
    fn success_replace_1st() {
        let source : String = "abc".to_string();
        let result : String = "dbc".to_string();

        assert_eq!(result, source.as_str().replace_char_at(0, 'd'));
    }

    #[test]
    fn success_replace_empty_str() {
        let source : String = "".to_string();
        let result : String = "".to_string();

        assert_eq!(result, source.as_str().replace_char_at(0, 'd'));
    }
}