use crate::helpers::{values::*, *};
use ostrov::errors::RuntimeError::*;

#[test]
fn list_() {
    assert_eval("(list)", "'()");
    assert_eval("(list (+ 0 1) 2 3)", "'(1 2 3)");
}

#[test]
fn length() {
    assert_eval("(length '())", "0");
    assert_eval("(length '(1 2 3))", "3");
    assert_eval("(length (list (+ 0 1)))", "1");
}

#[test]
fn length_bad_arity() {
    assert_eval_err("(length '() '())", BadArity(Some("length".into())));
}

#[test]
fn length_bad_arguments() {
    assert_eval_err("(length 1)", WrongArgumentType(integer(1)));
}

#[test]
fn pair_() {
    assert_eval("(pair? 1)", "#f");
    assert_eval("(pair? #t)", "#f");
    assert_eval("(pair? '())", "#f");
    assert_eval("(pair? (lambda (x) 0))", "#f");
    assert_eval("(pair? '(1 2 3))", "#t");
    assert_eval("(pair? '(1 2 . 3))", "#t");
}

#[test]
fn pair_bad_arity() {
    assert_eval_err("(pair?)", BadArity(Some("pair?".into())));
    assert_eval_err("(pair? 1 2)", BadArity(Some("pair?".into())));
}

#[test]
fn cons() {
    assert_eval("(cons 'a '())", "'(a)");
    assert_eval("(cons '(a) '(b c d))", "'((a) b c d)");
    assert_eval("(cons 1 '(b c))", "'(1 b c)");
    assert_eval("(cons 'a 3)", "'(a . 3)");
    assert_eval("(cons '(a b) 'c)", "'((a b) . c)");
}

#[test]
fn cons_bad_arity() {
    assert_eval_err("(cons)", BadArity(Some("cons".into())));
    assert_eval_err("(cons 1)", BadArity(Some("cons".into())));
    assert_eval_err("(cons 1 2 3)", BadArity(Some("cons".into())));
}

#[test]
fn car() {
    assert_eval("(car '(a b c))", "'a");
    assert_eval("(car '(1 . 2))", "1");
}

#[test]
fn car_bad_arity() {
    assert_eval_err("(car)", BadArity(Some("car".into())));
    assert_eval_err("(car '(a) '(b))", BadArity(Some("car".into())));
}

#[test]
fn car_wrong_argument_type() {
    assert_eval_err("(car 12)", WrongArgumentType(integer(12)));
    assert_eval_err("(car '())", WrongArgumentType(nil()));
}

#[test]
fn cdr() {
    assert_eval("(cdr '((a) b c d))", "'(b c d)");
    assert_eval("(cdr '(1 . 2))", "2");
}

#[test]
fn cdr_bad_arity() {
    assert_eval_err("(cdr)", BadArity(Some("cdr".into())));
    assert_eval_err("(cdr '(a) '(b))", BadArity(Some("cdr".into())));
}

#[test]
fn cdr_wrong_argument_type() {
    assert_eval_err("(cdr 12)", WrongArgumentType(integer(12)));
    assert_eval_err("(cdr '())", WrongArgumentType(nil()));
}

#[test]
fn null() {
    assert_eval("(null? '())", "#t");
    assert_eval("(null? 1)", "#f");
    assert_eval("(null? '(1 2 3))", "#f");
    assert_eval("(null? #t)", "#f");
}

#[test]
fn null_bad_arity() {
    assert_eval_err("(null?)", BadArity(Some("null?".into())));
    assert_eval_err("(null? '(a) '(b))", BadArity(Some("null?".into())));
}

#[test]
fn list_question_mark() {
    assert_eval("(list? '(a b c))", "#t");
    assert_eval("(list? '())", "#t");
    assert_eval("(list? '(a . b))", "#f");
    assert_eval("(list? 1)", "#f");
}

#[test]
fn list_question_mark_bad_arity() {
    assert_eval_err("(list?)", BadArity(Some("list?".into())));
    assert_eval_err("(list? '() '())", BadArity(Some("list?".into())));
}
