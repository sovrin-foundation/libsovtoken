#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate sovtoken;
extern crate rust_indy_sdk as indy;
pub mod utils;

use indy::{IndyHandle, ErrorCode};
use indy::utils::results::ResultHandler;
use sovtoken::utils::ffi_support::c_pointer_from_string;
use sovtoken::utils::ffi_support::c_pointer_from_str;
use sovtoken::logic::parsers::common::TXO;
use sovtoken::utils::test::callbacks;
use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::time::Duration;
use utils::wallet::Wallet;


fn call_add_fees(wallet_handle: IndyHandle, inputs: String, outputs: String, request: String) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = callbacks::cb_ec_string();
    let did = "mydid1";
    let error_code = sovtoken::api::add_request_fees_handler(
        command_handle,
        wallet_handle,
        c_pointer_from_str(did),
        c_pointer_from_string(request),
        c_pointer_from_string(inputs),
        c_pointer_from_string(outputs),
        cb
    );

    return ResultHandler::one(ErrorCode::from(error_code), receiver); 
}

fn init_wallet_with_address() -> (utils::wallet::Wallet, String) {
    sovtoken::api::sovtoken_init();

    let wallet = Wallet::new("p1");
    let seed = str::repeat("2", 32);

    let input_address = utils::payment::address::generate(&wallet, Some(&seed));
    return (wallet, input_address);
}

#[test]
fn test_add_fees_to_request_valid() {
    let (wallet, input_address) = init_wallet_with_address();

    let fake_request = json!({
       "operation": {
           "type": "3"
       }
    });

    let txo = TXO { address: input_address, seq_no: 1 };

    let inputs = json!([txo.to_libindy_string().unwrap()]);
    
    let outputs = json!([{
            "paymentAddress": "pay:sov:dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q",
            "amount": 20,
    }]);

    let expected_fees_request = json!({
       "fees": [
           [["iTQzpdRdugkJ2gLD5vW5c159dncSL9jbAtu3WfPcb8qWD9bUd", 1]],
           [["dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q", 20]],
           ["5qqD2vk3nTeG5ZS1jVAvgPozPeSsBw8E8rux2jV8KsoFd1CiAzzpfez7ixMKvUpYaiQdEhsQwXaLNJRHHyF5g24R"]
       ],
       "operation": {
           "type": "3"
       }
    });

    let result = call_add_fees(
        wallet.handle,
        inputs.to_string(),
        outputs.to_string(),
        fake_request.to_string()
    ).unwrap();

    assert_eq!(expected_fees_request.to_string(), result);
}

#[test]
fn test_add_fees_to_request_valid_from_libindy() {
    let (wallet, input_address) = init_wallet_with_address();
    let did = "Th7MpTaRZVRYnPiabds81Y";

    let fake_request = json!({
       "operation": {
           "type": "3"
       }
    });

    let txo = TXO { address: input_address, seq_no: 1 };

    let inputs = json!([txo.to_libindy_string().unwrap()]);

    let outputs = json!([{
            "paymentAddress": "pay:sov:dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q",
            "amount": 20,
    }]);

    let expected_fees_request = json!({
       "fees": [
           [["iTQzpdRdugkJ2gLD5vW5c159dncSL9jbAtu3WfPcb8qWD9bUd", 1]],
           [["dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q", 20]],
           ["5qqD2vk3nTeG5ZS1jVAvgPozPeSsBw8E8rux2jV8KsoFd1CiAzzpfez7ixMKvUpYaiQdEhsQwXaLNJRHHyF5g24R"]
       ],
       "operation": {
           "type": "3"
       }
    });

    let (sender, receiver) = channel();

    let cb = move |ec, req, method| {
        sender.send((ec, req, method)).unwrap();
    };

    let return_error = indy::payments::Payment::add_request_fees_async(
        wallet.handle,
        did,
        &fake_request.to_string(),
        &inputs.to_string(),
        &outputs.to_string(),
        cb
    );

    let (req, method) = ResultHandler::two_timeout(return_error, receiver, Duration::from_secs(15)).unwrap();
    assert_eq!("sov", method);
    assert_eq!(expected_fees_request.to_string(), req);
}

