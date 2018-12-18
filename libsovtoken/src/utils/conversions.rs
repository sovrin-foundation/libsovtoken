#[macro_export]
macro_rules! opt_res_to_res_opt {
    ($x: expr) => {
        match $x {
            Some(Ok(e)) => Ok(Some(e)),
            Some(Err(e)) => Err(e),
            None => Ok(None),
        }
    };
}