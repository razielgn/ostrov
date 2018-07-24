use helpers::values::*;
use helpers::*;

#[test]
fn integers() {
    assert_eval_err("(0 1 2)", unappliable_value_error(integer(0)));
}

#[test]
fn dotted_lists() {
    assert_eval_err("(1 . 2)", malformed_expr());
}

#[test]
fn booleans() {
    assert_eval_err("(#t #f #t)", unappliable_value_error(bool(true)));
}

#[test]
fn atoms() {
    assert_eval_err("(a b c)", unbound_variable_error("b"));
}

#[test]
fn lists() {
    assert_eval_err("('(1))", unappliable_value_error(pair(integer(1), nil())));
}

#[test]
fn expressions_in_first_position() {
    assert_eval(
        "(define (x) 1)
                    ((if #t x))",
        "1",
    );
}

#[test]
fn procedure_with_no_args() {
    assert_eval(
        "(define (x) 1)
                    (x)",
        "1",
    );
}

#[test]
fn procedure_with_one_arg() {
    assert_eval(
        "(define (x y) y)
                    (x (+ 2 9))",
        "11",
    );
}

#[test]
fn procedure_with_two_args() {
    assert_eval(
        "(define (sum x y) (+ x y))
                    (sum 4 9)",
        "13",
    );
}

#[test]
fn procedure_with_var_args() {
    assert_eval(
        "(define (sum x . y) y)
                    (sum 4 9)",
        "'(9)",
    );
}

#[test]
fn procedure_with_any_args() {
    assert_eval(
        "(define (sum . y) y)
                    (sum 4 9)",
        "'(4 9)",
    );
}

#[test]
fn procedure_with_two_args_scoping() {
    assert_eval(
        "(define x 10)
                    (define (sum x y) (+ x y))
                    (sum 4 9)",
        "13",
    );
}

#[test]
fn procedure_with_two_args_previous_scoping_is_kept() {
    assert_eval(
        "(define x 10)
                    (define (sum x y) (+ x y))
                    (sum 4 9)
                    x",
        "10",
    );
}

#[test]
#[ignore]
fn procedure_with_mismatched_arity() {
    assert_eval_err(
        "(define (sum x y) (+ x y))
                        (sum 4)",
        bad_arity("sum"),
    );
}

#[test]
fn lambda_multiple_expressions_are_evaluated_sequentially() {
    assert_eval(
        "(define (foo)
                      (define a 10)
                      (define b (+ a 1))
                      (define c (+ b 1))
                      c)
                    (foo)",
        "12",
    );
}

#[test]
fn lambda_with_fixed_arguments_number() {
    assert_eval_err("((lambda ()))", malformed_expr());
    assert_eval("((lambda () 1))", "1");
    assert_eval("((lambda () 1 2 3))", "3");
    assert_eval("((lambda (x y) (+ x y)) 6 8)", "14");
}

#[test]
fn lambda_with_fixed_arguments_number_bad_arity() {
    assert_eval_err("((lambda (x y) (+ x y)) 6 8 9)", bad_arity_lambda());
}

#[test]
fn lambda_with_any_arguments_number() {
    assert_eval("((lambda x x))", "'()");
    assert_eval("((lambda x x) 1 2 3)", "'(1 2 3)");
}

#[test]
fn lambda_with_variable_arguments_number() {
    assert_eval("((lambda (x . y) x) 1)", "1");
    assert_eval("((lambda (x . y) y) 1)", "'()");
    assert_eval("((lambda (x . y) y) 1 2 3)", "'(2 3)");
    assert_eval("((lambda (x y . z) z) 1 2 3)", "'(3)");
}

#[test]
fn lambda_with_variable_arguments_number_bad_arity() {
    assert_eval_err("((lambda (x . y) 1))", bad_arity_lambda());
    assert_eval_err("((lambda (x y . z) 1) 1)", bad_arity_lambda());
}
