extern crate env_logger;
extern crate libc;
extern crate sovtoken;
extern crate rust_indy_sdk as indy;                      // lib-sdk project

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_json;

use indy::ErrorCode;
use indy::payments::Payment;
use indy::utils::results::ResultHandler;
use libc::c_char;
use sovtoken::logic::address;
use sovtoken::utils::ffi_support::c_pointer_from_string;
use std::ptr;
use std::ffi::CString;
mod utils;


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


fn generate_payment_addresses(wallet_id: i32) -> (Vec<String>, Vec<String>) {
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
        Payment::create_payment_address(wallet_id, "pay:sov:", &seed_1).unwrap(),
        Payment::create_payment_address(wallet_id, "pay:sov:", &seed_2).unwrap(),
        Payment::create_payment_address(wallet_id, "pay:sov:", &seed_3).unwrap(),
        Payment::create_payment_address(wallet_id, "pay:sov:", &seed_4).unwrap(),
    ];

    payment_addresses
        .iter()
        .enumerate()
        .for_each(|(idx, address)| debug!("payment_address[{:?}] = {:?}", idx, address));

    let addresses = payment_addresses
        .iter()
        .map(|address| address::verkey_checksum_from_address(address.clone()).unwrap())
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

    let wallet_id : i32 = utils::wallet::create_wallet("my_new_wallet");
    debug!("wallet id = {:?}", wallet_id);

    let (payment_addresses, addresses) = generate_payment_addresses(wallet_id);

    let inputs = json!({
        "ver": 1,
        "inputs": [
            {
                "address": payment_addresses[0],
                "seqNo": 1
            },
            {
                "address": payment_addresses[1],
                "seqNo": 1,
                "extra": "extra data",
            }
        ]
    });

    let outputs = json!({
        "ver": 1,
        "outputs": [
            {
                "address": payment_addresses[2],
                "amount": 10
            },
            {
                "address": payment_addresses[3],
                "amount": 22,
                "extra": "extra data"
            }
        ]
    });

    let expected_operation = json!({
        "type": "10000",
        "inputs": [
            [addresses[0], 1, "57R59eV1mFJPejU34j9XBuEWvMK15KoXSVrTBENUTtYo33DbCmyfVya3pq2pLU4EuKxUBMmcYz9yP4HTw6S2pRQd"],
            [addresses[1], 1, "WgJWvF4b4FPB5Z2CPVC7C1drDU4AWhAqdALAuQmwp4mgJwPFAHNmvqCDiLD2h4rYiYoJhKN6jnvHpBLkGuYHJeE"]
        ],
        "outputs": [[addresses[2], 10], [addresses[3], 22]],
    });

    let (receiver, command_handle, cb) = utils::callbacks::closure_to_cb_ec_string();


    trace!("Calling build_payment_req");

    let error_code = sovtoken::api::build_payment_req_handler(
        command_handle,
        wallet_id,
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
    assert_eq!(&addresses[0], request.get("identifier").unwrap());
    assert!(request.get("reqId").is_some());
}
