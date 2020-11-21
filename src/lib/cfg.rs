use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Debug)]
pub struct CFG {
    pub rules: Vec<Grammar>,
}

impl CFG {
    pub fn build() -> CFG {
        CFG { rules: vec![] }
    }
}

impl Serialize for CFG {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("CFG", 1)?;
        state.serialize_field("rules", &self.rules)?;
        state.end()
    }
}

#[derive(Debug)]
pub struct Grammar {
    pub rule_name: String,
    pub rule_desc: String,
}

impl Grammar {
    pub fn new(rule_name: String, rule_desc: String) -> Self {
        Self {
            rule_name,
            rule_desc,
        }
    }
}

impl Serialize for Grammar {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Grammar", 2)?;
        state.serialize_field(
            "Rule",
            &format!("{} -> {}", &self.rule_name, &self.rule_desc),
        )?;
        state.end()
    }
}
