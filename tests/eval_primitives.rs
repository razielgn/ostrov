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
fn lists() {
    assert_eval_err("()", irreducible_val_error(list(vec!())));
}

#[test]
fn atoms() {
    assert_eval_err("atom", irreducible_val_error(atom("atom")));
}
