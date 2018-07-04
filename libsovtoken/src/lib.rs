//!
//! Module for pulling in all the crates and defining Libsovtoken high level modules.
//! no code is allowed here.
//!
extern crate base64;
extern crate env_logger;
extern crate libc;
extern crate rust_base58;
extern crate rand;
extern crate serde;
extern crate sodiumoxide;



#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

extern crate rust_indy_sdk as indy;                      // lib-sdk project

#[cfg(any(test, feature = "integration"))]
#[macro_use] extern crate lazy_static;
extern crate openssl;


// define our crate by defining the modules in the project
#[allow(unused_variables)]
#[macro_use]
pub mod utils;
pub mod api;
pub mod logic;
pub mod libraries;
