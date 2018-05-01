//
// this module contains functions that assist with std::ffi related behaviors
// such as: converting const char * to str
//
use std::ffi::CStr;
use std::str::Utf8Error;
use libc::c_char;


// utility method for converting const char * to a str.  Returns None
// if the input is invalid
pub fn str_from_char_ptr<'a>(str_ptr: *const c_char) -> Option<&'a str> {

    if str_ptr.is_null() {
        return None;
    }

    let c_str: &CStr = unsafe { CStr::from_ptr(str_ptr)};
    let str_slice: &str = c_str.to_str().unwrap();
    return Some(str_slice);
}