/*!
 * Signing of [`Inputs`] and [`Outputs`]
 * 
 * [`Inputs`]: Inputs
 * [`Outputs`]: Outputs
 */
#![allow(unused_must_use)]

use indy::IndyHandle;
use indy::ErrorCode;
use logic::address;
use logic::indy_sdk_api::crypto_api::CryptoAPI;
use logic::input::{Input, Inputs};
use logic::output::{Outputs};
use serde_json;
use std::sync::Arc;
use std::sync::Mutex;


/**
 * Holds `inputs` and `outputs`
 * 
 * ### Fields
 * - `inputs`
 * - `outputs`
 * 
 * ## Example
 *
 * ```
 *  # extern crate sovtoken;
 *  # fn main() {
 *      use sovtoken::logic::input::Input;
 *      use sovtoken::logic::output::Output;
 *      use sovtoken::logic::xfer_payload::XferPayload;
 *      use sovtoken::logic::indy_sdk_api::crypto_api::CryptoSdk;
 *
 *      // Need an actual wallet_handle
 *      let wallet_handle = 1;
 *      let address_input = String::from("pay:sov:SBD8oNfQNm1aEGE6KkYI1khYEGqG5zmEqrEw7maqKitIs121");
 *      let address_output = String::from("pay:sov:FekbDoBkdsj3nH2a2nNhhedoPju2UmyKrr1ZzMZGT0KENbvp");
 *      let inputs = vec![Input::new(address_input, 1)];
 *      let outputs = vec![Output::new(address_output, 20, None)];
 *
 *      let payload = XferPayload::new(inputs, outputs);
 *      let signed_payload = payload.sign(&CryptoSdk{}, wallet_handle, Box::new(|res| Default::default()));
 *  # }
 * ```
 */
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct XferPayload {
    pub outputs: Outputs,
    pub inputs: Inputs,
    pub signatures: Option<Vec<String>>
}

unsafe impl Send for XferPayload {}
unsafe impl Sync for XferPayload {}

impl<A: CryptoAPI> InputSigner<A> for XferPayload {}
impl XferPayload {
    pub fn new(inputs: Inputs, outputs: Outputs) -> Self
    {
        return XferPayload { inputs, outputs, signatures: None };
    }

    /** 
     * Signs [`Inputs`]
     * 
     * Validates that inputs and outputs both have a valid `address`.
     * Signs each [`Input`] with [`sign_input`]
     * 
     * [`Input`]: Input
     * [`Inputs`]: Inputs
     */
    pub fn sign<A: CryptoAPI>(mut self, crypto_api: &'static A, wallet_handle: IndyHandle, cb: Box<Fn(Result<XferPayload, ErrorCode>) + Send + Sync>) -> Result<(), ErrorCode> {
        if self.outputs.len() < 1 || self.inputs.len() < 1 {
            return Err(ErrorCode::CommonInvalidStructure);
        }

        for output in &mut self.outputs {
            output.address = address::unqualified_address_from_address(&output.address)?;
        }
        trace!("Indicator stripped from outputs");

        for input in &mut self.inputs {
            input.address = address::unqualified_address_from_address(&input.address)?;
        }

        trace!("Indicator stripped from inputs");

        XferPayload::sign_inputs(crypto_api, wallet_handle, &self.inputs.clone(), &self.outputs.clone(), Box::new(move |signatures| {
                    match signatures {
                        Ok(signatures) => {
                            let mut payload = self.clone();
                            payload.signatures = Some(signatures);
                            cb(Ok(payload));
                        }
                        Err(err) => {cb(Err(err));}
                    };
                }
            )
        )?;

        return Ok(());
    }
}

