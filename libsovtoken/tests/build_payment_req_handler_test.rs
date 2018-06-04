extern crate env_logger;
extern crate libc;
extern crate sovtoken;
extern crate rust_indy_sdk as indy;                      // lib-sdk project

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_json;

use indy::ErrorCode;
use indy::payments::Payment;
use libc::c_char;
use sovtoken::utils::ffi_support::c_pointer_from_string;
use std::ptr;
use std::ffi::CString;
mod utils;
use self::indy::wallet::Wallet;


// ***** HELPER METHODS *****
extern "C" fn empty_create_payment_callback(_command_handle_: i32, _err: ErrorCode, _payment_req: *const c_char) -> ErrorCode {
    return ErrorCode::Success;
}

// ***** HELPER TEST DATA  *****

const COMMAND_HANDLE:i32 = 10;
static INVALID_OUTPUT_JSON: &'static str = r#"{"totally" : "Not a Number", "bobby" : "DROP ALL TABLES"}"#;
static VALID_OUTPUT_JSON: &'static str = r#"{"outputs":[["AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja",10]]}"#;
const WALLET_HANDLE:i32 = 0;
const CB : Option<extern fn(_command_handle_: i32, err: ErrorCode, payment_req_json: *const c_char) -> ErrorCode > = Some(empty_create_payment_callback);


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
    assert_eq!(return_error, ErrorCode::CommonInvalidStructure, "Expecting Callback for 'build_payment_req_handler'");
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
    assert_eq!(return_error, ErrorCode::CommonInvalidStructure, "Expecting inputs_json for 'build_payment_req_handler'");
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
    assert_eq!(return_error, ErrorCode::CommonInvalidStructure, "Expecting outputs_json for 'build_payment_req_handler'");
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
    assert_eq!(return_error, ErrorCode::CommonInvalidStructure, "Expecting outputs_json for 'build_payment_req_handler'");
}

#[test]
fn success_signed_request() {

    sovtoken::api::sovtoken_init();

    let did = String::from("287asdjkh2323kjnbakjs");

    let wallet_id : i32 = utils::wallet::create_wallet("my_new_wallet");

    let payment_address_1 = Payment::create_payment_address(wallet_id,"pay:sov:", "{}").unwrap();
    let payment_address_2 = Payment::create_payment_address(wallet_id, "pay:sov:", "{}").unwrap();
    let payment_address_3 = Payment::create_payment_address(wallet_id, "pay:sov:", "{}").unwrap();
    let payment_address_4 = Payment::create_payment_address(wallet_id, "pay:sov:", "{}").unwrap();

    debug!("wallet id = {:?}", wallet_id);
    debug!("payment_address_1 = {:?}", payment_address_1);
    debug!("payment_address_2 = {:?}", payment_address_2);
    debug!("payment_address_3 = {:?}", payment_address_3);
    debug!("payment_address_4 = {:?}", payment_address_4);

    let inputs = json!({
        "ver": 1,
        "inputs": [
            {
                "address": payment_address_1,
                "seqNo": 1
            },
            {
                "address": payment_address_2,
                "seqNo": 1,
                "extra": "extra data",
            }
        ]
    });

    let outputs = json!({
        "ver": 1,
        "outputs": [
            {
                "address": payment_address_3,
                "amount": 10
            },
            {
                "address": payment_address_4,
                "amount": 22,
                "extra": "extra data"
            }
        ]
    });


    trace!("Calling build_payment_req");

    let result = sovtoken::api::build_payment_req_handler(
        COMMAND_HANDLE,
        wallet_id,
        c_pointer_from_string(did),
        c_pointer_from_string(inputs.to_string()),
        c_pointer_from_string(outputs.to_string()),
        CB
    );

    trace!("Received result {:?}", result);

    assert_eq!(result, ErrorCode::Success);
}
