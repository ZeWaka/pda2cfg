use pest::iterators::Pair;
use pest::Parser;

use colored::*;

use std::panic;
use std::{error::Error, fs::read_to_string};

use crate::lib::cfg;
use crate::lib::pda;
use crate::lib::transformer;

static PARSE_ERROR: &'static str =
    "Error parsing: PDA input file does not conform to specification.";

#[derive(Parser)]
#[grammar = "pda.pest"]
pub struct PDAParser;

pub fn run(config: Config) -> Result<String, Box<dyn Error>> {
    let contents = read_to_string(config.filename)?;

    let file = parse_file(&contents);

    let parsed_file = match file {
        Some(p) => p,
        None => return Ok(PARSE_ERROR.into()),
    };

    let mut our_pda = pda::PDA::build();

    for pair in parsed_file.into_inner() {
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
    if let Err(e) = transformer::empty_stack(&mut our_pda) {
        println!("{}", e);
    }
    transformer::single_accept(&mut our_pda);
    transformer::only_push_pop(&mut our_pda);

    // Start generation of CFG
    result_cfg.rules.push(cfg::Grammar::new(
        "S".into(),
        format!("A_{}{}", our_pda.start_state.clone(), pda::ACCEPT),
    ));

    // Time for our rules
    transformer::eps_rule(&our_pda, &mut result_cfg);
    transformer::ijk_rule(&our_pda, &mut result_cfg);
    transformer::pair_rule(&our_pda, &mut result_cfg);

    // Print out result
    let result = serde_json::to_string_pretty(&result_cfg).unwrap();
    println!("{}", result);

    Ok(result)
}

/// Parses the pda string we give it, catches panic
fn parse_file(contents: &String) -> Option<Pair<Rule>> {
    panic::set_hook(Box::new(|_info| {
        // do nothing
    }));
    let mut parsed: Option<Pair<Rule>> = None;
    let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        parsed = Some(
            PDAParser::parse(Rule::file, &contents)
                .expect("parse error")
                .next()
                .unwrap(), // get and unwrap the `file` rule; never fails
        );
    }));

    match result {
        Ok(res) => res,
        Err(_) => {
            println!("{}", PARSE_ERROR.red());
            return None;
        }
    }

    return parsed;
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
                    states.push(state.as_str().trim().to_owned());
                }
            }
            Rule::ialpha => {
                for alpha_d in inner.into_inner() {
                    input_alpha.push(alpha_d.as_str().trim().to_owned())
                }
            }
            Rule::salpha => {
                for alpha_d in inner.into_inner() {
                    stack_alpha.push(alpha_d.as_str().trim().to_owned())
                }
            }
            Rule::start => {
                for state in inner.into_inner() {
                    start_state.push_str(state.as_str().trim())
                }
            }
            Rule::accept => {
                for state in inner.into_inner() {
                    for states in state.into_inner() {
                        // Another layer of onion to peel
                        accep_state.push(states.as_str().trim().to_owned());
                    }
                }
            }
            Rule::trans => {
                for trans_set in inner.into_inner() {
                    let mut t_rules = trans_set.into_inner();

                    let t_state: String = t_rules.next().unwrap().as_str().trim().to_owned();
                    let t_input: String = t_rules.next().unwrap().as_str().trim().to_owned();
                    let t_symb: String = t_rules.next().unwrap().as_str().trim().to_owned();
                    let t_next: String = t_rules.next().unwrap().as_str().trim().to_owned();
                    let t_new: String = t_rules.next().unwrap().as_str().trim().to_owned();
                    transitions.push(pda::Trans::new(t_state, t_input, t_symb, t_next, t_new));
                }
            }
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
    pub filename: String,
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
