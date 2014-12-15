use helpers::*;

#[test]
fn list_() {
    assert_eval("(list)", list(vec!()));
    assert_eval("(list (+ 0 1) 2 3)", list(vec!(integer(1), integer(2), integer(3))));
}

#[test]
fn length() {
    assert_eval("(length '())", integer(0));
    assert_eval("(length '(1 2 3))", integer(3));
    assert_eval("(length (list (+ 0 1)))", integer(1));
}

#[test]
fn length_bad_arity() {
    assert_eval_err("(length '() '())", bad_arity("length"));
}

#[test]
fn length_bad_arguments() {
    assert_eval_err("(length 1)", wrong_argument_type(integer(1)));
}
