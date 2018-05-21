use indy::api::ErrorCode;
use std::os::raw::c_char;

// common types of callbacks or structures
pub type JsonCallback  = Option<extern fn(command_handle_: i32,
                                  err: ErrorCode,
                                  json: *const c_char) -> ErrorCode>;

pub type ErrorCodeStringClosure = Box<FnMut(ErrorCode, String) + Send>;
pub type ErrorCodeStringCallback = Option<extern fn(command_handle: i32, err: ErrorCode, c_str: *const c_char)>;

