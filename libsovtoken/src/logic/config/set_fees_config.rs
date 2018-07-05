/*!
    Provides structures for the [`build_set_txn_fees_handler`].

    [`build_set_txn_fees_handler`]: ../../../api/fn.build_set_txn_fees_handler.html
 */

use logic::request::Request;
use logic::did::Did;
use std::collections::HashMap;
use std::fmt;
use std::error::Error;
use utils::constants::txn_types::SET_FEES;
use logic::type_aliases::TokenAmount;
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
pub type SetFeesMap = HashMap<String, TokenAmount>;

/**
    Struct for [`build_set_txn_fees_handler`] request.

    Can build a Request<SetFees> which can be serialized into the request json.

    ```
        use std::collections::HashMap;
        use sovtoken::utils::constants::txn_types;
        use sovtoken::logic::did::Did;
        use sovtoken::logic::config::set_fees_config::{
            SetFees,
            SetFeesError,
        };

        let mut fees = HashMap::new();
        fees.insert(String::from(txn_types::XFER_PUBLIC), 10);
        fees.insert(String::from("15"), 3);
        let identifier = String::from("hgrhyNXqW4KNTz4wwiV8v");
        let did = Did::new(&identifier).validate().unwrap();
        let set_fees = SetFees::new(fees).validate().unwrap();
        let set_fees_request = set_fees.as_request(did);
        let json_pointer = set_fees_request.serialize_to_pointer().unwrap();
    ```

    [`build_set_txn_fees_handler`]: ../../../api/fn.build_set_txn_fees_handler.html
 */
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct SetFees {
    #[serde(rename = "type")]
    txn_type: &'static str,
    fees: SetFeesMap,
}

impl SetFees {

    /**
        Create a new [`SetFees`] struct.

        [`SetFees`]: ./struct.SetFees.html
    */
    pub fn new(fees: SetFeesMap) -> SetFees {
        return SetFees {
            txn_type: SET_FEES,
            fees,
        };
    }


    /**
        Transform `self` to a [`Request`] struct.

        [`Request`]: ../../request/struct.Request.html
    */
    // TODO: Remove identifier, this is temporary, just to get around the current incorrect way
    // of signing and being consistent with MINT.
    // More details here https://docs.google.com/document/d/15m3XPEUfwhI5GPWh3kuMj6rML52ydWTLsBiurHKfmnU/edit
    pub fn as_request(self, identifier: Did) -> Request<SetFees> {
        return Request::new(self, Some(String::from(identifier)));
    }

    /**
        Validate `self.fees`.

        Checks `self.fees` is not empty and the keys are string
        integers.

        ## Examples

        #### Empty Fees
        Returns a [`SetFeesError::Empty`].
        ```
            use std::collections::HashMap;
            use sovtoken::logic::config::set_fees_config::{
                SetFees,
                SetFeesError,
            };

            let fees = HashMap::new();
            let set_fees = SetFees::new(fees);
            let validated = set_fees.validate();

            assert_eq!(SetFeesError::Empty, validated.unwrap_err());
        ```

        #### Fees with non-string-integer keys
        Returns a [`SetFeesError::KeyNotInteger`].
        ```
            use std::collections::HashMap;
            use sovtoken::logic::config::set_fees_config::{
                SetFees,
                SetFeesError,
            };

            let mut fees = HashMap::new();
            // Key should be "10001"
            let key = String::from("XFER_PUBLIC");
            fees.insert(key.clone(), 10);
            let set_fees = SetFees::new(fees);
            let validated = set_fees.validate();

            assert_eq!(SetFeesError::KeyNotInteger(key), validated.unwrap_err())
        ```

        #### Valid Fees
        ```
            use std::collections::HashMap;
            use sovtoken::utils::constants::txn_types;
            use sovtoken::logic::config::set_fees_config::{
                SetFees,
                SetFeesError,
            };

            let mut fees = HashMap::new();
            fees.insert(String::from(txn_types::XFER_PUBLIC), 10);
            fees.insert(String::from("15"), 3);
            let set_fees = SetFees::new(fees);
            let validated = set_fees.validate();

            assert!(validated.is_ok());
        ```

        [`SetFeesError::Empty`]: ./enum.SetFeesError.html#variant.Empty
        [`SetFeesError::KeyNotInteger`]: ./enum.SetFeesError.html#variant.KeyNotInteger
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

    [`SetFees::validate`]: ./struct.SetFees.html#method.validate
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

        let hash_map: SetFeesMap = serde_json::from_value(set_fees_json).unwrap();
        let set_fees = SetFees::new(hash_map).validate().unwrap();
        let identifier = String::from("hgrhyNXqW4KNTz4wwiV8v");
        let did = Did::new(&identifier).validate().unwrap();
        let request = set_fees.as_request(did);
        let fees_from_request = serde_json::to_value(&request.operation.fees).unwrap();
        assert_eq!(expected, fees_from_request)
    }
}