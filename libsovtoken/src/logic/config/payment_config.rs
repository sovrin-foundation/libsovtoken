/*!
 *  Defines structure and implementation inputs and outputs and submitter did
 *  these are the structures for the [`build_payment_req_handler`]
 *
 *  [`build_payment_req_handler`]: ../../../api/fn.build_payment_req_handler.html
 */
use logic::request::Request;
use utils::constants::txn_types::XFER_PUBLIC;
use logic::xfer_payload::XferPayload;


#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct PaymentRequest {
    #[serde(rename = "type")]
    txn_type: String,
    #[serde(flatten)]
    signed_inputs_outputs: XferPayload
}

/**
 * A struct that can be transformed into a Fees JSON object.
 */
impl PaymentRequest {
    
    /**
     * Creates a new `PaymentRequest` with `inputs` and `outputs`
     */
    pub fn new(signed_inputs_outputs: XferPayload, identifier: String) -> Request<PaymentRequest> {
        let fees = PaymentRequest {
            txn_type: XFER_PUBLIC.to_string(),
            signed_inputs_outputs,
        };

        return Request::new(fees, Some(identifier));
    }
}

#[cfg(test)]
mod payment_request_test {
    use super::*;
    use logic::input::Input;
    use logic::output::Output;
    use serde_json;
    use utils::ffi_support::str_from_char_ptr;
    use utils::json_conversion::{JsonDeserialize, JsonSerialize};
    use utils::random::rand_string;

    fn initial_xfer_request() -> Request<PaymentRequest> {
        let identifier: String = rand_string(21);
        let output = Output::new(String::from("a8QAXMjRwEGoGLmMFEc5sTcntZxEF1BpqAs8GoKFa9Ck81fo7"), 10, None);
        let input = Input::new(String::from("E9LNHk8shQ6xe2RfydzXDSsyhWC6vJaUeKE2mmc6mWraDfmKm"),30);
    
        let mut payload = XferPayload::new(vec![input], vec![output]);
        payload.signatures = Some(vec![String::from("239asdkj3298uadkljasd98u234ijasdlkj")]);
        return PaymentRequest::new(payload, identifier);
    }

    fn assert_fees_request<F>(expected: serde_json::Value, f: F)
        where F: Fn(&mut Request<PaymentRequest>) -> ()
    {
        let mut xfer_req = initial_xfer_request();
        f(&mut xfer_req);
        let xfer_req_c_string = xfer_req.serialize_to_cstring().unwrap();
        let xfer_req_json_str = str_from_char_ptr(xfer_req_c_string.as_ptr()).unwrap();
        let deserialized_xfer_request: Request<PaymentRequest> = Request::<PaymentRequest>::from_json(xfer_req_json_str).unwrap();
        assert_eq!(deserialized_xfer_request.protocol_version, 1);

        let operation_json_value : serde_json::Value = serde_json::from_str(&deserialized_xfer_request.operation.to_json().unwrap()).unwrap();
        assert_eq!(operation_json_value, expected);
    }

    #[test]
    fn valid_request() {
        assert_fees_request(
            json!({
                "type": XFER_PUBLIC.to_string(),
                "outputs": [["a8QAXMjRwEGoGLmMFEc5sTcntZxEF1BpqAs8GoKFa9Ck81fo7",10]],
                "inputs": [["E9LNHk8shQ6xe2RfydzXDSsyhWC6vJaUeKE2mmc6mWraDfmKm", 30]],
                "signatures": ["239asdkj3298uadkljasd98u234ijasdlkj"]
            }),
            |_fees_req| {}
        )
    }
}
