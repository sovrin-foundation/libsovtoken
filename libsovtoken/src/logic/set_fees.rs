#![allow(unused_variables)]
#![allow(unused_imports)]
#[warn(unused_imports)]

use libc::c_char;
use log::*;
use serde::{Serialize, Deserialize};
use std::{str, thread};
use std::ffi::{CString};
use std::collections::HashMap;


use indy::api::ErrorCode;
use super::fees_config::{SetFeesRequest, Fees};