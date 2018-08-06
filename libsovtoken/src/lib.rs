//!
//! Module for pulling in all the crates and defining Libsovtoken high level modules.
//! no code is allowed here.
//!
//!

// ------------------------------------------
// crates from crate.io etc
// ------------------------------------------
extern crate base64;
extern crate env_logger;
extern crate hex;
extern crate libc;
extern crate log_panics;
extern crate openssl;
extern crate rust_base58;
extern crate rand;
extern crate serde;
extern crate sodiumoxide;
extern crate sha2;

// ------------------------------------------
// crates from crate.io etc that require macro
// ------------------------------------------

#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

#[cfg(any(test, feature = "integration"))]
#[macro_use] extern crate lazy_static;

// ------------------------------------------
// android crates
// ------------------------------------------
#[cfg(target_os = "android")]
extern crate android_logger;

// ------------------------------------------
// evernym/sovrin crates
// ------------------------------------------


extern crate rust_indy_sdk as indy;                      // lib-sdk project


// ------------------------------------------
// define our crate by defining the modules in the project
// ------------------------------------------


#[allow(unused_variables)]
#[macro_use]
pub mod utils;
pub mod api;
pub mod logic;
pub mod libraries;
