
#[allow(unused_variables)]
#[allow(dead_code)]

extern crate libc;

extern crate sovtoken;
extern crate indy;                      // lib-sdk project

use libc::c_char;
use std::ptr;
use std::ffi::CString;

use indy::api::ErrorCode;



// Unit Tests

// the build_mint_txn
 #[test]
 fn errors_with_no_call_back() {
     assert!(false);
 }
 