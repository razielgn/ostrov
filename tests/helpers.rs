pub use ostrov::runtime::Runtime;
use ostrov::ast::AST;
use ostrov::runtime::Error;
use ostrov::values::Value;

use std::fmt::Show;

pub fn assert_parse(input: &str, expected: AST) {
    let runtime = Runtime::new();

    match runtime.parse_str(input) {
        Ok(exprs)  => assert_eq!(expected, *exprs.iter().last().unwrap()),
        Err(error) => panic_expected(input, &expected, &error),
    }
}

pub fn assert_eval(input: &str, expected: Value) {
    let mut runtime = Runtime::new();

    match runtime.eval_str(input) {
        Ok(exprs)  => assert_eq!(expected, **exprs.iter().last().unwrap()),
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

pub fn assert_fmt<T: Show>(input: &str, value: T) {
    assert_eq!(input, format!("{}", value));
}

fn panic_expected<A: Show, B: Show>(input: &str, expected: &A, actual: &B) {
    panic!("Expected {} from input \"{}\" , got {}", expected, input, actual);
}

pub mod ast {
    use ostrov::ast::AST;

    pub fn integer(val: i64)   -> AST { AST::Integer(val) }
    pub fn atom(val: &str)     -> AST { AST::Atom(val.to_string()) }
    pub fn list(val: Vec<AST>) -> AST { AST::List(val) }
    pub fn dotted_list(list: Vec<AST>, val: AST) -> AST {
        AST::DottedList(list, box val)
    }
    pub fn empty_list()        -> AST { AST::List(vec!()) }
    pub fn bool(val: bool)     -> AST { AST::Bool(val) }
}

pub mod values {
    use ostrov::values::ArgumentsType;
    use ostrov::values::Value;
    use ostrov::ast::AST;

    pub fn integer(val: i64) -> Value { Value::Integer(val) }
    pub fn atom(val: &str) -> Value { Value::Atom(val.to_string()) }
    pub fn list(val: Vec<Value>) -> Value { Value::List(val) }
    pub fn dotted_list(list: Vec<Value>, val: Value) -> Value {
        Value::DottedList(list, box val)
    }
    pub fn empty_list() -> Value { Value::List(vec!()) }
    pub fn bool(val: bool) -> Value { Value::Bool(val) }
    pub fn func(name: &str, args: Vec<&str>, body: Vec<AST>) -> Value {
        let args = args.iter().map(|s| s.to_string()).collect();
        Value::Fn(Some(name.to_string()), ArgumentsType::Fixed, args, body)
    }
    pub fn func_var(name: &str, args: Vec<&str>, body: Vec<AST>) -> Value {
        let args = args.iter().map(|s| s.to_string()).collect();
        Value::Fn(Some(name.to_string()), ArgumentsType::Variable, args, body)
    }
    pub fn func_any(name: &str, arg: &str, body: Vec<AST>) -> Value {
        Value::Fn(Some(name.to_string()), ArgumentsType::Any, vec!(arg.to_string()), body)
    }
    pub fn lambda(args: Vec<&str>, body: Vec<AST>) -> Value {
        let args = args.iter().map(|s| s.to_string()).collect();
        Value::Fn(None, ArgumentsType::Fixed, args, body)
    }
    pub fn lambda_var(args: Vec<&str>, body: Vec<AST>) -> Value {
        let args = args.iter().map(|s| s.to_string()).collect();
        Value::Fn(None, ArgumentsType::Variable, args, body)
    }
    pub fn lambda_any(arg: &str, body: Vec<AST>) -> Value {
        let args = vec!(arg).iter().map(|s| s.to_string()).collect();
        Value::Fn(None, ArgumentsType::Any, args, body)
    }
    pub fn primitive_func(name: &str) -> Value {
        Value::PrimitiveFn(name.to_string())
    }
}

pub fn unbound_variable_error(val: &str) -> Error { Error::UnboundVariable(val.to_string()) }
pub fn unappliable_value_error(val: Value) -> Error { Error::UnappliableValue(val) }
pub fn irreducible_value(val: AST) -> Error { Error::IrreducibleValue(val) }
pub fn wrong_argument_type(val: Value) -> Error { Error::WrongArgumentType(val) }
pub fn bad_arity(val: &str) -> Error { Error::BadArity(Some(val.to_string())) }
pub fn bad_arity_lambda() -> Error { Error::BadArity(None) }
