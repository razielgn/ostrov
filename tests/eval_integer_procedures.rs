use helpers::*;
use helpers::values::*;

#[test]
fn plus() {
    assert_eval("(+)", "0");
    assert_eval("(+ 2)", "2");
    assert_eval("(+ 2 3)", "5");
    assert_eval("(+ 2 3 -9)", "-4");
    assert_eval("(+ 2 3 -9 1)", "-3");
}

#[test]
fn plus_bad_arity() {
    assert_eval_err("(+ ())", wrong_argument_type(nil()));
}

#[test]
fn minus() {
    assert_eval("(- 2)", "-2");
    assert_eval("(- 2 3)", "-1");
    assert_eval("(- 2 3 -9)", "8");
    assert_eval("(- 2 3 -9 1)", "7");
}

#[test]
fn minus_bad_arity() {
    assert_eval_err("(-)", bad_arity("-"));
}

#[test]
fn minus_wrong_argument_type() {
    assert_eval_err("(- ())", wrong_argument_type(nil()));
}

#[test]
fn product() {
    assert_eval("(*)", "1");
    assert_eval("(* 2)", "2");
    assert_eval("(* 2 3)", "6");
    assert_eval("(* 2 3 -9)", "-54");
    assert_eval("(* 2 3 -9 2)", "-108");
}

#[test]
fn product_wrong_argument_type() {
    assert_eval_err("(* ())", wrong_argument_type(nil()));
}

#[test]
fn division() {
    assert_eval("(/ 2)", "0");
    assert_eval("(/ 9 3)", "3");
    assert_eval("(/ 9 3 3)", "1");
    assert_eval("(/ 27 3 3 3)", "1");
}

#[test]
fn division_bad_arity() {
    assert_eval_err("(/)", bad_arity("/"));
}

#[test]
fn division_wrong_argument_type() {
    assert_eval_err("(/ ())", wrong_argument_type(nil()));
}

#[test]
fn equal_sign() {
    assert_eval("(=)", "#t");
    assert_eval("(= 1)", "#t");
    assert_eval("(= 23 23)", "#t");
    assert_eval("(= 23 42)", "#f");
    assert_eval("(= 23 23 42)", "#f");
    assert_eval("(= 23 23 23 42)", "#f");
    assert_eval("(= 23 23 23 23)", "#t");
}

#[test]
fn less_than_sign() {
    assert_eval("(<)", "#t");
    assert_eval("(< 1)", "#t");
    assert_eval("(< 1 2)", "#t");
    assert_eval("(< 1 1)", "#f");
    assert_eval("(< 1 2 1)", "#f");
    assert_eval("(< 1 2 3 1)", "#f");
    assert_eval("(< 1 2 3 4 5)", "#t");
}

#[test]
fn less_than_or_equal_sign() {
    assert_eval("(<=)", "#t");
    assert_eval("(<= 1)", "#t");
    assert_eval("(<= 1 2)", "#t");
    assert_eval("(<= 1 1)", "#t");
    assert_eval("(<= 1 2 1)", "#f");
    assert_eval("(<= 1 2 3 1)", "#f");
    assert_eval("(<= 1 3 3 4 5)", "#t");
}

#[test]
fn greater_than_sign() {
    assert_eval("(>)", "#t");
    assert_eval("(> 1)", "#t");
    assert_eval("(> 2 1)", "#t");
    assert_eval("(> 1 1)", "#f");
    assert_eval("(> 1 2 1)", "#f");
    assert_eval("(> 1 3 2 1)", "#f");
    assert_eval("(> 5 4 3 2 1)", "#t");
}

#[test]
fn greater_than_or_equal_sign() {
    assert_eval("(>=)", "#t");
    assert_eval("(>= 1)", "#t");
    assert_eval("(>= 2 1)", "#t");
    assert_eval("(>= 1 1)", "#t");
    assert_eval("(>= 1 2 1)", "#f");
    assert_eval("(>= 1 3 2 1)", "#f");
    assert_eval("(>= 5 4 3 3 1)", "#t");
}
