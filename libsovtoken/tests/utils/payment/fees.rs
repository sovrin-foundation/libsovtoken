use sovtoken::utils::constants::general::PAYMENT_METHOD_NAME;
use sovtoken::logic::config::set_fees_config::SetFees;
use sovtoken::logic::request::Request;
use utils::wallet::Wallet;

use indy::future::Future;

use std::sync::{Once, ONCE_INIT};
use std::sync::Mutex;
use std::collections::HashMap;

/**
Structure for parsing GET_AUTH_RULE response
 # parameters
    result - the payload containing data relevant to the GET_AUTH_RULE transaction
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAuthRuleResponse {
    pub result: GetAuthRuleResult
}

/**
   Structure of the result value within the GAT_AUTH_RULE response
    # parameters
   identifier - The DID this request was submitted from
   req_id - Unique ID number of the request with transaction
   txn_type - the type of transaction that was submitted
   data - A key:value map with the action id as the key and the auth rule as the value
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAuthRuleResult {
    pub identifier: String,
    pub req_id: u64,
    // This is to change the json key to adhear to the functionality on ledger
    #[serde(rename = "type")]
    pub txn_type: String,
    pub data: Vec<AuthRule>,
}

/**
   Enum of the constraint type within the GAT_AUTH_RULE result data
    # parameters
   Role - The final constraint
   Combination - Combine multiple constraints all of them must be met
   Forbidden - action is forbidden
*/
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "constraint_id")]
pub enum Constraint {
    #[serde(rename = "OR")]
    OrConstraint(CombinationConstraint),
    #[serde(rename = "AND")]
    AndConstraint(CombinationConstraint),
    #[serde(rename = "ROLE")]
    RoleConstraint(RoleConstraint),
    #[serde(rename = "FORBIDDEN")]
    ForbiddenConstraint(ForbiddenConstraint),
}

/**
   The final constraint
    # parameters
   sig_count - The number of signatures required to execution action
   role - The role which the user must have to execute the action.
   metadata -  An additional parameters of the constraint (contains transaction FEE cost).
   need_to_be_owner - The flag specifying if a user must be an owner of the transaction.
*/
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoleConstraint {
    pub sig_count: Option<u32>,
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_to_be_owner: Option<bool>,
}

/**
   The empty constraint means that action is forbidden
*/
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ForbiddenConstraint {}

/**
   The constraint metadata
    # parameters
   fees - The action cost
*/
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    pub fees: Option<String>,
}

/**
   Combine multiple constraints
    # parameters
   auth_constraints - The type of the combination
*/
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CombinationConstraint {
    pub auth_constraints: Vec<Constraint>
}

/* Map contains default Auth Rules set on the Ledger*/
lazy_static! {
        static ref AUTH_RULES: Mutex<Vec<AuthRule>> = Default::default();
    }

/* Helper structure to store auth rule set on the Ledger */
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthRule {
    auth_action: String,
    auth_type: String,
    field: String,
    old_value: Option<String>,
    new_value: Option<String>,
    constraint: Constraint
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
        for auth_rule in auth_rules.iter() {
            if auth_rule.auth_type == txn_ {
                let mut constraint = auth_rule.constraint.clone();
                _set_fee_to_constraint(&mut constraint, &fee_alias);

                match constraint {
                    Constraint::ForbiddenConstraint(_) => {}
                    mut constraint @ _ => {
                        responses.push(_send_auth_rule(pool_handle, wallet_handle, submitter_did, auth_rule, &constraint));
                    }
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
                   auth_rule: &AuthRule, constraint: &Constraint) -> Box<Future<Item=String, Error=::indy::IndyError>> {
    let constraint_json = ::serde_json::to_string(&constraint).unwrap();

    let auth_rule_request = ::indy::ledger::build_auth_rule_request(submitter_did,
                                                                    &auth_rule.auth_type,
                                                                    &auth_rule.auth_action,
                                                                    &auth_rule.field,
                                                                    auth_rule.old_value.as_ref().map(String::as_str),
                                                                    auth_rule.new_value.as_ref().map(String::as_str),
                                                                    &constraint_json,
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

        let response: GetAuthRuleResponse = ::serde_json::from_str(&get_auth_rule_response).unwrap();

        let mut auth_rules = AUTH_RULES.lock().unwrap();

        *auth_rules = response.result.data;
    })
}

fn _set_fee_to_constraint(constraint: &mut Constraint, fee_alias: &str) {
    match constraint {
        Constraint::RoleConstraint(constraint) => {
            constraint.metadata.as_mut().map(|meta| meta.fees = Some(fee_alias.to_string()));
        }
        Constraint::OrConstraint(constraint) | Constraint::AndConstraint(constraint) => {
            for mut constraint in constraint.auth_constraints.iter_mut() {
                _set_fee_to_constraint(&mut constraint, fee_alias)
            }
        }
        Constraint::ForbiddenConstraint(_) => {}
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
