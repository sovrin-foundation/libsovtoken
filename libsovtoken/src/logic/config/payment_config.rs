/*!
 *  Defines structure and implementation inputs and outputs and submitter did
 *  these are the structures for the [`build_payment_req_handler`]
 *
 *  [`build_payment_req_handler`]: ../../../api/fn.build_payment_req_handler.html
 */
use logic::request::Request;
use utils::constants::txn_types::XFER_PUBLIC;
use logic::fees::Fees;


#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct PaymentRequest {
    #[serde(rename = "type")]
    txn_type: String,
    #[serde(flatten)]
    signed_inputs_outputs: Fees
}

/**
 * A struct that can be transformed into a Fees JSON object.
 */
impl PaymentRequest {
    
    /**
     * Creates a new `PaymentRequest` with `inputs` and `outputs`
     */
    pub fn new(signed_inputs_outputs: Fees, identifier: String) -> Request<PaymentRequest> {
        let fees = PaymentRequest {
            txn_type: XFER_PUBLIC.to_string(),
            signed_inputs_outputs,
        };

        return Request::new(fees, identifier);
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


    fn initial_fees_request() -> Request<PaymentRequest> {
        let identifier: String = rand_string(21);
        let output = Output::new(String::from("AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja"), 10, None);
        let input = Input::new(String::from("dakjhe238yad"),30,Some(String::from("239asdkj3298uadkljasd98u234ijasdlkj")));
    
        let fees = Fees::new(vec![input], vec![output]);
        return PaymentRequest::new(fees, identifier);
    }

    fn assert_fees_request<F>(expected: serde_json::Value, f: F)
        where F: Fn(&mut Request<PaymentRequest>) -> ()
    {
        let mut fees_req = initial_fees_request();
        f(&mut fees_req);
        let fees_req_c_string = fees_req.serialize_to_cstring().unwrap();
        let fees_req_json_str = str_from_char_ptr(fees_req_c_string.as_ptr()).unwrap();
        let deserialized_fees_request: Request<PaymentRequest> = Request::<PaymentRequest>::from_json(fees_req_json_str).unwrap();
        assert_eq!(deserialized_fees_request.protocol_version, 1);

        let operation_json_value : serde_json::Value = serde_json::from_str(&deserialized_fees_request.operation.to_json().unwrap()).unwrap();
        assert_eq!(operation_json_value, expected);
    }

    #[test]
    fn valid_request() {
        assert_fees_request(
            json!({
                "type": XFER_PUBLIC.to_string(),
                "outputs": [["AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja",10]],
                "inputs": [["dakjhe238yad", 30, "239asdkj3298uadkljasd98u234ijasdlkj"]]
            }),
            |_fees_req| {}
        )
    }
}
