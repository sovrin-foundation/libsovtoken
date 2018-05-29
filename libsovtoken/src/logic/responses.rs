//! Contains the response types

use serde::{de, ser, ser::{SerializeTuple}, Deserialize, Serialize};
use super::input::Input;
use super::output::Output;
use utils::random::rand_req_id;

/**
    enumeration matches values for the op field in json
*/
pub enum ReponseOperations {
    REPLY,
    REJECT,
    REQNACK,
}

/**
    Represents the success response object

    We chose to separate success from error so that its easier to understand the resulting
    json
*/

pub struct Result {
    pub audit_path: Vec<String>,
    pub extra : String,
    pub identifier : String,
    pub inputs: Option<Vec<Input>>,
    pub outputs: Vec<Output>,
    pub req_id : u32,
    pub root_hash : String,
    pub seq_no: u32,
    pub signature: String,
    pub signatures: Option<Vec<String>>,
    pub txn_time: u32,
    #[serde(rename = "type")]
    pub txn_type : String,

}

impl Result {
    pub fn new (identifier: String) -> Self {
        let req_id = rand_req_id();

        return Result {
            audit_path: Vec::new(),
            extra: "".to_string(),
            identifier,
            inputs: None,
            outputs: None,
            req_id,
            root_hash: "".to_string(),
            seq_no: 0,
            signature: "".to_string(),
            signatures: None,
            txn_time: 0,
            txn_type: "".to_string()
        };
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseSuccess {
    pub op : ReponseOperations,
    pub result: Result,
}

impl ResponseSuccess
{
    pub fn new(op : ReponseOperations, identifier: String) -> Self {

        return ResponseSuccess {
            op,
            result : Result::new(identifier),
        };
    }
}

/**
    Contains fields that define error responses
*/
#[derive(Debug, Eq, PartialEq, Clone)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseError {
    pub identifier : String,
    pub op : ReponseOperations,
    pub req_id : u32,
    pub reason : String
}


impl ResponseError
{
    pub fn new(op : ReponseOperations, identifier: String) -> Self {
        let req_id = rand_req_id();
        return ResponseError {
            op,
            req_id,
            identifier,
            reason : "".to_string(),
        }
    }
}