extern crate core;

use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;
use std::{env, fs};

mod lexer;
mod parser;

const HELP_MESSAGE: &str = "usage: cfmt <file path>";

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = args.get(1).expect(HELP_MESSAGE);
    let contents = fs::read_to_string(file_path).expect("Could not read file.");
    let lexer = Lexer::new(contents);

    // Error handling for the lexer.
    let tokens = lexer.map(|x| match x {
        Ok(token) => token,
        _ => {
            panic!("An error occurred during lexing.")
        }
    });

    let parser = Parser::new();
    let parse_tree = parser.parse(tokens);
}
