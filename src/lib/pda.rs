use serde::Serialize;

/// We represent epsilon transitions with a ~  (also blank input/output)
pub static EPSILON: &'static str = "~";
/// Tau = stack alphabet
pub static TAU: &'static str = "Γ";
/// pound/hash - used for ensuring we pop all
pub static HASH: &'static str = "#";
/// percent sign - used for pop/push rule
pub static SYMBOL: &'static str = "%";

/// Start state we add
pub static START: &'static str = "qS";
/// Accept state we add
pub static ACCEPT: &'static str = "qF";

/// Our PDA struct
#[derive(Debug, Serialize, Clone)]
pub struct PDA {
    pub states: Vec<String>,
    pub input_alphabet: Vec<String>,
    pub stack_alphabet: Vec<String>,
    pub start_state: String,
    pub accept_states: Vec<String>,
    pub transitions: Vec<Trans>,
}

impl PDA {
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
    pub fn set_trans(&mut self, vec: Vec<Trans>) {
        for tuple in vec {
            self.transitions.push(tuple);
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Trans {
    pub state: String,
    pub input: String,
    pub pop: String,
    pub next: String,
    pub push: String,
}

impl Trans {
    pub fn new(state: String, input: String, pop: String, next: String, push: String) -> Self {
        Self {
            state,
            input,
            pop,
            next,
            push,
        }
    }
}
