//! Implementation of the Indy-Sdk Payment API handlers.  No business logic in these methods.
//!

#![allow(unused_variables)]

use std;

use libc::c_char;

use indy::payments::Payment;
use indy::ErrorCode;
use logic::add_request_fees;
use logic::build_payment;
use logic::did::Did;
use logic::xfer_payload::XferPayload;
use logic::indy_sdk_api::crypto_api::CryptoSdk;
use logic::minting;
use logic::payments::{CreatePaymentHandler};
use logic::set_fees;

use logic::config::{
    payment_config::{PaymentRequest},
    payment_address_config::{PaymentAddressConfig},
    set_fees_config::{SetFeesRequest},
    get_fees_config::GetFeesRequest,
    get_utxo_config::*,
};

use logic::parsers::{
    parse_get_utxo_response::{ParseGetUtxoResponse, ParseGetUtxoReply},
    parse_payment_response::{ParsePaymentResponse, ParsePaymentReply},
    parse_response_with_fees_handler::{ParseResponseWithFees, ParseResponseWithFeesReply},
    parse_get_txn_fees::parse_fees_from_get_txn_fees_response
};

use utils::ffi_support::{str_from_char_ptr, cstring_from_str, string_from_char_ptr, c_pointer_from_string};
use utils::json_conversion::{JsonDeserialize, JsonSerialize};
use utils::general::ResultExtension;

/**
    Defines a callback to communicate results to Indy-sdk as type

    # Params
    command_handle : should be the same value as the API inputted command handle
    err:  results.
    json_pointer: results data.  format is defined by the API
*/
pub type JsonCallback = Option<JsonCallbackUnwrapped>;
pub type JsonCallbackUnwrapped =  extern fn(command_handle: i32, err: i32, json_pointer: *const c_char) -> i32;

/// This method generates private part of payment address
/// and stores it in a secure place. It should be a
/// secret in libindy wallet (see crypto module).
///
/// Note that payment method should be able to resolve this
/// secret by fully resolvable payment address format.
///
/// from tokens-interface.md/CreatePaymentAddressCB
///
/// # Params
/// command_handle: command handle to map callback to context
/// config_str: payment address config as json:
///   {
///     seed: <str>, // allows deterministic creation of payment address
///   }
/// cb: description
///
/// # Returns
/// on Success:  payment_address will have the format:
///              pay:sov:{32 byte public key}{4 digit check sum}
///
/// # Errors
/// description of errors
#[no_mangle]
pub extern "C" fn create_payment_address_handler(command_handle: i32,
                                                 wallet_handle: i32,
                                                 config_str: *const c_char,
                                                 cb: JsonCallback) -> i32 {

    check_useful_c_callback!(cb, ErrorCode::CommonInvalidStructure as i32);

    if config_str.is_null() {
        return ErrorCode::CommonInvalidStructure as i32;
    }

    let json_config_str: String = match string_from_char_ptr(config_str) {
        Some(s) => s,
        None => return ErrorCode::CommonInvalidStructure as i32,
    };

    // indy-sdk accepts { } for valid seed info to create a key.  Serde deseralization does not
    // like { } as valid.  if we get any kind of serialization failure assume we can use the default
    let config: PaymentAddressConfig = match PaymentAddressConfig::from_json(&json_config_str) {
        Ok(c) => c,
        Err(_) => PaymentAddressConfig { seed : "".to_string()},
    };

    let payment_closure = move | payment_address : String, err: ErrorCode | {

        if err != ErrorCode::Success {
            error!("create payment address failed ErrorCode={:?}", err);
            cb(command_handle, ErrorCode::CommonInvalidState as i32, std::ptr::null());
            return;
        }

        debug!("create_payment_address_handler returning payment address of '{}'", &payment_address);
        let payment_address_cstring = cstring_from_str(payment_address);
        let payment_address_ptr = payment_address_cstring.as_ptr();

        cb(command_handle, ErrorCode::Success as i32, payment_address_ptr);
    };

    let handler = CreatePaymentHandler::new(CryptoSdk {} );
    return handler.create_payment_address_async(wallet_handle, config, payment_closure) as i32;
}

