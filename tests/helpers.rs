pub use ostrov::runtime::Runtime;
use ostrov::ast::AST;
use ostrov::errors::Error;
use ostrov::values::RcValue;

use std::fmt::Debug;
use std::fmt::Display;

pub fn assert_parse(input: &str, expected: AST) {
    let runtime = Runtime::new();

    match runtime.parse_str(input) {
        Ok(exprs)  => assert_eq!(expected, *exprs.iter().last().unwrap()),
        Err(error) => panic_expected(input, &expected, &error),
    }
}

pub fn assert_eval(input: &str, expected: &str) {
    let mut runtime = Runtime::new();

    match (runtime.eval_str(input), runtime.eval_str(expected)) {
        (Ok(got), Ok(expected)) => assert_eq!(expected.iter().last().unwrap(), got.iter().last().unwrap()),
        error                   => panic_expected(input, &expected, &error),
    }
}

pub fn assert_eval_val(input: &str, expected: RcValue) {
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

pub fn assert_fmt<T: Display>(input: &str, value: T) {
    assert_eq!(input, format!("{}", value));
}

fn panic_expected<A: Debug, B: Debug>(input: &str, expected: &A, actual: &B) {
    panic!("Expected {:?} from input \"{:?}\" , got {:?}", expected, input, actual);
}

pub mod ast {
    use ostrov::ast::AST;

    pub fn integer(val: i64)   -> AST { AST::Integer(val) }
    pub fn atom(val: &str)     -> AST { AST::Atom(val.to_string()) }
    pub fn list(val: Vec<AST>) -> AST { AST::List(val) }
    pub fn dotted_list(list: Vec<AST>, val: AST) -> AST {
        AST::DottedList(list, Box::new(val))
    }
    pub fn empty_list()        -> AST { AST::List(vec!()) }
    pub fn bool(val: bool)     -> AST { AST::Bool(val) }
}

pub mod values {
    use ostrov::values::{ArgumentsType, Value, RcValue};
    use ostrov::ast::AST;
    use ostrov::env::CellEnv;

    use std::rc::Rc;

    pub fn integer(val: i64) -> RcValue { Rc::new(Value::Integer(val)) }
    pub fn atom(val: &str) -> RcValue { Rc::new(Value::Atom(val.to_string())) }
    pub fn pair(left: RcValue, right: RcValue) -> RcValue {
        Rc::new(Value::Pair(left, right))
    }
    pub fn nil() -> RcValue { Rc::new(Value::Nil) }
    pub fn unspecified() -> RcValue { Rc::new(Value::Unspecified) }
    pub fn bool(val: bool) -> RcValue { Rc::new(Value::Bool(val)) }
    pub fn func(name: &str, args: Vec<&str>, body: Vec<AST>) -> RcValue {
        let args = args.iter().map(|s| s.to_string()).collect();
        Rc::new(Value::Fn(Some(name.to_string()), ArgumentsType::Fixed, args, CellEnv::new(), body))
    }
    pub fn lambda(args: Vec<&str>, body: Vec<AST>) -> RcValue {
        let args = args.iter().map(|s| s.to_string()).collect();
        Rc::new(Value::Fn(None, ArgumentsType::Fixed, args, CellEnv::new(), body))
    }
    pub fn lambda_var(args: Vec<&str>, body: Vec<AST>) -> RcValue {
        let args = args.iter().map(|s| s.to_string()).collect();
        Rc::new(Value::Fn(None, ArgumentsType::Variable, args, CellEnv::new(), body))
    }
    pub fn lambda_any(arg: &str, body: Vec<AST>) -> RcValue {
        let args = vec!(arg).iter().map(|s| s.to_string()).collect();
        Rc::new(Value::Fn(None, ArgumentsType::Any, args, CellEnv::new(), body))
    }
    pub fn primitive_func(name: &str) -> RcValue {
        Rc::new(Value::PrimitiveFn(name.to_string()))
    }
}

pub fn unbound_variable_error(val: &str) -> Error { Error::UnboundVariable(val.to_string()) }
pub fn unappliable_value_error(val: RcValue) -> Error { Error::UnappliableValue(val) }
pub fn wrong_argument_type(val: RcValue) -> Error { Error::WrongArgumentType(val) }
pub fn bad_arity(val: &str) -> Error { Error::BadArity(Some(val.to_string())) }
pub fn bad_arity_lambda() -> Error { Error::BadArity(None) }
pub fn malformed_expr() -> Error { Error::MalformedExpression }
pub fn cannot_pop_last_frame() -> Error { Error::CannotPopLastFrame }
