//! Implementation of the Indy-Sdk Payment API handlers.  No business logic in these methods.
//!

use libc::c_char;
use indy;

use indy::payments::Payment;
use indy::ledger::Ledger;
use indy::ErrorCode;
use logic::api_internals::{
    add_request_fees,
    create_address
};
use logic::build_payment;
use logic::config::{
    get_fees_config::GetFeesRequest,
    get_utxo_config::*,
};
use logic::did::Did;
use logic::indy_sdk_api::crypto_api::CryptoSdk;
use logic::minting;
use logic::verify;
use logic::parsers::{
    parse_get_utxo_response,
    parse_response_with_fees_handler,
    parse_verify,
    parse_get_utxo_response::{ParseGetUtxoResponse, ParseGetUtxoReply},
    parse_payment_response::{ParsePaymentResponse, ParsePaymentReply, from_response},
    parse_response_with_fees_handler::{ParseResponseWithFees, ParseResponseWithFeesReply},
    parse_get_txn_fees::{parse_fees_from_get_txn_fees_response, get_fees_state_proof_extractor}
};
use logic::payments::{CreatePaymentHandler};
use logic::set_fees;
use logic::xfer_payload::XferPayload;

use utils::constants::general::{JsonCallback, PAYMENT_METHOD_NAME, LEDGER_ID};
use utils::constants::txn_types::{GET_FEES, GET_UTXO};
use utils::ffi_support::{str_from_char_ptr, string_from_char_ptr, c_pointer_from_string};
use utils::json_conversion::{JsonDeserialize, JsonSerialize};
use utils::general::ResultExtension;

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
pub extern "C" fn create_payment_address_handler(
    command_handle: i32,
    wallet_handle: i32,
    config_str: *const c_char,
    cb: JsonCallback
) -> i32 {

    trace!("api::create_payment_address_handler called");
    let (config, cb) = match create_address::deserialize_arguments(config_str, cb) {
        Ok(tup) => tup,
        Err(e) => return e as i32
    };

    let payment_closure = create_address::create_address_cb(command_handle, cb);

    let handler = CreatePaymentHandler::new(CryptoSdk {});
    let ec = handler.create_payment_address_async(wallet_handle, config, payment_closure);
    trace!("api::create_payment_address_handler << result: {:?}", ec);
    return ec as i32;
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
 * [<str: txo>]
 * ```
 * 
 * ### outputs_json
 * ```JSON
 * [
 *      {
 *          "recipient": <str: payment_address>,
 *          "amount": <int>
 *          "extra": <str>
 *      }
 * ]
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
 * ["txo:sov:fkjZEd8eTBnYJsw7m7twMph3UYD6KZCuNwGWnmmtGVgkXafzy7fgaWrpKnwVbNnxTdHF5T4vsAZPe3BVkk3Pg5dYdnGedFHaFhWW2PsgqGAyTLfh4Vit"]
 * ```
 * 
 * #### outputs_json
 * ```JSON
 * [
 *      {
 *          "recipient": "pay:sov:x39ETFpHu2WDGIKLMwxSWRilgyN9yfuPx8l6ZOev3ztG1MJ6",
 *          "amount": "10"
 *      }
 * ]
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
 *          "inputs": [["7LSfLv2S6K7zMPrgmJDkZoJNhWvWRzpU7qt9uMR5yz8GYjJM", 1]],
 *          "outputs": [["x39ETFpHu2WDGIKLMwxSWRilgyN9yfuPx8l6ZOev3ztG1MJ6", 10]],
 *          "signatures": ["2uU4zJWjVMKAmabQefkxhFc3K4BgPuwqVoZUiWYS2Ct9hidmKF9hcLNBjw76EjuDuN4RpzejKJUofJPcA3KhkBvi"]
 *      }
 * }
 * ```
 */
#[no_mangle]
pub extern "C" fn add_request_fees_handler(
    command_handle: i32,
    wallet_handle: i32,
    did: *const c_char, // TODO: Need to remove.
    req_json: *const c_char,
    inputs_json: *const c_char,
    outputs_json: *const c_char,
    extra: *const c_char,
    cb: JsonCallback
) -> i32 {

    trace!("api::add_request_fees_handler called did (address) >> {:?}", did);
    let (inputs, outputs, extra, request_json_map, cb) = match add_request_fees::deserialize_inputs(req_json, inputs_json, outputs_json, extra, cb) {
        Ok(tup) => tup,
        Err(error_code) => {
            trace!("api::add_request_fees_handler result >> {:?}", error_code);
            return error_code as i32;
        }
    };

    /*
        Errors when the request is a XFER request becaause the 
        fees should be implicit in the operation's inputs and
        outputs.
    */
    if let Err(_) = add_request_fees::validate_type_not_transfer(&request_json_map) {
        error!("api::add_request_fees_handler Can't add fees to a transfer request");
        return ErrorCode::CommonInvalidStructure as i32;
    }

    let result = add_request_fees::add_fees_to_request_and_serialize(
        wallet_handle,
        inputs,
        outputs,
        extra,
        request_json_map,
        Box::new(add_request_fees::closure_cb_response(command_handle, cb))
    );
    
    match result {
        Err(e) => {
            error!("api::add_request_fees_handler Received error adding fees to request_json");
            return e as i32;
        }
        _ => {
            let res = ErrorCode::Success;
            trace!("api::add_request_fees_handler result >> {:?}", res);
            return res as i32;
        }
    };
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
pub extern "C" fn parse_response_with_fees_handler(
    command_handle: i32,
    req_json: *const c_char,
    cb: JsonCallback
) -> i32 {

    trace!("api::parse_response_with_fees_handler called");
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidStructure as i32);

    if req_json.is_null() {
        trace!("api::parse_response_with_fees_handler << result: {:?}", ErrorCode::CommonInvalidStructure);
        return ErrorCode::CommonInvalidStructure as i32;
    }

    let resp_json_string = match string_from_char_ptr(req_json) {
        Some(s) => s,
        None => {
            error!("Failed to convert inputs_json pointer to string");
            return ErrorCode::CommonInvalidStructure as i32;
        }
    };

    debug!("api::parse_response_with_fees_handler >> req_json: {:?}", resp_json_string);

    let response: ParseResponseWithFees = match ParseResponseWithFees::from_json(&resp_json_string).map_err(map_err_err!()) {
        Ok(r) => r,
        Err(_) => return ErrorCode::CommonInvalidStructure as i32,
    };

    // here is where the magic happens--conversion from input structure to output structure
    // is handled in ParseResponseWithFeesReply::from_response
    let reply: ParseResponseWithFeesReply = match parse_response_with_fees_handler::from_response(response) {
        Ok(rep) => rep,
        Err(ec) => {
            trace!("api::parse_response_with_fees_handler << result: {:?}", ec);
            return ec as i32
        },
    };

    let reply_str: String = match reply.to_json().map_err(map_err_err!()) {
        Ok(j) => j,
        Err(_) => return ErrorCode::CommonInvalidState as i32,
    };

    let reply_str_ptr: *const c_char = c_pointer_from_string(reply_str);
    let ec = ErrorCode::Success;

    cb(command_handle, ec as i32, reply_str_ptr);

    trace!("api::parse_response_with_fees_handler << result: {:?}", ec);
    return ec as i32;
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
 * [<str: txo>, <str: txo>]
 * ```
 * 
 * ### outputs_json
 * ```JSON
 * [
 *      {
 *          "recipient": <str: payment_address>,
 *          "amount": <int>
 *          "extra": <str>
 *      }
 * ]
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
 *          "inputs": [<str: payment_address>, <int: seq_no>],
 *          "outputs": [<str: payment_address>, <int: amount>],
 *          "signatures": [<str: signature>]
 *      }
 * }
 * ```
 */
