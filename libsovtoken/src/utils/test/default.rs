/*! 
 * Supplies default data for *TESTS ONLY*
 * 
*/

use libc::c_char;
use utils::base58::IntoBase58;
use utils::constants::txn_types;
use utils::ffi_support::c_pointer_from_string;
use utils::random::rand_string;

use logic::parsers::common::TXO;
use logic::input::{Input, Inputs};
use logic::output::{Output, Outputs};
use logic::xfer_payload::XferPayload;


pub fn inputs_json_pointer() -> *const c_char {
    let txo_1 = TXO { address: "pay:sov:d0kitWxupHvZ4i0NHJhoj79RcUeyt3YlwAc8Hbcy87iRLSZC".to_string(), seq_no: 2 };
    let txo_2 = TXO { address: "pay:sov:d0kitWxupHvZ4i0NHJhoj79RcUeyt3YlwAc8Hbcy87iRLSZC".to_string(), seq_no: 2 };
    json_c_pointer!([
        txo_1.to_libindy_string().unwrap(),
        txo_2.to_libindy_string().unwrap()
    ])
}

pub fn inputs() -> Inputs {
    let address1 = String::from("pay:sov:iTQzpdRdugkJ2gLD5vW5c159dncSL9jbAtu3WfPcb8qWD9bUd");
    let address2 = String::from("pay:sov:BUoojqSQTLuvjkun4y2YoseVF76UZ3uYfHF1dbQyZVbCuTwQo");

    vec![
        Input::new(address1, 1),
        Input::new(address2, 1),
    ]
}

pub fn outputs() -> Outputs {
    let address1 = String::from("pay:sov:2naF9c9ZJnSRtpaptpyZxi18tfozozqFRTmxHk9M6wbBc68T9A");
    let address2 = String::from("pay:sov:YissN67riFhQ8W6viqtJoCRHFkXtqxaxeL9UyvCoz8sXq5B5A");

   vec![
        Output::new(address1, 10),
        Output::new(address2, 22),
    ]
}

pub fn outputs_json_pointer() -> *const c_char {
    json_c_pointer!([
        {
            "address": "pay:sov:ql33nBkjGw6szxPT6LLRUIejn9TZAYkVRPd0QJzfJ8FdhZWs",
            "amount": 10
        }
    ])
}

pub extern fn empty_callback_string(
    _: i32,
    e: i32,
    _: *const c_char
) -> i32 {
   e
}

pub fn did() -> *const c_char {
    let did = rand_string(16).as_bytes().into_base58();
    c_pointer_from_string(did)
}

pub fn set_fees_json() -> *const c_char {
    json_c_pointer!({
        txn_types::XFER_PUBLIC: 3,
        "3": 5
    })
}

pub fn xfer_payload_unsigned() -> XferPayload {
    let inputs = inputs();
    let outputs = outputs();
    XferPayload::new(inputs, outputs, None)
}

pub fn xfer_payload_signed() -> XferPayload {
    let ver1 = "iTQzpdRdugkJ2gLD5vW5c159dncSL9jbAtu3WfPcb8qWD9bUd";
    let ver2 = "BUoojqSQTLuvjkun4y2YoseVF76UZ3uYfHF1dbQyZVbCuTwQo";
    let ver3 = "2naF9c9ZJnSRtpaptpyZxi18tfozozqFRTmxHk9M6wbBc68T9A";
    let ver4 = "YissN67riFhQ8W6viqtJoCRHFkXtqxaxeL9UyvCoz8sXq5B5A";

    let sig1 = "2T9TfJvLg2EkfJRFvN8D9maUEwEBhvg6eCiFL6PUobgzhTXE1m6y1w7KKEw8MQaUPBkgM2APMdwmMM26UYUatmjd";
    let sig2 = "2rUrhusR7TmkFs9cyNeHoq2EZ6LQH2RvKSZnJMPJHRSEDAb3aj4GxkvX79JASiHLxMmtz1stu4ysjXpUYZGVCSvr";

    let inputs = vec![
        Input::new(String::from(ver1), 1),
        Input::new(String::from(ver2), 2)
    ];

    let outputs = vec![
        Output::new(String::from(ver3), 10),
        Output::new(String::from(ver4), 22)
    ];

    let signatures = Some(vec![
        String::from(sig1),
        String::from(sig2),
    ]);

    XferPayload {
        inputs,
        outputs,
        extra: None,
        signatures
    }
}

pub fn create_address_config() -> *const c_char {
    json_c_pointer!({})
}
