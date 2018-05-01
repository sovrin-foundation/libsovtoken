//
//  Implementations for Serde Json serialization/deserialization
//

use serde::{Serialize, Deserialize};
use serde_json::Error;
use serde_json::from_str;

//
// given a json formatted string, return object of given type
//
pub trait JsonDeserialize<'a>: Deserialize<'a> {
    fn from_json(json: &'a str) -> Result<Self, Error> {
        from_str(json)
    }
}

impl<'a, T: Deserialize<'a> > JsonDeserialize<'a> for T { }

