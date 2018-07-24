use ostrov::errors::Error;
pub use ostrov::runtime::Runtime;
use ostrov::values::RcValue;

macro_rules! panic_expected {
    ($input:expr, $expected:expr, $actual:expr) => {
        panic!(
            "Expected {:?} from input \"{:?}\" , got {:?}",
            $expected, $input, $actual
        );
    };
}

pub fn assert_eval(input: &str, expected: &str) {
    let mut runtime = Runtime::new();

    match (runtime.eval_str(input), runtime.eval_str(expected)) {
        (Ok(got), Ok(expected)) => assert_eq!(
            expected.iter().last().unwrap(),
            got.iter().last().unwrap()
        ),
        error => panic_expected!(input, &expected, &error),
    }
}

pub fn assert_eval_val(input: &str, expected: RcValue) {
    let mut runtime = Runtime::new();

    match runtime.eval_str(input) {
        Ok(exprs) => assert_eq!(expected, *exprs.iter().last().unwrap()),
        Err(error) => panic_expected!(input, &expected, &error),
    }
}

pub fn assert_eval_err(input: &str, expected: Error) {
    let mut runtime = Runtime::new();

    match runtime.eval_str(input) {
        Ok(exprs) => panic_expected!(input, &expected, &exprs),
        Err(error) => assert_eq!(expected, error),
    }
}

pub mod values {
    use ostrov::values::{RcValue, Value};

    use std::rc::Rc;

    pub fn integer(val: i64) -> RcValue {
        Rc::new(Value::Integer(val))
    }
    pub fn pair(left: RcValue, right: RcValue) -> RcValue {
        Rc::new(Value::Pair(left, right))
    }
    pub fn nil() -> RcValue {
        Rc::new(Value::Nil)
    }
    pub fn unspecified() -> RcValue {
        Rc::new(Value::Unspecified)
    }
    pub fn bool(val: bool) -> RcValue {
        Rc::new(Value::Bool(val))
    }
}

pub fn unbound_variable_error(val: &str) -> Error {
    Error::UnboundVariable(val.to_owned())
}
pub fn unappliable_value_error(val: RcValue) -> Error {
    Error::UnappliableValue(val)
}
pub fn wrong_argument_type(val: RcValue) -> Error {
    Error::WrongArgumentType(val)
}
pub fn bad_arity(val: &str) -> Error {
    Error::BadArity(Some(val.to_owned()))
}
pub fn bad_arity_lambda() -> Error {
    Error::BadArity(None)
}
pub fn malformed_expr() -> Error {
    Error::MalformedExpression
}
