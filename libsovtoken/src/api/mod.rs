//
// export public methods
//
//
//
use libc::c_char;

use indy::api::ErrorCode;


/// Description
///
///
/// from tokens-interface.md/create_payment_address
/// #Params
/// command_handle: description.
/// config: description
/// cb: description
///
/// #Returns
/// ErrorCode: description. example if json, etc...
///
/// #Errors
/// description of errors
#[no_mangle]
pub extern "C" fn create_payment_address_handler(command_handle: i32,
                                                 config: *const c_char,
                                                 cb: Option<extern fn(command_handle_: i32, err: ErrorCode, payment_address: *const c_char)>) -> ErrorCode {
    return ErrorCode::Success;
}

/// Description
/// ???? what is this ????
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
/// from tokens-interface.md/add_request_fees
/// #Params
/// param1: description.
///
/// #Returns
/// description. example if json, etc...
///
/// #Errors
/// description of errors
#[no_mangle]
pub extern "C" fn add_request_fees_handler(eq_json: *const c_char, inputs_json: *const c_char, outputs_json: *const c_char,
                                           cb: Option<extern fn(command_handle_: i32,
                                                               err: ErrorCode,
                                                               req_with_fees_json: *const c_char)>) -> ErrorCode {
    return ErrorCode::Success;
}

/// Description
/// ???? what is this ????
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
pub extern "C" fn build_payment_txn_handler()-> ErrorCode {
    return ErrorCode::Success;
}

/// Description
///
///
/// from tokens-interface.md/build_get_utxo_request
/// #Params
/// param1: description.
///
/// #Returns
/// description. example if json, etc...
///
/// #Errors
/// description of errors
#[no_mangle]
pub extern "C" fn build_get_utxo_request_handler(ayment_address: *const c_char,
                                                 cb: Option<extern fn(command_handle_: i32,
                                                                      err: ErrorCode,
                                                                      get_utxo_txn_json: *const c_char)>)-> ErrorCode {
    return ErrorCode::Success;
}

/// Description
///
///
///
/// from tokens-interface.md/parse_get_utxo_response
/// #Params
/// param1: description.
///
/// #Returns
/// description. example if json, etc...
///
/// #Errors
/// description of errors
#[no_mangle]
pub extern "C" fn parse_get_utxo_response_handler(resp_json: *const c_char,
                                                  cb: Option<extern fn(command_handle_: i32,
                                                                       err: ErrorCode,
                                                                       utxo_json: *const c_char)>)-> ErrorCode {
    return ErrorCode::Success;
}

/// Description
/// ???? what is this ????
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
pub extern "C" fn build_fees_txn_handler()-> ErrorCode {
    return ErrorCode::Success;
}

/// Description
/// ???? what is this ????
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
pub extern "C" fn build_get_fees_txn_handler()-> ErrorCode {
    return ErrorCode::Success;
}

/// Description
/// ???? what is this ????
///
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
pub extern "C" fn build_get_utxo_txn_handler()-> ErrorCode {
    return ErrorCode::Success;
}

/// Description
///
///
/// from tokens-interface.md/build_payment_req
/// #Params
/// param1: description.
///
/// #Returns
/// description. example if json, etc...
///
/// #Errors
/// description of errors
pub extern "C" fn build_payment_req_handler(req_json: *const c_char, inputs_json: *const c_char, outputs_json: *const c_char,
                                            cb: Option<extern fn(command_handle_: i32,
                                                                err: ErrorCode,
                                                                payment_req_json: *const c_char)>) -> ErrorCode {
    return ErrorCode::Success;
}

/// Description
///
/// from tokens-interface.md/build_mint_req
/// #Params
/// param1: description.
///
/// #Returns
/// description. example if json, etc...
///
/// #Errors
/// description of errors
#[no_mangle]
pub extern "C" fn build_mint_txn_handler(req_json: *const c_char, outputs_json: *const c_char,
                                         cb: Option<extern fn(command_handle_: i32, err: ErrorCode, mint_req_json: *const c_char)>)-> ErrorCode {
    return ErrorCode::Success;
}