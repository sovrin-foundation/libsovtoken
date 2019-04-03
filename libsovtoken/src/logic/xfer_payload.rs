//! Common structure and logic used for payments; token transfer and fees

/*!
 * Signing of [`Inputs`] and [`Outputs`]
 * 
 * [`Inputs`]: Inputs
 * [`Outputs`]: Outputs
 */


use hex::ToHex;
use serde_json;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;

use IndyHandle;
use ErrorCode;

use logic::address;
use logic::indy_sdk_api::crypto_api::CryptoAPI;
use logic::input::{Input, Inputs};
use logic::output::{Outputs};
use logic::hash::Hash;

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
 *      let outputs = vec![Output::new(address_output, 20)];
 *
 *      let payload = XferPayload::new(inputs, outputs, None);
 *      let signed_payload = payload.sign_transfer(&CryptoSdk{}, wallet_handle, Box::new(|res| Default::default()));
 *  # }
 * ```
 */
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct XferPayload {
    pub outputs: Outputs,
    pub inputs: Inputs,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
    pub signatures: Option<Vec<String>>
}

unsafe impl Send for XferPayload {}
unsafe impl Sync for XferPayload {}

impl<A: CryptoAPI> InputSigner<A> for XferPayload {}
impl XferPayload {
    pub fn new(inputs: Inputs, outputs: Outputs, extra: Option<String>) -> Self
    {
        return XferPayload { inputs, outputs, extra, signatures: None };
    }

    // TODO: Add request hash to include while signature
    pub fn sign_fees<A: CryptoAPI>(self, crypto_api: &'static A, wallet_handle: IndyHandle, txn_digest: &Option<String>, cb: Box<Fn(Result<XferPayload, ErrorCode>) + Send + Sync>) -> Result<(), ErrorCode> {
        trace!("logic::xfer_payload::xfer_payload::sign_fees >> wallet_handle: {:?}", wallet_handle);
        if self.inputs.len() < 1 {
            return Err(ErrorCode::CommonInvalidStructure);
        }
        self.sign(crypto_api, wallet_handle, txn_digest, cb)
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
    pub fn sign_transfer<A: CryptoAPI>(self, crypto_api: &'static A, wallet_handle: IndyHandle, cb: Box<Fn(Result<XferPayload, ErrorCode>) + Send + Sync>) -> Result<(), ErrorCode> {
        trace!("logic::xfer_payload::xfer_payload::sign >> wallet_handle: {:?}", wallet_handle);
        if self.outputs.len() < 1 || self.inputs.len() < 1 {
            return Err(ErrorCode::CommonInvalidStructure);
        }
        self.sign(crypto_api, wallet_handle, &None, cb)
    }

    fn sign<A: CryptoAPI>(mut self, crypto_api: &'static A, wallet_handle: IndyHandle, txn_digest: &Option<String>, cb: Box<Fn(Result<XferPayload, ErrorCode>) + Send + Sync>) -> Result<(), ErrorCode> {
        for output in &mut self.outputs {
            output.recipient = address::unqualified_address_from_address(&output.recipient)?;
        }
        debug!("Indicator stripped from outputs");

        for input in &mut self.inputs {
            input.address = address::unqualified_address_from_address(&input.address)?;
        }

        debug!("Indicator stripped from inputs");

        XferPayload::sign_inputs(crypto_api, wallet_handle, &self.inputs.clone(), &self.outputs.clone(), txn_digest, &self.extra.clone(),Box::new(move |signatures| {
            match signatures {
                Ok(signatures) => {
                    let payload = Self::clone_payload_add_signatures(&self, signatures);
                    info!("Built XFER payload: {:?}", payload);
                    cb(Ok(payload));
                }
                Err(err) => {
                    error!("Got an error while signing utxos: {:?}", err);
                    cb(Err(err));
                }
            };
        }))?;

        let res = Ok(());
        trace!("logic::xfer_payload::xfer_payload::sign << result: {:?}", res);
        res
    }

    fn clone_payload_add_signatures(prev: &Self, signatures: HashMap<String, String>) -> Self {
        let signatures = prev.inputs
            .iter()
            .map(|input_address| signatures.get(&input_address.to_string()))
            .filter(|signature| signature.is_some())
            .map(|signature| signature.unwrap().to_owned())
            .collect();
        
        XferPayload {
            inputs: prev.inputs.clone(),
            outputs: prev.outputs.clone(),
            extra: prev.extra.clone(),
            signatures: Some(signatures),
        }
    }
}

trait InputSigner<A: CryptoAPI> {
    fn sign_inputs(crypto_api: &'static A, wallet_handle: IndyHandle, inputs: &Inputs, outputs: &Outputs, txn_digest: &Option<String>, extra: &Option<String>, cb: Box<Fn(Result<HashMap<String, String>, ErrorCode>) + Send + Sync>)
                   -> Result<(), ErrorCode>
    {
        let inputs_result: Arc<Mutex<HashMap<String, String>>> = Default::default();

        let res_cnt = inputs.len();
        let cb = Arc::new(move |signature: Result<String, ErrorCode>, input| {
            match signature {
                Ok(signature) => {
                    let mut results = inputs_result.lock().unwrap();
                    results.insert(input, signature);
                    if results.len() == res_cnt {
                        cb(Ok(results.clone()))
                    }
                }
                Err(err) => cb(Err(err))
            }
        });

        for input in inputs {
            let cb = cb.clone();
            match Self::sign_input(crypto_api, wallet_handle, input, outputs, txn_digest, extra, Box::new(cb)) {
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
        txn_digest: &Option<String>,
        _extra: &Option<String>,
        cb: Box<Arc<Fn(Result<String, ErrorCode>, String) + Send + Sync>>,
    ) -> Result<(), ErrorCode>
    {
        trace!("logic::xfer_payload::input_signer::sign_input >> input: {:?}, outputs: {:?}, wallet_handle {:?}", input, outputs, wallet_handle);
        let verkey = address::verkey_from_unqualified_address(&input.address.clone())?;
        debug!("Received verkey for payment address >>> {:?}", verkey);

        let vals: Vec<serde_json::Value> = vec![
            Some(json!([input])),
            Some(json!(outputs)),
            txn_digest.clone().map(|e| json!(e)),
//            _extra.map(|e| json!(e))
        ].into_iter().filter_map(|e| e).collect();

        let message = serialize_signature(json!(vals))?;

        debug!("Message to sign >>> {:?}", &message);

        let input_key = input.to_string();

        let ca = move |signature: Result<String, ErrorCode>| {
            let key = input_key.clone();
            debug!("Received encoded signature >>> {:?} for input {:?}", signature, key);
            cb(signature, key);
        };

        let ec = crypto_api.indy_crypto_sign(
            wallet_handle,
            verkey.clone(),
            message.clone(),
            ca,
        );

        trace!("logic::xfer_payload::input_signer::sign_input << result: {:?}", ec);
        if ec == ErrorCode::Success {
            Ok(())
        } else {
            Err(ec)
        }
    }
}

pub fn serialize_signature(v: serde_json::Value) -> Result<String, ErrorCode> {
    do_serialize_signature(v, true)
}

fn do_serialize_signature(v: serde_json::Value, is_top_level: bool) -> Result<String, ErrorCode> {
    match v {
        serde_json::Value::Bool(value) => Ok(if value { "True".to_string() } else { "False".to_string() }),
        serde_json::Value::Number(value) => Ok(value.to_string()),
        serde_json::Value::String(value) => Ok(value),
        serde_json::Value::Array(array) => {
            let mut result = "".to_string();
            let length = array.len();
            for (index, element) in array.iter().enumerate() {
                result += &do_serialize_signature(element.clone(), false)?;
                if index < length - 1 {
                    result += ",";
                }
            }
            Ok(result)
        }
        serde_json::Value::Object(map) => {
            let mut result = "".to_string();
            let mut in_middle = false;
            for key in map.keys() {
                // Skip signature field at top level as in python code
                if is_top_level && (key == "signature" || key == "fees" || key == "signatures") { continue; }

                if in_middle {
                    result += "|";
                }

                let mut value = map[key].clone();
                if key == "raw" || key == "hash" || key == "enc" {
                    let mut ctx = Hash::new_context()?;
                    ctx.update(&value.as_str().ok_or(ErrorCode::CommonInvalidState)?.as_bytes()).map_err(|_| ErrorCode::CommonInvalidState)?;
                    value = serde_json::Value::String(ctx.finish().map_err(|_| ErrorCode::CommonInvalidState)?.as_ref().to_hex());
                }
                result = result + key + ":" + &do_serialize_signature(value, false)?;
                in_middle = true;
            }
            Ok(result)
        }
        _ => Ok("".to_string())
    }
}

#[cfg(test)]
mod test_xfer_payload {
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

        fn indy_create_key_async<F: 'static>(&self, _wallet_id: i32, _config: PaymentAddressConfig, _closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
            return ErrorCode::CommonInvalidState;
        }
    }
 
    fn inputs_outputs_valid() -> (Inputs, Outputs) {
        let outputs = vec![
            Output::new(String::from("TKe9eXtchV71J2qXX5HwP8rbkTBStnEEkMwQkHie265VtRSbs"), 10),
            Output::new(String::from("2FKYJkgXRZtjhFpTMHhuyfc17BHZWcFPyF2MWy2SZMBaSo64fb"), 22),
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
        let outs = outputs.iter().map(|ref mut o| Output::new(address::add_qualifer_to_address(&o.recipient), o.amount)).collect::<Vec<Output>>();

        return (inps, outs);
    }

    fn sign_input_sync(input: &Input, outputs: &Outputs, extra: &Option<String>) -> Result<String, ErrorCode> {
        let wallet_handle = 1;
        let (sender, receiver) = channel();
        let sender = Mutex::new(sender);
        let cb = move |result, _| {
            sender.lock().unwrap().send(result).unwrap();
        };
        XferPayload::sign_input(
            &CryptoApiHandler{},
            wallet_handle,
            input,
            outputs,
            &None,
            extra,
            Box::new(Arc::new(cb))
        )?;
        let result = receiver.recv().unwrap();
        return result;
    }

    fn sign_inputs_sync(inputs: &Inputs, outputs: &Outputs) -> Result<Vec<String>, ErrorCode> {
        let wallet_handle = 1;
        let (sender, receiver) = channel();
        let sender = Mutex::new(sender);
        let cb = move |result| { sender.lock().unwrap().send(result).unwrap(); };
        XferPayload::sign_inputs(&CryptoApiHandler{}, wallet_handle, inputs, outputs, &None, &None,
                                 Box::new(cb))?;
        receiver.recv().unwrap().map(|map| map.values().cloned().collect())
    }

    #[test]
    fn sign_input_invalid_address_input() {
        let (mut inputs, outputs) = inputs_outputs_valid();

        String::remove(&mut inputs[0].address, 5);
        let signed_input = sign_input_sync(&inputs[0], &outputs, &None).unwrap_err();
        assert_eq!(ErrorCode::CommonInvalidStructure, signed_input);
    }

    #[test]
    fn sign_input_valid() {
        let (inputs, outputs) = inputs_outputs_valid();

        // Question: Why are signatures dummy values?
        let signature = sign_input_sync(&inputs[0], &outputs, &None).unwrap();
        let expected = String::from("31VzUm5vZRfWPk38W3YJaNjrkUeD6tELmjxv42cp7Vnksigned");
        assert_eq!(expected, signature);
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
        
        let signed_inputs = sign_inputs_sync(&inputs, &outputs).unwrap();
        assert!(signed_inputs.contains(&"31VzUm5vZRfWPk38W3YJaNjrkUeD6tELmjxv42cp7Vnksigned".to_string()));
        assert!(signed_inputs.contains(&"GyPZzuu8S1KMs5p6iE1wBzjQsFtaB7eigssW4YbdXdtesigned".to_string()));
    }

    #[test]
    fn sign_payload_invalid_output_address() {
        let wallet_handle = 1;
        let (inputs, mut outputs) = inputs_outputs_valid_qualified();
        String::remove(&mut outputs[0].recipient, 5);

        let payload = XferPayload::new(inputs, outputs, None);
        let (sender, _receiver) = channel();
        let sender = Mutex::new(sender);
        let cb = move |result| { sender.lock().unwrap().send(result).unwrap(); };
        let signed_payload = payload.sign_transfer(&CryptoApiHandler{}, wallet_handle, Box::new(cb)).unwrap_err();

        assert_eq!(ErrorCode::CommonInvalidStructure, signed_payload);
    }

    #[test]
    fn sign_payload_invalid_input_address() {
        let wallet_handle = 1;
        let (mut inputs, outputs) = inputs_outputs_valid_qualified();
        String::remove(&mut inputs[0].address, 13);

        let (sender, _receiver) = channel();
        let sender = Mutex::new(sender);
        let cb = move |result| { sender.lock().unwrap().send(result).unwrap(); };
        let signed_payload = XferPayload::new(inputs, outputs, None).sign_transfer(&CryptoApiHandler{}, wallet_handle, Box::new(cb)).unwrap_err();

        assert_eq!(ErrorCode::CommonInvalidStructure, signed_payload);
    }

    #[test]
    fn sign_payload_invalid_empty_inputs() {
        let wallet_handle = 1;
        let (_, outputs) = inputs_outputs_valid_qualified();

        let (sender, _receiver) = channel();
        let sender = Mutex::new(sender);
        let cb = move |result| { sender.lock().unwrap().send(result).unwrap(); };
        let signed_payload = XferPayload::new(Vec::new(), outputs, None).sign_transfer(&CryptoApiHandler{}, wallet_handle, Box::new(cb)).unwrap_err();

        assert_eq!(ErrorCode::CommonInvalidStructure, signed_payload);
    }

    #[test]
    fn sign_payload_invalid_empty_outputs() {
        let wallet_handle = 1;
        let (inputs, _) = inputs_outputs_valid_qualified();

        let (sender, _receiver) = channel();
        let sender = Mutex::new(sender);
        let cb = move |result| { sender.lock().unwrap().send(result).unwrap(); };
        let signed_payload = XferPayload::new(inputs, Vec::new(), None).sign_transfer(&CryptoApiHandler{}, wallet_handle, Box::new(cb)).unwrap_err();

        assert_eq!(ErrorCode::CommonInvalidStructure, signed_payload);
    }

    #[test]
    fn sign_fees_valid_empty_outputs() {
        let wallet_handle = 1;
        let (inputs, _) = inputs_outputs_valid_qualified();

        let (sender, _receiver) = channel();
        let sender = Mutex::new(sender);
        let cb = move |result| { sender.lock().unwrap().send(result).unwrap(); };
        let signed_payload = XferPayload::new(inputs, Vec::new(), None).sign_fees(&CryptoApiHandler{}, wallet_handle, &None, Box::new(cb));

        assert!(signed_payload.is_ok());
    }

    #[test]
    fn sign_fees_valid_non_empty_outputs() {
        let wallet_handle = 1;
        let (inputs, outputs) = inputs_outputs_valid_qualified();

        let (sender, _receiver) = channel();
        let sender = Mutex::new(sender);
        let cb = move |result| { sender.lock().unwrap().send(result).unwrap(); };
        let signed_payload = XferPayload::new(inputs, outputs, None).sign_fees(&CryptoApiHandler{}, wallet_handle, &None, Box::new(cb));

        assert!(signed_payload.is_ok());
    }

    #[test]
    fn sign_fees_invalid_empty_inputs() {
        let wallet_handle = 1;
        let (_, outputs) = inputs_outputs_valid_qualified();

        let (sender, _receiver) = channel();
        let sender = Mutex::new(sender);
        let cb = move |result| { sender.lock().unwrap().send(result).unwrap(); };
        let signed_payload = XferPayload::new(Vec::new(), outputs, None).sign_fees(&CryptoApiHandler{}, wallet_handle, &None, Box::new(cb)).unwrap_err();

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
            Output::new(String::from("TKe9eXtchV71J2qXX5HwP8rbkTBStnEEkMwQkHie265VtRSbs"), 10),
            Output::new(String::from("2FKYJkgXRZtjhFpTMHhuyfc17BHZWcFPyF2MWy2SZMBaSo64fb"), 22),
        ];

        let expected_signatures = Some(vec![String::from("31VzUm5vZRfWPk38W3YJaNjrkUeD6tELmjxv42cp7Vnksigned"),
                                       String::from("GyPZzuu8S1KMs5p6iE1wBzjQsFtaB7eigssW4YbdXdtesigned")]);


        let (sender, receiver) = channel();
        let sender = Mutex::new(sender);
        let cb = move |result| { sender.lock().unwrap().send(result).unwrap(); };
        XferPayload::new(inputs, outputs, None).sign_transfer(&CryptoApiHandler{}, wallet_handle, Box::new(cb)).unwrap();
        let signed_payload = receiver.recv().unwrap().unwrap();

        assert_eq!(expected_inputs, signed_payload.inputs);
        assert_eq!(expected_outputs, signed_payload.outputs);
        assert_eq!(expected_signatures, signed_payload.signatures);
    }

    /*
    This test was created as a result of a bug where the signature ordering was
    arbitrary. This isn't a perfect test, but it does increase confidence.
    */
    #[test]
    fn sign_multi_input_preserve_ordering() {
        let attempts = 5;
        let wallet_handle = 1;
        let (mut inputs, outputs) = inputs_outputs_valid_qualified();   
        inputs.reverse();

        let expected_signatures = vec![
            String::from("GyPZzuu8S1KMs5p6iE1wBzjQsFtaB7eigssW4YbdXdtesigned"),
            String::from("31VzUm5vZRfWPk38W3YJaNjrkUeD6tELmjxv42cp7Vnksigned"),
        ];
        let payload = XferPayload::new(inputs, outputs, None);

        let (sender, receiver) = channel();
        let sender = Arc::new(Mutex::new(sender));

        for _ in 0..attempts {
            let sender = sender.clone();
            let cb = move |result| { sender.lock().unwrap().send(result).unwrap(); };
            payload.clone().sign_transfer(
                &CryptoApiHandler{},
                wallet_handle,
                Box::new(cb)
            ).unwrap();
        }

        for _ in 0..attempts {
            let signed_payload = receiver.recv().unwrap().unwrap();
            assert_eq!(expected_signatures, signed_payload.signatures.unwrap());
        }
    }
}
