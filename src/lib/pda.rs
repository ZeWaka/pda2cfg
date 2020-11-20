extern crate serde;

use serde::{Serialize, Deserialize};

/// Our PDA struct
#[derive(Debug, Serialize, Deserialize)]
pub struct PDA<> {
    pub states: Vec<String>,
    pub input_alphabet: Vec<String>,
    pub stack_alphabet: Vec<String>,
    pub start_state: String,
    pub accept_states: Vec<String>,
    /// state, input, symbol, next, new
    pub transitions: Vec<(String, String, String, String, String)>,
}

impl<> PDA<> {
    /// When called, builds an empty PDA
    pub fn build() -> PDA {
        PDA {
            states: vec![],
            input_alphabet: vec![],
            stack_alphabet: vec![],
            start_state: String::from("Error"),
            accept_states: vec![],
            transitions: vec![],
        }
    }

    /// Sets the states to given
    pub fn set_states(&mut self, vec: Vec<String>) {
        self.states = vec;
    }

    // Sets the input alphabet to given
    pub fn set_ialpha(&mut self, vec: Vec<String>) {
        self.input_alphabet = vec;
    }

    /// Sets stack alphabet to given
    pub fn set_salpha(&mut self, vec: Vec<String>) {
        self.stack_alphabet = vec;
    }

    /// Sets start state to given
    pub fn set_start(&mut self, stri: String) {
        self.start_state = stri;
    }

    /// Sets accept states to given
    pub fn set_accept(&mut self, vec: Vec<String>) {
        self.accept_states = vec;
    }

    /// Sets transitions to given tuple vector
    pub fn set_trans(&mut self, vec: Vec<(String, String, String, String, String)>) {
        for tuple in vec {
            self.transitions.push(tuple);
        }
    }
}
