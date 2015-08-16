use helpers::*;
use helpers::values::*;

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
    assert_eval_err("(length '() '())", bad_arity("length"));
}

#[test]
fn length_bad_arguments() {
    assert_eval_err("(length 1)", wrong_argument_type(integer(1)));
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
    assert_eval_err("(pair?)", bad_arity("pair?"));
    assert_eval_err("(pair? 1 2)", bad_arity("pair?"));
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
    assert_eval_err("(cons)", bad_arity("cons"));
    assert_eval_err("(cons 1)", bad_arity("cons"));
    assert_eval_err("(cons 1 2 3)", bad_arity("cons"));
}

#[test]
fn car() {
    assert_eval("(car '(a b c))", "'a");
    assert_eval("(car '(1 . 2))", "1");
}

#[test]
fn car_bad_arity() {
    assert_eval_err("(car)", bad_arity("car"));
    assert_eval_err("(car '(a) '(b))", bad_arity("car"));
}

#[test]
fn car_wrong_argument_type() {
    assert_eval_err("(car 12)", wrong_argument_type(integer(12)));
    assert_eval_err("(car '())", wrong_argument_type(nil()));
}

#[test]
fn cdr() {
    assert_eval("(cdr '((a) b c d))", "'(b c d)");
    assert_eval("(cdr '(1 . 2))", "2");
}

#[test]
fn cdr_bad_arity() {
    assert_eval_err("(cdr)", bad_arity("cdr"));
    assert_eval_err("(cdr '(a) '(b))", bad_arity("cdr"));
}

#[test]
fn cdr_wrong_argument_type() {
    assert_eval_err("(cdr 12)", wrong_argument_type(integer(12)));
    assert_eval_err("(cdr '())", wrong_argument_type(nil()));
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
    assert_eval_err("(null?)", bad_arity("null?"));
    assert_eval_err("(null? '(a) '(b))", bad_arity("null?"));
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
    assert_eval_err("(list?)", bad_arity("list?"));
    assert_eval_err("(list? '() '())", bad_arity("list?"));
}
