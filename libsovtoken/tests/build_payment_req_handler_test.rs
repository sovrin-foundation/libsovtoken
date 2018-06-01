extern crate libc;

extern crate sovtoken;
extern crate rust_indy_sdk as indy;                      // lib-sdk project

#[macro_use] extern crate lazy_static;

#[macro_use]
extern crate serde_json;

use indy::ErrorCode;
//use indy::IndyHandle;
//use indy::utils::results::ResultHandler;

use libc::c_char;
use std::ptr;
use std::ffi::CString;

mod utils;
use self::indy::wallet::Wallet;

//use sovtoken::logic::fees::{Fees};
//use sovtoken::logic::output::Output;
//use sovtoken::logic::input::Input;
use sovtoken::utils::ffi_support::c_pointer_from_string;
use indy::payments::Payment;

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


//fn call_build_req(wallet_handle: IndyHandle, inputs: String, outputs: String, did: String) -> Result<String, ErrorCode> {
//    let (receiver, command_handle, cb) = utils::callbacks::closure_to_cb_ec_string();
//
//    let error_code = sovtoken::api::build_payment_req_handler(
//        COMMAND_HANDLE,
//        WALLET_HANDLE,
//        c_pointer_from_string(did),
//        c_pointer_from_string(inputs),
//        c_pointer_from_string(outputs),
//        CB
//    );
//
//    return ResultHandler::one(error_code, receiver);
//}

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
    assert_eq!(return_error, ErrorCode::CommonInvalidParam5, "Expecting Callback for 'build_payment_req_handler'");
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
    assert_eq!(return_error, ErrorCode::CommonInvalidParam2, "Expecting inputs_json for 'build_payment_req_handler'");
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
    assert_eq!(return_error, ErrorCode::CommonInvalidParam2, "Expecting outputs_json for 'build_payment_req_handler'");
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
    assert_eq!(return_error, ErrorCode::CommonInvalidParam2, "Expecting outputs_json for 'build_payment_req_handler'");
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

    println!("wallet id = {:?}", wallet_id);
    println!("payment_address_1 = {:?}", payment_address_1);
    println!("payment_address_2 = {:?}", payment_address_2);
    println!("payment_address_3 = {:?}", payment_address_3);
    println!("payment_address_4 = {:?}", payment_address_4);



    let inputs = json!([
        {
            "address": payment_address_1,
            "seqno": 1
        },
        {
            "address": payment_address_2,
            "seqno": 1,
            "extra": "extra data",
        }
     ]);

    let outputs = json!([
        {
            "address": payment_address_3,
            "amount": 10
        },
        {
            "address": payment_address_4,
            "amount": 22,
            "extra": "extra data"
        }
    ]);



    let _expected_request = json!({


    });




    println!("Calling build_payment_req");

    let result = sovtoken::api::build_payment_req_handler(
        COMMAND_HANDLE,
        wallet_id,
        c_pointer_from_string(did),
        c_pointer_from_string(inputs.to_string()),
        c_pointer_from_string(outputs.to_string()),
        CB
    );

    println!("Received result {:?}", result);

    assert!(true);
   // assert_eq!(return_error, ErrorCode::CommonInvalidParam2, "Expecting outputs_json for 'build_payment_req_handler'");
}


//#[test]
fn create_a_new_wallet(){

    let wallet_name = "new_wallet";

    Wallet::create("pool_1", wallet_name, None, Some("{}"), None).unwrap();
    let wallet_id: i32 = Wallet::open(wallet_name, None, None).unwrap();

    println!("wallet_id = {:?}", wallet_id);

//    let payment_address = create

    assert!(true);

}