//!
//! Module for pulling in all the crates and defining Libsovtoken high level modules.
//! no code is allowed here.
//!
//!

// ------------------------------------------
// crates from crate.io etc
// ------------------------------------------
extern crate base64;
extern crate bs58;
extern crate hex;
extern crate libc;
extern crate openssl;
extern crate rand;
extern crate serde;
extern crate sodiumoxide;
extern crate sha2;
extern crate time;
// ------------------------------------------
// crates from crate.io etc that require macro
// ------------------------------------------

#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

#[cfg(any(test, feature = "integration"))]
#[macro_use] extern crate lazy_static;


// ------------------------------------------
// evernym/sovrin crates
// ------------------------------------------

extern crate indy_sys;                      // lib-sdk project
extern crate indyrs as indy;                      // lib-sdk rust wrapper to get ErrorCodes

// ------------------------------------------
// define our crate by defining the modules in the project
// ------------------------------------------


#[macro_use]
pub mod utils;
pub mod api;
pub mod logic;
pub mod libraries;

pub use indy::{ErrorCode, IndyHandle};