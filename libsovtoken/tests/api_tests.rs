//!
//! tests for API related functions

extern crate sovtoken;
extern crate indy;                      // lib-sdk project

use indy::ErrorCode;
use sovtoken::api::sovtoken_init;


#[test]
fn sovtoken_init_executes_successfully() {

   let err : i32 = sovtoken_init();

   assert_eq!(err, ErrorCode::Success as i32, "sovtoken_init did not return ErrorCode::Success");

}