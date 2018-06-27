/*! 
 * Supplies default data for *TESTS ONLY*
 * 
*/

use libc::c_char;
use utils::ffi_support::c_pointer_from_string;
use utils::random::rand_string;
use utils::constants::txn_types;
use logic::parsers::common::TXO;
use rust_base58::ToBase58;

pub fn inputs_json_pointer() -> *const c_char {
    let txo_1 = TXO { address: "pay:sov:d0kitWxupHvZ4i0NHJhoj79RcUeyt3YlwAc8Hbcy87iRLSZC".to_string(), seq_no: 2 };
    let txo_2 = TXO { address: "pay:sov:d0kitWxupHvZ4i0NHJhoj79RcUeyt3YlwAc8Hbcy87iRLSZC".to_string(), seq_no: 2 };
    let json = json!([
        txo_1.to_libindy_string().unwrap(),
        txo_2.to_libindy_string().unwrap()
    ]);

    return c_pointer_from_string(json.to_string());
}

pub fn outputs_json_pointer() -> *const c_char {
    let json = json!([
            {
                "paymentAddress": "pay:sov:ql33nBkjGw6szxPT6LLRUIejn9TZAYkVRPd0QJzfJ8FdhZWs",
                "amount": 10
            }
        ]);

    return c_pointer_from_string(json.to_string());
}

pub extern fn empty_callback_string(
    _: i32,
    e: i32,
    _: *const c_char
) -> i32 {
    return e;
}

pub fn did() -> *const c_char {
    let did = rand_string(16).as_bytes().to_base58();
    return c_pointer_from_string(did);
}

pub fn set_fees_json() -> *const c_char {
    let json = json!({
        txn_types::XFER_PUBLIC: 3,
        "3": 5
    });

    return c_pointer_from_string(json.to_string());
}