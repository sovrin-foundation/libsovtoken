extern crate env_logger;
extern crate libc;
extern crate sovtoken;
extern crate rust_indy_sdk as indy;                      // lib-sdk project
extern crate rust_base58;

#[macro_use] extern crate log;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;

use indy::ErrorCode;
use indy::payments::Payment;
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
use std::collections::HashMap;


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


fn generate_payment_addresses(wallet_handle: i32) -> (Vec<String>, Vec<String>) {
    let seed_1 = json!({
        "seed": str::repeat("1", 32),
    }).to_string();

    let seed_2 = json!({
        "seed": str::repeat("2", 32),
    }).to_string();

    let seed_3 = json!({
        "seed": str::repeat("3", 32),
    }).to_string();

    let seed_4 = json!({
        "seed": str::repeat("4", 32),
    }).to_string();


    let payment_addresses = vec![
        Payment::create_payment_address(wallet_handle, "sov", &seed_2).unwrap(),
        Payment::create_payment_address(wallet_handle, "sov", &seed_3).unwrap(),
        Payment::create_payment_address(wallet_handle, "sov", &seed_4).unwrap(),
        Payment::create_payment_address(wallet_handle, "sov", &seed_1).unwrap(),
    ];

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

    let wallet = Wallet::new("p1");
    debug!("wallet id = {:?}", wallet.handle);

    let (payment_addresses, addresses) = generate_payment_addresses(wallet.handle);
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

    let wallet = Wallet::new("p1");
    debug!("wallet id = {:?}", wallet.handle);

    let (payment_addresses, addresses) = generate_payment_addresses(wallet.handle);

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
    sovtoken::api::sovtoken_init();
    let payment_method = sovtoken::api::PAYMENT_METHOD_NAME;
    indy::pool::Pool::set_protocol_version(2).unwrap();

    let pool_config = utils::pool::create_pool_config();
    let pool_name = utils::pool::create_pool_ledger(Some(&pool_config));
    let wallet = Wallet::new(&pool_name);
    let pool_handle = indy::pool::Pool::open_ledger(&pool_name, None).unwrap();

    let pa1 = indy::payments::Payment::create_payment_address(wallet.handle, payment_method, "{}").unwrap();
    let pa2 = indy::payments::Payment::create_payment_address(wallet.handle, payment_method, "{}").unwrap();

    let mut mint_cfg = HashMap::new();
    mint_cfg.insert(pa1.clone(), 30);

    let (did_trustee, _) = indy::did::Did::new(wallet.handle, &json!({"seed":"000000000000000000000000Trustee1"}).to_string()).unwrap();

    utils::mint::mint_tokens(mint_cfg, pool_handle, wallet.handle, &did_trustee).unwrap();

    let (req, method) = indy::payments::Payment::build_get_utxo_request(wallet.handle, &did_trustee, &pa1).unwrap();
    let res = indy::ledger::Ledger::sign_and_submit_request(pool_handle, wallet.handle, &did_trustee, &req).unwrap();
    let res = indy::payments::Payment::parse_get_utxo_response(&method, &res).unwrap();

    let res_parsed: serde_json::Value = serde_json::from_str(&res).unwrap();
    let utxos = res_parsed.as_array().unwrap();
    let value = utxos.get(0).unwrap().as_object().unwrap();
    let utxo = value.get("txo").unwrap().as_str().unwrap();

    let inputs = json!([utxo]).to_string();
    let outputs = json!([
        {
            "paymentAddress": pa2,
            "amount": 20
        },
        {
            "paymentAddress": pa1,
            "amount": 10
        }
    ]).to_string();
    let (req, method) = indy::payments::Payment::build_payment_req(wallet.handle, &did_trustee, &inputs, &outputs).unwrap();
    let res = indy::ledger::Ledger::submit_request(pool_handle, &req).unwrap();
    let res = indy::payments::Payment::parse_payment_response(&method, &res).unwrap();

    let res_parsed: serde_json::Value = serde_json::from_str(&res).unwrap();
    let utxos = res_parsed.as_array().unwrap();
    assert_eq!(utxos.len(), 2);

    let value = utxos.get(0).unwrap().as_object().unwrap();
    let pa1_rc = value.get("paymentAddress").unwrap().as_str().unwrap();
    if pa1_rc == pa1 {
        assert_eq!(value.get("amount").unwrap().as_i64().unwrap(), 10);
    } else if pa1_rc == pa2 {
        assert_eq!(value.get("amount").unwrap().as_i64().unwrap(), 20);
    } else {
        assert!(false);
    }

    let value = utxos.get(1).unwrap().as_object().unwrap();
    let pa1_rc = value.get("paymentAddress").unwrap().as_str().unwrap();
    if pa1_rc == pa1 {
        assert_eq!(value.get("amount").unwrap().as_i64().unwrap(), 10);
    } else if pa1_rc == pa2 {
        assert_eq!(value.get("amount").unwrap().as_i64().unwrap(), 20);
    } else {
        assert!(false);
    }
}