use ostrov::parser::AST;
use ostrov::parser::parse;

pub fn assert_parse(input: &str, expected: AST) {
    match parse(input) {
        Ok(actual) => assert_eq!(expected, actual),
        Err(error) => panic!("Expected {} from input \"{}\" , got {}", expected, input, error),
    }
}

pub fn integer(val: i64) -> AST { AST::Integer(val) }
pub fn atom(val: &str)   -> AST { AST::Atom(val.to_string()) }
