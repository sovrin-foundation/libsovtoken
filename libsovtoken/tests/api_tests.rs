//!
//! tests for API related functions

#[macro_use] extern crate sovtoken;

use sovtoken::utils::ErrorCode;
use sovtoken::api::sovtoken_init;


#[test]
fn sovtoken_init_executes_successfully() {

   let err : i32 = sovtoken_init();

   assert_eq!(err, ErrorCode::Success as i32, "sovtoken_init did not return ErrorCode::Success");

}