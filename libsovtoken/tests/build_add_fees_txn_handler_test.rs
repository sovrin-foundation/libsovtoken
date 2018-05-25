#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_json;
extern crate sovtoken;
extern crate rust_indy_sdk as indy;
mod utils;

use indy::payments::Payment;
use indy::{IndyHandle, ErrorCode};
use indy::utils::results::ResultHandler;
use sovtoken::utils::ffi_support::c_pointer_from_string;
use sovtoken::utils::ffi_support::c_pointer_from_str;


fn call_add_fees(wallet_handle: IndyHandle, inputs: String, outputs: String, request: String) -> Result<String, ErrorCode> {
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

#[test]
fn test_add_fees_to_request_valid() {
    println!("Starting a add_fees_to_request_valid test");
    sovtoken::api::sovtoken_init();

    let wallet_handle = utils::wallet::create_wallet("Wallet1");
    let input_address = Payment::create_payment_address(wallet_handle, "pay:sov:", "{}").unwrap();
    let fake_request = json!({
       "operation": {
           "type": ""
       }
    });

    let fake_request = "{}";

    let inputs = json!([
       {
           "paymentAddress": input_address,
           "seq_no": 1,
       }
    ]);
    let outputs = json!([
       {
           "paymentAddress": "pay:sov:gGpXeIzxDaZmeVhJZs6qWrdBPbDG3AfTW7RD",
           "amount": 20,
       }
    ]);

    let expected_fees_request = json!({
       "fees": {
           "inputs": [{
               "paymentAddress": input_address,
               "seq_no": 1,
               "signature": "",
           }],

           "outputs": [{
               "paymentAddress": "pay:sov:gGpXeIzxDaZmeVhJZs6qWrdBPbDG3AfTW7RD",
               "amount": 20
           }]
       }
    });

    println!("Calling call_add_fees");
    let result = call_add_fees(wallet_handle, inputs.to_string(), outputs.to_string(), fake_request.to_string()).unwrap();
    println!("Received result {:?}", result);
    let result_as_json_value = serde_json::to_value(&result).unwrap();
    assert_eq!(expected_fees_request, result_as_json_value);
}