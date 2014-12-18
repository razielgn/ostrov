use helpers::*;
use helpers::ast::*;

#[test]
fn plus() {
    assert_eval("(+)", integer(0));
    assert_eval("(+ 2)", integer(2));
    assert_eval("(+ 2 3)", integer(5));
    assert_eval("(+ 2 3 -9)", integer(-4));
    assert_eval("(+ 2 3 -9 1)", integer(-3));
}

#[test]
fn plus_bad_arity() {
    assert_eval_err("(+ ())", wrong_argument_type(empty_list()));
}

#[test]
fn minus() {
    assert_eval("(- 2)", integer(-2));
    assert_eval("(- 2 3)", integer(-1));
    assert_eval("(- 2 3 -9)", integer(8));
    assert_eval("(- 2 3 -9 1)", integer(7));
}

#[test]
fn minus_bad_arity() {
    assert_eval_err("(-)", bad_arity("-"));
}

#[test]
fn minus_wrong_argument_type() {
    assert_eval_err("(- ())", wrong_argument_type(empty_list()));
}

#[test]
fn product() {
    assert_eval("(*)", integer(1));
    assert_eval("(* 2)", integer(2));
    assert_eval("(* 2 3)", integer(6));
    assert_eval("(* 2 3 -9)", integer(-54));
    assert_eval("(* 2 3 -9 2)", integer(-108));
}

#[test]
fn product_wrong_argument_type() {
    assert_eval_err("(* ())", wrong_argument_type(empty_list()));
}

#[test]
fn division() {
    assert_eval("(/ 2)", integer(0));
    assert_eval("(/ 9 3)", integer(3));
    assert_eval("(/ 9 3 3)", integer(1));
    assert_eval("(/ 27 3 3 3)", integer(1));
}

#[test]
fn division_bad_arity() {
    assert_eval_err("(/)", bad_arity("/"));
}

#[test]
fn division_wrong_argument_type() {
    assert_eval_err("(/ ())", wrong_argument_type(empty_list()));
}

#[test]
fn equal_sign() {
    assert_eval("(=)", bool(true));
    assert_eval("(= 1)", bool(true));
    assert_eval("(= 23 23)", bool(true));
    assert_eval("(= 23 42)", bool(false));
    assert_eval("(= 23 23 42)", bool(false));
    assert_eval("(= 23 23 23 42)", bool(false));
    assert_eval("(= 23 23 23 23)", bool(true));
}

#[test]
fn less_than_sign() {
    assert_eval("(<)", bool(true));
    assert_eval("(< 1)", bool(true));
    assert_eval("(< 1 2)", bool(true));
    assert_eval("(< 1 1)", bool(false));
    assert_eval("(< 1 2 1)", bool(false));
    assert_eval("(< 1 2 3 1)", bool(false));
    assert_eval("(< 1 2 3 4 5)", bool(true));
}

#[test]
fn less_than_or_equal_sign() {
    assert_eval("(<=)", bool(true));
    assert_eval("(<= 1)", bool(true));
    assert_eval("(<= 1 2)", bool(true));
    assert_eval("(<= 1 1)", bool(true));
    assert_eval("(<= 1 2 1)", bool(false));
    assert_eval("(<= 1 2 3 1)", bool(false));
    assert_eval("(<= 1 3 3 4 5)", bool(true));
}

#[test]
fn greater_than_sign() {
    assert_eval("(>)", bool(true));
    assert_eval("(> 1)", bool(true));
    assert_eval("(> 2 1)", bool(true));
    assert_eval("(> 1 1)", bool(false));
    assert_eval("(> 1 2 1)", bool(false));
    assert_eval("(> 1 3 2 1)", bool(false));
    assert_eval("(> 5 4 3 2 1)", bool(true));
}

#[test]
fn greater_than_or_equal_sign() {
    assert_eval("(>=)", bool(true));
    assert_eval("(>= 1)", bool(true));
    assert_eval("(>= 2 1)", bool(true));
    assert_eval("(>= 1 1)", bool(true));
    assert_eval("(>= 1 2 1)", bool(false));
    assert_eval("(>= 1 3 2 1)", bool(false));
    assert_eval("(>= 5 4 3 3 1)", bool(true));
}
