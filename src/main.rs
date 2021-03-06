#![deny(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    unsafe_code,
    broken_intra_doc_links
)]

extern crate pest;
#[macro_use]
extern crate pest_derive;
use std::env;
use std::process;

pub mod lib;
pub mod test;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = lib::parser::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = lib::parser::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