/// Description
/// call made to wallet to list payment addresses
///    * missing from Slava
///
/// #Params
/// param1: description.
///
/// #Returns
/// description. example if json, etc...
///
/// #Errors
/// description of errors
#[no_mangle]
pub extern "C" fn list_payment_addresses_handler() -> i32 {
    return ErrorCode::Success as i32;
}

/**
 * Add fees to a request.
 * 
 * Adds the inputs and outputs to fees for a **non transfer ("10000")** request.
 * If you are building a transfer request, fees should be included in the 
 * `inputs_json` and `outputs_json` of the [`build_payment_req_handler`].
 * 
 * 
 * ## Parameters
 * 
 * ### request_json
 * Request json needs to contain an operation field. The operation needs to
 * contain a type field. The type can not be "10000".
 * 
 * Here is the minimal version that could work.
 * ```JSON
 * {
 *      "operation": {
 *          "type:": "3"
 *      }
 * }
 * ```
 * 
 * ### inputs_json
 * ```JSON
 * {
 *     "ver": <int>
 *     "inputs": [
 *          {
 *              "address": <str: payment_address>,
 *              "seqNo": <int>
 *          }
 *     ]
 * }
 * ```
 * 
 * ### outputs_json
 * ```JSON
 * {
 *      "ver": <int>
 *      "outputs": [
 *          {
 *              "address": <str: payment_address>,
 *              "amount": <int>
 *              "extra": <str>
 *          }
 *      ]
 * }
 * ```
 * 
 * ## Example
 * 
 * ### Parameters
 * 
 * #### request_json
 * ```JSON
 * {
 *      "operation": {
 *          "type": "3"
 *      }
 * }
 * ```
 * 
 * #### inputs_json
 * ```JSON
 * {
 *      "ver": 1,
 *      "inputs": [
 *          {
 *              "address": "pay:sov:7LSfLv2S6K7zMPrgmJDkZoJNhWvWRzpU7qt9uMR5yz8GYjJM",
 *              "seqNo": 1
 *          }
 *      ]
 * }
 * ```
 * 
 * #### outputs_json
 * ```JSON
 * {
 *      "ver": 1,
 *      "outputs": [
 *          {
 *              "address": "pay:sov:x39ETFpHu2WDGIKLMwxSWRilgyN9yfuPx8l6ZOev3ztG1MJ6",
 *              "amount": "10"
 *          }
 *      ]
 * }
 * ```
 * 
 * ### Return
 * 
 * #### Expected req_with_fees_json
 * ```JSON
 * {
 *      "operation": {
 *          "type": 3
 *      },
 *      "fees": {
 *          "inputs": [["7LSfLv2S6K7zMPrgmJDkZoJNhWvWRzpU7qt9uMR5yz8GYjJM", 1, "2uU4zJWjVMKAmabQefkxhFc3K4BgPuwqVoZUiWYS2Ct9hidmKF9hcLNBjw76EjuDuN4RpzejKJUofJPcA3KhkBvi"]],
 *          "outputs": [["x39ETFpHu2WDGIKLMwxSWRilgyN9yfuPx8l6ZOev3ztG1MJ6", 10]]
 *      }
 * }
 * ```
 */
#[no_mangle]
pub extern "C" fn add_request_fees_handler(command_handle: i32,
                                           wallet_handle: i32,
                                           did: *const c_char, // TODO: Need to remove.
                                           req_json: *const c_char,
                                           inputs_json: *const c_char,
                                           outputs_json: *const c_char,
                                           cb: Option<extern fn(command_handle_: i32,
                                                               err: i32,
                                                               req_with_fees_json: *const c_char) -> i32>) -> i32 {

    let (inputs, outputs, request_json_map, cb) = match add_request_fees::deserialize_inputs(req_json, inputs_json, outputs_json, cb) {
        Ok(tup) => tup,
        Err(error_code) => {
            error!("Error in deserializing the add_request_fees_handler arguments.");
            return error_code as i32;
        }
    };

    /*
        Errors when the request is a XFER request becaause the 
        fees should be implicit in the operation's inputs and
        outputs.
    */
    if let Err(_) = add_request_fees::validate_type_not_transfer(&request_json_map) {
        error!("Can't add fees to a transfer request");
        return ErrorCode::CommonInvalidStructure as i32;
    }

    let serialized_request_with_fees = match add_request_fees::add_fees_to_request_and_serialize(wallet_handle, inputs, outputs, request_json_map) {
        Ok(map) => map,
        Err(e) => {
            error!("Received error adding fees to request_json'");
            return e as i32;
        }
    };

    cb(command_handle, ErrorCode::Success as i32, c_pointer_from_string(serialized_request_with_fees));

    return ErrorCode::Success as i32;
}


