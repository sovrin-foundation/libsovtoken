/*!
 *  Configs used in multiple places
 */

use logic::input::Input;
use logic::output::Output;

/**
 * Config which holds a vec of [`Input`]s
 * 
 * [`Inputs`]: ../../input/struct.Input.html
 */
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct InputConfig {
    pub inputs: Vec<Input>,
}

/**
 * Config which holds a vec of [`Output`]s
 * 
 * [`Outputs`]: ../../output/struct.Input.html
 */
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct OutputConfig {
    pub outputs: Vec<(Output)>,
}