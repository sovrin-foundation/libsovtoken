extern crate env_logger;
extern crate libc;
extern crate sovtoken;
extern crate rust_indy_sdk as indy;                      // lib-sdk project
extern crate rust_base58;

#[macro_use] extern crate log;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;

use indy::ErrorCode;
use indy::utils::results::ResultHandler;
use libc::c_char;
use sovtoken::logic::address;
use sovtoken::logic::parsers::common::TXO;
use sovtoken::utils::constants::txn_types::XFER_PUBLIC;
use sovtoken::utils::ffi_support::c_pointer_from_string;
use sovtoken::utils::test::callbacks;
use std::ptr;
use std::ffi::CString;
use std::time::Duration;
use std::sync::mpsc::channel;
use rust_base58::{FromBase58, ToBase58};

mod utils;
use utils::wallet::Wallet;
use utils::setup::{SetupConfig, Setup};


// ***** HELPER METHODS *****
extern "C" fn empty_create_payment_callback(_command_handle_: i32, _err: i32, _payment_req: *const c_char) -> i32 {
    return ErrorCode::Success as i32;
}

// ***** HELPER TEST DATA  *****

const COMMAND_HANDLE:i32 = 10;
static INVALID_OUTPUT_JSON: &'static str = r#"{"totally" : "Not a Number", "bobby" : "DROP ALL TABLES"}"#;
static VALID_OUTPUT_JSON: &'static str = r#"{"outputs":[["AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja",10]]}"#;
const WALLET_HANDLE:i32 = 0;
const CB : Option<extern fn(_command_handle_: i32, err: i32, payment_req_json: *const c_char) -> i32 > = Some(empty_create_payment_callback);


fn generate_payment_addresses(wallet: &Wallet) -> (Vec<String>, Vec<String>) {
    let seeds = vec![
        str::repeat("2", 32),
        str::repeat("3", 32),
        str::repeat("4", 32),
        str::repeat("1", 32),
    ];

    let payment_addresses = utils::payment::address::generate_n_seeded(wallet, seeds);

    payment_addresses
        .iter()
        .enumerate()
        .for_each(|(idx, address)| debug!("payment_address[{:?}] = {:?}", idx, address));

    let addresses = payment_addresses
        .iter()
        .map(|address| address::unqualified_address_from_address(&address).unwrap())
        .collect();

    return (payment_addresses, addresses);
}

// ***** UNIT TESTS ****

// the build_mint_txn_handler requires a callback and this test ensures that we
// receive an error when no callback is provided
#[test]
fn errors_with_no_call_back() {
    let return_error = sovtoken::api::build_payment_req_handler(COMMAND_HANDLE,
                                                                WALLET_HANDLE,
                                                                ptr::null(),
                                                                ptr::null(),
                                                                ptr::null(),
                                                                None);
    assert_eq!(return_error, ErrorCode::CommonInvalidStructure as i32, "Expecting Callback for 'build_payment_req_handler'");
}

// the build payment req handler method requires an inputs_json parameter and this test ensures that
// a error is returned when no config is provided
#[test]
fn errors_with_no_inputs_json() {
    let return_error = sovtoken::api::build_payment_req_handler(COMMAND_HANDLE,
                                                                WALLET_HANDLE,
                                                                ptr::null(),
                                                                ptr::null(),
                                                                ptr::null(),
                                                                CB);
    assert_eq!(return_error, ErrorCode::CommonInvalidStructure as i32, "Expecting inputs_json for 'build_payment_req_handler'");
}

// the build payment req handler method requires an outputs_json parameter and this test ensures that
// a error is returned when no config is provided
#[test]
fn errors_with_no_outputs_json() {
    let input_json :CString = CString::new(INVALID_OUTPUT_JSON).unwrap();
    let input_json_ptr = input_json.as_ptr();
    let return_error = sovtoken::api::build_payment_req_handler(COMMAND_HANDLE,
                                                                WALLET_HANDLE,
                                                                ptr::null(),
                                                                input_json_ptr,
                                                                ptr::null(),
                                                                CB);
    assert_eq!(return_error, ErrorCode::CommonInvalidStructure as i32, "Expecting outputs_json for 'build_payment_req_handler'");
}

