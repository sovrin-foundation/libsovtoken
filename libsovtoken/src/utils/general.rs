//! general helper methods.   Dont go nuts and add everything under the sun here.
//! general rule if its multiple similar functions (or data) it should be in its own
//! module.  keep these organized too please.


/*
    Given an u8 array of 0 to any length, convert it to an Option type where
    a zero length array becomes Option<None>

    inputs:  &[u8]
    outputs: Option<&[u8]>
*/
pub fn some_or_none_option_u8(data : &[u8]) -> Option<&[u8]> {
    if 0 == data.len() {
        return None;
    }

    return Some(data);
}