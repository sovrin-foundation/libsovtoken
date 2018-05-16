extern crate rand;
use self::rand::Rng;

pub fn rand_string(length : usize) -> String {
    let s = rand::thread_rng()
        .gen_ascii_chars()
        .take(length)
        .collect::<String>();

    return s;
}