pub use ostrov::runtime::Runtime;
use ostrov::ast::AST;
use ostrov::runtime::Error;

use std::fmt::Show;

pub fn assert_parse(input: &str, expected: AST) {
    let runtime = Runtime::new();

    match runtime.parse_str(input) {
        Ok(exprs)  => assert_eq!(expected, *exprs.iter().last().unwrap()),
        Err(error) => panic_expected(input, &expected, &error),
    }
}

pub fn assert_eval(input: &str, expected: AST) {
    let mut runtime = Runtime::new();

    match runtime.eval_str(input) {
        Ok(exprs)  => assert_eq!(expected, *exprs.iter().last().unwrap()),
        Err(error) => panic_expected(input, &expected, &error),
    }
}

pub fn assert_eval_err(input: &str, expected: Error) {
    let mut runtime = Runtime::new();

    match runtime.eval_str(input) {
        Ok(exprs)  => panic_expected(input, &expected, &exprs),
        Err(error) => assert_eq!(expected, error),
    }
}

pub fn assert_fmt(input: &str, value: AST) {
    assert_eq!(input, format!("{}", value));
}

fn panic_expected<A: Show, B: Show>(input: &str, expected: &A, actual: &B) {
    panic!("Expected {} from input \"{}\" , got {}", expected, input, actual);
}

pub fn integer(val: i64)   -> AST { AST::Integer(val) }
pub fn atom(val: &str)     -> AST { AST::Atom(val.to_string()) }
pub fn list(val: Vec<AST>) -> AST { AST::List(val) }
pub fn dotted_list(list: Vec<AST>, val: AST) -> AST {
    AST::DottedList(list, box val)
}
pub fn empty_list()        -> AST { AST::List(vec!()) }
pub fn bool(val: bool)     -> AST { AST::Bool(val) }
pub fn func(name: &str, args: Vec<&str>, body: AST) -> AST {
    let args = args.iter().map(|s| s.to_string()).collect();
    AST::Fn(Some(name.to_string()), args, box body)
}
pub fn lambda(args: Vec<&str>, body: AST) -> AST {
    let args = args.iter().map(|s| s.to_string()).collect();
    AST::Fn(None, args, box body)
}

pub fn unbound_variable_error(val: &str) -> Error { Error::UnboundVariable(val.to_string()) }
pub fn unappliable_value_error(val: AST) -> Error { Error::UnappliableValue(val) }
pub fn irreducible_value(val: AST) -> Error { Error::IrreducibleValue(val) }
pub fn wrong_argument_type(val: AST) -> Error { Error::WrongArgumentType(val) }
pub fn bad_arity(val: &str) -> Error { Error::BadArity(Some(val.to_string())) }
