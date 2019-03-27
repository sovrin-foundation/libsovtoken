use ErrorCode;

const INSUFFICIENT_FUNDS_ERROR: &str = "InsufficientFundsError";
const EXTRA_FUNDS_ERROR: &str = "ExtraFundsError";
const INVALID_FUNDS: &str = "InvalidFundsError";

pub fn parse_error_code_from_string(reason: &str) -> ErrorCode {
    error!("{}", reason);
    if reason.contains(INSUFFICIENT_FUNDS_ERROR) {
        ErrorCode::PaymentInsufficientFundsError
    } else if reason.contains(EXTRA_FUNDS_ERROR) {
        ErrorCode::PaymentExtraFundsError
    } else if reason.contains(INVALID_FUNDS) {
        ErrorCode::PaymentSourceDoesNotExistError
    } else {
        ErrorCode::CommonInvalidStructure
    }
}