use ostrov::parser::AST;
use ostrov::parser::parse;

use std::fmt::Show;

pub fn assert_parse(input: &str, expected: AST) {
    match parse(input) {
        Ok(actual) => assert_eq!(expected, actual),
        Err(error) => panic_expected(input, &expected, &error),
    }
}

fn panic_expected<A: Show, B: Show>(input: &str, expected: &A, actual: &B) {
    panic!("Expected {} from input \"{}\" , got {}", expected, input, actual);
}

pub fn integer(val: i64)   -> AST { AST::Integer(val) }
pub fn atom(val: &str)     -> AST { AST::Atom(val.to_string()) }
pub fn list(val: Vec<AST>) -> AST { AST::List(val) }
pub fn bool(val: bool)     -> AST { AST::Bool(val) }
