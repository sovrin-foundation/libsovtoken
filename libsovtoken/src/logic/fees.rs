#![allow(dead_code)]

use indy::IndyHandle;
use serde_json;
use indy::ErrorCode;
use logic::address;
use logic::input::Input;
use logic::output::Output;
use indy::crypto::Crypto;

type Inputs = Vec<Input>;
type Outputs = Vec<Output>;

#[derive(Debug)]
struct Fees {
    outputs: Outputs,
    inputs: Inputs,
}

impl InputSigner for Fees {}
impl Fees {
    fn new(inputs: Inputs, outputs: Outputs) -> Self
    {
        return Fees { inputs, outputs };
    }
}

trait InputSigner:  {

    fn sign_inputs(wallet_handle: IndyHandle, inputs: &Inputs, outputs: &Outputs)
        -> Result<Inputs, ErrorCode>
    {
        let signed_inputs: Result<Inputs, ErrorCode> = inputs.iter()
            .map(|input| Self::sign_input(wallet_handle, input, outputs))
            .collect();

        return signed_inputs;
    }

    fn sign_input(wallet_handle: IndyHandle, input: &Input, outputs: &Outputs) -> Result<Input, ErrorCode>
    {
        println!("get to a new line for readability");
        println!("signing input = {:?}", input);
        println!("input payment_address = {:?}", input.payment_address);

//      let deserialized_address = base58::deserialize_string(input.payment_address.clone())?;

        let deserialized_address = input.payment_address.clone();

        println!("deserialized address = {:?}", deserialized_address);

        let verkey = address::verkey_from_address(deserialized_address)?;

        println!("verkey = {:?}", verkey);

        let message_json_value = json!([[input.payment_address, input.sequence_number], outputs]);

        println!("message_json_value to sign = {:?}", message_json_value);

        let message = serde_json::to_string(&message_json_value)
            .map_err(|_| ErrorCode::CommonInvalidStructure)?
            .to_string();

        println!("message to sign = {:?}", message);

        return Self::indy_crypto_sign(wallet_handle, verkey, message)
            .map(|signed_string| input.clone().sign_with(signed_string));
    }

    fn indy_crypto_sign (
        wallet_handle: IndyHandle,
        verkey: String,
        message: String,
    ) -> Result<String, ErrorCode>
    {
         return Crypto::sign(wallet_handle, &verkey, message.as_bytes())
             .map(|vec| String::from_utf8(vec).unwrap());
    }
}

#[cfg(test)]
mod test_fees {
    use super::*;

    struct MockedFees {}

    impl InputSigner for MockedFees {
        fn indy_crypto_sign(
            _wallet_handle: IndyHandle,
            verkey: String,
            _message: String
        ) -> Result<String, ErrorCode> {
            return Ok(verkey + "signed");
        }
    }

    fn inputs_outputs_valid() -> (Inputs, Outputs) {
        let outputs = vec![
            Output::new(String::from("pay:sov:gGpXeIzxDaZmeVhJZs6qWrdBPbDG3AfTW7RD"), 10, None),
            Output::new(String::from("pay:sov:jtCpdpjVjIJ5vrIlD3KwFjzz8LBaJGIJVUn2"), 22, None),
        ];

        let inputs = vec![
            Input::new(String::from("pay:sov:dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5F"), 1, None),
            Input::new(String::from("pay:sov:anotherGGUf33VxAwgTFjkxu1A9JM3Sscd5F"), 1, None),
        ]; 

        return (inputs, outputs);
    }

    #[test]
    fn sign_input_invalid_sequence_number() {
        unimplemented!();
    }

    fn sign_input_invalid_address_output() {
        unimplemented!();
    }

    #[test]
    fn sign_input_invalid_address_input() {
        let wallet_handle = 1;
        let (inputs, outputs) = inputs_outputs_valid();

        let mut input = inputs.into_iter().nth(0).unwrap();
        String::remove(&mut input.payment_address, 5);

        let signed_input = MockedFees::sign_input(wallet_handle, &input, &outputs).unwrap_err();
        assert_eq!(ErrorCode::CommonInvalidStructure, signed_input);
    }

    #[test]
    fn sign_input() {
        let (inputs, outputs) = inputs_outputs_valid();

        let input = inputs.into_iter().nth(0).unwrap();
        let wallet_handle = 1;

        let signed_input = MockedFees::sign_input( wallet_handle, &input, &outputs).unwrap();
        let expected = Input::new(String::from("pay:sov:dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5F"), 1, Some(String::from("dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sssigned")));
        assert_eq!(expected, signed_input);
    }

    #[test]
    fn sign_multi_input() {
        let wallet_handle = 1;
        let (inputs, outputs) = inputs_outputs_valid();
        
        let expected_signed_inputs = vec![
            Input::new(String::from("pay:sov:dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5F"), 1, Some(String::from("dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sssigned"))),
            Input::new(String::from("pay:sov:anotherGGUf33VxAwgTFjkxu1A9JM3Sscd5F"), 1, Some(String::from("anotherGGUf33VxAwgTFjkxu1A9JM3Sssigned"))),
        ];
        
        let signed_inputs = MockedFees::sign_inputs(wallet_handle, &inputs, &outputs).unwrap();
        assert_eq!(expected_signed_inputs, signed_inputs);
    }
}