/// Parses inputted output fees section and returns it in utxo format
///
///
/// from tokens-interface.md/ParseResponseWithFeesCB
/// # Params
/// command_handle: standard command handle
/// req_json: json. \For format see https://github.com/evernym/libsovtoken/blob/master/doc/data_structures.md
///
/// # Returns
/// utxo_json: json. For format see https://github.com/evernym/libsovtoken/blob/master/doc/data_structures.md
///
/// # Errors
/// CommonInvalidStructure when any of the inputs are invalid
/// CommonInvalidState when any processing of inputs produces invalid results
#[no_mangle]
pub extern "C" fn parse_response_with_fees_handler(command_handle: i32,
                                                   req_json: *const c_char,
                                                   cb: Option<extern fn(command_handle_: i32,
                                                               err: i32,
                                                               utxo_json: *const c_char) -> i32>) -> i32 {
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidStructure as i32);

    if req_json.is_null() {
        return ErrorCode::CommonInvalidStructure as i32;
    }

    let resp_json_string = match string_from_char_ptr(req_json) {
        Some(s) => s,
        None => {
            error!("Failed to convert inputs_json pointer to string");
            return ErrorCode::CommonInvalidStructure as i32;
        }
    };

    let response: ParseResponseWithFees = match ParseResponseWithFees::from_json(&resp_json_string) {
        Ok(r) => r,
        Err(e) => return ErrorCode::CommonInvalidStructure as i32,
    };

    // here is where the magic happens--conversion from input structure to output structure
    // is handled in ParseResponseWithFeesReply::from_response
    let reply: ParseResponseWithFeesReply = ParseResponseWithFeesReply::from_response(response);

    let reply_str: String = match reply.to_json() {
        Ok(j) => j,
        Err(e) => return ErrorCode::CommonInvalidState as i32,
    };

    let reply_str_ptr: *const c_char = c_pointer_from_string(reply_str);

    cb(command_handle, ErrorCode::Success as i32, reply_str_ptr);

    return ErrorCode::Success as i32;
}


/**
 * Build a payment request.
 * 
 * Builds a payment request which can transfer funds from
 * addresses to other addresses.
 * 
 * The amount in the output addresses needs to match the
 * amount stored in input addresses.
 * 
 * ## Parameters
 * 
 * ### inputs_json
 * ```JSON
 * {
 *     "ver": <int>
 *     "inputs": [
 *          {
 *              "address": <str: payment_address>,
 *              "seqNo": <int>
 *          }
 *     ]
 * }
 * ```
 * 
 * ### outputs_json
 * ```JSON
 * {
 *      "ver": <int>
 *      "outputs": [
 *          {
 *              "address": <str: payment_address>,
 *              "amount": <int>
 *              "extra": <str>
 *          }
 *      ]
 * }
 * ```
 * 
 * ## Returns
 * Returns a json object of the payment request.
 * ```JSON
 * {
 *      "identifier": <str>,
 *      "reqId": <int>,
 *      "operation" {
 *          "type": "10001",
 *          "inputs": [<str: payment_address>, <int: seq_no>, <str: signature>],
 *          "outputs": [<str: payment_address>, <int: amount>]
 *      }
 * }
 * ```
 */
