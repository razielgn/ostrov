use ostrov::ast::AST;
use ostrov::eval::Error;
use ostrov::eval::eval;
use ostrov::parser::parse;

use std::fmt::Show;

pub fn assert_parse(input: &str, expected: AST) {
    match parse(input) {
        Ok(actual) => assert_eq!(expected, actual),
        Err(error) => panic_expected(input, &expected, &error),
    }
}

pub fn assert_eval(input: &str, expected: AST) {
    match parse(input) {
        Ok(ast) => match eval(ast) {
            Ok(actual) => assert_eq!(expected, actual),
            Err(error) => panic_expected(input, &expected, &error),
        },
        Err(error) => panic_parse(&error, input),
    }
}

pub fn assert_eval_err(input: &str, expected: Error) {
    match parse(input) {
        Ok(ast) => match eval(ast) {
            Ok(value)   => panic_expected(input, &expected, &value),
            Err(actual) => assert_eq!(expected, actual),
        },
        Err(error) => panic_parse(&error, input),
    }
}

fn panic_expected<A: Show, B: Show>(input: &str, expected: &A, actual: &B) {
    panic!("Expected {} from input \"{}\" , got {}", expected, input, actual);
}

fn panic_parse(error: &String, input: &str) {
    panic!("Parse error {} from input \"{}\"", error, input);
}

pub fn integer(val: i64)   -> AST { AST::Integer(val) }
pub fn atom(val: &str)     -> AST { AST::Atom(val.to_string()) }
pub fn list(val: Vec<AST>) -> AST { AST::List(val) }
pub fn empty_list()        -> AST { AST::List(vec!()) }
pub fn bool(val: bool)     -> AST { AST::Bool(val) }

pub fn irreducible_val_error(val: AST) -> Error { Error::IrreducibleValue(val) }
