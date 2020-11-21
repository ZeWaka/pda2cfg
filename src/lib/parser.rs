
use pest::Parser;
use pest::iterators::Pair;

use std::error::Error;
use std::fs;

use crate::lib::pda;
use crate::lib::cfg;
use crate::lib::transformer;

#[derive(Parser)]
#[grammar = "pda.pest"]
pub struct PDAParser;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let file = PDAParser::parse(Rule::file, &contents)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap(); // get and unwrap the `file` rule; never fails

    let mut our_pda = pda::PDA::build();

    for pair in file.into_inner() {
        match pair.as_rule() {
            Rule::pda => setup_pda(&mut our_pda, pair),
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    let mut result_cfg = cfg::CFG::build();

    // Prelim checks
    if let Err(e) = transformer::ensure_accept_nostates(our_pda.clone()) {
        println!("{}", e);
    }

    // PDA modification
    transformer::single_accept(&mut our_pda);

    // Start generation of CFG
    result_cfg.rules.push(cfg::Grammar::new("S".into(), format!("A_{}{}", our_pda.start_state.trim().clone(), "q_accept")));

    // Time for our rules
    transformer::eps_rule(&our_pda, &mut result_cfg);
    transformer::ijk_rule(&our_pda, &mut result_cfg);
    transformer::pair_rule(&our_pda, &mut result_cfg);

    let seralized = serde_json::to_string_pretty(&result_cfg).unwrap();

    println!("{}", seralized);

    Ok(())
}

/// Sets up our passed in PDA with data we get from pest
fn setup_pda(passed: &mut pda::PDA, pair: Pair<Rule>) -> () {
    let mut states: Vec<String> = vec![];
    let mut input_alpha: Vec<String> = vec![];
    let mut stack_alpha: Vec<String> = vec![];
    let mut start_state: String = String::new();
    let mut accep_state: Vec<String> = vec![];
    let mut transitions: Vec<pda::Trans> = vec![];

    let pda = pair.into_inner();
    for inner in pda {
        match inner.as_rule() {
            Rule::states => {
                for state in inner.into_inner() {
                    states.push(state.as_str().to_owned());
                }
            },
            Rule::ialpha => {
                for alpha_d in inner.into_inner() {
                    input_alpha.push(alpha_d.as_str().to_owned())
                }
            },
            Rule::salpha => {
                for alpha_d in inner.into_inner() {
                    stack_alpha.push(alpha_d.as_str().to_owned())
                }
            },
            Rule::start => {
                for state in inner.into_inner() {
                    start_state.push_str(state.as_str())
                }
            },
            Rule::accept => {
                for state in inner.into_inner() {
                    for states in state.into_inner() { // Another layer of onion to peel
                        accep_state.push(states.as_str().to_owned());
                    }
                }
            },
            Rule::trans => {
                for trans_set in inner.into_inner() {
                    let mut t_rules = trans_set.into_inner();

                    let t_state: String = t_rules.next().unwrap().as_str().to_owned();
                    let t_input: String = t_rules.next().unwrap().as_str().to_owned();
                    let t_symb: String = t_rules.next().unwrap().as_str().to_owned();
                    let t_next: String = t_rules.next().unwrap().as_str().to_owned();
                    let t_new: String = t_rules.next().unwrap().as_str().to_owned();
                    transitions.push(pda::Trans::new(t_state, t_input, t_symb, t_next, t_new));
                }
            },
            _ => unreachable!(),
        }
    }
    passed.set_states(states);
    passed.set_ialpha(input_alpha);
    passed.set_salpha(stack_alpha);
    passed.set_start(start_state);
    passed.set_accept(accep_state);
    passed.set_trans(transitions);

}

pub struct Config {
    pub filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments (expected 1 filename)");
        }

        let filename = args[1].clone();

        Ok(Config { filename })
    }
}