#[no_mangle]
pub extern "C" fn build_payment_req_handler(command_handle: i32,
                                            wallet_handle: i32,
                                            submitter_did: *const c_char,
                                            inputs_json: *const c_char,
                                            outputs_json: *const c_char,
                                            cb: Option<extern fn(command_handle_: i32,
                                                                 err: i32,
                                                                 payment_req_json: *const c_char) -> i32>) -> i32 {
    let (inputs, outputs, cb) = match build_payment::deserialize_inputs(inputs_json, outputs_json, cb) {
        Ok(tup) => tup,
        Err(error_code) => {
            error!("Error in deserializing the build_payment_req_handler arguments.");
            return error_code as i32;
        }
    };

    let payload = XferPayload::new(inputs, outputs);
    let payload_signed = payload.sign(&CryptoSdk {}, wallet_handle).unwrap();
    debug!("Signed payload >>> {:?}", payload_signed);

    let identifier = payload_signed.inputs[0].address.clone();

    let payment_request = PaymentRequest::new(payload_signed, identifier);

    let payment_request = payment_request.serialize_to_cstring().unwrap();

    debug!("payment_request >>> {:?}", payment_request);

    cb(command_handle, ErrorCode::Success as i32, payment_request.as_ptr());
    return ErrorCode::Success as i32;
}

/// Parses inputted payment data and returns formatted UTXOs
///
///
/// from tokens-interface.md/ParsePaymentResponseCB
/// # Params
/// command_handle: standard command handle
/// resp_json: json. \For format see https://github.com/evernym/libsovtoken/blob/master/doc/data_structures.md
///
/// # Returns
/// utxo_json: json. For format see https://github.com/evernym/libsovtoken/blob/master/doc/data_structures.md
///
/// # Errors
/// CommonInvalidStructure when any of the inputs are invalid
/// CommonInvalidState when any processing of inputs produces invalid results
#[no_mangle]
pub extern "C" fn parse_payment_response_handler(command_handle: i32,
                                                 resp_json: *const c_char,
                                                 cb: Option<extern fn(command_handle_: i32,
                                                             err: i32,
                                                             utxo_json: *const c_char) -> i32>) -> i32 {

    check_useful_c_callback!(cb, ErrorCode::CommonInvalidStructure as i32);

    if resp_json.is_null() {
        return ErrorCode::CommonInvalidStructure as i32;
    }

    let resp_json_string = match string_from_char_ptr(resp_json) {
        Some(s) => s,
        None => {
            error!("Failed to convert inputs_json pointer to string");
            return ErrorCode::CommonInvalidStructure as i32;
        }
    };

    let response: ParsePaymentResponse = match ParsePaymentResponse::from_json(&resp_json_string) {
        Ok(r) => r,
        Err(e) => return ErrorCode::CommonInvalidStructure as i32,
    };

    // here is where the magic happens--conversion from input structure to output structure
    // is handled in ParsePaymentReply::from_response
    let reply: ParsePaymentReply = ParsePaymentReply::from_response(response);

    let reply_str: String = match reply.to_json() {
        Ok(j) => j,
        Err(e) => return ErrorCode::CommonInvalidState as i32,
    };

    let reply_str_ptr: *const c_char = c_pointer_from_string(reply_str);

    cb(command_handle, ErrorCode::Success as i32, reply_str_ptr);

    return ErrorCode::Success as i32;
}


/// Description
///
///
/// from tokens-interface.md/BuildGetUTXORequestCB
/// #Params
/// param1: description.
///
/// #Returns
/// description. example if json, etc...
///
/// #Errors
/// description of errors
#[no_mangle]
pub extern "C" fn build_get_utxo_request_handler(command_handle: i32,
                                                 wallet_handle: i32,
                                                 submitter_did: *const c_char,
                                                 payment_address: *const c_char,
                                                 cb: JsonCallback)-> i32 {

    let handle_result = api_result_handler!(< *const c_char >, command_handle, cb);

    let payment_address = match str_from_char_ptr(payment_address) {
        Some(s) => s,
        None => {
            error!("Failed to convert submitter_did pointer to string");
            return ErrorCode::CommonInvalidStructure as i32;
        }
    };

    let did = match Did::from_pointer(submitter_did) {
        Some(did) => did,
        None => return ErrorCode::CommonInvalidStructure as i32
    };

    let did = match did.validate() {
        Ok(did_valid) => did_valid,
        Err(_) => return ErrorCode::CommonInvalidStructure as i32
    };

    let utxo_request = GetUtxoRequest::new(String::from(payment_address), did.into());
    let utxo_request = utxo_request.serialize_to_cstring().unwrap();

    handle_result(Ok(utxo_request.as_ptr())) as i32
}

