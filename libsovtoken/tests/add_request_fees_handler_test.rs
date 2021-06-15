#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate sovtoken;
extern crate indyrs as indy;
#[macro_use]
extern crate lazy_static;
extern crate indy_sys;

pub mod utils;

use sovtoken::utils::results::ResultHandler;
use sovtoken::utils::test::callbacks;
use sovtoken::logic::parsers::common::TXO;
use sovtoken::utils::ffi_support::c_pointer_from_string;
use sovtoken::utils::ffi_support::c_pointer_from_str;
use sovtoken::ErrorCode;
use utils::wallet::Wallet;


fn call_add_request_fees(wallet_handle: indy_sys::WalletHandle, inputs: String, outputs: String, extra: Option<String>, request: String) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) =  callbacks::cb_ec_string();

    let did = "mydid1";
    let extra = extra.map(c_pointer_from_string).unwrap_or(std::ptr::null());
    let error_code = sovtoken::api::add_request_fees_handler(
        command_handle,
        wallet_handle,
        c_pointer_from_str(did),
        c_pointer_from_string(request),
        c_pointer_from_string(inputs),
        c_pointer_from_string(outputs),
        extra,
        cb
    );

    return ResultHandler::one(ErrorCode::from(error_code), receiver);
}

fn init_wallet_with_address() -> (utils::wallet::Wallet, String) {
    sovtoken::api::sovtoken_init();

    let wallet = Wallet::new();
    let seed = str::repeat("2", 32);

    let input_address = utils::payment::address::generate(&wallet, Some(&seed));
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
            "recipient": "pay:sov:dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q",
            "amount": 20,
    }]);

    let expected_fees_request = json!({
        "fees": [
            [
                {
                    "address": "iTQzpdRdugkJ2gLD5vW5c159dncSL9jbAtu3WfPcb8qWD9bUd",
                    "seqNo": 1
                }
            ],
            [
                {
                    "address": "dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q",
                    "amount": 20
                }
            ],
            ["64wPLDPMjGxgqTdrNTZFa9CK4NtvBx7eLJkgnjW3JchRGyMUr29tjkAiHCTnhLtkdW81k5BtBiiqM2tkaMB2eouv"]
        ],
        "operation": {
            "type": "3"
        }
    });

    let result = call_add_request_fees(
        wallet.handle,
        inputs.to_string(),
        outputs.to_string(),
        None,
        fake_request.to_string()
    ).unwrap();

    assert_eq!(expected_fees_request.to_string(), result);
}

#[test]
fn test_add_fees_to_request_works_for_invalid_request() {
    let (wallet, input_address) = init_wallet_with_address();

    let fake_request = "INVALID REQUEST";

    let txo = TXO { address: input_address, seq_no: 1 };

    let inputs = json!([txo.to_libindy_string().unwrap()]);

    let outputs = json!([{
            "recipient": "pay:sov:dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q",
            "amount": 20,
    }]);

    let err = call_add_request_fees(
        wallet.handle,
        inputs.to_string(),
        outputs.to_string(),
        None,
        fake_request.to_string()
    ).unwrap_err();

    assert_eq!(err, ErrorCode::CommonInvalidStructure)
}

#[test]
fn build_add_fees_to_request_works_for_invalid_utxo() {
    sovtoken::api::sovtoken_init();
    let wallet = Wallet::new();

    let fake_request = json!({
       "operation": {
           "type": "3"
       }
    }).to_string();

    let inputs = json!(["txo:sov:1234"]).to_string();

    let outputs = json!([{
            "recipient": "pay:sov:dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q",
            "amount": 20,
    }]).to_string();

    let err = call_add_request_fees(
        wallet.handle,
        inputs.to_string(),
        outputs.to_string(),
        None,
        fake_request.to_string()
    ).unwrap_err();

    assert_eq!(err, ErrorCode::CommonInvalidStructure)
}

#[test]
fn test_add_fees_to_request_works_for_invalid_receipt() {
    let (wallet, input_address) = init_wallet_with_address();

    let fake_request = json!({
       "operation": {
           "type": "3"
       }
    });

    let txo = TXO { address: input_address, seq_no: 1 };

    let inputs = json!([txo.to_libindy_string().unwrap()]);

    let outputs = json!([{
            "recipient": "pay:sov:1234",
            "amount": 20,
    }]);

    let err = call_add_request_fees(
        wallet.handle,
        inputs.to_string(),
        outputs.to_string(),
        None,
        fake_request.to_string()
    ).unwrap_err();

    assert_eq!(err, ErrorCode::CommonInvalidStructure)
}

#[test]
fn test_add_fees_to_request_works_for_invalid_amount() {
    let (wallet, input_address) = init_wallet_with_address();

    let fake_request = json!({
       "operation": {
           "type": "3"
       }
    });

    let txo = TXO { address: input_address, seq_no: 1 };

    let inputs = json!([txo.to_libindy_string().unwrap()]);

    let outputs = json!([{
            "recipient": "pay:sov:1234",
            "amount": -20,
    }]);

    let err = call_add_request_fees(
        wallet.handle,
        inputs.to_string(),
        outputs.to_string(),
        None,
        fake_request.to_string()
    ).unwrap_err();

    assert_eq!(err, ErrorCode::CommonInvalidStructure)
}