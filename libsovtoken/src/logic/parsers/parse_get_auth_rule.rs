//!
use std::collections::HashMap;

use serde_json;
use ErrorCode;

use logic::parsers::common::ResponseOperations;
use utils::json_conversion::JsonDeserialize;
use logic::type_aliases::{ProtocolVersion, TokenAmount, ReqId};

use std::collections::HashSet;

/**
    Structure for parsing GET_AUTH_RULE response

    # parameters
    op - the operation type received
    protocol_version - the protocol version of the format of the transaction
    result - the payload containing data relevant to the GET_AUTH_RULE transaction
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParseGetAuthRuleResponse {
    pub op: ResponseOperations,
    pub protocol_version: Option<ProtocolVersion>,
    pub result: ParseGetAuthRuleResult
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
pub struct ParseGetAuthRuleResult {
    pub identifier: String,
    pub req_id: ReqId,
    // This is to change the json key to adhear to the functionality on ledger
    #[serde(rename = "type")]
    pub txn_type: String,
    pub data: HashMap<String, Constraint>,
}

/**
    Enum of the constraint type within the GAT_AUTH_RULE result data

    # parameters
    ROLE - The final constraint
    AND - Combine multiple constraints all of them must be met
    OR - Combine multiple constraints any of them must be met
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "constraint_id")]
pub enum Constraint {
    #[serde(rename = "OR")]
    OrConstraint(CombinationConstraint),
    #[serde(rename = "AND")]
    AndConstraint(CombinationConstraint),
    #[serde(rename = "ROLE")]
    RoleConstraint(RoleConstraint),
}

/**
    The final constraint

    # parameters
    sig_count - The number of signatures required to execution action
    role - The role which the user must have to execute the action.
    metadata -  An additional parameters of the constraint (contains transaction FEE cost).
    need_to_be_owner - The flag specifying if a user must be an owner of the transaction.
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct RoleConstraint {
    pub sig_count: Option<u32>,
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_to_be_owner: Option<bool>,
}

/**
    The constraint metadata

    # parameters
    fees - The action cost
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub fees: Option<TokenAmount>,
}

/**
    Combine multiple constraints

    # parameters
    auth_constraints - The type of the combination
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct CombinationConstraint {
    pub auth_constraints: Vec<Constraint>
}


pub fn parse_fees_from_get_auth_rule_response(response: String) -> Result<String, ErrorCode> {
    trace!("logic::parsers::parse_fees_from_get_txn_fees_response >> response: {:?}", response);
    let get_auth_rule_response: ParseGetAuthRuleResponse =
        ParseGetAuthRuleResponse::from_json(&response).map_err(map_err_err!()).map_err(|_| ErrorCode::CommonInvalidStructure)?;
    let fees = collect_fees_from_auth_rules(&get_auth_rule_response.result.data).map_err(map_err_err!())?;
    let res = serde_json::to_string(&fees).map_err(map_err_err!()).map_err(|_| ErrorCode::CommonInvalidStructure);
    trace!("logic::parsers::parse_fees_from_get_txn_fees_response << result: {:?}", res);
    return res;
}

fn collect_fees_from_auth_rules(rules: &HashMap<String, Constraint>) -> Result<HashMap<String, Option<TokenAmount>>, ErrorCode> {
    let mut fees: HashMap<String, Option<TokenAmount>> = HashMap::new();

    for (constraint_id, constraint) in rules.iter() {
        let txn_type = extract_txn_type(&constraint_id.as_str())?;
        let txn_fee = extract_fee(constraint, None)?;

        let contains_this_type = fees.contains_key(&txn_type);
        let current_fee = fees.get(&txn_type).and_then(|f| f.clone());

        match (current_fee, txn_fee) {
            (None, None) if !contains_this_type => {
                fees.insert(txn_type, None);
            }
            (None, None) => {}
            (None, Some(fee)) if contains_this_type => {
                error!("Fee values are different for the same transaction. txn_type: {}, current fee: None, new fee: {}", txn_type, fee);
                return Err(ErrorCode::CommonInvalidStructure);
            }
            (None, Some(fee)) => {
                fees.insert(txn_type, Some(fee));
            }
            (Some(fee), None) => {
                error!("Fee values are different for the same transaction. txn_type: {}, current fee: {}, new fee: None", txn_type, fee);
                return Err(ErrorCode::CommonInvalidStructure);
            }
            (Some(amount), Some(fee)) => {
                if amount != fee {
                    error!("Fee values are different for the same transaction. txn_type: {}, current fee: {}, new fee: {}", txn_type, amount, fee);
                    return Err(ErrorCode::CommonInvalidStructure);
                }
            }
        }
    }

    fees.retain(|&_, v| v.is_some());

    Ok(fees)
}

fn extract_txn_type(constraint_id: &str) -> Result<String, ErrorCode> {
    constraint_id.split("--").collect::<Vec<&str>>().get(0)
        .map(|a| a.to_string())
        .ok_or(ErrorCode::CommonInvalidStructure)
}

fn extract_fee(constraint: &Constraint, token_amount: Option<TokenAmount>) -> Result<Option<TokenAmount>, ErrorCode> {
    let fee = match constraint {
        Constraint::RoleConstraint(constraint) => {
            constraint.metadata.as_ref().and_then(|metadata| metadata.fees)
        }
        Constraint::AndConstraint(constraint) | Constraint::OrConstraint(constraint) => {
            let fees: HashSet<Option<TokenAmount>> = constraint.auth_constraints
                .iter()
                .map(|constraint| extract_fee(constraint, token_amount))
                .collect::<Result<HashSet<Option<TokenAmount>>, ErrorCode>>()?;
            if fees.len() != 1 {
                return Err(ErrorCode::CommonInvalidStructure);
            }

            fees.into_iter().next().unwrap()
        }
    };

    match (token_amount, fee) {
        (None, None) => Ok(None),
        (Some(amount), None) => Ok(Some(amount)),
        (None, Some(fee)) => Ok(Some(fee)),
        (Some(amount), Some(fee)) => {
            if amount != fee {
                error!("Fee values are different. current fee: {}, new fee: {}", amount, fee);
                return Err(ErrorCode::CommonInvalidStructure);
            } else {
                Ok(Some(amount))
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[cfg(test)]
    mod parse_fees_responses_test {
        use super::{parse_fees_from_get_auth_rule_response, ErrorCode};
        use serde_json::Value;
        use serde_json;

        #[test]
        fn success_parse_fees_from_reply_response() {
            let get_auth_rule_response =
                r#"{
                "op":"REPLY",
                "result":{
                    "identifier":"LibindyDid111111111111",
                    "type":"121",
                    "data":{
                        "1--EDIT--role--201--0":{"sig_count":1,"constraint_id":"ROLE","role":"0","metadata":{"fees":100},"need_to_be_owner":false},
                        "1--EDIT--role----201":{"constraint_id":"OR","auth_constraints":[{"sig_count":1,"constraint_id":"ROLE","role":"2","metadata":{"fees":100},"need_to_be_owner":false}, {"sig_count":1,"constraint_id":"ROLE","role":"0","metadata":{"fees":100},"need_to_be_owner":false}]},
                        "1--ADD--role--*--0":{"sig_count":1,"constraint_id":"ROLE","role":"0","metadata":{"fees":100},"need_to_be_owner":false},
                        "1--ADD--role--*--2":{"sig_count":1,"constraint_id":"ROLE","role":"0","metadata":{"fees":100},"need_to_be_owner":false},
                        "0--ADD--services--*--['VALIDATOR']":{"sig_count":1,"constraint_id":"ROLE","role":"2","metadata":{"fees":200},"need_to_be_owner":true},
                        "0--EDIT--services--['VALIDATOR']--[]":{"constraint_id":"OR","auth_constraints":[{"sig_count":1,"constraint_id":"ROLE","role":"0","metadata":{"fees":200},"need_to_be_owner":false},{"sig_count":1,"constraint_id":"ROLE","role":"2","metadata":{"fees":200},"need_to_be_owner":true}]},
                        "119--EDIT--*--*--*":{"sig_count":1,"constraint_id":"ROLE","role":"0","metadata":{"fees":110},"need_to_be_owner":false}
                    },
                    "reqId":15550536
                }
            }"#;

            //setup output of method's data
            let fees_json: String = parse_fees_from_get_auth_rule_response(
                get_auth_rule_response.to_string()).unwrap();
            let parsed_fees_json: Value = serde_json::from_str(&fees_json).unwrap();

            //define and setup expected output from the function
            let expected_json: Value = serde_json::from_str(
                r#"{"0":200,"1":100,"119":110}"#).unwrap();

            //comparison
            assert_eq!(parsed_fees_json, expected_json, "The json objects don't match");
        }

        #[test]
        fn success_parse_fees_from_reply_response_no_fees_for_txn() {
            let get_auth_rule_response =
                r#"{
                "op":"REPLY",
                "result":{
                    "identifier":"LibindyDid111111111111",
                    "type":"121",
                    "data":{
                        "120--EDIT--*--*--*":{"sig_count":1,"constraint_id":"ROLE","role":"0","metadata":{},"need_to_be_owner":false}
                    },
                    "reqId":15550536
                }
            }"#;

            //setup output of method's data
            let fees_json: String = parse_fees_from_get_auth_rule_response(
                get_auth_rule_response.to_string()).unwrap();
            let parsed_fees_json: Value = serde_json::from_str(&fees_json).unwrap();

            //define and setup expected output from the function
            let expected_json: Value = serde_json::from_str(
                r#"{}"#).unwrap();

            //comparison
            assert_eq!(parsed_fees_json, expected_json, "The json objects don't match");
        }

        #[test]
        fn failure_parse_fees_from_reply_response_contained_different_fees() {
            let get_auth_rule_response =
                r#"{
                "op":"REPLY",
                "result":{
                    "identifier":"LibindyDid111111111111",
                    "type":"121",
                    "data":{
                        "1--EDIT--role--201--0":{"sig_count":1,"constraint_id":"ROLE","role":"0","metadata":{"fees":100},"need_to_be_owner":false},
                        "1--ADD--role--*--0":{"sig_count":1,"constraint_id":"ROLE","role":"0","metadata":{"fees":200},"need_to_be_owner":false},
                    },
                    "reqId":15550536
                }
            }"#;

            let err = parse_fees_from_get_auth_rule_response(
                get_auth_rule_response.to_string()).unwrap_err();

            //comparison
            assert_eq!(ErrorCode::CommonInvalidStructure, err);
        }

        #[test]
        fn failure_parse_fees_from_reply_response() {
            let invalid_json_response =
                r#"{
                "op": "REPLY",
                "result": {
                    "identifier": "6ouriXMZkLeHsuXrN1X1fd",
                    "reqId": 47660,
                    "type":"121",
                    "data": INVALID_JSON
                }
            }"#;

            //convert to Error
            let invalid_fees_json: Result<String, ErrorCode> = parse_fees_from_get_auth_rule_response(
                invalid_json_response.to_string());

            let json_error_bool: bool = invalid_fees_json.is_err();
            assert!(json_error_bool);
        }
    }

    #[cfg(test)]
    mod collect_fees_from_auth_rules {
        use super::*;

        fn _role_constraint(fees: Option<TokenAmount>) -> Constraint {
            Constraint::RoleConstraint(RoleConstraint {
                sig_count: Some(0),
                role: Some(String::new()),
                metadata: Some(Metadata { fees }),
                need_to_be_owner: None,
            })
        }

        #[test]
        fn collect_fees_from_auth_rules_works_for_unique_txn_types() {
            let mut rules: HashMap<String, Constraint> = HashMap::new();
            rules.insert("0--EDIT--client_ip--*--*".to_string(), _role_constraint(Some(200)));
            rules.insert("1--EDIT--role--201--0".to_string(), _role_constraint(Some(10)));
            rules.insert("113--ADD--*--*--*".to_string(), _role_constraint(Some(90)));
            rules.insert("114--EDIT--*--*--*".to_string(), _role_constraint(Some(110)));

            let mut expected_fees: HashMap<String, Option<TokenAmount>> = HashMap::new();
            expected_fees.insert("0".to_string(), Some(200));
            expected_fees.insert("1".to_string(), Some(10));
            expected_fees.insert("113".to_string(), Some(90));
            expected_fees.insert("114".to_string(), Some(110));

            let fees = collect_fees_from_auth_rules(&rules).unwrap();
            assert_eq!(expected_fees, fees);
        }

        #[test]
        fn collect_fees_from_auth_rules_works_for_repeatable_txn_types_with_same_fees() {
            let mut rules: HashMap<String, Constraint> = HashMap::new();
            rules.insert("0--EDIT--client_ip--*--*".to_string(), _role_constraint(Some(200)));
            rules.insert("0--EDIT--node_ip--*--*".to_string(), _role_constraint(Some(200)));
            rules.insert("0--EDIT--client_port--*--*".to_string(), _role_constraint(Some(200)));
            rules.insert("0--ADD--services--*--".to_string(), _role_constraint(Some(200)));
            rules.insert("1--EDIT--role--0--0".to_string(), _role_constraint(Some(10)));
            rules.insert("1--EDIT--role--201--0".to_string(), _role_constraint(Some(10)));
            rules.insert("1--EDIT--role--201--101".to_string(), _role_constraint(Some(10)));

            let mut expected_fees: HashMap<String, Option<TokenAmount>> = HashMap::new();
            expected_fees.insert("0".to_string(), Some(200));
            expected_fees.insert("1".to_string(), Some(10));

            let fees = collect_fees_from_auth_rules(&rules).unwrap();
            assert_eq!(expected_fees, fees);
        }

        #[test]
        fn collect_fees_from_auth_rules_works_for_repeatable_txn_types_with_different_fees() {
            let mut rules: HashMap<String, Constraint> = HashMap::new();
            rules.insert("0--EDIT--client_ip--*--*".to_string(), _role_constraint(Some(200)));
            rules.insert("0--EDIT--node_ip--*--*".to_string(), _role_constraint(Some(2)));

            let err = collect_fees_from_auth_rules(&rules).unwrap_err();
            assert_eq!(ErrorCode::CommonInvalidStructure, err);
        }

        #[test]
        fn collect_fees_from_auth_rules_works_for_repeatable_txn_types_with_missed_fees() {
            let mut rules: HashMap<String, Constraint> = HashMap::new();
            rules.insert("0--EDIT--client_ip--*--*".to_string(), _role_constraint(Some(200)));
            rules.insert("0--EDIT--node_ip--*--*".to_string(), _role_constraint(None));

            let err = collect_fees_from_auth_rules(&rules).unwrap_err();
            assert_eq!(ErrorCode::CommonInvalidStructure, err);
        }

        #[test]
        fn collect_fees_from_auth_rules_works_for_no_fees_set_for_txn() {
            let mut rules: HashMap<String, Constraint> = HashMap::new();
            rules.insert("0--EDIT--client_ip--*--*".to_string(), _role_constraint(None));
            rules.insert("0--EDIT--node_ip--*--*".to_string(), _role_constraint(None));
            rules.insert("113--ADD--*--*--*".to_string(), _role_constraint(Some(90)));
            rules.insert("114--EDIT--*--*--*".to_string(), _role_constraint(Some(110)));

            let mut expected_fees: HashMap<String, Option<TokenAmount>> = HashMap::new();
            expected_fees.insert("113".to_string(), Some(90));
            expected_fees.insert("114".to_string(), Some(110));

            let fees = collect_fees_from_auth_rules(&rules).unwrap();
            assert_eq!(expected_fees, fees);
        }
    }

    #[cfg(test)]
    mod extract_txn_type {
        use super::*;

        #[test]
        fn extract_txn_type_works() {
            let txn_type = extract_txn_type("1--EDIT--role--201--0").unwrap();
            assert_eq!("1".to_string(), txn_type);
        }
    }

    #[cfg(test)]
    mod extract_fee {
        use super::*;

        #[test]
        fn extract_fee_works_for_single_role_constraint() {
            let constraint = Constraint::RoleConstraint(RoleConstraint {
                sig_count: Some(0),
                role: Some(String::new()),
                metadata: Some(Metadata { fees: Some(10) }),
                need_to_be_owner: None,
            });

            let fee = extract_fee(&constraint, None).unwrap();
            assert_eq!(10, fee.unwrap());
        }

        #[test]
        fn extract_fee_works_for_combination_constraints() {
            let constraint = Constraint::AndConstraint(CombinationConstraint {
                auth_constraints: vec![
                    Constraint::RoleConstraint(RoleConstraint {
                        sig_count: Some(0),
                        role: Some(String::new()),
                        metadata: Some(Metadata { fees: Some(10) }),
                        need_to_be_owner: None,
                    }),
                    Constraint::RoleConstraint(RoleConstraint {
                        sig_count: Some(0),
                        role: Some(String::new()),
                        metadata: Some(Metadata { fees: Some(10) }),
                        need_to_be_owner: None,
                    })
                ]
            });

            let fee = extract_fee(&constraint, None).unwrap();
            assert_eq!(10, fee.unwrap());
        }

        #[test]
        fn extrac_fee_works_for_two_level_combination_constraints() {
            let constraint = Constraint::OrConstraint(CombinationConstraint {
                auth_constraints: vec![
                    Constraint::RoleConstraint(RoleConstraint {
                        sig_count: Some(0),
                        role: Some(String::new()),
                        metadata: Some(Metadata { fees: Some(10) }),
                        need_to_be_owner: None,
                    }),
                    Constraint::OrConstraint(CombinationConstraint {
                        auth_constraints: vec![
                            Constraint::RoleConstraint(RoleConstraint {
                                sig_count: Some(0),
                                role: Some(String::new()),
                                metadata: Some(Metadata { fees: Some(10) }),
                                need_to_be_owner: None,
                            }),
                            Constraint::RoleConstraint(RoleConstraint {
                                sig_count: Some(0),
                                role: Some(String::new()),
                                metadata: Some(Metadata { fees: Some(10) }),
                                need_to_be_owner: None,
                            })
                        ]
                    })
                ]
            });

            let fee = extract_fee(&constraint, None).unwrap();
            assert_eq!(10, fee.unwrap());
        }

        #[test]
        fn extract_fee_works_for_combination_constraints_with_different_fees() {
            let constraint = Constraint::AndConstraint(CombinationConstraint {
                auth_constraints: vec![
                    Constraint::RoleConstraint(RoleConstraint {
                        sig_count: Some(0),
                        role: Some(String::new()),
                        metadata: Some(Metadata { fees: Some(10) }),
                        need_to_be_owner: None,
                    }),
                    Constraint::RoleConstraint(RoleConstraint {
                        sig_count: Some(0),
                        role: Some(String::new()),
                        metadata: Some(Metadata { fees: Some(20) }),
                        need_to_be_owner: None,
                    })
                ]
            });

            let err = extract_fee(&constraint, None).unwrap_err();
            assert_eq!(ErrorCode::CommonInvalidStructure, err);
        }

        #[test]
        fn extract_fee_works_for_role_constraints_no_fee_set() {
            let constraint = Constraint::RoleConstraint(RoleConstraint {
                sig_count: Some(0),
                role: Some(String::new()),
                metadata: None,
                need_to_be_owner: None,
            });

            let fee = extract_fee(&constraint, None).unwrap();
            assert_eq!(None, fee);
        }

        #[test]
        fn extract_fee_works_for_combination_constraints_with_empty_fee_present() {
            let constraint = Constraint::AndConstraint(CombinationConstraint {
                auth_constraints: vec![
                    Constraint::RoleConstraint(RoleConstraint {
                        sig_count: Some(0),
                        role: Some(String::new()),
                        metadata: Some(Metadata { fees: Some(10) }),
                        need_to_be_owner: None,
                    }),
                    Constraint::RoleConstraint(RoleConstraint {
                        sig_count: Some(0),
                        role: Some(String::new()),
                        metadata: None,
                        need_to_be_owner: None,
                    })
                ]
            });

            let err = extract_fee(&constraint, None).unwrap_err();
            assert_eq!(ErrorCode::CommonInvalidStructure, err);
        }
    }
}