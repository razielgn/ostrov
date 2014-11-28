use helpers::*;

#[test]
fn integers() {
    assert_eval("0", integer(0));
}

#[test]
fn booleans() {
    assert_eval("#t", bool(true));
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
fn atoms() {
    assert_eval_err("atom", unbound_variable_error("atom"));
}
