//!
//! tests for API related functions


#![warn(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#[allow(unused_imports)]

extern crate sovtoken;
extern crate indy;                      // lib-sdk project

use indy::api::ErrorCode;
use sovtoken::api::sovtoken_init;



#[test]
fn sovtoken_init_executes_successfully() {

    let err : ErrorCode = sovtoken_init();

    assert_eq!(err, ErrorCode::Success, "sovtoken_init did not return ErrorCode::Success");

}