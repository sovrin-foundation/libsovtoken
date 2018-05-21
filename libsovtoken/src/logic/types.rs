use indy::api::ErrorCode;

pub type ClosureString = Box<FnMut(ErrorCode, String) + Send>;
