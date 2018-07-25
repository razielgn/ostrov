use helpers::*;
use ostrov::errors::RuntimeError::*;

#[test]
fn and() {
    assert_eval("(and)", "#t");
    assert_eval("(and (+ 2 3))", "5");
    assert_eval("(and #f)", "#f");
    assert_eval("(and #f 2)", "#f");
    assert_eval("(and #t 2)", "2");
    assert_eval("(and 1 #f a)", "#f");
}

#[test]
fn or() {
    assert_eval("(or)", "#f");
    assert_eval("(or (+ 2 3))", "5");
    assert_eval("(or #f 2)", "2");
    assert_eval("(or 1 a)", "1");
}

#[test]
fn not() {
    assert_eval("(not #f)", "#t");
    assert_eval("(not #t)", "#f");
    assert_eval("(not 2)", "#f");
    assert_eval("(not 'a)", "#f");
}

#[test]
fn not_bad_arity() {
    assert_eval_err("(not)", BadArity(Some("not".into())));
    assert_eval_err("(not 2 3)", BadArity(Some("not".into())));
}
