//!
//! tests for API related functions

extern crate sovtoken;

use sovtoken::api::sovtoken_init;
use sovtoken::ErrorCode;


#[test]
fn sovtoken_init_executes_successfully() {

   let err : i32 = sovtoken_init();

   assert_eq!(err, ErrorCode::Success as i32, "sovtoken_init did not return ErrorCode::Success");

}