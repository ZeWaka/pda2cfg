use std::error::Error;

use pest::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "pda.pest"]
pub struct PDAParser;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let file = PDAParser::parse(Rule::file, &contents)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap(); // get and unwrap the `file` rule; never fails

    let mut state_count: u64 = 0;
    let mut input_alpha: String = String::new();
    let mut stack_alpha: String = String::new();
    let mut start_state: String = String::new();
    let mut accep_state: String = String::new();
    let mut transitions: String = String::new();

    for pda in file.into_inner() {
        match pda.as_rule() {
            Rule::states => {
                for _state in pda.into_inner() {
                    state_count += 1;
                }
            },
            Rule::ialpha => {
                for alpha_d in pda.into_inner() {
                    input_alpha.push_str(alpha_d.as_str())
                }
            },
            Rule::salpha => {
                for alpha_d in pda.into_inner() {
                    stack_alpha.push_str(alpha_d.as_str())
                }
            },
            Rule::start => {
                for state in pda.into_inner() {
                    start_state.push_str(state.as_str())
                }
            },
            Rule::accept => {
                for state in pda.into_inner() {
                    accep_state.push_str(state.as_str())
                }
            },
            Rule::trans => {
                for trans_set in pda.into_inner() {
                    let mut inner_rules = trans_set.into_inner();

                    let t_state: &str = inner_rules.next().unwrap().as_str();
                    let t_input: &str = inner_rules.next().unwrap().as_str();
                    let t_symb: &str = inner_rules.next().unwrap().as_str();
                    let t_next: &str = inner_rules.next().unwrap().as_str();
                    let t_new: &str = inner_rules.next().unwrap().as_str();
                    let pushing =  format!("{},{},{},{},{}",t_state, t_input, t_symb, t_next, t_new);
                    transitions.push_str(&pushing);
                }
            },                   
            Rule::EOI => (),
            _ => unreachable!(),

        }
    }

    println!("nstates: {}", state_count);
    println!("ialpha: {}", input_alpha);
    println!("salpha: {}", stack_alpha);
    println!("start: {}", start_state);
    println!("accept: {}", accep_state);
    println!("trans: {}", transitions);

    Ok(())
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
