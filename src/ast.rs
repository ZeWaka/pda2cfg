extern crate serde;

use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

#[derive(Debug, Serialize)]
pub struct PDA<> {
    states: Vec<State>,
    input_alpha: Vec<String>,
    stack_alpha: Vec<String>,
    start_state: State,
    accept_states: Vec<State>,
    transitions: (State, char, char, State, char),
}

#[derive(Debug)]
pub struct State {
    name: String
}

impl Serialize for State {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("State", 1)?;
        s.serialize_field("name", &self.name)?;
        s.end()
    }
}
