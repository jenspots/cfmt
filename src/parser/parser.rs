use crate::lexer::token::Token;
use crate::parser::parse_tree::ParseTree;

pub struct Parser;

impl Parser {
    pub fn new() -> Parser {
        Parser
    }

    pub fn parse(&self, iter: impl Iterator<Item = Token>) -> ParseTree {
        for token in iter {
            // TODO: do stuff
        }

        ParseTree
    }
}
