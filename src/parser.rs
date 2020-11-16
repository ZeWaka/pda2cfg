use std::error::Error;

use pest::Parser;
use pest::iterators::Pair;
use std::fs;

#[derive(Parser)]
#[grammar = "pda.pest"]
pub struct PDAParser;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let file = PDAParser::parse(Rule::file, &contents)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap(); // get and unwrap the `file` rule; never fails

    for pair in file.into_inner() {
        match pair.as_rule() {
            Rule::pda => create_pda_struct(pair),
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    Ok(())
}

fn create_pda_struct(pair: Pair<Rule>) -> () {

    let mut state_count: u64 = 0;
    let mut input_alpha: String = String::new();
    let mut stack_alpha: String = String::new();
    let mut start_state: String = String::new();
    let mut accep_state: String = String::new();
    let mut transitions: String = String::new();

    let pda = pair.into_inner();
    for inner in pda {
        println!("Rule:   {:?}", inner.as_rule());
        match inner.as_rule() {
            Rule::states => {
                for _state in inner.into_inner() {
                    state_count += 1;
                }
            },
            Rule::ialpha => {
                for alpha_d in inner.into_inner() {
                    input_alpha.push_str(alpha_d.as_str())
                }
            },
            Rule::salpha => {
                for alpha_d in inner.into_inner() {
                    stack_alpha.push_str(alpha_d.as_str())
                }
            },
            Rule::start => {
                for state in inner.into_inner() {
                    start_state.push_str(state.as_str())
                }
            },
            Rule::accept => {
                for state in inner.into_inner() {
                    accep_state.push_str(state.as_str())
                }
            },
            Rule::trans => {
                for trans_set in inner.into_inner() {
                    let mut t_rules = trans_set.into_inner();

                    let t_state: &str = t_rules.next().unwrap().as_str();
                    let t_input: &str = t_rules.next().unwrap().as_str();
                    let t_symb: &str = t_rules.next().unwrap().as_str();
                    let t_next: &str = t_rules.next().unwrap().as_str();
                    let t_new: &str = t_rules.next().unwrap().as_str();
                    let pushing =  format!("{},{},{},{},{}; ",t_state, t_input, t_symb, t_next, t_new);
                    transitions.push_str(&pushing);
                }
            },
            _ => unreachable!(),
        }
    }

    println!("nstates: {}", state_count);
    println!("ialpha: {}", input_alpha);
    println!("salpha: {}", stack_alpha);
    println!("start: {}", start_state);
    println!("accept: {}", accep_state);
    println!("trans: {}", transitions);

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
