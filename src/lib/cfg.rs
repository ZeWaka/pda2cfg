extern crate serde;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CFG<> {
    pub rules: Vec<Grammar>
}

impl<> CFG<> {
    pub fn build() -> CFG {
        CFG {
            rules: vec![],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Grammar<> {
    pub rule_name: String,
    pub rule_desc: String,
}

impl<> Grammar<> {
    pub fn new(rule_name: String, rule_desc: String) -> Self { Self { rule_name, rule_desc } }
}

