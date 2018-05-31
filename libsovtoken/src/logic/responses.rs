//! Contains the response types

use serde_json;

use super::input::Input;
use super::output::Output;
use utils::json_conversion::JsonDeserialize;
use utils::random::rand_req_id;

/**
    enumeration matches values for the op field in json
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
pub enum ResponseOperations {
    REPLY,
    REJECT,
    REQNACK,
}


#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReadRequestResult {
    pub identifier : String,
    pub outputs: Option<Vec<Output>>,
    pub req_id : u32,
    pub seq_no: u32,
    pub txn_time: u32,
    #[serde(rename = "type")]
    pub txn_type : String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseReadRequest {
    pub op : ResponseOperations,
    pub result: ReadRequestResult,
}

impl ResponseReadRequest {
    /**
    */
    pub fn deserialize_from_cstring(json : &str) -> Result<ResponseReadRequest, serde_json::Error> {
        let serialized = JsonDeserialize::from_json(json)?;
        return Ok(serialized);
    }
}

/**
    Represents the success response object

    We chose to separate success from error so that its easier to understand the resulting
    json
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseSuccessResult {
    pub audit_path: Vec<String>,
    pub extra : String,
    pub identifier : String,
    pub inputs: Option<Vec<Input>>,
    pub outputs: Option<Vec<Output>>,
    pub req_id : u32,
    pub root_hash : String,
    pub seq_no: u32,
    pub signature: String,
    pub signatures: Option<Vec<String>>,
    pub txn_time: u32,
    #[serde(rename = "type")]
    pub txn_type : String,

}

impl ResponseSuccessResult {
    pub fn new (identifier: String) -> Self {
        let req_id = rand_req_id();

        return ResponseSuccessResult {
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

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseSuccess {
    pub op : ResponseOperations,
    pub result: ResponseSuccessResult,
}

impl ResponseSuccess
{
    pub fn new(op : ResponseOperations, identifier: String) -> Self {

        return ResponseSuccess {
            op,
            result : ResponseSuccessResult::new(identifier),
        };
    }
}

/**
    Contains fields that define error responses
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseError {
    pub identifier : String,
    pub op : ResponseOperations,
    pub req_id : u32,
    pub reason : String
}

impl ResponseError
{
    pub fn new(op : ResponseOperations, identifier: String) -> Self {
        let req_id = rand_req_id();
        return ResponseError {
            op,
            req_id,
            identifier,
            reason : "".to_string(),
        }
    }
}

