/*!
 *  Defines structure and implementation inputs and outputs and submitter did
 *  these are the structures for the [`build_payment_req_handler`]
 *
 *  `build_payment_req_handler`]: ../../../api/fn.build_payment_req_handler.html
 */

use logic::request::Request;
use utils::constants::txn_types::XFER_PUBLIC;
use logic::xfer_payload::XferPayload;

/**
    Struct for [`build_payment_req_handler`]

    Can build a Request<PaymentRequest> which can be serialized into request json.

    ```
        use sovtoken::logic::config::payment_config::PaymentRequest;
        use sovtoken::logic::indy_sdk_api::crypto_api::CryptoSdk;
        use sovtoken::logic::input::Input;
        use sovtoken::logic::output::Output;
        use sovtoken::logic::xfer_payload::XferPayload;

        sovtoken::api::sovtoken_init();

        let wallet_handle = 1;
        let identifier = String::from("hgrhyNXqW4KNTz4wwiV8v");
        let address1 = String::from("pay:sov:d0kitWxupHvZ4i0NHJhoj79RcUeyt3YlwAc8Hbcy87iRLSZC");
        let address2 = String::from("pay:sov:XuBhXW6gKcUAq6fmyKsdxxjOZEbLy66FEDkQwTPeoXBmTZKy");
        let address3 = String::from("pay:sov:ql33nBkjGw6szxPT6LLRUIejn9TZAYkVRPd0QJzfJ8FdhZWs");

        let inputs = vec![
            Input::new(address1, 2),
            Input::new(address2, 3)
        ];

        let outputs = vec![
            Output::new(address3, 10, None)
        ];

        let transfer_data = XferPayload::new(inputs, outputs)
            .sign(&CryptoSdk {}, wallet_handle)
            .unwrap();

        // let payment = PaymentRequest::new(transfer_data);
        // let payment_request = payment.as_request(identifier);
        // let json_pointer = payment_request.serialize_to_pointer();
    ```

     [`build_payment_req_handler`]: ../../../api/fn.build_payment_req_handler.html
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct PaymentRequest {
    #[serde(rename = "type")]
    txn_type: String,
    #[serde(flatten)]
    signed_inputs_outputs: XferPayload
}


impl PaymentRequest {
    
    /**
        Create a new [`PaymentRequest`] with a signed [`XferPayload`].

        [`PaymentRequest`]: ./struct.PaymentRequest.html
        [`XferPayload`]: ../../xfer_payments/struct.XferPayload.html
    */
    pub fn new(signed_inputs_outputs: XferPayload ) -> PaymentRequest {
        return PaymentRequest {
            txn_type: XFER_PUBLIC.to_string(),
            signed_inputs_outputs,
        };
    }

    /**
        Transforms `self` to a [`Request<PaymentRequest>`] struct.

        [`Request<PaymentRequest>`]: ../../request/struct.Request.html
    */
    pub fn as_request(self, identifier: String) -> Request<PaymentRequest> {
        return Request::new(self, Some(identifier));
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
        return PaymentRequest::new(payload).as_request(identifier);
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
                "type": XFER_PUBLIC,
                "outputs": [["a8QAXMjRwEGoGLmMFEc5sTcntZxEF1BpqAs8GoKFa9Ck81fo7",10]],
                "inputs": [["E9LNHk8shQ6xe2RfydzXDSsyhWC6vJaUeKE2mmc6mWraDfmKm", 30]],
                "signatures": ["239asdkj3298uadkljasd98u234ijasdlkj"]
            }),
            |_fees_req| {}
        )
    }

    #[test]
    fn test_blah() {
        use sovtoken::logic::config::payment_config::PaymentRequest;
        use sovtoken::logic::indy_sdk_api::crypto_api::CryptoSdk;
        use sovtoken::logic::input::Input;
        use sovtoken::logic::output::Output;
        use sovtoken::logic::xfer_payload::XferPayload;

        sovtoken::api::sovtoken_init();

        let wallet_handle = 1;
        let identifier = String::from("hgrhyNXqW4KNTz4wwiV8v");
        let address1 = String::from("pay:sov:d0kitWxupHvZ4i0NHJhoj79RcUeyt3YlwAc8Hbcy87iRLSZC");
        let address2 = String::from("pay:sov:XuBhXW6gKcUAq6fmyKsdxxjOZEbLy66FEDkQwTPeoXBmTZKy");
        let address3 = String::from("pay:sov:ql33nBkjGw6szxPT6LLRUIejn9TZAYkVRPd0QJzfJ8FdhZWs");

        let inputs = vec![
            Input::new(address1, 2),
            Input::new(address2, 3)
        ];

        let outputs = vec![
            Output::new(address3, 10, None)
        ];

        let transfer_data = XferPayload::new(inputs, outputs)
            .sign(&CryptoSdk {}, wallet_handle)
            .unwrap();
    }
}
