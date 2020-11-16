extern crate serde;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PDA<> {
    states: Vec<String>,
    input_alpha: Vec<String>,
    stack_alpha: Vec<String>,
    start_state: String,
    accept_states: Vec<String>,
    transitions: (String, char, char, String, char),
}