/// Description
///
///
///
/// from tokens-interface.md/ParseGetUTXOResponseCB
/// # Params
/// command_handle: standard command handle
/// resp_json: json. \For format see https://github.com/evernym/libsovtoken/blob/master/doc/data_structures.md
///
/// # Returns
/// utxo_json: json. For format see https://github.com/evernym/libsovtoken/blob/master/doc/data_structures.md
///
/// # Errors
/// CommonInvalidStructure when any of the inputs are invalid
/// CommonInvalidState when any processing of inputs produces invalid results
#[no_mangle]
pub extern "C" fn parse_get_utxo_response_handler(command_handle: i32,
                                                  resp_json: *const c_char,
                                                  cb: Option<extern fn(command_handle_: i32,
                                                                       err: i32,
                                                                       utxo_json: *const c_char) -> i32>)-> i32 {

    check_useful_c_callback!(cb, ErrorCode::CommonInvalidStructure as i32);

    if resp_json.is_null() {
        return ErrorCode::CommonInvalidStructure as i32;
    }

    let resp_json_string = match string_from_char_ptr(resp_json) {
        Some(s) => s,
        None => {
            error!("Failed to convert inputs_json pointer to string");
            return ErrorCode::CommonInvalidStructure as i32;
        }
    };

    let response: ParseGetUtxoResponse = match ParseGetUtxoResponse::from_json(&resp_json_string) {
        Ok(r) => r,
        Err(e) => return ErrorCode::CommonInvalidStructure as i32,
    };

    // here is where the magic happens--conversion from input structure to output structure
    // is handled in ParseGetUtxoReply::from_response
    let reply: ParseGetUtxoReply = ParseGetUtxoReply::from_response(response);

    let reply_str: String = match reply.to_json() {
        Ok(j) => j,
        Err(e) => return ErrorCode::CommonInvalidState as i32,
    };

    let reply_str_ptr: *const c_char = c_pointer_from_string(reply_str);

    cb(command_handle, ErrorCode::Success as i32, reply_str_ptr);
    return ErrorCode::Success as i32;
}

/// Description
///
///
/// from tokens-interface.md/BuildSetTxnFeesReqCB
/// #Params
/// param1: description.
///
/// #Returns
/// description. example if json, etc...
///
/// #Errors
/// description of errors
#[no_mangle]
pub extern "C" fn build_set_txn_fees_handler(command_handle: i32,
                                         wallet_handle: i32,
                                         submitter_did: *const c_char,
                                         fees_json: *const c_char,
                                         cb: Option<extern fn(command_handle_: i32, err: i32, set_txn_fees_json: *const c_char) -> i32>) -> i32 {

    let (did, fees_config, cb) = match set_fees::deserialize_inputs(
        submitter_did,
        fees_json,
        cb
    ) {
        Ok(tup) => tup,
        Err(e) => return e as i32
    };

    let fees_request = SetFeesRequest::from_fee_config(fees_config, did.into());

    let fees_request_pointer_option = fees_request.serialize_to_pointer()
        .or(Err(ErrorCode::CommonInvalidStructure));

    let fees_request_pointer = match fees_request_pointer_option {
        Ok(ptr) => ptr,
        Err(e) => return e as i32,
    };

    cb(command_handle, ErrorCode::Success as i32, fees_request_pointer);

    return ErrorCode::Success as i32;
}

/// Description
///
///
/// from tokens-interface.md/BuildGetTxnFeesReqCB
/// # Params
/// param1: description.
///
/// # Returns
/// description. example if json, etc...
///
/// # Errors
/// description of errors
#[no_mangle]
pub extern "C" fn build_get_txn_fees_handler(command_handle: i32,
                                             wallet_handle: i32,
                                             submitter_did: *const c_char,
                                             cb: Option<extern fn(command_handle_: i32, err: i32, get_txn_fees_json: *const c_char) -> i32>) -> i32 {

    let handle_result = api_result_handler!(< *const c_char >, command_handle, cb);

    if cb.is_none() {
        return handle_result(Err(ErrorCode::CommonInvalidStructure)) as i32;
    }

    let submitter_did = match string_from_char_ptr(submitter_did) {
        Some(s) => s,
        None => {
            error!("Failed to convert submitter_did pointer to string");
            return ErrorCode::CommonInvalidStructure as i32;
        }
    };

    let get_txn_request = GetFeesRequest::new(submitter_did);

    let get_txn_request = get_txn_request.serialize_to_cstring().unwrap();

    return handle_result(Ok(get_txn_request.as_ptr())) as i32;
}

