extern crate libc;

use self::libc::c_char;

use sovtoken::ErrorCode;

pub extern "C" fn empty_callback(_command_handle: i32, _err: i32, _req_json: *const c_char) -> i32 {
    return ErrorCode::Success as i32;
}