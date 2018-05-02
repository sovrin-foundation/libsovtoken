
#[allow(unused_variables)]
#[allow(dead_code)]

extern crate libc;
extern crate rand;

extern crate sovtoken;
extern crate indy;                      // lib-sdk project

use rand::{Rng, random};

use sovtoken::logic::payment_address_config::PaymentAddressConfig;
use sovtoken::logic::payments::{create_payment_address, PAY_INDICATOR, SOVRIN_INDICATOR, PAYMENT_ADDRESS_FIELD_SEP};

static VALID_ADDRESS_LEN: usize = 32;
static CHECKSUM_LEN: usize = 4;

// helper methods
fn rand_string() -> String {
    let s = rand::thread_rng()
            .gen_ascii_chars()
            .take(VALID_ADDRESS_LEN)
            .collect::<String>();

    return s;
}


// This is the happy path test.  Config contains a seed and
// a fully formatted address is returned.
#[test]
fn success_create_payment_with_seed_returns_address() {
    let seed = rand_string();
    println!("using seed {} len {}", seed, seed.chars().count());

    let config: PaymentAddressConfig = PaymentAddressConfig { seed };

    let address = create_payment_address(config);

    println!("got back address {} len {}", address, address.chars().count());

    let pay_indicator = &address[0..3];
    let first_separator = &address[3..4];
    let sov_indicator = &address[4..7];
    let second_indicator = &address[7..8];
    let result_address = &address[8..40];
    let checksum = &address[40..44];


    println!("got pay indicator {} len {}", pay_indicator, pay_indicator.chars().count());
    println!("got result address {} len {}", result_address, result_address.chars().count());
    println!("got checksum {} len {}", checksum, checksum.chars().count());


    assert_eq!(PAY_INDICATOR, pay_indicator, "PAY_INDICATOR not found");
    assert_eq!(PAYMENT_ADDRESS_FIELD_SEP, first_separator, "first PAYMENT_ADDRESS_FIELD_SEP not found");
    assert_eq!(SOVRIN_INDICATOR, sov_indicator, "SOVRIN_INDICATOR not found");
    assert_eq!(PAYMENT_ADDRESS_FIELD_SEP, second_indicator, "second PAYMENT_ADDRESS_FIELD_SEP not found");
    assert_eq!(VALID_ADDRESS_LEN, result_address.chars().count(), "address is not 32 bytes");
    assert_eq!(CHECKSUM_LEN, checksum.chars().count(), "checksum is not 4 bytes");

}