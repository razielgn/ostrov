use crate::helpers::{values::*, *};
use ostrov::errors::RuntimeError::*;

#[test]
fn returns_expression() {
    assert_eval_val(
        "(define x 0)
         (set! x (+ x 1))",
        unspecified(),
    );
}

#[test]
fn overwrites_variables() {
    assert_eval(
        "(define x 0)
         (set! x (+ x 1))
         x",
        "1",
    );
}

#[test]
fn overwrites_variables_on_upper_scopes() {
    assert_eval(
        "(define x 0)
         (define (f)
           (set! x (+ x 1)))
         (f)
         (f)
         (f)
         x",
        "3",
    );
}

#[test]
fn overwrites_variables_in_captured_scopes() {
    assert_eval(
        "(define (gen-counter)
           (define counter 0)
           (lambda ()
             (set! counter (+ counter 1))
             counter))
         (define count (gen-counter))
         (count)
         (count)
         (count)",
        "3",
    );
}

#[test]
fn malformed_variable_name() {
    assert_eval_err("(set! 3 3)", MalformedExpression);
}

#[test]
fn unknown_variable() {
    assert_eval_err("(set! x 3)", UnboundVariable("x".into()));
}

#[test]
fn wrong_arguments_number() {
    assert_eval_err("(set!)", BadArity(Some("set!".into())));
    assert_eval_err("(set! x)", BadArity(Some("set!".into())));
    assert_eval_err("(set! x 2 3)", BadArity(Some("set!".into())));
}
