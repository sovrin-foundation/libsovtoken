//
// export public methods
//
//
//
use libc::c_char;

use indy::api::ErrorCode;


/// Description
/// description
///
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
/// #Params
/// param1: description.
///
/// #Returns
/// description. example if json, etc...
///
/// #Errors
/// description of errors
#[no_mangle]
pub extern "C" fn add_request_fees_handler() -> ErrorCode {
    return ErrorCode::Success;
}

/// Description
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
/// #Params
/// param1: description.
///
/// #Returns
/// description. example if json, etc...
///
/// #Errors
/// description of errors
#[no_mangle]
pub extern "C" fn build_mint_txn_handler()-> ErrorCode {
    return ErrorCode::Success;
}