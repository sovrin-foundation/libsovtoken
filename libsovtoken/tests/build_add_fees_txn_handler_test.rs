#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate sovtoken;
extern crate rust_indy_sdk as indy;
pub mod utils;

use std::sync::mpsc::channel;
use std::time::Duration;

use indy::{IndyHandle, ErrorCode};
use indy::utils::results::ResultHandler;
use sovtoken::utils::ffi_support::c_pointer_from_string;
use sovtoken::utils::ffi_support::c_pointer_from_str;
use sovtoken::logic::parsers::common::TXO;
use sovtoken::utils::test::callbacks;
use utils::wallet::Wallet;


fn call_add_fees(wallet_handle: IndyHandle, inputs: String, outputs: String, extra: Option<String>, request: String) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = callbacks::cb_ec_string();
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
           [["iTQzpdRdugkJ2gLD5vW5c159dncSL9jbAtu3WfPcb8qWD9bUd", 1]],
           [["dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q", 20]],
           ["2pTQPLHmv1xfw5qFrk6aET5eukoKsZNvHvvHhX6vV5Zi9t1co1L5CPTUVXeoyh6hG83N9crkhLb4KSfPisWaqM3E"]
       ],
       "operation": {
           "type": "3"
       }
    });

    let result = call_add_fees(
        wallet.handle,
        inputs.to_string(),
        outputs.to_string(),
        None,
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
            "recipient": "pay:sov:dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q",
            "amount": 20,
    }]);

    let expected_fees_request = json!({
       "fees": [
           [["iTQzpdRdugkJ2gLD5vW5c159dncSL9jbAtu3WfPcb8qWD9bUd", 1]],
           [["dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q", 20]],
           ["2pTQPLHmv1xfw5qFrk6aET5eukoKsZNvHvvHhX6vV5Zi9t1co1L5CPTUVXeoyh6hG83N9crkhLb4KSfPisWaqM3E"]
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
        None,
        cb
    );

    let (req, method) = ResultHandler::two_timeout(return_error, receiver, Duration::from_secs(15)).unwrap();
    assert_eq!("sov", method);
    assert_eq!(expected_fees_request.to_string(), req);
}

#[test]
fn test_add_fees_to_request_valid_from_libindy_for_not_owned_payment_address() {
    let wallet_1 = utils::wallet::Wallet::new();
    let wallet_2 = utils::wallet::Wallet::new();

    let setup = utils::setup::Setup::new(&wallet_1, utils::setup::SetupConfig {
        num_addresses: 1,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: Some(vec![30]),
        fees: None,
    });
    let addresses = &setup.addresses;
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();

    let fake_request = json!({
        "operation": {
            "type": "3"
        }
    });

    let utxo = utils::payment::get_utxo::get_first_utxo_txo_for_payment_address(&wallet_1, pool_handle, dids[0], &addresses[0]);

    let inputs = json!([utxo]);

    let outputs = json!([{
            "recipient": addresses[0],
            "amount": 20,
    }]);

    let err = indy::payments::Payment::add_request_fees(wallet_2.handle, dids[0], &fake_request.to_string(), &inputs.to_string(), &outputs.to_string(), None).unwrap_err();
    assert_eq!(err, indy::ErrorCode::WalletItemNotFound);
}

#[test]
fn build_add_fees_to_request_works_for_invalid_utxo() {
    sovtoken::api::sovtoken_init();
    let wallet = Wallet::new();
    let (did, _) = indy::did::Did::new(wallet.handle, &json!({"seed": "000000000000000000000000Trustee1"}).to_string()).unwrap();

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

    let err = indy::payments::Payment::add_request_fees(wallet.handle, &did, &fake_request, &inputs, &outputs, None).unwrap_err();

    assert_eq!(err, ErrorCode::CommonInvalidStructure)
}