extern crate indy;
extern crate sovtoken;

use std::time::Duration;
use sovtoken::utils::ErrorCode;

const SUBMIT_RETRY_CNT: usize = 3;

pub fn submit_request_with_retries(pool_handle: i32, request_json: &str, previous_response: &str) -> Result<String, ErrorCode> {
    _submit_retry(_extract_seq_no_from_reply(previous_response).unwrap(), || {
        indy::ledger::Ledger::submit_request(pool_handle, request_json)
    })
}

fn _submit_retry<F>(minimal_timestamp: u64, submit_action: F) -> Result<String, ErrorCode>
    where F: Fn() -> Result<String, ErrorCode> {
    let mut i = 0;
    let action_result = loop {
        let action_result = submit_action()?;

        let retry = _extract_seq_no_from_reply(&action_result)
            .map(|received_timestamp| received_timestamp < minimal_timestamp)
            .unwrap_or(true);

        if retry && i < SUBMIT_RETRY_CNT {
            ::std::thread::sleep(Duration::from_secs(5));
            i += 1;
        } else {
            break action_result;
        }
    };
    Ok(action_result)
}

fn _extract_seq_no_from_reply(reply: &str) -> Result<u64, &'static str> {
    ::serde_json::from_str::<::serde_json::Value>(reply).map_err(|_| "Reply isn't valid JSON")?
        ["result"]["txnMetadata"]["seqNo"]
        .as_u64().ok_or("Missed seqNo in reply")
}