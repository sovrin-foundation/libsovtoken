//
// export public methods
//
//
//
#![allow(unused_variables)]
#![allow(unused_imports)]
#[warn(unused_imports)]

use std::ffi::{CString, CStr};
use std::ptr;
use libc::c_char;
use indy::api::ErrorCode;
use logic::payment_address_config::PaymentAddressConfig;
use logic::payments::create_payment_address;
use logic::output_mint_config::OutputMintConfig;
use utils::ffi_support::{str_from_char_ptr, cstring_from_str};
use utils::json_conversion::JsonDeserialize;

/// This method generates private part of payment address
/// and stores it in a secure place. Ideally it should be
/// secret in libindy wallet (see crypto module).
///
/// Note that payment method should be able to resolve this
/// secret by fully resolvable payment address format.
///
/// from tokens-interface.md/CreatePaymentAddressCB
///
/// #Params
/// command_handle: command handle to map callback to context
/// config_str: payment address config as json:
///   {
///     seed: <str>, // allows deterministic creation of payment address
///   }
/// cb: description
///
/// #Returns
/// on Success:  payment_address will have the format:
///              pay:sov:{32 byte public key}{4 digit check sum}
///
/// #Errors
/// description of errors
#[no_mangle]
pub extern "C" fn create_payment_address_handler(command_handle: i32,
                                                 config_str: *const c_char,
                                                 cb: Option<extern fn(command_handle_: i32, err: ErrorCode, payment_address: *const c_char)>) -> ErrorCode {
    // TODO:  missing wallet id

    if false == cb.is_some() {
        return ErrorCode::CommonInvalidParam3;
    }

    let json_config_str: &str = unpack_c_string_or_error!(config_str, ErrorCode::CommonInvalidParam2);

    let config: PaymentAddressConfig = match PaymentAddressConfig::from_json(json_config_str) {
        Ok(c) => c,
        Err(_) => return ErrorCode::CommonInvalidStructure,
    };

    // TODO:  once we get wallet id in the input, we will want to update create_payment_address
    // to return both payment address and private key pair so that we can write the private
    // key into the ledger
    let payment_address = create_payment_address(config);
    let payment_address_cstring = cstring_from_str(payment_address);
    let payment_address_ptr = payment_address_cstring.as_ptr();

    match cb {
        Some(f) => f(command_handle, ErrorCode::Success, payment_address_ptr),
        None => panic!("cb was null even after check"),
    };

    return ErrorCode::Success;
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
pub extern "C" fn list_payment_addresses_handler() -> ErrorCode {
    return ErrorCode::Success;
}

/// Description
///
///
/// from tokens-interface.md/AddRequestFeesCB
/// #Params
/// param1: description.
///
/// #Returns
/// description. example if json, etc...
///
/// #Errors
/// description of errors
#[no_mangle]
pub extern "C" fn add_request_fees_handler(command_handle: i32, req_json: *const c_char, inputs_json: *const c_char,
                                           outputs_json: *const c_char,
                                           cb: Option<extern fn(command_handle_: i32,
                                                               err: ErrorCode,
                                                               req_with_fees_json: *const c_char)>) -> ErrorCode {
    return ErrorCode::Success;
}

/// Description
///
///
/// from tokens-interface.md/ParseResponseWithFeesCB
/// #Params
/// param1: description.
///
/// #Returns
/// description. example if json, etc...
///
/// #Errors
/// description of errors
#[no_mangle]
pub extern "C" fn parse_response_with_fees_handler(command_handle: i32,
                                                   req_json: *const c_char,
                                                   cb: Option<extern fn(command_handle_: i32,
                                                               err: ErrorCode,
                                                               utxo_json: *const c_char)>) -> ErrorCode {
    return ErrorCode::Success;
}


/// Description
///
///
/// from tokens-interface.md/BuildPaymentReqCB
/// #Params
/// param1: description.
///
/// #Returns
/// description. example if json, etc...
///
/// #Errors
/// description of errors
pub extern "C" fn build_payment_req_handler(command_handle: i32,
                                            inputs_json: *const c_char,
                                            outputs_json: *const c_char,
                                            cb: Option<extern fn(command_handle_: i32,
                                                        err: ErrorCode,
                                                        payment_req_json: *const c_char) -> ErrorCode>) -> ErrorCode {
    return ErrorCode::Success;
}

/// Description
///
///
/// from tokens-interface.md/ParsePaymentResponseCB
/// #Params
/// param1: description.
///
/// #Returns
/// description. example if json, etc...
///
/// #Errors
/// description of errors
pub extern "C" fn parse_payment_response_handler(command_handle: i32,
                                                 resp_json: *const c_char,
                                                 cb: Option<extern fn(command_handle_: i32,
                                                             err: ErrorCode,
                                                             utxo_json: *const c_char) -> ErrorCode>) -> ErrorCode {

    return ErrorCode::Success;
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
                                                 payment_address: *const c_char,
                                                 cb: Option<extern fn(command_handle_: i32,
                                                                      err: ErrorCode,
                                                                      get_utxo_txn_json: *const c_char)>)-> ErrorCode {
    return ErrorCode::Success;
}

/// Description
///
///
///
/// from tokens-interface.md/ParseGetUTXOResponseCB
/// #Params
/// param1: description.
///
/// #Returns
/// description. example if json, etc...
///
/// #Errors
/// description of errors
#[no_mangle]
pub extern "C" fn parse_get_utxo_response_handler(command_handle: i32,
                                                  resp_json: *const c_char,
                                                  cb: Option<extern fn(command_handle_: i32,
                                                                       err: ErrorCode,
                                                                       utxo_json: *const c_char)>)-> ErrorCode {
    return ErrorCode::Success;
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
pub extern "C" fn build_fees_txn_handler(command_handle: i32,
                                         fees_json: *const c_char,
                                         cb: Option<extern fn(command_handle_: i32,
                                                           err: ErrorCode,
                                                           set_txn_fees_json: *const c_char) -> ErrorCode>)-> ErrorCode {
    return ErrorCode::Success;
}

/// Description
///
///
/// from tokens-interface.md/BuildGetTxnFeesReqCB
/// #Params
/// param1: description.
///
/// #Returns
/// description. example if json, etc...
///
/// #Errors
/// description of errors
#[no_mangle]
pub extern "C" fn build_get_fees_txn_handler(command_handle: i32,
                                             cb: Option<extern fn(command_handle_: i32,
                                                           err: ErrorCode,
                                                           get_txn_fees_json: *const c_char) -> ErrorCode>)-> ErrorCode {
    return ErrorCode::Success;
}

/// Description
///
///
/// from tokens-interface.md/ParseGetTxnFeesResponseCB
/// #Params
/// param1: description.
///
/// #Returns
/// description. example if json, etc...
///
/// #Errors
/// description of errors
#[no_mangle]
pub extern "C" fn parse_get_fees_txn_response_handler(command_handle: i32,
                                                      resp_json: *const c_char,
                                                      cb: Option<extern fn(command_handle_: i32,
                                                                err: ErrorCode,
                                                                fees_json: *const c_char) -> ErrorCode>)-> ErrorCode {
    return ErrorCode::Success;
}


/// Description
///
/// from tokens-interface.md/BuildMintReqCB
/// #Params
/// param1: description.
///
/// #Returns
/// description. example if json, etc...
///
/// #Errors
/// description of errors
#[no_mangle]
pub extern "C" fn build_mint_txn_handler(command_handle: i32, outputs_json: *const c_char,
                                         cb: Option<extern fn(command_handle_: i32, err: ErrorCode, mint_req_json: *const c_char)>)-> ErrorCode {
    if cb.is_some() == false {
        return ErrorCode::CommonInvalidParam3;
    }

    let outputs_json_str : &str = unpack_c_string_or_error!(outputs_json, ErrorCode::CommonInvalidParam2);

    let outputs_config: OutputMintConfig = match OutputMintConfig::from_json(outputs_json_str) {
        Ok(c) => c,
        Err(_) => return ErrorCode::CommonInvalidStructure ,
    };

    return ErrorCode::Success;
}