
//
// Pull in all external dependencies
//
extern crate base64;
extern crate libc;
extern crate rust_base58;
extern crate serde;
extern crate sodiumoxide;

#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;

#[cfg(test)]
#[macro_use]
extern crate serde_json;
#[cfg(not(test))]
extern crate serde_json;

extern crate indy;                      // lib-sdk project



// define our crate by defining the modules in the project
#[allow(unused_variables)]
#[macro_use]
pub mod utils;
pub mod api;
pub mod logic;
pub mod libraries;