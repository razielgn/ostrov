use helpers::*;

#[test]
fn integers() {
    assert_eval("0", integer(0));

    assert_eval_err("(0)", unappliable_value_error(integer(0)));
}

#[test]
fn booleans() {
    assert_eval("#t", bool(true));

    assert_eval_err("(#t)", unappliable_value_error(bool(true)));
}

#[test]
fn empty_list_() {
    assert_eval("()", empty_list());
}

#[test]
fn plus_procedure() {
    assert_eval("(+)", integer(0));
    assert_eval("(+ 2)", integer(2));
    assert_eval("(+ 2 3)", integer(5));
    assert_eval("(+ 2 3 -9)", integer(-4));
    assert_eval("(+ 2 3 -9 1)", integer(-3));

    assert_eval_err("(+ ())", wrong_argument_type(empty_list()));
}

#[test]
fn minus_procedure() {
    assert_eval("(- 2)", integer(-2));
    assert_eval("(- 2 3)", integer(-1));
    assert_eval("(- 2 3 -9)", integer(8));
    assert_eval("(- 2 3 -9 1)", integer(7));

    assert_eval_err("(-)", bad_arity("-"));
    assert_eval_err("(- ())", wrong_argument_type(empty_list()));
}

#[test]
fn product_procedure() {
    assert_eval("(*)", integer(1));
    assert_eval("(* 2)", integer(2));
    assert_eval("(* 2 3)", integer(6));
    assert_eval("(* 2 3 -9)", integer(-54));
    assert_eval("(* 2 3 -9 2)", integer(-108));

    assert_eval_err("(* ())", wrong_argument_type(empty_list()));
}

#[test]
fn division_procedure() {
    assert_eval("(/ 2)", integer(0));
    assert_eval("(/ 9 3)", integer(3));
    assert_eval("(/ 9 3 3)", integer(1));
    assert_eval("(/ 27 3 3 3)", integer(1));

    assert_eval_err("(/)", bad_arity("/"));
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
fn atoms() {
    assert_eval_err("atom", unbound_variable_error("atom"));
}
