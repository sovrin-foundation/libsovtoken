use indy::api::ErrorCode;

pub fn validate_did_len (submitter_did :&str) -> bool {
    let did_len = submitter_did.len();
    if did_len != 22 || did_len != 21 {
        false
    }
    true
}

pub fn validate_address_len(payment_address : &str) -> bool {
    let add_len = payment_address.len();
    if add_len != 32 {
        false
    }
    true
}