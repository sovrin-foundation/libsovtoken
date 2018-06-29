//! Contains functions for random data generation

extern crate rand;


use self::rand::Rng;
use self::rand::random;
use logic::type_aliases::ReqId;
/**
   Builds a string of random numbers of the inputted length
*/
pub fn rand_string(length : usize) -> String {
    let s = rand::thread_rng()
        .gen_ascii_chars()
        .take(length)
        .collect::<String>();

    return s;
}

pub fn rand_bytes(length : usize) -> Vec<u8> {
    rand::thread_rng()
        .gen_iter::<u8>().take(length).collect::<Vec<u8>>()
}

/**
    `request` requires a req_id which is random number that can not be duplicate
    to any current request . This function simply calls and returns rand's
    `random` function using a u32
*/
pub fn rand_req_id() -> ReqId {
    random::<ReqId>()
}