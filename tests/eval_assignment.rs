use helpers::*;
use helpers::values::*;

#[test]
fn returns_expression() {
    assert_eval("(define x 0)
                 (set! x (+ x 1))", "1"); // unspecified behaviour
}

#[test]
fn overwrites_variables() {
    assert_eval("(define x 0)
                 (set! x (+ x 1))
                 x", "1");
}

#[test]
fn overwrites_variables_on_upper_scopes() {
    assert_eval("(define x 0)
                 (define (f) (set! x (+ x 1)))
                 (f)
                 x", "1");
}

#[test]
fn overwrites_variables_in_captured_scopes() {
    assert_eval("(define incr 1)
                 (define (f)
                   (define x 0)
                   (lambda ()
                     (set! x (+ x incr))
                     x))
                 (define a (f))
                 (a)
                 (a)
                 (a)", "3");
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
