use helpers::*;
use helpers::values::*;

#[test]
fn fixed_arguments() {
    assert_eval_val("(lambda () 1)", lambda(vec!(), vec!(ast::integer(1))));
    assert_eval_val("(lambda (x y z) 1)", lambda(vec!("x", "y", "z"), vec!(ast::integer(1))));
}

#[test]
fn variable_arguments() {
    assert_eval_val("(lambda (h . t) 1)", lambda_var(vec!("h", "t"), vec!(ast::integer(1))));
    assert_eval_val("(lambda (h g . t) 1)", lambda_var(vec!("h", "g", "t"), vec!(ast::integer(1))));
}

#[test]
fn any_arguments() {
    assert_eval_val("(lambda h 1)", lambda_any("h", vec!(ast::integer(1))));
}

#[test]
fn multiple_exprs_in_body() {
    assert_eval_val("(lambda h 1 2 3)", lambda_any("h", vec!(ast::integer(1), ast::integer(2), ast::integer(3))));
}

#[test]
fn bad_arity_() {
    assert_eval_err("(lambda)", bad_arity("lambda"));
    assert_eval_err("(lambda 1)", bad_arity("lambda"));
}

#[test]
fn bad_arguments() {
    assert_eval_err("(lambda 9 1)", wrong_argument_type(integer(9)));
    assert_eval_err("(lambda (1 2 3) 1)", wrong_argument_type(integer(1)));
}
