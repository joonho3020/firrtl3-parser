pub mod lexer;

use rusty_firrtl::Circuit;
use crate::firrtl::*;
use crate::lexer::{FIRRTLLexer, Token, LexicalError};
use lalrpop_util::{lalrpop_mod, ParseError};

lalrpop_mod!(pub firrtl);

pub type FIRRTLParserError = ParseError<usize, Token, LexicalError>;

/// Given a path to a FIRRTL file, parse it and return a `Circuit` which represents the FIRRTL AST
pub fn parse_circuit(source: &str) -> Result<Circuit, FIRRTLParserError> {
    let lexer = FIRRTLLexer::new(source);
    let parser = CircuitParser::new();
    parser.parse(lexer)
}
