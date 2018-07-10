extern crate rust_indy_sdk as indy;
extern crate serde_json;

pub fn get_first_utxo_for_payment_address(wallet_handle: i32, pool_handle: i32, did: &str, pa: &str) -> (String, u64, String) {
    let (req, method) = indy::payments::Payment::build_get_utxo_request(wallet_handle, did, pa).unwrap();
    let res = indy::ledger::Ledger::sign_and_submit_request(pool_handle, wallet_handle, did, &req).unwrap();
    let res = indy::payments::Payment::parse_get_utxo_response(&method, &res).unwrap();

    let res_parsed: serde_json::Value = serde_json::from_str(&res).unwrap();
    let utxos = res_parsed.as_array().unwrap();
    let value = utxos.get(0).unwrap().as_object().unwrap();
    (
        value.get("txo").unwrap().as_str().unwrap().to_string(),
        value.get("amount").unwrap().as_u64().unwrap(),
        value.get("paymentAddress").unwrap().as_str().unwrap().to_string()
    )
}