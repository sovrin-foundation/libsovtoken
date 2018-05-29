extern crate rand;
use self::rand::Rng;
use self::rand::random;
pub fn rand_string(length : usize) -> String {
    let s = rand::thread_rng()
        .gen_ascii_chars()
        .take(length)
        .collect::<String>();

    return s;
}
/**
    `request` requires a req_id which is random number that can not be duplicate
    to any current request . This function simply calls and returns rand's
    `random` function using a u32
*/
pub fn rand_req_id() -> u32 {
    random::<u32>()
}