// the build payment req handler method requires an submitter_did parameter and this test ensures that
// a error is returned when no config is provided
#[test]
fn errors_with_no_submitter_did_json() {
    let input_json :CString = CString::new(INVALID_OUTPUT_JSON).unwrap();
    let input_json_ptr = input_json.as_ptr();
    let output_json :CString = CString::new(VALID_OUTPUT_JSON).unwrap();
    let output_json_ptr = output_json.as_ptr();

    let return_error = sovtoken::api::build_payment_req_handler(COMMAND_HANDLE,
                                                                WALLET_HANDLE,
                                                                ptr::null(),
                                                                input_json_ptr,
                                                                output_json_ptr,
                                                                CB);
    assert_eq!(return_error, ErrorCode::CommonInvalidStructure as i32, "Expecting outputs_json for 'build_payment_req_handler'");
}

#[test]
fn success_signed_request() {
    sovtoken::api::sovtoken_init();

    let did = String::from("287asdjkh2323kjnbakjs");

    let wallet = Wallet::new();
    debug!("wallet id = {:?}", wallet.handle);

    let (payment_addresses, addresses) = generate_payment_addresses(&wallet);
    let txo_1 = TXO { address: payment_addresses[0].clone(), seq_no: 1 }.to_libindy_string().unwrap();
    let txo_2 = TXO { address: payment_addresses[1].clone(), seq_no: 1 }.to_libindy_string().unwrap();

    let inputs = json!([
            txo_1, txo_2
        ]);

    let outputs = json!([
            {
                "paymentAddress": payment_addresses[2],
                "amount": 10
            },
            {
                "paymentAddress": payment_addresses[3],
                "amount": 22,
                "extra": "extra data"
            }
        ]);

    let expected_operation = json!({
        "type": XFER_PUBLIC,
        "inputs": [
            [addresses[0], 1],
            [addresses[1], 1]
        ],
        "outputs": [[addresses[2], 10], [addresses[3], 22]],
        "signatures": [
            "4XFsKCBsuW2e4rUfzy6SxbtyfvyL3WbK8bg1wkm4jLrnbnPmbed8oERdm58pmTG7xEoVZxmYsqHjhYaMqDJH48Dx",
            "4MrrBDrYzJvj2ADEkz8Lg3kfZeTJmTrYNtALdxyAqhuJ9xBoKrjKgtcYS7nKDBubNsPH1pb6oMqYBQFiGm28ikzX"
        ]
    });

    let (receiver, command_handle, cb) = callbacks::cb_ec_string();


    trace!("Calling build_payment_req");

    let error_code = sovtoken::api::build_payment_req_handler(
        command_handle,
        wallet.handle,
        c_pointer_from_string(did),
        c_pointer_from_string(inputs.to_string()),
        c_pointer_from_string(outputs.to_string()),
        cb
    );

    assert_eq!(ErrorCode::from(error_code), ErrorCode::Success);

    let request_string = ResultHandler::one(ErrorCode::Success, receiver).unwrap();

    let request: serde_json::value::Value = serde_json::from_str(&request_string).unwrap();
    debug!("Received request {:?}", request);

    assert_eq!(&expected_operation, request.get("operation").unwrap());
    let ident = addresses[0].from_base58_check().unwrap().to_base58();
    assert_eq!(&ident, request.get("identifier").unwrap().as_str().unwrap());
    assert!(request.get("reqId").is_some());
}

