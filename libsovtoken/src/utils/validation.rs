/**
    `validate_did_len` expects a did and then validates that
    it is the correct length
*/
pub fn validate_did_len (submitter_did :&str) -> bool {
    let did_len = submitter_did.len();
    if did_len != 22 || did_len != 21 {
        return false;
    }
    true
}

