use indy::api::ErrorCode;
use std::os::raw::c_char;


pub type JsonCallback  = Option<extern fn(command_handle_: i32,
                                  err: ErrorCode,
                                  json: *const c_char) -> ErrorCode>;