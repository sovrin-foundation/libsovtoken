use sovtoken::utils::constants::general::PAYMENT_METHOD_NAME;
use sovtoken::logic::config::set_fees_config::SetFees;
use sovtoken::logic::request::Request;
use utils::wallet::Wallet;

use indy::future::Future;

use std::sync::{Once, ONCE_INIT};
use std::sync::Mutex;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

lazy_static! {
    static ref AUTH_RULES: Mutex<HashMap<String, Vec<AuthRule>>> = Default::default();
}

#[derive(Debug)]
struct AuthRule {
    action: String,
    txn_type: String,
    field: String,
    old_value: Option<String>,
    new_value: Option<String>,
    constraint: serde_json::Value
}

pub fn set_fees(pool_handle: i32, wallet_handle: i32, payment_method: &str, fees: &str, dids: &Vec<&str>, submitter_did: Option<&str>) {
    let set_fees_req = ::indy::payments::build_set_txn_fees_req(wallet_handle, submitter_did, payment_method, &fees).wait().unwrap();
    let set_fees_req = Request::<SetFees>::multi_sign_request(wallet_handle, &set_fees_req, dids.to_vec()).unwrap();
    ::indy::ledger::submit_request(pool_handle, &set_fees_req).wait().unwrap();

    let txn_fees: HashMap<String, String> =
        ::serde_json::from_str::<HashMap<String, u64>>(fees).unwrap()
            .iter_mut()
            .map(|(k, _v)| (k.to_string(), k.to_string()))
            .collect();

    set_auth_rules_fee(pool_handle, wallet_handle, &submitter_did.unwrap(), &json!(txn_fees).to_string());
}

// Helper to set fee alias for auth rules
pub fn set_auth_rules_fee(pool_handle: i32, wallet_handle: i32, submitter_did: &str, txn_fees: &str) {
    _get_default_ledger_auth_rules(pool_handle);

    let auth_rules = AUTH_RULES.lock().unwrap();

    let fees: HashMap<String, String> = ::serde_json::from_str(txn_fees).unwrap();

    let mut responses: Vec<Box<Future<Item=String, Error=::indy::IndyError>>> = Vec::new();

    for (txn_, fee_alias) in fees {
        if let Some(rules) = auth_rules.get(&txn_) {
            for auth_rule in rules {
                let mut constraint: ::serde_json::Value = auth_rule.constraint.clone();
                if !constraint.as_object().map(::serde_json::Map::is_empty).unwrap_or(true) {
                    _set_fee_to_constraint(&mut constraint, &fee_alias);
                    responses.push(_send_auth_rule(pool_handle, wallet_handle, submitter_did, auth_rule, &constraint));
                }
            }
        }
    }

    let _response = responses
        .into_iter()
        .map(|response| _check_auth_rule_responses(response))
        .collect::<Vec<()>>();
}

fn _send_auth_rule(pool_handle: i32, wallet_handle: i32, submitter_did: &str,
                   auth_rule: &AuthRule, constraint: &serde_json::Value) -> Box<Future<Item=String, Error=::indy::IndyError>> {
    let auth_rule_request = ::indy::ledger::build_auth_rule_request(submitter_did,
                                                                    &auth_rule.txn_type,
                                                                    &auth_rule.action,
                                                                    &auth_rule.field,
                                                                    auth_rule.old_value.as_ref().map(String::as_str),
                                                                    auth_rule.new_value.as_ref().map(String::as_str),
                                                                    &constraint.to_string(),
    ).wait().unwrap();

    ::indy::ledger::sign_and_submit_request(pool_handle, wallet_handle, submitter_did, &auth_rule_request)
}

fn _check_auth_rule_responses(response: Box<Future<Item=String, Error=::indy::IndyError>>) {
    let response = response.wait().unwrap();
    let response: serde_json::Value = ::serde_json::from_str(&response).unwrap();
    assert_eq!("REPLY", response["op"].as_str().unwrap());
}

fn _get_default_ledger_auth_rules(pool_handle: i32) {
    lazy_static! {
        static ref GET_DEFAULT_AUTH_CONSTRAINTS: Once = ONCE_INIT;

    }

    GET_DEFAULT_AUTH_CONSTRAINTS.call_once(|| {
        let get_auth_rule_request = ::indy::ledger::build_get_auth_rule_request(None, None, None, None, None, None).wait().unwrap();
        let get_auth_rule_response = ::indy::ledger::submit_request(pool_handle, &get_auth_rule_request).wait().unwrap();
        let mut get_auth_rule_response: serde_json::Value = ::serde_json::from_str(&get_auth_rule_response).unwrap();

        let constraints = get_auth_rule_response["result"]["data"].as_object_mut().unwrap();

        for (constraint_id, constraint) in constraints.iter_mut() {
            let parts: Vec<&str> = constraint_id.split("--").collect();

            let txn_type = parts[0].to_string();
            let action = parts[1].to_string();
            let field = parts[2].to_string();
            let old_value = if action == "ADD" { None } else { Some(parts[3].to_string()) };
            let new_value = if parts[4] == "" { None } else { Some(parts[4].to_string()) };

            let mut map = AUTH_RULES.lock().unwrap();

            let rule = AuthRule { action, txn_type: txn_type.clone(), field, old_value, new_value, constraint: constraint.clone() };

            match map.entry(txn_type) {
                Entry::Occupied(rules) => {
                    let &mut ref mut rules = rules.into_mut();
                    rules.push(rule);
                }
                Entry::Vacant(rules) => {
                    rules.insert(vec![rule]);
                }
            };
        }
    })
}

fn _set_fee_to_constraint(constraint: &mut serde_json::Value, fee_alias: &str) {
    match constraint["constraint_id"].as_str().unwrap() {
        "ROLE" => {
            constraint["metadata"]["fees"] = json!(fee_alias);
        }
        "OR" | "AND" => {
            for mut constraint in constraint["auth_constraints"].as_array_mut().unwrap() {
                _set_fee_to_constraint(&mut constraint, fee_alias)
            }
        }
        _ => { panic!() }
    }
}

pub fn get_fees(wallet: &Wallet, pool_handle: i32, submitter_did: Option<&str>) -> String {
    let get_fees_req = ::indy::payments::build_get_txn_fees_req(
        wallet.handle,
        submitter_did,
        PAYMENT_METHOD_NAME
    ).wait().unwrap();
    let result = ::indy::ledger::submit_request(pool_handle, &get_fees_req).wait().unwrap();
    ::indy::payments::parse_get_txn_fees_response(PAYMENT_METHOD_NAME, &result).wait().unwrap()
}
