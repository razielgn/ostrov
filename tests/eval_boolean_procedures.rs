use helpers::*;
use helpers::ast::*;

#[test]
fn and() {
    assert_eval("(and)", bool(true));
    assert_eval("(and (+ 2 3))", integer(5));
    assert_eval("(and #t 2)", integer(2));
    assert_eval("(and 1 #f a)", bool(false));
}

#[test]
fn or() {
    assert_eval("(or)", bool(false));
    assert_eval("(or (+ 2 3))", integer(5));
    assert_eval("(or #f 2)", integer(2));
    assert_eval("(or 1 a)", integer(1));
}

#[test]
fn not() {
    assert_eval("(not #f)", bool(true));
    assert_eval("(not #t)", bool(false));
    assert_eval("(not 2)", bool(false));
    assert_eval("(not 'a)", bool(false));
}

#[test]
fn not_bad_arity() {
    assert_eval_err("(not)", bad_arity("not"));
    assert_eval_err("(not 2 3)", bad_arity("not"));
}
