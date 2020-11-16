extern crate pest;
#[macro_use]
extern crate pest_derive;
use std::env;
use std::process;

mod parser;
use parser::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = parser::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
