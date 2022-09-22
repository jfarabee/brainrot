use std::{env, fs};
extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

// derive parser
#[derive(Parser)]
#[grammar = "pest/brainfuck.pest"]
pub struct BrainfuckParser;

fn main() {

    let args: Vec<String> = env::args().collect();

    // need path to bf file
    if args.len() < 2 {
        panic!("Not enough arguments.");
    }

    let bf_path = &args[1];

    dbg!("reading contents of {}", bf_path);
    let bf_code = fs::read_to_string(bf_path).expect("File read error. Check path or permissions.");
    dbg!("contents read successfully");


    dbg!("parsing contents of {}", bf_path);
    let parse_result = BrainfuckParser::parse(Rule::main, &bf_code)
        .expect("Parse unsuccessful.")
        .next().unwrap();
    let tokens = parse_result.tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}
