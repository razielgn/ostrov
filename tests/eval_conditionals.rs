use helpers::values::*;
use helpers::*;

#[test]
fn if_with_one_arg() {
    assert_eval("(if #t 3)", "3");
    assert_eval_val("(if #f 3)", unspecified());
}

#[test]
fn if_with_two_args() {
    assert_eval("(if #t (+ 1) a)", "1");
    assert_eval("(if #f a (+ 1))", "1");
    assert_eval("(if (and #t #f) a 1)", "1");
}

#[test]
fn if_bad_arity() {
    assert_eval_err("(if)", bad_arity("if"));
    assert_eval_err("(if a b c d)", bad_arity("if"));
}