#[no_mangle]
pub extern "C" fn build_payment_req_handler(
    command_handle: i32,
    wallet_handle: i32,
    submitter_did: *const c_char,
    inputs_json: *const c_char,
    outputs_json: *const c_char,
    extra: *const c_char,
    cb: JsonCallback
) -> i32 {
    trace!("api::build_payment_req_handler called >> submitter_did (address) {:?}", submitter_did);
    let (inputs, outputs, extra, cb) = match build_payment::deserialize_inputs(inputs_json, outputs_json, extra, cb) {
        Ok(tup) => tup,
        Err(error_code) => {
            trace!("api::build_payment_req_handler << result: {:?}", error_code);
            return error_code as i32;
        }
    };

    let payload = XferPayload::new(inputs, outputs, extra);

    let result = payload.sign_transfer(
        &CryptoSdk {},
        wallet_handle,
        Box::new(move |result| build_payment::handle_signing(command_handle, result, cb))
    );

    let ec = match result {
        Ok(()) => ErrorCode::Success,
        Err(ec) => ec
    };
    trace!("api::build_payment_req_handler << result {:?}", ec);
    return ec as i32;
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
pub extern "C" fn parse_payment_response_handler(
    command_handle: i32,
    resp_json: *const c_char,
    cb: JsonCallback
) -> i32 {
    trace!("api::parse_payment_response_handler called");
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidStructure as i32);

    if resp_json.is_null() {
        trace!("api::parse_payment_response_handler << result: {:?}", ErrorCode::CommonInvalidStructure);
        return ErrorCode::CommonInvalidStructure as i32;
    }

    let resp_json_string = match string_from_char_ptr(resp_json) {
        Some(s) => s,
        None => {
            error!("Failed to convert inputs_json pointer to string");
            return ErrorCode::CommonInvalidStructure as i32;
        }
    };

    debug!("api::parse_payment_response_handler >> resp_json: {:?}", &resp_json_string);

    let response: ParsePaymentResponse = match ParsePaymentResponse::from_json(&resp_json_string)
        .map_err(map_err_err!()) {
        Ok(r) => r,
        Err(_) => return ErrorCode::CommonInvalidStructure as i32,
    };

    // here is where the magic happens--conversion from input structure to output structure
    // is handled in ParsePaymentReply::from_response
    let reply: ParsePaymentReply = match from_response(response) {
        Ok(rep) => rep,
        Err(ec) => {
            trace!("api::parse_payment_response_handler << result: {:?}", ec);
            return ec as i32
        },
    };

    let reply_str: String = match reply.to_json().map_err(map_err_err!()) {
        Ok(j) => j,
        Err(_) => return ErrorCode::CommonInvalidState as i32,
    };

    info!("Parsed payment response: {:?}", reply_str);

    let reply_str_ptr: *const c_char = c_pointer_from_string(reply_str);

    cb(command_handle, ErrorCode::Success as i32, reply_str_ptr);
    trace!("api::parse_payment_response_handler << result: {:?}", ErrorCode::Success);
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
                                                 _submitter_did: *const c_char,
                                                 payment_address: *const c_char,
                                                 cb: JsonCallback)-> i32 {
    trace!("api::build_get_utxo_request_handler called");
    let handle_result = api_result_handler!(< *const c_char >, command_handle, cb);

    let payment_address = match str_from_char_ptr(payment_address) {
        Some(s) => s,
        None => {
            error!("Failed to convert payment_address pointer to string");
            return ErrorCode::CommonInvalidStructure as i32;
        }
    };
    debug!("api::build_get_utxo_request_handler >> wallet_handle: {:?}, payment_address: {:?}", wallet_handle, payment_address);

    let utxo_request =
        GetUtxoOperationRequest::new(String::from(payment_address));
    info!("Built GET_UTXO request: {:?}", utxo_request);
    let utxo_request = utxo_request.serialize_to_pointer()
        .map_err(|_| ErrorCode::CommonInvalidStructure);

    let res = handle_result(utxo_request) as i32;
    trace!("api::build_get_utxo_request_handler << result: {:?}", res);
    return res;
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
pub extern "C" fn parse_get_utxo_response_handler(
    command_handle: i32,
    resp_json: *const c_char,
    cb: JsonCallback
)-> i32 {

    trace!("api::parse_get_utxo_response_handler called");
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidStructure as i32);

    if resp_json.is_null() {
        trace!("api::parse_get_utxo_response_handler << result: {:?}", ErrorCode::CommonInvalidStructure);
        return ErrorCode::CommonInvalidStructure as i32;
    }

    let resp_json_string = match string_from_char_ptr(resp_json) {
        Some(s) => s,
        None => {
            error!("Failed to convert inputs_json pointer to string");
            trace!("api::parse_get_utxo_response_handler << result: {:?}", ErrorCode::CommonInvalidStructure);
            return ErrorCode::CommonInvalidStructure as i32;
        }
    };

    debug!("api::parse_get_utxo_response_handler >> resp_json: {:?}", resp_json_string);

    let response: ParseGetUtxoResponse = match ParseGetUtxoResponse::from_json(&resp_json_string)
        .map_err(map_err_err!()) {
        Ok(r) => r,
        Err(_) => return ErrorCode::CommonInvalidStructure as i32,
    };

    // here is where the magic happens--conversion from input structure to output structure
    // is handled in ParseGetUtxoReply::from_response
    let reply: ParseGetUtxoReply = match parse_get_utxo_response::from_response(response) {
        Ok(reply) => reply,
        Err(err) => {
            trace!("api::parse_get_utxo_response_handler << result: {:?}", err);
            return err as i32
        }
    };

    let reply_str: String = match reply.to_json().map_err(map_err_err!())  {
        Ok(j) => j,
        Err(_) => return ErrorCode::CommonInvalidState as i32,
    };
    info!("Parsed GET_UTXO response, received: {:?}", reply_str);

    let reply_str_ptr: *const c_char = c_pointer_from_string(reply_str);

    cb(command_handle, ErrorCode::Success as i32, reply_str_ptr);
    trace!("api::parse_get_utxo_response_handler << result: {:?}", ErrorCode::Success);
    return ErrorCode::Success as i32;
}

