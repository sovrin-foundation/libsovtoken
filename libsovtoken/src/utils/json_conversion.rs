//
//  Implementations for Serde Json serialization/deserialization
//

use serde::{Serialize, Deserialize};
use serde_json::{Error, from_str, to_string};

//
// given a json formatted string, return object of given type
// any type with the Deserialize attribute will be supported
//
pub trait JsonDeserialize<'a>: Deserialize<'a> {
    fn from_json(json: &'a str) -> Result<Self, Error> {
        from_str(json)
    }
}

// this impl adds json deseralization to any object with Deserialize attribute
impl<'a, T: Deserialize<'a> > JsonDeserialize<'a> for T { }


// given a type with the attribute of Serialize, this trait
// will support serializing the public data members into properly
// formatted json
pub trait JsonSerialize : Serialize + Sized {
    fn to_json(&self) -> Result<String, Error> {
        to_string(self)
    }
}

// this impl adds json seralization to any object with Serialize attribute
impl<T:Serialize> JsonSerialize for T { }
