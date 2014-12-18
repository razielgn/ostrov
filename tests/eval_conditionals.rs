use helpers::*;
use helpers::ast::*;

#[test]
fn if_with_one_arg() {
    assert_eval("(if #t 3)", integer(3));
    assert_eval("(if #f 3)", bool(false)); // unspecified behaviour
}

#[test]
fn if_with_two_args() {
    assert_eval("(if #t (+ 1) a)", integer(1));
    assert_eval("(if #f a (+ 1))", integer(1));
    assert_eval("(if (and #t #f) a 1)", integer(1));
}

#[test]
fn if_bad_arity() {
    assert_eval_err("(if)", bad_arity("if"));
    assert_eval_err("(if a b c d)", bad_arity("if"));
}
