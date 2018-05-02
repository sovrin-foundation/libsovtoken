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

/**
    Transforms a c_string into a &str or returns an error_code.

    ```
        let c_string = std::ffi::CString::from("My test str");
        let my_str: &str = unpack_c_string_or_error!(c_string, ErrorCode::CommonInvalidParam2);
        // assert_eq!(my_str, "My test str");
    ```
*/
macro_rules! unpack_c_string_or_error {
    ( $c_string:ident, $error_code:expr ) => {
        match $crate::utils::ffi_support::str_from_char_ptr($c_string) {
            Some(s) => s,
            None => return $error_code,
        }
    }
}

#[cfg(test)]
mod ffi_support_tests {
    use std::ptr;
    use std::ffi;
    use libc::c_char;

    fn unpack_c_string_wrapper(c_str: *const c_char) -> u32 {
        unpack_c_string_or_error!(c_str, 0);
        return 1;
    }

    #[test]
    fn unpack_c_string_or_error_error_on_null() {
        let status = self::unpack_c_string_wrapper(ptr::null());
        assert_eq!(status, 0);
    }

    #[test]
    fn unpack_c_string_success() {
        let c_string = ffi::CString::new("My test str").unwrap().as_ptr();
        let status = unpack_c_string_wrapper(c_string);
        assert_eq!(status, 1);
    }
}