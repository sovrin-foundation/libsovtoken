/*!
    Provides structures for the [`build_fees_txn_handler`].

    [`build_fees_txn_handler`]: sovtoken::logic::api::build_fees_txn_handler

    TODO: Links need to be updated so they actually work.
 */

use logic::did::Did;
use logic::request::Request;
use std::collections::HashMap;
use std::fmt;
use std::error::Error;
use utils::constants::txn_types::SET_FEES;

/**
    Hashmap for the set_fees json.

    The key is an integer string.

    ## Example
    ```
        use sovtoken::logic::config::set_fees_config::SetFeesMap;
        use std::collections::HashMap;
        let mut set_fees_map : SetFeesMap = HashMap::new();
        set_fees_map.insert(String::from("1002"), 10);
    ```
*/
pub type SetFeesMap = HashMap<String, u64>;

/**
 *  Struct for [`build_fees_txn_handler`].
 *
 *  [`build_fees_txn_handler`]: sovtoken::logic::api::build_fees_txn_handler
 */
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct SetFees {
    #[serde(rename = "type")]
    txn_type: &'static str,
    fees: SetFeesMap,
}

impl SetFees {

    /**
        Creates a new [`SetFees`] struct.
    */
    pub fn new(fees: SetFeesMap) -> SetFees {
        return SetFees {
            txn_type: SET_FEES,
            fees,
        };
    }


    /**
        Transforms the [`SetFees`] to a [`Request`] struct.

        [`Request`]: sovtoken::logic::request::Request
    */
    pub fn as_request(self, identifier: Did) -> Request<SetFees> {
        return Request::new(self, String::from(identifier));
    }

    /**
        Validates a [`SetFees`].

        Checks the [`SetFees.map`] is not empty and the keys are string
        integers.
    */
    pub fn validate(self) -> Result<Self, SetFeesError> {
        if self.fees.is_empty() {
            return Err(SetFeesError::Empty);
        }

        {
            let key_not_integer = self.fees
                .keys()
                .find(|&key| key.parse::<u32>().is_err());
            
            if let Some(key) = key_not_integer {
                return Err(SetFeesError::KeyNotInteger(key.to_owned()));
            }
        }
    
        return Ok(self);
    }

}

/**
    Enum which holds possible errors for [`SetFees::validate`].

    ### Includes
    - `SetFeesError::Empty`
    - `SetFeesError::KeyNotInteger(&str)`

    [`SetFees::validate`]: SetFees::validate
*/
#[derive(Debug, PartialEq, Eq)]
pub enum SetFeesError {
    Empty,
    KeyNotInteger(String)
}

impl fmt::Display for SetFeesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self);
    }
}

impl Error for SetFeesError {
    fn description(&self) -> &str {
        match self {
            &SetFeesError::Empty => "Set fees was empty.",
            &SetFeesError::KeyNotInteger(_) => "A Set fees key wasn't a integer string.",
        }
    }
}

#[cfg(test)]
mod fees_config_test {
    use super::*;
    use serde_json;
    use utils::random::rand_string;

    #[test]
    fn test_set_fees_map_value_string() {
        let set_fees_json = json!({
            "3": "10",
            "1000": "12"
        });
        let hash_map: Result<SetFeesMap, _> = serde_json::from_value(set_fees_json);

        assert!(hash_map.is_err());
    }

    #[test]
    fn test_validation_empty_fees() {
        let set_fees_json = json!({});
        let hash_map: SetFeesMap = serde_json::from_value(set_fees_json).unwrap();
        let set_fees = SetFees::new(hash_map);
        assert_eq!(SetFeesError::Empty, set_fees.validate().unwrap_err());
    }

    #[test]
    fn test_validation_fees_key_not_string_integer() {
        let set_fees_json = json!({
            "XFER_PUBLIC": 10,
        });
        let expected = SetFeesError::KeyNotInteger(String::from("XFER_PUBLIC"));

        let hash_map: SetFeesMap = serde_json::from_value(set_fees_json).unwrap();
        let set_fees = SetFees::new(hash_map);
        assert_eq!(expected, set_fees.validate().unwrap_err());
    }

    #[test]
    fn create_valid_set_fees_request() {
        let set_fees_json = json!({
            "3": 10,
            "1000": 12
        });
        let expected = set_fees_json.clone();
        let rand_identifier = rand_string(21);
        let identifier = Did::new(&rand_identifier);

        let hash_map: SetFeesMap = serde_json::from_value(set_fees_json).unwrap();
        let set_fees = SetFees::new(hash_map).validate().unwrap();
        let request = set_fees.as_request(identifier);
        let fees_from_request = serde_json::to_value(&request.operation.fees).unwrap();
        assert_eq!(expected, fees_from_request)
    }
}