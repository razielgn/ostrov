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

pub fn assert_fmt(input: &str, value: AST) {
    assert_eq!(input, format!("{}", value));
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
pub fn dotted_list(list: Vec<AST>, val: AST) -> AST {
    AST::DottedList(list, box val)
}
pub fn empty_list()        -> AST { AST::List(vec!()) }
pub fn bool(val: bool)     -> AST { AST::Bool(val) }

pub fn unbound_variable_error(val: &str) -> Error { Error::UnboundVariable(val.to_string()) }
pub fn unappliable_value_error(val: AST) -> Error { Error::UnappliableValue(val) }
pub fn irreducible_value(val: AST) -> Error { Error::IrreducibleValue(val) }
pub fn wrong_argument_type(val: AST) -> Error { Error::WrongArgumentType(val) }
pub fn bad_arity(val: &str) -> Error { Error::BadArity(val.to_string()) }
