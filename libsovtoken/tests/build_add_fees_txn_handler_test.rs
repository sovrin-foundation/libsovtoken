#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate sovtoken;
extern crate rust_indy_sdk as indy;
pub mod utils;

use indy::payments::Payment;
use indy::{IndyHandle, ErrorCode};
use indy::utils::results::ResultHandler;
use sovtoken::utils::ffi_support::c_pointer_from_string;
use sovtoken::utils::ffi_support::c_pointer_from_str;
use std::sync::mpsc::channel;
use std::time::Duration;
use sovtoken::logic::parsers::common::TXO;
use sovtoken::utils::test::callbacks;
use utils::wallet::Wallet;


fn call_add_fees(wallet_handle: IndyHandle, inputs: String, outputs: String, request: String) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = callbacks::cb_ec_string();
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

    return ResultHandler::one(ErrorCode::from(error_code), receiver); 
}

fn init_wallet_with_address() -> (utils::wallet::Wallet, String) {
    sovtoken::api::sovtoken_init();

    let wallet = Wallet::new("p1");
    let key_config = json!({
        "seed": str::repeat("2", 32),
    });

    let input_address = Payment::create_payment_address(wallet.handle, "sov", &key_config.to_string()).unwrap();
    return (wallet, input_address);
}

#[test]
fn test_add_fees_to_request_valid() {
    let (wallet, input_address) = init_wallet_with_address();

    let fake_request = json!({
       "operation": {
           "type": "3"
       }
    });

    let txo = TXO { address: input_address, seq_no: 1 };

    let inputs = json!([txo.to_libindy_string().unwrap()]);
    
    let outputs = json!([{
            "paymentAddress": "pay:sov:dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q",
            "amount": 20,
    }]);

    let expected_fees_request = json!({
       "fees": [
           [["iTQzpdRdugkJ2gLD5vW5c159dncSL9jbAtu3WfPcb8qWD9bUd", 1]],
           [["dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q", 20]],
           ["5qqD2vk3nTeG5ZS1jVAvgPozPeSsBw8E8rux2jV8KsoFd1CiAzzpfez7ixMKvUpYaiQdEhsQwXaLNJRHHyF5g24R"]
       ],
       "operation": {
           "type": "3"
       }
    });

    let result = call_add_fees(
        wallet.handle,
        inputs.to_string(),
        outputs.to_string(),
        fake_request.to_string()
    ).unwrap();

    assert_eq!(expected_fees_request.to_string(), result);
}

#[test]
fn test_add_fees_to_request_valid_from_libindy() {
    let (wallet, input_address) = init_wallet_with_address();
    let did = "Th7MpTaRZVRYnPiabds81Y";

    let fake_request = json!({
       "operation": {
           "type": "3"
       }
    });

    let txo = TXO { address: input_address, seq_no: 1 };

    let inputs = json!([txo.to_libindy_string().unwrap()]);

    let outputs = json!([{
            "paymentAddress": "pay:sov:dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q",
            "amount": 20,
    }]);

    let expected_fees_request = json!({
       "fees": [
           [["iTQzpdRdugkJ2gLD5vW5c159dncSL9jbAtu3WfPcb8qWD9bUd", 1]],
           [["dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q", 20]],
           ["5qqD2vk3nTeG5ZS1jVAvgPozPeSsBw8E8rux2jV8KsoFd1CiAzzpfez7ixMKvUpYaiQdEhsQwXaLNJRHHyF5g24R"]
       ],
       "operation": {
           "type": "3"
       }
    });

    let (sender, receiver) = channel();

    let cb = move |ec, req, method| {
        sender.send((ec, req, method)).unwrap();
    };

    let return_error = indy::payments::Payment::add_request_fees_async(
        wallet.handle,
        did,
        &fake_request.to_string(),
        &inputs.to_string(),
        &outputs.to_string(),
        cb
    );

    let (req, method) = ResultHandler::two_timeout(return_error, receiver, Duration::from_secs(15)).unwrap();
    assert_eq!("sov", method);
    assert_eq!(expected_fees_request.to_string(), req);
}