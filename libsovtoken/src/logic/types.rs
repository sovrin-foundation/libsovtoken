use indy::ErrorCode;

pub type ClosureString = Box<FnMut(ErrorCode, String) + Send>;