#[test]
pub fn build_and_submit_nym_with_fees() {
    sovtoken::api::sovtoken_init();
    let payment_method = sovtoken::api::PAYMENT_METHOD_NAME;
    let pc_str = utils::pool::create_pool_config();
    let pool_config = Some(pc_str.as_str());
    indy::pool::Pool::set_protocol_version(2).unwrap();

    let pool_name = utils::pool::create_pool_ledger(pool_config);
    let pool_handle = indy::pool::Pool::open_ledger(&pool_name, None).unwrap();
    let wallet = utils::wallet::Wallet::new(&pool_name);

    let (did_trustee, _) = indy::did::Did::new(wallet.handle, &json!({"seed":"000000000000000000000000Trustee1"}).to_string()).unwrap();

    let pa1 = indy::payments::Payment::create_payment_address(wallet.handle, payment_method, "{}").unwrap();

    let mut mint_cfg = HashMap::new();
    mint_cfg.insert(pa1.clone(), 10);

    utils::mint::mint_tokens(mint_cfg, pool_handle, wallet.handle, &did_trustee).unwrap();

    let (req, method) = indy::payments::Payment::build_get_utxo_request(wallet.handle, &did_trustee, &pa1).unwrap();
    let res = indy::ledger::Ledger::sign_and_submit_request(pool_handle, wallet.handle, &did_trustee, &req).unwrap();
    let res = indy::payments::Payment::parse_get_utxo_response(&method, &res).unwrap();

    let res_parsed: serde_json::Value = serde_json::from_str(&res).unwrap();
    let utxos = res_parsed.as_array().unwrap();
    let value = utxos.get(0).unwrap().as_object().unwrap();
    let utxo = value.get("txo").unwrap().as_str().unwrap();

    let inputs = json!([utxo]).to_string();
    let outputs = json!([{
        "paymentAddress": pa1,
        "amount": 9
    }]).to_string();

    let fees = json!({
        "1": 1
    }).to_string();

    let dids = vec![did_trustee.as_str()];

    utils::fees::set_fees(pool_handle, wallet.handle, payment_method, &fees, &dids);

    let (did_new, verkey_new) = indy::did::Did::new(wallet.handle, "{}").unwrap();

    let nym_req = indy::ledger::Ledger::build_nym_request(&did_trustee, &did_new,  Some(&verkey_new), None, None).unwrap();
    let nym_req_signed = indy::ledger::Ledger::sign_request(wallet.handle, &did_trustee, &nym_req).unwrap();
    let (nym_req_with_fees, pm) = indy::payments::Payment::add_request_fees(wallet.handle, &did_trustee, &nym_req_signed, &inputs, &outputs).unwrap();
    let nym_resp = indy::ledger::Ledger::submit_request(pool_handle, &nym_req_with_fees).unwrap();
    let parsed_resp = indy::payments::Payment::parse_response_with_fees(&pm, &nym_resp).unwrap();

    let parsed_resp_json: Vec<HashMap<String, serde_json::Value>> = serde_json::from_str(&parsed_resp).unwrap();
    assert_eq!(parsed_resp_json.len(), 1);
    assert_eq!(parsed_resp_json[0].get("amount").unwrap().as_u64().unwrap(), 9);
    assert_eq!(parsed_resp_json[0].get("paymentAddress").unwrap().as_str().unwrap(), pa1);

    let get_nym_req = indy::ledger::Ledger::build_get_nym_request(&did_trustee, &did_new).unwrap();
    let get_nym_resp = indy::ledger::Ledger::sign_and_submit_request(pool_handle, wallet.handle, &did_trustee, &get_nym_req).unwrap();
    let get_nym_resp_json: serde_json::Value = serde_json::from_str(&get_nym_resp).unwrap();
    assert!(get_nym_resp_json.as_object().unwrap().get("result").unwrap().as_object().unwrap().get("data").is_some());

    let fees = json!({
        "1": 0
    }).to_string();

    utils::fees::set_fees(pool_handle, wallet.handle, payment_method, &fees, &dids);
}

