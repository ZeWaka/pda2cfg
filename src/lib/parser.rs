use std::error::Error;

use pest::Parser;
use pest::iterators::Pair;
use std::fs;

use crate::lib::pda;
#[path = "transformer.rs"] mod transformer;

#[derive(Parser)]
#[grammar = "pda.pest"]
pub struct PDAParser;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let file = PDAParser::parse(Rule::file, &contents)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap(); // get and unwrap the `file` rule; never fails

    let mut result_pda = pda::PDA::build();

    for pair in file.into_inner() {
        match pair.as_rule() {
            Rule::pda => create_pda_struct(&mut result_pda, pair),
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    // Time for our rules
    if let Err(e) = transformer::ensure_accept(&result_pda) {
        println!("{}", e);
    }
    if let Err(e) = transformer::single_accept(&result_pda) {
        println!("{}", e);
    }

    let seralized = serde_json::to_string(&result_pda).unwrap();

    println!("generated: {}", seralized);

    Ok(())
}

fn create_pda_struct(passed: &mut pda::PDA, pair: Pair<Rule>) -> () {
    let mut states: Vec<String> = vec![];
    let mut input_alpha: Vec<String> = vec![];
    let mut stack_alpha: Vec<String> = vec![];
    let mut start_state: String = String::new();
    let mut accep_state: Vec<String> = vec![];
    let mut transitions: Vec<(String, String, String, String, String)> = vec![];

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
                    accep_state.push(state.as_str().to_owned())
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
                    let transition = (t_state, t_input, t_symb, t_next, t_new);
                    transitions.push(transition);
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
