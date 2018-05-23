#![allow(dead_code)]

use serde_json;
use indy::ErrorCode;
use logic::address;
use logic::indysdk_api::CryptoAPI;
use logic::input::Input;
use logic::output::Output;
use logic::payments::{CreatePaymentSDK};
use logic::types::ClosureString;

type Inputs = Vec<Input>;
type Outputs = Vec<Output>;

#[allow(dead_code)]
#[derive(Debug)]
struct Fees {
    outputs: Outputs,
    inputs: Inputs,
}


impl Fees {
    pub fn new(inputs: Inputs, outputs: Outputs) -> Self {
        return Fees { inputs, outputs };
    }

    pub fn sign_inputs<C>(self, wallet_handle: i32, cb: C)
        where C: Fn(Fees) + Send + Sync + 'static
    {
        let outputs = self.outputs;
        let outputs_cloned = outputs.clone();

        // map_async(
        //     self.inputs,
        //     move |input, done| {
        //         let input_to_be_signed = input.clone();
        //         let sign_result = Fees::sign_input(wallet_handle, &input, &outputs, Box::new(move |_error_code, signature| {
        //             let signed_input = input_to_be_signed.to_owned().sign_with(signature);
        //             done(Ok(signed_input));
        //         }));
        //         if sign_result.is_err() {
        //             done(sign_result);
        //         }
        //     },
        //     move |inputs| {
        //         let new_fees = Fees::new(inputs, outputs_cloned.to_owned());
        //         cb(new_fees);
        //     }
        // );

//        return ErrorCode::Success;
    }

    pub fn sign_input(wallet_handle: i32, input: &Input, outputs: &Outputs, mut cb: ClosureString) -> Result<ErrorCode, ErrorCode>
    {
        println!("get to a new line for readability");
        println!("signing input = {:?}", input);
        println!("input payment_address = {:?}", input.payment_address);

//        let deserialized_address = base58::deserialize_string(input.payment_address.clone())?;

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

        // let payment_handler = CreatePaymentSDK {};
        // payment_handler.indy_crypto_sign(
        //     wallet_handle,
        //     verkey,
        //     message,
        //     cb
        // );
        cb(ErrorCode::Success, verkey + "signed");

        return Ok(ErrorCode::Success);
    }
}

#[cfg(test)]
mod test_fees {
    use super::*;

    #[test]
    fn sign_input() {
        static mut CALLBACK_CALLED: bool = false;
        let outputs = vec![
            Output::new(String::from("2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es"), 10, None),
            Output::new(String::from("dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q"), 22, None),
        ];

        let input = Input::new(String::from("pay:sov:dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5F"), 1, None);
        let wallet_handle = 1;

        let _ = Fees::sign_input( wallet_handle, &input, &outputs, Box::new(|ec, signature| {
            unsafe {CALLBACK_CALLED = true;}
            assert_eq!(String::from("dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sssigned"), signature);
            assert_eq!(ec, ErrorCode::Success);
//            return ErrorCode::Success;
        }));

        assert!(unsafe {CALLBACK_CALLED});
    }

    #[test]
    fn sign_multi_input() {
        static mut CALLBACK_CALLED: bool = false;
        let outputs = vec![
            Output::new(String::from("2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es"), 10, None),
            Output::new(String::from("dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q"), 22, None),
        ];

        let inputs = vec![
            Input::new(String::from("pay:sov:dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5F"), 1, None),
            Input::new(String::from("pay:sov:anotherGGUgTFjkxu1A9JM3Sscd5F"), 1, None),
        ]; 
        
        let wallet_handle = 1;

        let fees = Fees::new(inputs, outputs);
        
        // let _ = fees.sign_inputs(wallet_handle, | ec, fees | {
        //     unsafe {CALLBACK_CALLED = true;}

        //     // assert_eq!(String::from("dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sssigned"), signatures);
        //     assert_eq!(ec, ErrorCode::Success);
        // });



        assert!(unsafe {CALLBACK_CALLED});
    }
}
