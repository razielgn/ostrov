use helpers::*;
use helpers::values::*;

#[test]
fn returns_expression() {
    assert_eval("(define x 0)
                 (set! x (+ x 1))", integer(1)); // unspecified behaviour
}

#[test]
fn overwrites_variables() {
    assert_eval("(define x 0)
                 (set! x (+ x 1))
                 x", integer(1));
}

#[test]
fn malformed_variable_name() {
    assert_eval_err("(set! 3 3)", wrong_argument_type(integer(3)));
}

#[test]
fn unknown_variable() {
    assert_eval_err("(set! x 3)", unbound_variable_error("x"));
}

#[test]
fn wrong_arguments_number() {
    assert_eval_err("(set!)", bad_arity("set!"));
    assert_eval_err("(set! x)", bad_arity("set!"));
    assert_eval_err("(set! x 2 3)", bad_arity("set!"));
}