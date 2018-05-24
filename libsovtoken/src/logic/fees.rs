#![allow(dead_code)]

use serde_json;
use indy::ErrorCode;
use logic::address;
use logic::indysdk_api::CryptoAPI;
use logic::input::Input;
use logic::output::Output;
use logic::payments::{CreatePaymentSDK};
//use logic::types::ClosureString;
//use utils::json_conversion::JsonDeserialize;

use indy::wallet::Wallet;
use std;

type Inputs = Vec<Input>;
type Outputs = Vec<Output>;

static WALLET_NAME_1: &'static str = "sign_test_wallet_1";
static VALID_CONFIG_EMPTY_SEED_JSON: &'static str = r#"{}"#;

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

    pub fn sign_inputs(self, wallet_handle: i32)
        -> Result<Fees, ErrorCode>
    {
        let outputs = self.outputs;

        let signed_inputs: Result<Vec<Input>, ErrorCode> = self.inputs.iter()
            .map(|input| Fees::sign_input(wallet_handle, input, &outputs))
            .collect();

        let signed_fees = Fees::new(signed_inputs?, outputs);

        return Ok(signed_fees);
    }

    pub fn sign_input(wallet_handle: i32, input: &Input, outputs: &Outputs) -> Result<Input, ErrorCode>
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

        let payment_handler = CreatePaymentSDK {};
        return payment_handler
            .indy_crypto_sign(wallet_handle, verkey, message)
            .map(|signed_string| input.clone().sign_with(signed_string));
    }
}

#[cfg(test)]
mod test_fees {
    use super::*;

    // deletes, creates and opens a wallet.  it will successfully create and open the wallet,
    // regardless if the wallet exists
    fn safely_create_wallet(wallet_name : &str) -> i32 {
        let panic_result = std::panic::catch_unwind( ||
             {
                 Wallet::delete_wallet(wallet_name);
             });

        Wallet::create_wallet("pool_1", wallet_name, None, Some(VALID_CONFIG_EMPTY_SEED_JSON), None);
        let wallet_id: i32 = Wallet::open_wallet(wallet_name, None, None).unwrap();

        return wallet_id;
    }


    #[test]
    fn sign_input() {
        let outputs = vec![
            Output::new(String::from("2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es"), 10, None),
            Output::new(String::from("dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q"), 22, None),
        ];

        let input = Input::new(String::from("pay:sov:dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5F"), 1, None);

        let wallet_handle: i32 = safely_create_wallet(WALLET_NAME_1);
        println!("wallet id = {:?}", wallet_handle);

//        let wallet_handle = 1;

        let signed_input = Fees::sign_input( wallet_handle, &input, &outputs).unwrap();
        let expected = Input::new(String::from("pay:sov:dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5F"), 1, Some(String::from("dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sssigned")));
        assert_eq!(expected, signed_input);
    }

    #[test]
    fn sign_multi_input() {
        let outputs = vec![
            Output::new(String::from("2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es"), 10, None),
            Output::new(String::from("dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q"), 22, None),
        ];

        let inputs = vec![
            Input::new(String::from("pay:sov:dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5F"), 1, None),
            Input::new(String::from("pay:sov:anotherGGUf33VxAwgTFjkxu1A9JM3Sscd5F"), 1, None),
        ]; 

        let expected_signed_inputs = vec![
            Input::new(String::from("pay:sov:dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5F"), 1, Some(String::from("dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sssigned"))),
            Input::new(String::from("pay:sov:anotherGGUf33VxAwgTFjkxu1A9JM3Sscd5F"), 1, Some(String::from("anotherGGUf33VxAwgTFjkxu1A9JM3Sssigned"))),
        ];
        
        let wallet_handle = 1;

        let fees = Fees::new(inputs, outputs);

        println!("these are the fees = {:?}", fees);

        let fees_signed = fees.sign_inputs(wallet_handle).unwrap();

        println!("Completed signed_fees = {:?} ", fees_signed);

        assert_eq!(expected_signed_inputs, fees_signed.inputs);

    }
}
