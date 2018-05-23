use serde::Serialize;
use serde_json;
use std::os::raw::c_char;
use std::ffi::CString;

use indy::api::ErrorCode;
use utils::callbacks::*;
use utils::json_conversion::JsonSerialize;
use utils::ffi_support::cstring_from_str;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request<T>
    where T: Serialize
{
    pub operation: T,
    pub protocol_version: u32,
}

impl<T> Request<T> 
    where T: Serialize
{
    pub fn new(operation: T) -> Self {
        return Request {
            operation,
            protocol_version: 1,
        }
    }

    pub fn serialize_to_cstring(&self) -> Result<CString, serde_json::Error> {
        let serialized = JsonSerialize::to_json(&self)?;
        return Ok(cstring_from_str(serialized));
    }
}

/**
 *  This function sets up the callback and them calls indy-sdk
 *
 *  After checking for valid parameters we will use the generalized
 *  function 'closure_to_cb_ec_string' to receive an error code and
 *  our callback. These are the individual parameters needed for
 *  'indy_build_get_txn_request'. This is then declared as an
 *  extern to support the dynamic linking of libsovtoken and indy-sdk
 */
pub fn build_get_txn_request (submitter_did: &str,
                              seq_no: i32,
                              cb: Box<FnMut(ErrorCode, String) + Send>,) -> ErrorCode{


    let (command_handle, cb) = CallBackWithClosureParamsUtils::closure_to_cb_ec_string(cb);
    let submitter_did = CString::new(submitter_did).unwrap();

    unsafe {
        indy_build_get_txn_request(
            command_handle,
            submitter_did.as_ptr(),
            seq_no,
            cb,
        )
    }
}

extern {
    #[no_mangle]
    pub fn indy_build_get_txn_request(command_handle: i32,
                                      submitter_did: *const c_char,
                                      seq_no: i32,
                                      cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                           request_json: *const c_char)>) -> ErrorCode;
}

// -- TODO: add tests to validate function input
#[cfg(test)]
mod build_get_txn_request_test {

    use super::*;
    use std::os::raw::c_char;
    use utils::types::JsonCallback;

    extern "C" fn empty_create_payment_callback(command_handle_: i32, err: ErrorCode, mint_req_json: *const c_char) -> ErrorCode {
        return ErrorCode::Success;
    }

    #[test]
    fn incorrect_size_did () {
        let empty_box = Box::new(move |error_code, res| {} );
        assert_eq!(build_get_txn_request("",15,empty_box), ErrorCode::CommonInvalidParam1);
    }
}