/// Description
/// from tokens-interface.md/ParseGetTxnFeesResponseCB
///
/// # Params
/// command_handle: a standard command handle
/// resp_json: JSON String. Structure of JSON available in libsovtoken/docs/data_structures.md
///
/// # Returns
/// fees_json: JSON String. Structure of JSON available in libsovtoken/docs/data_structures.md
///
/// # Errors
///
#[no_mangle]
pub extern "C" fn parse_get_txn_fees_response_handler(command_handle: i32,
                                                      resp_json: *const c_char,
                                                      cb: Option<extern fn(command_handle_: i32,
                                                                err: i32,
                                                                fees_json: *const c_char) -> i32>)-> i32{
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidStructure as i32);
    if resp_json.is_null() {
        return ErrorCode::CommonInvalidStructure as i32;
    }
    let resp_json_string = match string_from_char_ptr(resp_json) {
        Some(s) => s,
        None => {
            error!("Failed to convert inputs_json pointer to string");
            return ErrorCode::CommonInvalidStructure as i32;
        }
    };
    let fees_json_obj =
        match parse_fees_from_get_txn_fees_response(resp_json_string){
            Ok(s) => {
                s
            },
            Err(_) => {
                return ErrorCode::CommonInvalidStructure as i32;
            }
        };
    let fees_json_ptr : *const c_char = c_pointer_from_string(fees_json_obj);
    cb(command_handle, ErrorCode::Success as i32, fees_json_ptr);
    return ErrorCode::Success as i32;
}


/**
 * Build a mint transaction request.
 * 
 * A mint transaction will need to be signed by a quorum of trustees.
 * 
 * The mint transaction can only be used once.
 * 
 * ## Parameters
 * 
 * ### DID (Decentralized Identifier)
 * 
 * ### outputs_json
 * ```JSON
 * {
 *      "ver": <int>
 *      "outputs": [
 *          {
 *              "address": <str: payment_address>,
 *              "amount": <int>
 *              "extra": <str>
 *          }
 *      ]
 * }
 */
#[no_mangle]
pub extern "C" fn build_mint_txn_handler(
    command_handle:i32,
    wallet_handle: i32,
    submitter_did: *const c_char,
    outputs_json: *const c_char,
    cb: JsonCallback) -> i32
{
    let (did, outputs, cb) = match minting::deserialize_inputs(
        submitter_did,
        outputs_json,
        cb
    ) {
        Ok(tup) => tup,
        Err(e) => return e as i32,
    };
    trace!("Deserialized build_mint_txn_handler arguments.");

    let mint_request = match minting::build_mint_request(did.into(), outputs) {
        Ok(json) => json,
        Err(e) => return e as i32
    };
    trace!("Serialized mint request as pointer.");

    cb(command_handle, ErrorCode::Success as i32, mint_request);
    return ErrorCode::Success as i32;
}

/**
    exported method indy-sdk will call for us to register our payment methods with indy-sdk

    # Params
    none

    # Returns
    ErrorCode from register_payment_method
*/
#[no_mangle]
pub extern fn sovtoken_init() -> i32 {

    super::utils::logger::init_log();

    debug!("sovtoken_init() started");
    let result = match Payment::register(
        "sov",
        create_payment_address_handler,
        add_request_fees_handler,
        parse_response_with_fees_handler,
        build_get_utxo_request_handler,
        parse_get_utxo_response_handler,
        build_payment_req_handler,
        parse_payment_response_handler,
        build_mint_txn_handler,
        build_set_txn_fees_handler,
        build_get_txn_fees_handler,
        parse_get_txn_fees_response_handler
    ) {
        Ok(()) => ErrorCode::Success ,
        Err(e) => e ,
    };

    debug!("sovtoken_init() returning {:?}", result);
    return result as i32;
}
