/*!
 * Signing of [`Inputs`] and [`Outputs`]
 * 
 * [`Inputs`]: Inputs
 * [`Outputs`]: Outputs
 */

use indy::IndyHandle;
use indy::ErrorCode;
use logic::address;
use logic::indysdk_api::CryptoAPI;
use logic::input::{Input, Inputs};
use logic::output::{Outputs};
use serde_json;
use std::sync;

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
 *      use sovtoken::logic::fees::Fees;
 *      use sovtoken::logic::payments::CreatePaymentSDK;
 *  
 *      // Need an actual wallet_handle
 *      let wallet_handle = 1;
 *      let address_input = String::from("pay:sov:SBD8oNfQNm1aEGE6KkYI1khYEGqG5zmEqrEw7maqKitIs121");
 *      let address_output = String::from("pay:sov:FekbDoBkdsj3nH2a2nNhhedoPju2UmyKrr1ZzMZGT0KENbvp");
 *      let inputs = vec![Input::new(address_input, 1, None)];
 *      let outputs = vec![Output::new(address_output, 20, None)];
 * 
 *      let fees = Fees::new(inputs, outputs);
 *      let signed_fees = fees.sign(&CreatePaymentSDK{}, wallet_handle);
 *  # }
 * ```
 */
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Fees {
    pub outputs: Outputs,
    pub inputs: Inputs,
}

impl<A: CryptoAPI> InputSigner<A> for Fees {}
impl Fees {
    pub fn new(inputs: Inputs, outputs: Outputs) -> Self
    {
        return Fees { inputs, outputs };
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
    pub fn sign<A: CryptoAPI>(mut self, crypto_api: &'static A, wallet_handle: IndyHandle) -> Result<Fees, ErrorCode> {
        if self.outputs.len() < 1 || self.inputs.len() < 1 {
            return Err(ErrorCode::CommonInvalidStructure);
        }

        for output in &mut self.outputs {
            let address = address::verkey_checksum_from_address(output.address.clone())?;
            output.address = address;
        }
        trace!("Indicator stripped from outputs");

        self.inputs = Fees::sign_inputs(crypto_api, wallet_handle, &self.inputs, &self.outputs)?;

        for input in &mut self.inputs {
            let address = address::verkey_checksum_from_address(input.address.clone())?;
            input.address = address;
        } 
        trace!("Indicator stripped from inputs");

        return Ok(self);
    }
}

trait InputSigner<A: CryptoAPI> {

    fn sign_inputs(crypto_api: &'static A, wallet_handle: IndyHandle, inputs: &Inputs, outputs: &Outputs)
        -> Result<Inputs, ErrorCode>
    {

        let signing_cbs: Result<Vec<_>, _> = inputs.iter()
            .map(|input| Self::sign_input(crypto_api, wallet_handle, input, outputs))
            .collect();
        let signing_cbs = signing_cbs?;
        trace!("Received signing callbacks.");

        let (sender, receiver) = sync::mpsc::channel();
    
        signing_cbs.iter().for_each(|signing_cb| {
            let sender_clone = sender.clone();
            let cb = move |result| { sender_clone.send(result); };
            signing_cb(Box::new(cb));
        });

        let mut signed_inputs = Vec::new();
        for _ in 0..inputs.len() {
            let signed_input = receiver.recv()
                .unwrap_or(Err(ErrorCode::CommonInvalidState))?;
            signed_inputs.push(signed_input);
        }

        return Ok(signed_inputs);
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
    fn sign_input<F: Fn(Result<Input, ErrorCode>) + Send + 'static>(
        crypto_api: &'static A,
        wallet_handle: IndyHandle,
        input: &Input,
        outputs: &Outputs
    ) -> Result<Box<Fn(Box<F>)>, ErrorCode>
    {
        let verkey = address::verkey_from_address(input.address.clone())?;
        debug!("Received verkey for payment address >>> {:?}", verkey);

        let message_json_value = json!([[input.address, input.seq_no], outputs]);
        debug!("Message to sign >>> {:?}", message_json_value);

        let message = serde_json::to_string(&message_json_value)
            .map_err(|_| ErrorCode::CommonInvalidStructure)?
            .to_string();

        let input = input.to_owned();
        return Ok(Box::new(move |func: Box<F>| {
            let input_clone = input.clone();
            // this needs to be a mutatable function
            let ca = move |signed_string: Result<String, ErrorCode>| {
                debug!("Received encoded signature >>> {:?}", signed_string);
                let signed_input = signed_string.map(|sig| input_clone.clone().sign_with(sig));
                func(signed_input);
            };

            crypto_api.indy_crypto_sign(
                wallet_handle,
                verkey.clone(),
                message.clone(),
                ca
            );
        }));
    }
}

#[cfg(test)]
mod test_fees {
    use super::*;
    use logic::config::payment_address_config::PaymentAddressConfig;
    use logic::output::Output;

    struct CryptoApiHandler {}
    impl CryptoAPI for CryptoApiHandler {
        fn indy_create_key(&self, _: IndyHandle, _: PaymentAddressConfig) -> Result<String, ErrorCode> {
            return Err(ErrorCode::CommonInvalidState);
        }

        fn indy_crypto_sign<F: FnMut(Result<String, ErrorCode>) + 'static + Send>(&self, _wallet_handle: IndyHandle, verkey: String, _message: String, mut cb: F) -> ErrorCode {
            cb(Ok(verkey + "signed"));
            return ErrorCode::Success;
        } 
    }
 
    fn inputs_outputs_valid() -> (Inputs, Outputs) {
        let outputs = vec![
            Output::new(String::from("pay:sov:Va8VcAE9CDnDEXSDQlbluWBRO5hFpTEqbSzK1UgnpbUabg9Q"), 10, None),
            Output::new(String::from("pay:sov:FekbDoBkdsj3nH2a2nNhhedoPju2UmyKrr1ZzMZGT0KENbvp"), 22, None),
        ];

        let inputs = vec![
            Input::new(String::from("pay:sov:SBD8oNfQNm1aEGE6KkYI1khYEGqG5zmEqrEw7maqKitIs121"), 1, None),
            Input::new(String::from("pay:sov:hhX4LejW7N23hPwC2yLKdor1ppXy3RhJ38TeXCZLgoBMSGfg"), 1, None),
        ]; 

        return (inputs, outputs);
    }

    fn sign_input_sync(input: &Input, outputs: &Outputs) -> Result<Input, ErrorCode> {
        let wallet_handle = 1;
        let (sender, receiver) = sync::mpsc::channel();
        let signing_cb = Fees::sign_input(
            &CryptoApiHandler{},
            wallet_handle,
            input,
            outputs
        )?;
        let cb = move |result| { sender.send(result); };
        signing_cb(Box::new(cb));
        let result = receiver.recv().unwrap();
        return result;
    }

    fn sign_inputs_sync(inputs: &Inputs, outputs: &Outputs) -> Result<Inputs, ErrorCode> {
        let wallet_handle = 1;
        return Fees::sign_inputs(&CryptoApiHandler{}, wallet_handle, inputs, outputs);
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

        let signed_input = sign_input_sync(&inputs[0], &outputs).unwrap();
        let expected = Input::new(String::from("pay:sov:SBD8oNfQNm1aEGE6KkYI1khYEGqG5zmEqrEw7maqKitIs121"), 1, Some(String::from("SBD8oNfQNm1aEGE6KkYI1khYEGqG5zmEqrEw7maqKitIsigned")));
        assert_eq!(expected, signed_input);
    }

    #[test]
    fn sign_multi_input_valid_empty_inputs() {
        let (_, outputs) = inputs_outputs_valid();
        let signed_inputs = sign_inputs_sync(&Vec::new(), &outputs).unwrap();
        assert!(signed_inputs.is_empty());
    }

    #[test]
    fn sign_multi_input_invalid_input_address() {
        let (mut inputs, outputs) = inputs_outputs_valid();
        String::remove(&mut inputs[0].address, 5);
    
        let signed_inputs = sign_inputs_sync(&inputs, &outputs).unwrap_err();
        assert_eq!(ErrorCode::CommonInvalidStructure, signed_inputs);
    }

    #[test]
    fn sign_multi_input() {
        let (inputs, outputs) = inputs_outputs_valid();
        
        let expected_signed_inputs = vec![
            Input::new(String::from("pay:sov:SBD8oNfQNm1aEGE6KkYI1khYEGqG5zmEqrEw7maqKitIs121"), 1, Some(String::from("SBD8oNfQNm1aEGE6KkYI1khYEGqG5zmEqrEw7maqKitIsigned"))),
            Input::new(String::from("pay:sov:hhX4LejW7N23hPwC2yLKdor1ppXy3RhJ38TeXCZLgoBMSGfg"), 1, Some(String::from("hhX4LejW7N23hPwC2yLKdor1ppXy3RhJ38TeXCZLgoBMsigned"))),
        ];
        
        let signed_inputs = sign_inputs_sync(&inputs, &outputs).unwrap();
        assert_eq!(expected_signed_inputs, signed_inputs);
    }

    #[test]
    fn sign_fees_invalid_address_output() {
        let wallet_handle = 1;
        let (inputs, mut outputs) = inputs_outputs_valid();
        String::remove(&mut outputs[0].address, 5);

        let fees = Fees::new(inputs, outputs);
        let signed_fees = fees.sign(&CryptoApiHandler{}, wallet_handle).unwrap_err();

        assert_eq!(ErrorCode::CommonInvalidStructure, signed_fees);
    }

    #[test]
    fn sign_fees_invalid_addresss() {
        let wallet_handle = 1;
        let (mut inputs, outputs) = inputs_outputs_valid();
        String::remove(&mut inputs[0].address, 13);

        let signed_fees = Fees::new(inputs, outputs).sign(&CryptoApiHandler{}, wallet_handle).unwrap_err();

        assert_eq!(ErrorCode::CommonInvalidStructure, signed_fees);
    }

    #[test]
    fn sign_fees_invalid_empty_inputs() {
        let wallet_handle = 1;
        let (_, outputs) = inputs_outputs_valid();

        let signed_fees = Fees::new(Vec::new(), outputs).sign(&CryptoApiHandler{}, wallet_handle).unwrap_err();

        assert_eq!(ErrorCode::CommonInvalidStructure, signed_fees);
    }

    #[test]
    fn sign_fees_invalid_empty_outputs() {
        let wallet_handle = 1;
        let (inputs, _) = inputs_outputs_valid();

        let signed_fees = Fees::new(inputs, Vec::new()).sign(&CryptoApiHandler{}, wallet_handle).unwrap_err();

        assert_eq!(ErrorCode::CommonInvalidStructure, signed_fees);
    }

    #[test]
    fn sign_address_inputs_valid() {
        let wallet_handle = 1;
        let (inputs, outputs) = inputs_outputs_valid();

        let expected_inputs = vec![
            Input::new(String::from("SBD8oNfQNm1aEGE6KkYI1khYEGqG5zmEqrEw7maqKitIs121"), 1, Some(String::from("SBD8oNfQNm1aEGE6KkYI1khYEGqG5zmEqrEw7maqKitIsigned"))),
            Input::new(String::from("hhX4LejW7N23hPwC2yLKdor1ppXy3RhJ38TeXCZLgoBMSGfg"), 1, Some(String::from("hhX4LejW7N23hPwC2yLKdor1ppXy3RhJ38TeXCZLgoBMsigned"))),
        ];

        let expected_outputs = vec![
            Output::new(String::from("Va8VcAE9CDnDEXSDQlbluWBRO5hFpTEqbSzK1UgnpbUabg9Q"), 10, None),
            Output::new(String::from("FekbDoBkdsj3nH2a2nNhhedoPju2UmyKrr1ZzMZGT0KENbvp"), 22, None),
        ];  

        let signed_fees = Fees::new(inputs, outputs).sign(&CryptoApiHandler{}, wallet_handle).unwrap();

        assert_eq!(expected_inputs, signed_fees.inputs);
        assert_eq!(expected_outputs, signed_fees.outputs);
    }
}
