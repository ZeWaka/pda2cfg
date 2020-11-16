extern crate serde;

use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

#[derive(Debug, Serialize)]
pub enum PDA<> {
    States(Vec<State>),
    InputAlpha(Vec<String>),
    StackAlpha(Vec<String>),
    StartState(State),
    AcceptStates(Vec<State>),
    Transitions(State, char, char, State, char),
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