#[test]
fn success_signed_request_from_libindy() {

    sovtoken::api::sovtoken_init();

    let did = String::from("Th7MpTaRZVRYnPiabds81Y");

    let wallet = Wallet::new();
    debug!("wallet id = {:?}", wallet.handle);

    let (payment_addresses, addresses) = generate_payment_addresses(&wallet);

    let txo_1 = TXO { address: payment_addresses[0].clone(), seq_no: 1 }.to_libindy_string().unwrap();
    let txo_2 = TXO { address: payment_addresses[1].clone(), seq_no: 1 }.to_libindy_string().unwrap();

    let inputs = json!([
            txo_1, txo_2
        ]);

    let outputs = json!([
            {
                "paymentAddress": payment_addresses[2],
                "amount": 10
            },
            {
                "paymentAddress": payment_addresses[3],
                "amount": 22,
                "extra": "extra data"
            }
        ]);

    let expected_operation = json!({
        "type": XFER_PUBLIC,
        "inputs": [
            [addresses[0], 1],
            [addresses[1], 1]
        ],
        "outputs": [[addresses[2], 10], [addresses[3], 22]],
        "signatures": [
            "4XFsKCBsuW2e4rUfzy6SxbtyfvyL3WbK8bg1wkm4jLrnbnPmbed8oERdm58pmTG7xEoVZxmYsqHjhYaMqDJH48Dx",
            "4MrrBDrYzJvj2ADEkz8Lg3kfZeTJmTrYNtALdxyAqhuJ9xBoKrjKgtcYS7nKDBubNsPH1pb6oMqYBQFiGm28ikzX"
        ]
    });

    let (sender, receiver) = channel();

    let closure = move|error_code, req, _| {
        sender.send((error_code, req)).unwrap();
    };


    trace!("Calling build_payment_req");

    let _ = indy::payments::Payment::build_payment_req_async(
        wallet.handle,
        &did,
        &inputs.to_string(),
        &outputs.to_string(),
        closure
    );

    let request_string = ResultHandler::one_timeout(ErrorCode::Success, receiver, Duration::from_secs(5)).unwrap();

    let request: serde_json::value::Value = serde_json::from_str(&request_string).unwrap();
    debug!("Received request {:?}", request);

    assert_eq!(&expected_operation, request.get("operation").unwrap());
    let ident = addresses[0].from_base58_check().unwrap().to_base58();
    assert_eq!(&ident, request.get("identifier").unwrap().as_str().unwrap());
    assert!(request.get("reqId").is_some());

}

#[test]
pub fn build_and_submit_payment_req() {
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 2,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: Some(vec![30])
    });
    let Setup {addresses: payment_addresses, pool_handle, trustees, ..} = setup;
    let dids = trustees.dids();

    let (utxo, _, _) = utils::get_utxo::get_first_utxo_for_payment_address(wallet.handle, pool_handle, dids[0], &payment_addresses[0]);

    let inputs = json!([utxo]).to_string();
    let outputs = json!([
        {
            "paymentAddress": payment_addresses[1],
            "amount": 20
        },
        {
            "paymentAddress": payment_addresses[0],
            "amount": 10
        }
    ]).to_string();
    let (req, method) = indy::payments::Payment::build_payment_req(wallet.handle, dids[0], &inputs, &outputs).unwrap();
    let res = indy::ledger::Ledger::submit_request(pool_handle, &req).unwrap();
    let res = indy::payments::Payment::parse_payment_response(&method, &res).unwrap();

    let res_parsed: serde_json::Value = serde_json::from_str(&res).unwrap();
    let utxos = res_parsed.as_array().unwrap();
    assert_eq!(utxos.len(), 2);

    let value = utxos.get(0).unwrap().as_object().unwrap();
    let pa1_rc = value.get("paymentAddress").unwrap().as_str().unwrap();
    if pa1_rc == payment_addresses[0] {
        assert_eq!(value.get("amount").unwrap().as_i64().unwrap(), 10);
    } else if pa1_rc == payment_addresses[1] {
        assert_eq!(value.get("amount").unwrap().as_i64().unwrap(), 20);
    } else {
        assert!(false);
    }

    let value = utxos.get(1).unwrap().as_object().unwrap();
    let pa1_rc = value.get("paymentAddress").unwrap().as_str().unwrap();
    if pa1_rc == payment_addresses[0] {
        assert_eq!(value.get("amount").unwrap().as_i64().unwrap(), 10);
    } else if pa1_rc == payment_addresses[1] {
        assert_eq!(value.get("amount").unwrap().as_i64().unwrap(), 20);
    } else {
        assert!(false);
    }
}

#[test]
#[ignore]
pub fn build_and_submit_payment_req_insufficient_funds() {
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 2,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: Some(vec![30])
    });
    let Setup {addresses, pool_handle, trustees, ..} = setup;
    let dids = trustees.dids();

    let (utxo, _, _) = utils::get_utxo::get_first_utxo_for_payment_address(wallet.handle, pool_handle, dids[0], &addresses[0]);

    let inputs = json!([utxo]).to_string();
    let outputs = json!([
        {
            "paymentAddress": addresses[1],
            "amount": 20
        },
        {
            "paymentAddress": addresses[0],
            "amount": 20
        }
    ]).to_string();
    let (req, method) = indy::payments::Payment::build_payment_req(wallet.handle, dids[0], &inputs, &outputs).unwrap();
    let res = indy::ledger::Ledger::submit_request(pool_handle, &req).unwrap();
    let res = indy::payments::Payment::parse_payment_response(&method, &res).unwrap_err();
    assert_eq!(res, ErrorCode::PaymentInsufficientFundsError);
}