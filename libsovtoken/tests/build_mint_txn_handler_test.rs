
#[allow(unused_variables)]
#[allow(dead_code)]

extern crate libc;

extern crate sovtoken;
extern crate indy;                      // lib-sdk project

use libc::c_char;
use std::ptr;
use std::ffi::CString;

use indy::api::ErrorCode;

const COMMAND_HANDLE: i32 = 10;

// Unit Tests

// the build_mint_txn_handler requires a callback and this test ensures that we
// receive an error when no callback is provided
 #[test]
 fn errors_with_no_call_back() {
     let return_error = sovtoken::api::build_mint_txn_handler(COMMAND_HANDLE, ptr::null(), None);
     assert!(return_error == ErrorCode::CommonInvalidParam3, "Expecting Callback for 'create_payment_address_handler'"); 
 }