#[test]
#[ignore]
pub fn build_and_submit_nym_with_fees_and_get_utxo() {
    sovtoken::api::sovtoken_init();
    let payment_method = sovtoken::api::PAYMENT_METHOD_NAME;
    let pc_str = utils::pool::create_pool_config();
    let pool_config = Some(pc_str.as_str());
    indy::pool::Pool::set_protocol_version(2).unwrap();

    let pool_name = utils::pool::create_pool_ledger(pool_config);
    let pool_handle = indy::pool::Pool::open_ledger(&pool_name, None).unwrap();
    let wallet = utils::wallet::Wallet::new(&pool_name);

    let (did_trustee, _) = indy::did::Did::new(wallet.handle, &json!({"seed":"000000000000000000000000Trustee1"}).to_string()).unwrap();

    let pa1 = indy::payments::Payment::create_payment_address(wallet.handle, payment_method, "{}").unwrap();

    let mut mint_cfg = HashMap::new();
    mint_cfg.insert(pa1.clone(), 10);

    utils::mint::mint_tokens(mint_cfg, pool_handle, wallet.handle, &did_trustee).unwrap();

    let (req, method) = indy::payments::Payment::build_get_utxo_request(wallet.handle, &did_trustee, &pa1).unwrap();
    let res = indy::ledger::Ledger::sign_and_submit_request(pool_handle, wallet.handle, &did_trustee, &req).unwrap();
    let res = indy::payments::Payment::parse_get_utxo_response(&method, &res).unwrap();

    let res_parsed: serde_json::Value = serde_json::from_str(&res).unwrap();
    let utxos = res_parsed.as_array().unwrap();
    let value = utxos.get(0).unwrap().as_object().unwrap();
    let utxo = value.get("txo").unwrap().as_str().unwrap();

    let inputs = json!([utxo]).to_string();
    let outputs = json!([{
        "paymentAddress": pa1,
        "amount": 9
    }]).to_string();

    let fees = json!({
        "1": 1
    }).to_string();

    let dids = vec![did_trustee.as_str()];

    utils::fees::set_fees(pool_handle, wallet.handle, payment_method, &fees, &dids);

    let (did_new, verkey_new) = indy::did::Did::new(wallet.handle, "{}").unwrap();

    let nym_req = indy::ledger::Ledger::build_nym_request(&did_trustee, &did_new,  Some(&verkey_new), None, None).unwrap();
    let nym_req_signed = indy::ledger::Ledger::sign_request(wallet.handle, &did_trustee, &nym_req).unwrap();
    let (nym_req_with_fees, pm) = indy::payments::Payment::add_request_fees(wallet.handle, &did_trustee, &nym_req_signed, &inputs, &outputs).unwrap();
    let nym_resp = indy::ledger::Ledger::submit_request(pool_handle, &nym_req_with_fees).unwrap();
    let parsed_resp = indy::payments::Payment::parse_response_with_fees(&pm, &nym_resp).unwrap();

    let parsed_resp_json: Vec<HashMap<String, serde_json::Value>> = serde_json::from_str(&parsed_resp).unwrap();
    assert_eq!(parsed_resp_json.len(), 1);
    assert_eq!(parsed_resp_json[0].get("amount").unwrap().as_u64().unwrap(), 9);
    assert_eq!(parsed_resp_json[0].get("paymentAddress").unwrap().as_str().unwrap(), pa1);

    let fees = json!({
        "1": 0
    }).to_string();

    utils::fees::set_fees(pool_handle, wallet.handle, payment_method, &fees, &dids);

    let (req, method) = indy::payments::Payment::build_get_utxo_request(wallet.handle, &did_trustee, &pa1).unwrap();
    let res = indy::ledger::Ledger::sign_and_submit_request(pool_handle, wallet.handle, &did_trustee, &req).unwrap();
    let res = indy::payments::Payment::parse_get_utxo_response(&method, &res).unwrap();

    let res_parsed: serde_json::Value = serde_json::from_str(&res).unwrap();
    let utxos = res_parsed.as_array().unwrap();
    assert_eq!(utxos.len(), 1);
    let value = utxos.get(0).unwrap().as_object().unwrap();
    assert_eq!(value.get("paymentAddress").unwrap().as_str().unwrap(), pa1);
    assert_eq!(value.get("amount").unwrap().as_u64().unwrap(), 9);
}