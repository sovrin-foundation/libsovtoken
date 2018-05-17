//!
//! tests for Payment related functions


#![warn(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#[allow(unused_imports)]


extern crate libc;
extern crate rand;

#[macro_use] extern crate log;

extern crate sovtoken;
extern crate indy;                      // lib-sdk project

use libc::c_char;
use rand::Rng;
use std::ptr;
use std::ffi::CString;


use indy::api::ErrorCode;
use sovtoken::api::sovtoken_init;
use sovtoken::logic::payment_address_config::PaymentAddressConfig;
use sovtoken::utils::logger::*;
use sovtoken::utils::callbacks::*;



#[test]
fn sovtoken_init_executes_successfully() {

    let err : ErrorCode = sovtoken_init();

    assert_eq!(err, ErrorCode::Success, "sovtoken_init did not return ErrorCode::Success");

}