trait InputSigner<A: CryptoAPI> {
    fn sign_inputs(crypto_api: &'static A, wallet_handle: IndyHandle, inputs: &Inputs, outputs: &Outputs, cb: Box<Fn(Result<Vec<String>, ErrorCode>) + Send + Sync>)
                   -> Result<(), ErrorCode>
    {
        if inputs.is_empty() {
            cb(Ok(vec![]));
            return Ok(());
        }
        let inputs_result: Arc<Mutex<Vec<String>>> = Default::default();

        let res_cnt = inputs.len();
        let cb = Arc::new(move |input: Result<String, ErrorCode>| {
            match input {
                Ok(input) => {
                    let mut results = inputs_result.lock().unwrap();
                    results.push(input);
                    if results.len() == res_cnt {
                        cb(Ok(results.to_vec()))
                    }
                }
                Err(err) => cb(Err(err))
            }
        });

        for input in inputs {
            let cb = cb.clone();
            match Self::sign_input(crypto_api, wallet_handle, input, outputs, Box::new(cb)) {
                err @ Err(_) => { return err; }
                _ => ()
            }
        }

        Ok(())
    }

    /**
     * Signs an [`Input`] with indy_crypto_sign
     *
     * Validates the `input`'s `address`, but not the `outputs`.
     * The message that will be signed is
     * `[[<address>, <seq_no>], [<Output>, <Output>, ...]]`
     *
     * [`Input`]: Input
     */
    fn sign_input(
        crypto_api: &'static A,
        wallet_handle: IndyHandle,
        input: &Input,
        outputs: &Outputs,
        cb: Box<Arc<Fn(Result<String, ErrorCode>) + Send + Sync>>,
    ) -> Result<(), ErrorCode>
    {
        let verkey = address::verkey_from_unqualified_address(&input.address.clone())?;
        debug!("Received verkey for payment address >>> {:?}", verkey);

        let message_json_value = json!([[input.address, input.seq_no], outputs]);
        debug!("Message to sign >>> {:?}", message_json_value);

        let message = serde_json::to_string(&message_json_value)
            .map_err(|_| ErrorCode::CommonInvalidStructure)?
            .to_string();

        let ca = move |signature: Result<String, ErrorCode>| {
            debug!("Received encoded signature >>> {:?}", signature);
            cb(signature);
        };

        let ec = crypto_api.indy_crypto_sign(
            wallet_handle,
            verkey.clone(),
            message.clone(),
            ca,
        );

        if ec == ErrorCode::Success {
            Ok(())
        } else {
            Err(ec)
        }
    }
}

#[cfg(test)]
mod test_xfer_payload {
    #![allow(unused_variables)]
    use super::*;
    use logic::config::payment_address_config::PaymentAddressConfig;
    use logic::output::Output;
    use std::sync::mpsc::channel;

    struct CryptoApiHandler {}

    impl CryptoAPI for CryptoApiHandler {
        fn indy_create_key(&self, _: IndyHandle, _: PaymentAddressConfig) -> Result<String, ErrorCode> {
            return Err(ErrorCode::CommonInvalidState);
        }

        fn indy_crypto_sign<F: FnMut(Result<String, ErrorCode>) + 'static + Send>(&self, _wallet_handle: IndyHandle, verkey: String, _message: String, mut cb: F) -> ErrorCode {
            cb(Ok(verkey + "signed"));
            return ErrorCode::Success;
        }

