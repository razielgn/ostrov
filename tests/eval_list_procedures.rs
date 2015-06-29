use helpers::*;
use helpers::values::*;

#[test]
fn list_() {
    assert_eval_vm("(list)", "'()");
    assert_eval_vm("(list (+ 0 1) 2 3)", "'(1 2 3)");
}

#[test]
fn length() {
    assert_eval_vm("(length '())", "0");
    assert_eval_vm("(length '(1 2 3))", "3");
    assert_eval_vm("(length (list (+ 0 1)))", "1");
}

#[test]
fn length_bad_arity() {
    assert_eval_vm_err("(length '() '())", bad_arity("length"));
}

#[test]
fn length_bad_arguments() {
    assert_eval_vm_err("(length 1)", wrong_argument_type(integer(1)));
}

#[test]
fn pair_() {
    assert_eval_vm("(pair? 1)", "#f");
    assert_eval_vm("(pair? #t)", "#f");
    assert_eval_vm("(pair? '())", "#f");
    assert_eval("(pair? (lambda (x) 0))", "#f");
    assert_eval_vm("(pair? '(1 2 3))", "#t");
    assert_eval_vm("(pair? '(1 2 . 3))", "#t");
}

#[test]
fn pair_bad_arity() {
    assert_eval_vm_err("(pair?)", bad_arity("pair?"));
    assert_eval_vm_err("(pair? 1 2)", bad_arity("pair?"));
}

#[test]
fn cons() {
    assert_eval_vm("(cons 'a '())", "'(a)");
    assert_eval_vm("(cons '(a) '(b c d))", "'((a) b c d)");
    assert_eval_vm("(cons 1 '(b c))", "'(1 b c)");
    assert_eval_vm("(cons 'a 3)", "'(a . 3)");
    assert_eval_vm("(cons '(a b) 'c)", "'((a b) . c)");
}

#[test]
fn cons_bad_arity() {
    assert_eval_vm_err("(cons)", bad_arity("cons"));
    assert_eval_vm_err("(cons 1)", bad_arity("cons"));
    assert_eval_vm_err("(cons 1 2 3)", bad_arity("cons"));
}

#[test]
fn car() {
    assert_eval_vm("(car '(a b c))", "'a");
    assert_eval_vm("(car '(1 . 2))", "1");
}

#[test]
fn car_bad_arity() {
    assert_eval_vm_err("(car)", bad_arity("car"));
    assert_eval_vm_err("(car '(a) '(b))", bad_arity("car"));
}

#[test]
fn car_wrong_argument_type() {
    assert_eval_vm_err("(car 12)", wrong_argument_type(integer(12)));
    assert_eval_vm_err("(car '())", wrong_argument_type(nil()));
}

#[test]
fn cdr() {
    assert_eval_vm("(cdr '((a) b c d))", "'(b c d)");
    assert_eval_vm("(cdr '(1 . 2))", "2");
}

#[test]
fn cdr_bad_arity() {
    assert_eval_vm_err("(cdr)", bad_arity("cdr"));
    assert_eval_vm_err("(cdr '(a) '(b))", bad_arity("cdr"));
}

#[test]
fn cdr_wrong_argument_type() {
    assert_eval_vm_err("(cdr 12)", wrong_argument_type(integer(12)));
    assert_eval_vm_err("(cdr '())", wrong_argument_type(nil()));
}

#[test]
fn null() {
    assert_eval_vm("(null? '())", "#t");
    assert_eval_vm("(null? 1)", "#f");
    assert_eval_vm("(null? '(1 2 3))", "#f");
    assert_eval_vm("(null? #t)", "#f");
}

#[test]
fn null_bad_arity() {
    assert_eval_vm_err("(null?)", bad_arity("null?"));
    assert_eval_vm_err("(null? '(a) '(b))", bad_arity("null?"));
}

#[test]
fn list_question_mark() {
    assert_eval_vm("(list? '(a b c))", "#t");
    assert_eval_vm("(list? '())", "#t");
    assert_eval_vm("(list? '(a . b))", "#f");
    assert_eval_vm("(list? 1)", "#f");
}

#[test]
fn list_question_mark_bad_arity() {
    assert_eval_vm_err("(list?)", bad_arity("list?"));
    assert_eval_vm_err("(list? '() '())", bad_arity("list?"));
}
