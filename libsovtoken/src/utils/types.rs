use indy::ErrorCode;
use std::os::raw::c_char;

// common types of callbacks or structures
pub type JsonCallback  = Option<extern fn(command_handle_: i32,
                                  err: ErrorCode,
                                  json: *const c_char) -> ErrorCode>;

