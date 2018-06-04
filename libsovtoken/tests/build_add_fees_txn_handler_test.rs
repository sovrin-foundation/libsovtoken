#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_json;
extern crate sovtoken;
extern crate rust_indy_sdk as indy;
pub mod utils;

use indy::payments::Payment;
use indy::{IndyHandle, ErrorCode};
use indy::utils::results::ResultHandler;
use sovtoken::utils::ffi_support::c_pointer_from_string;
use sovtoken::utils::ffi_support::c_pointer_from_str;


fn call_add_fees(wallet_handle: IndyHandle, inputs: String, outputs: String, request: String) -> Result<String, i32> {
    let (receiver, command_handle, cb) = utils::callbacks::closure_to_cb_ec_string();
    let did = "mydid1";
    let error_code = sovtoken::api::add_request_fees_handler(
        command_handle,
        wallet_handle,
        c_pointer_from_str(did),
        c_pointer_from_string(request),
        c_pointer_from_string(inputs),
        c_pointer_from_string(outputs),
        cb
    );
    return ResultHandler::one(error_code, receiver); 
}

fn init_wallet_with_address() -> (IndyHandle, String) {
    sovtoken::api::sovtoken_init();

    let wallet_handle = utils::wallet::create_wallet("wallet_add_fees");
    let key_config = json!({
        "seed": str::repeat("2", 32),
    });

    let input_address = Payment::create_payment_address(wallet_handle, "pay:sov:", &key_config.to_string()).unwrap();
    return (wallet_handle, input_address);
}

#[test]
fn test_add_fees_to_request_valid() {
    let (wallet_handle, input_address) = init_wallet_with_address();

    let fake_request = json!({
       "operation": {
           "type": "3"
       }
    });

    let inputs = json!({
        "ver": 1,
        "inputs": [{
            "address": input_address,
            "seqNo": 1,
        }]
    });
    
    let outputs = json!({
        "ver": 1,
        "outputs": [{
            "address": "pay:sov:x39ETFpHu2WDGIKLMwxSWRilgyN9yfuPx8l6ZOev3ztG1MJ6",
            "amount": 20,
        }]
    });

    let expected_fees_request = json!({
       "fees": {
           "inputs": [["7LSfLv2S6K7zMPrgmJDkZoJNhWvWRzpU7qt9uMR5yz8GYjJM", 1, "2uU4zJWjVMKAmabQefkxhFc3K4BgPuwqVoZUiWYS2Ct9hidmKF9hcLNBjw76EjuDuN4RpzejKJUofJPcA3KhkBvi"]],
           "outputs": [["x39ETFpHu2WDGIKLMwxSWRilgyN9yfuPx8l6ZOev3ztG1MJ6", 20]]
       },
       "operation": {
           "type": "3"
       }
    });

    let result = call_add_fees(wallet_handle, inputs.to_string(), outputs.to_string(), fake_request.to_string()).unwrap();
    assert_eq!(expected_fees_request.to_string(), result);
}