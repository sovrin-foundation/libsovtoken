
#[allow(unused_variables)]
#[allow(dead_code)]

extern crate libc;

extern crate sovtoken;
extern crate indy;                      // lib-sdk project

use libc::c_char;
use std::ptr;
use std::ffi::CString;

use indy::api::ErrorCode;

extern "C" fn empty_create_payment_callback(command_handle_: i32, err: ErrorCode, payment_address: *const c_char) { }

const COMMAND_HANDLE: i32 = 10;
const cb : Option<extern fn(command_handle_: i32, err: ErrorCode, payment_address: *const c_char)> = Some(empty_create_payment_callback);

// Unit Tests

// the build_mint_txn_handler requires a callback and this test ensures that we
// receive an error when no callback is provided
 #[test]
 fn errors_with_no_call_back() {
     let return_error = sovtoken::api::build_mint_txn_handler(COMMAND_HANDLE, ptr::null(), None);
     assert!(return_error == ErrorCode::CommonInvalidParam3, "Expecting Callback for 'create_payment_address_handler'"); 
}

// the build mint txn handler method requires an outputs_json parameter and this test ensures that 
// a error is returned when no config is provided
#[test]
fn errors_with_no_outputs_json() {
    let return_error = sovtoken::api::build_mint_txn_handler(COMMAND_HANDLE, ptr::null(), cb);
    assert!(return_error == ErrorCode::CommonInvalidParam2, "Expecting outputs_json for 'create_payment_address_handler'");
}