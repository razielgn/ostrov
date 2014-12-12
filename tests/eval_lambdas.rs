use helpers::*;

#[test]
fn creation_with_fixed_arguments_number() {
    assert_eval("(lambda (x y z) 1)",
                 lambda(vec!("x", "y", "z"), integer(1)));
}

#[test]
fn bad_arity_() {
    assert_eval_err("(lambda)", bad_arity("lambda"));
    assert_eval_err("(lambda 1)", bad_arity("lambda"));
    assert_eval_err("(lambda 1 2 3)", bad_arity("lambda"));
}

#[test]
fn bad_arguments() {
    assert_eval_err("(lambda 9 1)", wrong_argument_type(integer(9)));
    assert_eval_err("(lambda (1 2 3) 1)", wrong_argument_type(integer(1)));
}
