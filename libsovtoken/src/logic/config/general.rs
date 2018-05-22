use logic::input::Input;
use logic::output::Output;

/**
 *  Json config to customize [`build_payment_req_handler`]
 *
 *  [`build_fees_txn_handler`]: ../../api/fn.build_payment_req_handler.html
 */
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct InputConfig {
    pub inputs: Vec<Input>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct OutputConfig {
    pub outputs: Vec<(Output)>,
}