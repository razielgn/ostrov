use helpers::*;

#[test]
fn integers() {
    assert_eval_err("(0 1 2)", unappliable_value_error(integer(0)));
}

#[test]
fn booleans() {
    assert_eval_err("(#t #f #t)", unappliable_value_error(bool(true)));
}

#[test]
#[should_fail]
fn atoms() {
    assert_eval_err("(a b c)", unbound_variable_error("a"));
}

#[test]
#[should_fail]
fn lists() {
    assert_eval_err("('(1))", unappliable_value_error(list(vec!(integer(1)))));
}

#[test]
#[should_fail]
fn dotted_lists() {
    assert_eval_err("('(1 . 2))", unappliable_value_error(dotted_list(vec!(integer(1)), integer(2))));
}
