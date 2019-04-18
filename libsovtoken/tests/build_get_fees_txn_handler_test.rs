#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;
extern crate sovtoken;
extern crate indyrs as indy;                     // lib-sdk project

mod utils;
use utils::payment::fees;
use utils::setup::{Setup, SetupConfig};
use utils::wallet::Wallet;

// ***** HELPER METHODS *****

// ***** HELPER TEST DATA  *****

#[test]
pub fn build_and_submit_set_fees() {
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 0,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: None,
        fees: None,
    });
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();

    let fees = json!({
//        "0": 1,
//        "1": 1,
        "101": 101,
        "102": 102,
        "109": 109,
        "111": 111,
        "113": 113,
        "114": 114,
        "118": 118,
        "119": 119,
        "120": 120,
    });

    fees::set_fees(pool_handle, wallet.handle,  &dids[0], &fees.to_string());
    let current_fees = fees::get_fees(&wallet, pool_handle, Some(dids[0]));
    let current_fees_value: serde_json::Value = serde_json::from_str(&current_fees).unwrap();

    assert_eq!(current_fees_value, fees);

    let fees = json!({
//        "0": 0,
//        "1": 0,
        "101": 0,
        "102": 0,
        "109": 0,
        "111": 0,
        "113": 0,
        "114": 0,
        "118": 0,
        "119": 0,
        "120": 0,
    }).to_string();

    fees::set_fees(pool_handle, wallet.handle,  &dids[0], &fees);
}