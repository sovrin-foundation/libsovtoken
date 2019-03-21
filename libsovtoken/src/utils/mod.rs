/*
   Utils Mod contains useful helper functions
*/

/// use statements are listed the following pattern:
/// follow this or risk having gum thrown in your hair
///
/// first: standard rust imports
/// second: imported crates
/// third: libsovtoken name spaces


#[macro_use] pub mod ffi_support;
#[macro_use] pub mod json_conversion;
#[macro_use] pub mod conversions;
#[macro_use] pub mod macros;
#[macro_use] pub mod logger;

pub mod base58;
pub mod callbacks;
pub mod constants;
pub mod general;
pub mod random;
pub mod sequence;
pub mod results;
#[cfg(any(test, feature = "integration"))]
pub mod test;

pub type IndyHandle = i32;

pub use indy::ErrorCode;

