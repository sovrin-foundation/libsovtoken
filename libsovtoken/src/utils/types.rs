use indy::ErrorCode;
use std::os::raw::c_char;

// common types of callbacks or structures
pub type JsonCallback  = Option<extern fn(command_handle_: i32,
                                  err: ErrorCode,
                                  json: *const c_char) -> ErrorCode>;

pub type ErrorCodeStringClosure = Box<FnMut(ErrorCode, String) + Send>;
pub type ErrorCodeStringCallback = Option<extern fn(command_handle: i32, err: ErrorCode, c_str: *const c_char)>;


#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct UTXOInfo {
    pub input: String,
    pub amount: i32,
    pub extra: Option<String>
}


#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct UTXOOutput {
    #[serde(rename = "paymentAddress")]
    pub payment_address: String,
    pub amount: i32,
    pub extra: Option<String>
}

impl Clone for UTXOOutput {
    fn clone(&self) -> Self {
        UTXOOutput {
            payment_address: self.payment_address.clone(),
            amount: self.amount.clone(),
            extra: self.extra.clone()
        }
    }
}
