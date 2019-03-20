use utils::{ErrorCode, IndyHandle};

use std::ffi::CString;
use std::time::Duration;

use indy_sys::ledger;
use indy_sys::ResponseStringCB;

use utils::results::ResultHandler;
use utils::callbacks::ClosureHandler;

pub struct Ledger {}

impl Ledger {
    /// Builds a GET_TXN request. Request to get any transaction by its seq_no.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the request submitter.
    /// * `ledger_type` - (Optional) type of the ledger the requested transaction belongs to:
    ///     DOMAIN - used default,
    ///     POOL,
    ///     CONFIG
    /// * `seq_no` - seq_no of transaction in ledger.
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_get_txn_request(submitter_did: Option<&str>, ledger_type: Option<&str>, seq_no: i32) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_get_txn_request(command_handle, submitter_did, ledger_type, seq_no, cb);

        ResultHandler::one(err, receiver)
    }

    /// Builds a GET_TXN request. Request to get any transaction by its seq_no.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the request submitter.
    /// * `ledger_type` - (Optional) type of the ledger the requested transaction belongs to:
    ///     DOMAIN - used default,
    ///     POOL,
    ///     CONFIG
    /// * `seq_no` - seq_no of transaction in ledger.
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_get_txn_request_timeout(submitter_did: Option<&str>, ledger_type: Option<&str>, seq_no: i32, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_get_txn_request(command_handle, submitter_did, ledger_type, seq_no, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Builds a GET_TXN request. Request to get any transaction by its seq_no.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the request submitter.
    /// * `ledger_type` - (Optional) type of the ledger the requested transaction belongs to:
    ///     DOMAIN - used default,
    ///     POOL,
    ///     CONFIG
    /// * `seq_no` - seq_no of transaction in ledger.
    /// * `closure` - The closure that is called when finished
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_get_txn_request_async<F: 'static>(submitter_did: Option<&str>, ledger_type: Option<&str>, seq_no: i32, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Ledger::_build_get_txn_request(command_handle, submitter_did, ledger_type, seq_no, cb)
    }

    fn _build_get_txn_request(command_handle: IndyHandle, submitter_did: Option<&str>, ledger_type: Option<&str>, seq_no: i32, cb: Option<ResponseStringCB>) -> ErrorCode {
        let submitter_did= opt_c_str_json!(submitter_did);
        let ledger_type = opt_c_str_json!(ledger_type);

        ErrorCode::from(unsafe { ledger::indy_build_get_txn_request(command_handle, submitter_did.as_ptr(), ledger_type.as_ptr(), seq_no, cb) })
    }
}