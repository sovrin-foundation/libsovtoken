/*!
Helpers dealing with addresses

For integration tests
*/

use indy::payments::Payment;
use utils::wallet::Wallet;
use sovtoken::utils::constants::general::PAYMENT_METHOD_NAME;

/**
Generate a address and store it in wallet.
*/
pub fn generate(wallet: &Wallet, seed: Option<&str>) -> String {
    let seed = seed
        .map(seed_json)
        .unwrap_or(String::from("{}"));

    Payment::create_payment_address(wallet.handle, PAYMENT_METHOD_NAME, &seed).unwrap()
}

/**
Generate `n` random addresses and store them in the wallet.

Calls [`generate`]

[`generate`]: self::generate
*/
pub fn generate_n(wallet: &Wallet, n: usize) -> Vec<String> {
    let mut addresses = Vec::with_capacity(n);
    for _ in 0..n {
        addresses.push(generate(wallet, None));
    }
    
    addresses
}

/**
Generates `n` seeded addresses and stores them in the wallet.

Calls [`generate`]

[`generate`]: self::generate
*/
pub fn generate_n_seeded<S>(wallet: &Wallet, seeds: Vec<S>) -> Vec<String>
    where S: AsRef<str>
{
    seeds
        .into_iter()
        .map(|seed| generate(wallet, Some(seed.as_ref())))
        .collect()
}


/**
Create config json for create_payment_address

Encapsulates the seed in json object.
The seed needs to be 32 chars in length.

## Example
```
{
    seed: "8k9wCLEkEKU8GGwRh4WedTDx8SHVDjGX"
}
```
*/
fn seed_json(seed: &str) -> String {
    json!({
        "seed": seed
    }).to_string()
}