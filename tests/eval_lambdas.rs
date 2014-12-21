use helpers::*;
use helpers::values::*;

#[test]
fn fixed_arguments() {
    assert_eval("(lambda () 1)", lambda(vec!(), ast::integer(1)));
    assert_eval("(lambda (x y z) 1)",
                 lambda(vec!("x", "y", "z"), ast::integer(1)));
}

#[test]
fn variable_arguments() {
    assert_eval("(lambda (h . t) 1)", lambda_var(vec!("h", "t"), ast::integer(1)));
    assert_eval("(lambda (h g . t) 1)", lambda_var(vec!("h", "g", "t"), ast::integer(1)));
}

#[test]
fn any_arguments() {
    assert_eval("(lambda h 1)", lambda_any("h", ast::integer(1)));
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
