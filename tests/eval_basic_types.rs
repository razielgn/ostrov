use helpers::*;
use helpers::values::*;

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
fn atoms() {
    assert_eval_err("atom", unbound_variable_error("atom"));
}

#[test]
fn dotted_lists() {
    assert_eval_err("(1 . 2)", irreducible_value(dotted_list(vec!(integer(1)), integer(2))));
}