/**
    Set the fees for different transactions.

    ### fees_json
    A JSON object where the key is a transaction type and the value is the fee.

    Will be serialized into a [`SetFeesMap`]

    #### Example
    ```JSON
        {
            "3": 4,
            "10000": 12
        }
    ```

    [`SetFeesMap`]: sovtoken::logic::config::set_fees_config::SetFeesMap

    TODO: Fix links
    TODO: Remove submitter_did, doesn't need did because request doesn't have
    identifier.
*/
#[no_mangle]
pub extern "C" fn build_set_txn_fees_handler(
    command_handle: i32,
    wallet_handle: i32,
    submitter_did: *const c_char,
    fees_json: *const c_char,
    cb: JsonCallback
) -> i32 {

    trace!("api::build_set_txn_fees_handler called >> wallet_handle {}", wallet_handle);
    let (did, set_fees, cb) = match set_fees::deserialize_inputs(
        submitter_did,
        fees_json,
        cb
    ) {
        Ok(tup) => tup,
        Err(e) => {
            trace!("api::build_set_txn_fees_handler << result: {:?}", e);
            return e as i32
        }
    };

    let fees_request = set_fees.as_request(did);

    let fees_request_pointer_option = fees_request.serialize_to_pointer()
        .or(Err(ErrorCode::CommonInvalidStructure));

    let fees_request_pointer = match fees_request_pointer_option {
        Ok(ptr) => ptr,
        Err(e) => {
            trace!("api::build_set_txn_fees_handler << result: {:?}", e);
            return e as i32
        },
    };

    cb(command_handle, ErrorCode::Success as i32, fees_request_pointer);

    trace!("api::build_set_txn_fees_handler << result: {:?}", ErrorCode::Success);
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
pub extern "C" fn build_get_txn_fees_handler(
    command_handle: i32,
    wallet_handle: i32,
    submitter_did: *const c_char,
    cb: JsonCallback
) -> i32 {

    let handle_result = api_result_handler!(< *const c_char >, command_handle, cb);
    trace!("api::build_get_txn_fees_handler called");

    if cb.is_none() {
        return handle_result(Err(ErrorCode::CommonInvalidStructure)) as i32;
    }

    let did = match Did::from_pointer(submitter_did) {
        Some(did) => did,
        None => {
            error!("Failed to convert submitter_did pointer to string");
            trace!("api::build_get_txn_fees_handler << result: {:?}", ErrorCode::CommonInvalidStructure);
            return ErrorCode::CommonInvalidStructure as i32;
        }
    };

    debug!("api::build_get_txn_fees_handler >> wallet_handle: {:?}, submitter_did: {:?}", wallet_handle, did);

    let did = match did.validate() {
        Ok(d) => d,
        Err(_) => {
            trace!("api::build_get_txn_fees_handler << result: {:?}", ErrorCode::CommonInvalidStructure);
            return ErrorCode::CommonInvalidStructure as i32
        }
    };

    let get_txn_request = GetFeesRequest::new().as_request(did);
    info!("Built GET_TXN_FEES request: {:?}", get_txn_request);

    let request_pointer = match get_txn_request.serialize_to_pointer() {
        Ok(p) => p,
        Err(_) => {
            trace!("api::build_get_txn_fees_handler << result: {:?}", ErrorCode::CommonInvalidStructure);
            return ErrorCode::CommonInvalidState as i32
        }
    };

    let res = handle_result(Ok(request_pointer)) as i32;
    trace!("api::build_get_txn_fees_handler << res: {:?}", res);
    return res;
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
pub extern "C" fn parse_get_txn_fees_response_handler(
    command_handle: i32,
    resp_json: *const c_char,
    cb: JsonCallback
)-> i32{
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidStructure as i32);

    trace!("api::parse_get_txn_fees_response_handler called");
    if resp_json.is_null() {
        return ErrorCode::CommonInvalidStructure as i32;
    }
    let resp_json_string = match string_from_char_ptr(resp_json) {
        Some(s) => s,
        None => {
            error!("Failed to convert resp_json pointer to string");
            trace!("api::parse_get_txn_fees_response_handler << result: {:?}", ErrorCode::CommonInvalidStructure);
            return ErrorCode::CommonInvalidStructure as i32;
        }
    };

    debug!("api::parse_get_txn_fees_response_handler >> resp_json: {:?}", resp_json_string);
    debug!("Deserialized parse_get_txn_fees_response_handler arguments");

    let fees_json_obj =
        match parse_fees_from_get_txn_fees_response(resp_json_string){
            Ok(s) => {
                s
            },
            Err(_) => {
                trace!("api::parse_get_txn_fees_response_handler << result: {:?}", ErrorCode::CommonInvalidStructure);
                return ErrorCode::CommonInvalidStructure as i32;
            }
        };
    info!("Parsed get_txn_fees_response, result: {:?}", fees_json_obj);
    let fees_json_ptr : *const c_char = c_pointer_from_string(fees_json_obj);
    cb(command_handle, ErrorCode::Success as i32, fees_json_ptr);

    let res = ErrorCode::Success as i32;
    trace!("api::parse_get_txn_fees_response_handler << result: {:?}", res);
    return res;
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
 * [
 *      {
 *          "recipient": <str: payment_address>,
 *          "amount": <int>
 *          "extra": <str>
 *      }
 * ]
 */
#[no_mangle]
pub extern "C" fn build_mint_txn_handler(
    command_handle:i32,
    wallet_handle: i32,
    submitter_did: *const c_char,
    outputs_json: *const c_char,
    extra: *const c_char,
    cb: JsonCallback
) -> i32
{
    trace!("api::build_mint_txn_handle called >> wallet_handle {}", wallet_handle);
    let (did, outputs, extra, cb) = match minting::deserialize_inputs(
        submitter_did,
        outputs_json,
        extra,
        cb
    ) {
        Ok(tup) => tup,
        Err(e) => {
            trace!("api::build_mint_txn_handle << res: {:?}", e);
            return e as i32
        },
    };

    debug!("Deserialized build_mint_txn_handler arguments.");

    let mint_request = match minting::build_mint_request(did, outputs, extra) {
        Ok(json) => json,
        Err(e) => {
            trace!("api::build_mint_txn_handle << res: {:?}", e);
            return e as i32
        }
    };
    debug!("Serialized mint request as pointer.");

    cb(command_handle, ErrorCode::Success as i32, mint_request);
    let res = ErrorCode::Success;
    trace!("api::build_mint_txn_handle << res: {:?}", res);
    return res as i32;
}

/// Build a verify transaction request.
///
/// # Parameters
/// wallet_handle
/// did
/// txo -- txo to get transaction
///
/// # Returns
/// Request to send to ledger for verification of transaction
#[no_mangle]
pub extern "C" fn build_verify_req_handler(
    command_handle: i32,
    wallet_handle: i32,
    did: *const c_char,
    txo: *const c_char,
    cb: JsonCallback
) -> i32 {
    trace!("api::build_verify_req called >> wallet_handle {}", wallet_handle);

    let (did, txo, cb) = match verify::deserialize(did, txo, cb) {
        Ok(a) => a,
        Err(ec) => {
            trace!("api::build_verify_req << res {:?}", ec);
            return ec as i32;
        }
    };

    let res = indy::ledger::Ledger::build_get_txn_request_async(
        &String::from(did),
        Some(LEDGER_ID),
        txo.seq_no as i32,
        move |ec, res| {
            trace!("api::build_verify_req cb << ec: {:?}, res: {:?}", ec, res);
            cb(command_handle, ec as i32, c_pointer_from_string(res));
        }
    );

    trace!("api::build_verify_req << res {:?}", res);

    res as i32
}

/// Parse response of verification of txo
///
/// # Parameters
/// resp_json -- response from ledger
///
/// # Return
/// txn_json: {
///     sources: [<str>, ]
///     receipts: [ {
///         recipient: <str>, // payment address of recipient
///         receipt: <str>, // receipt that can be used for payment referencing and verification
///         amount: <int>, // amount
///     } ],
///     extra: <str>, //optional data
/// }
#[no_mangle]
pub extern "C" fn parse_verify_response_handler(
    command_handle: i32,
    resp_json: *const c_char,
    cb: JsonCallback
) -> i32 {
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidStructure as i32);

    trace!("api::parse_verify_response_handler called");
    if resp_json.is_null() {
        return ErrorCode::CommonInvalidStructure as i32;
    }

    let resp_json_string = match string_from_char_ptr(resp_json) {
        Some(s) => s,
        None => {
            error!("Failed to convert resp_json pointer to string");
            trace!("api::parse_verify_response_handler << result: {:?}", ErrorCode::CommonInvalidStructure);
            return ErrorCode::CommonInvalidStructure as i32;
        }
    };

    debug!("api::parse_verify_response_handler >> resp_json: {:?}", resp_json_string);

    let result = match parse_verify::parse_response(&resp_json_string) {
        Ok(e) => e,
        Err(ec) => {
            trace!("api::parse_verify_response_handler << result: {:?}", ec);
            return ec as i32;
        }
    };

    let ec = ErrorCode::Success;

    trace!("api::parse_verify_response_handler << result: {:?}", result);
    let result = c_pointer_from_string(result);
    cb(command_handle, ErrorCode::Success as i32, result);

    ec as i32
}

#[no_mangle]
pub extern "C" fn get_utxo_state_proof_parser(reply_from_node: *const c_char,
                                              parsed_sp: *mut *const c_char) -> i32 {
    trace!("Calling get_utxo_state_proof_parser.");

    check_useful_c_ptr!(reply_from_node, ErrorCode::CommonInvalidParam1 as i32);

    let res = parse_get_utxo_response::get_utxo_state_proof_extractor(reply_from_node, parsed_sp) as i32;

    trace!("Called get_utxo_state_proof_parser: <<< res: {:?}", res);

    return res;
}

#[no_mangle]
pub extern "C" fn get_fees_state_proof_parser(reply_from_node: *const c_char,
                                              parsed_sp: *mut *const c_char) -> i32 {
    trace!("Calling get_fees_state_proof_parser.");

    check_useful_c_ptr!(reply_from_node, ErrorCode::CommonInvalidParam1 as i32);

    let res = get_fees_state_proof_extractor(reply_from_node, parsed_sp) as i32;

    trace!("Called get_fees_state_proof_parser: <<< res: {:?}", res);

    return res;
}

#[no_mangle]
pub extern fn free_parsed_state_proof(sp: *const c_char) -> i32 {
    trace!("Calling free_parsed_state_proof.");

    check_useful_c_ptr!(sp, ErrorCode::CommonInvalidParam1 as i32);

    unsafe { Box::from_raw(sp as *mut &str); }

    trace!("Called free_parsed_state_proof");

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
    debug!("Going to call Payment::register");

    if let Err(e) = Payment::register_method(
        PAYMENT_METHOD_NAME,
        Some(create_payment_address_handler),
        Some(add_request_fees_handler),
        Some(parse_response_with_fees_handler),
        Some(build_get_utxo_request_handler),
        Some(parse_get_utxo_response_handler),
        Some(build_payment_req_handler),
        Some(parse_payment_response_handler),
        Some(build_mint_txn_handler),
        Some(build_set_txn_fees_handler),
        Some(build_get_txn_fees_handler),
        Some(parse_get_txn_fees_response_handler),
        Some(build_verify_req_handler),
        Some(parse_verify_response_handler),
    ) {
        debug!("Payment::register failed with {:?}", e);
        return e as i32
    };

    debug!("Going to call Ledger::register_transaction_parser_for_sp for GET_UTXO");

    if let Err(e) = Ledger::register_transaction_parser_for_sp(
        GET_UTXO,
        Some(get_utxo_state_proof_parser),
        Some(free_parsed_state_proof)
    ) {
        debug!("Ledger::register_transaction_parser_for_sp for GET_UTXO failed with {:?}", e);
        return e as i32
    };

    debug!("Going to call Ledger::register_transaction_parser_for_sp for GET_FEES");
    if let Err(e) =  Ledger::register_transaction_parser_for_sp(
        GET_FEES,
        Some(get_fees_state_proof_parser),
        Some(free_parsed_state_proof)
    ) {
        debug!("Ledger::register_transaction_parser_for_sp for GET_FEES failed with {:?}", e);
        return e as i32
    };

    debug!("sovtoken_init() returning ErrorCode::Success");
    return ErrorCode::Success as i32;
}