        fn indy_create_key_async<F: 'static>(&self, wallet_id: i32, config: PaymentAddressConfig, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
            return ErrorCode::CommonInvalidState;
        }
    }
 
    fn inputs_outputs_valid() -> (Inputs, Outputs) {
        let outputs = vec![
            Output::new(String::from("TKe9eXtchV71J2qXX5HwP8rbkTBStnEEkMwQkHie265VtRSbs"), 10, None),
            Output::new(String::from("2FKYJkgXRZtjhFpTMHhuyfc17BHZWcFPyF2MWy2SZMBaSo64fb"), 22, None),
        ];

        let inputs = vec![
            Input::new(String::from("E9LNHk8shQ6xe2RfydzXDSsyhWC6vJaUeKE2mmc6mWraDfmKm"), 1),
            Input::new(String::from("2oWxuFMbhPewEbCEeKnvjcpVq8qpHHrN5y4aU81MWG5dYfeM7V"), 1),
        ]; 

        return (inputs, outputs);
    }

    fn inputs_outputs_valid_qualified() -> (Inputs, Outputs) {
        let (inputs, outputs) = inputs_outputs_valid();
        let inps = inputs.iter().map(|ref mut i| Input::new(address::add_qualifer_to_address(&i.address), i.seq_no)).collect::<Vec<Input>>();
        let outs = outputs.iter().map(|ref mut o| Output::new(address::add_qualifer_to_address(&o.address), o.amount, o.extra.clone())).collect::<Vec<Output>>();

        return (inps, outs);
    }

    fn sign_input_sync(input: &Input, outputs: &Outputs) -> Result<String, ErrorCode> {
        let wallet_handle = 1;
        let (sender, receiver) = channel();
        let sender = Mutex::new(sender);
        let cb = move |result| { sender.lock().unwrap().send(result); };
        XferPayload::sign_input(
            &CryptoApiHandler{},
            wallet_handle,
            input,
            outputs,
            Box::new(Arc::new(cb))
        )?;
        let result = receiver.recv().unwrap();
        return result;
    }

    fn sign_inputs_sync(inputs: &Inputs, outputs: &Outputs) -> Result<Vec<String>, ErrorCode> {
        let wallet_handle = 1;
        let (sender, receiver) = channel();
        let sender = Mutex::new(sender);
        let cb = move |result| { sender.lock().unwrap().send(result); };
        XferPayload::sign_inputs(&CryptoApiHandler{}, wallet_handle, inputs, outputs,
                                 Box::new(cb))?;
        receiver.recv().unwrap()
    }

    #[test]
    fn sign_input_invalid_address_input() {
        let (mut inputs, outputs) = inputs_outputs_valid();

        String::remove(&mut inputs[0].address, 5);
        let signed_input = sign_input_sync(&inputs[0], &outputs).unwrap_err();
        assert_eq!(ErrorCode::CommonInvalidStructure, signed_input);
    }

    #[test]
    fn sign_input_valid() {
        let (inputs, outputs) = inputs_outputs_valid();

        // Question: Why are signatures dummy values?
        let signature = sign_input_sync(&inputs[0], &outputs).unwrap();
        let expected = String::from("31VzUm5vZRfWPk38W3YJaNjrkUeD6tELmjxv42cp7Vnksigned");
        assert_eq!(expected, signature);
    }

    #[test]
    fn sign_multi_input_valid_empty_inputs() {
        let (_, outputs) = inputs_outputs_valid();
        let signatures = sign_inputs_sync(&Vec::new(), &outputs).unwrap();
        assert!(signatures.is_empty());
    }

    #[test]
    fn sign_multi_input_invalid_input_address() {
        let (mut inputs, outputs) = inputs_outputs_valid();
        String::remove(&mut inputs[0].address, 5);
    
        let signatures = sign_inputs_sync(&inputs, &outputs).unwrap_err();
        assert_eq!(ErrorCode::CommonInvalidStructure, signatures);
    }

    #[test]
    fn sign_multi_input() {
        let (inputs, outputs) = inputs_outputs_valid();

        // Question: Why are signatures dummy values?
        let expected_signed_inputs = vec![
            String::from("31VzUm5vZRfWPk38W3YJaNjrkUeD6tELmjxv42cp7Vnksigned"),
            String::from("GyPZzuu8S1KMs5p6iE1wBzjQsFtaB7eigssW4YbdXdtesigned"),
        ];
        
        let signed_inputs = sign_inputs_sync(&inputs, &outputs).unwrap();
        assert_eq!(expected_signed_inputs, signed_inputs);
    }

    #[test]
    fn sign_payload_invalid_output_address() {
        let wallet_handle = 1;
        let (inputs, mut outputs) = inputs_outputs_valid_qualified();
        String::remove(&mut outputs[0].address, 5);

        let payload = XferPayload::new(inputs, outputs);
        let (sender, receiver) = channel();
        let sender = Mutex::new(sender);
        let cb = move |result| { sender.lock().unwrap().send(result); };
        let signed_payload = payload.sign(&CryptoApiHandler{}, wallet_handle, Box::new(cb)).unwrap_err();

        assert_eq!(ErrorCode::CommonInvalidStructure, signed_payload);
    }

    #[test]
    fn sign_payload_invalid_input_address() {
        let wallet_handle = 1;
        let (mut inputs, outputs) = inputs_outputs_valid_qualified();
        String::remove(&mut inputs[0].address, 13);

        let (sender, receiver) = channel();
        let sender = Mutex::new(sender);
        let cb = move |result| { sender.lock().unwrap().send(result); };
        let signed_payload = XferPayload::new(inputs, outputs).sign(&CryptoApiHandler{}, wallet_handle, Box::new(cb)).unwrap_err();

        assert_eq!(ErrorCode::CommonInvalidStructure, signed_payload);
    }

    #[test]
    fn sign_payload_invalid_empty_inputs() {
        let wallet_handle = 1;
        let (_, outputs) = inputs_outputs_valid_qualified();

        let (sender, receiver) = channel();
        let sender = Mutex::new(sender);
        let cb = move |result| { sender.lock().unwrap().send(result); };
        let signed_payload = XferPayload::new(Vec::new(), outputs).sign(&CryptoApiHandler{}, wallet_handle, Box::new(cb)).unwrap_err();

        assert_eq!(ErrorCode::CommonInvalidStructure, signed_payload);
    }

    #[test]
    fn sign_payload_invalid_empty_outputs() {
        let wallet_handle = 1;
        let (inputs, _) = inputs_outputs_valid_qualified();

        let (sender, receiver) = channel();
        let sender = Mutex::new(sender);
        let cb = move |result| { sender.lock().unwrap().send(result); };
        let signed_payload = XferPayload::new(inputs, Vec::new()).sign(&CryptoApiHandler{}, wallet_handle, Box::new(cb)).unwrap_err();

        assert_eq!(ErrorCode::CommonInvalidStructure, signed_payload);
    }

    #[test]
    fn sign_address_inputs_valid() {
        let wallet_handle = 1;
        let (inputs, outputs) = inputs_outputs_valid_qualified();

        // Question: Why are signatures dummy values?
        let expected_inputs = vec![
            Input::new(String::from("E9LNHk8shQ6xe2RfydzXDSsyhWC6vJaUeKE2mmc6mWraDfmKm"), 1),
            Input::new(String::from("2oWxuFMbhPewEbCEeKnvjcpVq8qpHHrN5y4aU81MWG5dYfeM7V"), 1),
        ];

        let expected_outputs = vec![
            Output::new(String::from("TKe9eXtchV71J2qXX5HwP8rbkTBStnEEkMwQkHie265VtRSbs"), 10, None),
            Output::new(String::from("2FKYJkgXRZtjhFpTMHhuyfc17BHZWcFPyF2MWy2SZMBaSo64fb"), 22, None),
        ];

        let expected_signatures = Some(vec![String::from("31VzUm5vZRfWPk38W3YJaNjrkUeD6tELmjxv42cp7Vnksigned"),
                                       String::from("GyPZzuu8S1KMs5p6iE1wBzjQsFtaB7eigssW4YbdXdtesigned")]);


        let (sender, receiver) = channel();
        let sender = Mutex::new(sender);
        let cb = move |result| { sender.lock().unwrap().send(result); };
        XferPayload::new(inputs, outputs).sign(&CryptoApiHandler{}, wallet_handle, Box::new(cb)).unwrap();
        let signed_payload = receiver.recv().unwrap().unwrap();

        assert_eq!(expected_inputs, signed_payload.inputs);
        assert_eq!(expected_outputs, signed_payload.outputs);
    }
}
