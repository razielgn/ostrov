use helpers::values::*;
use helpers::*;

#[test]
fn define_with_one_arg() {
    assert_eval_val("(define x)", unspecified());
    assert_eval_val(
        "(define x)
                        x",
        unspecified(),
    );
}

#[test]
fn define_with_two_args() {
    assert_eval_val("(define x 2)", unspecified());

    assert_eval(
        "(define x 2)
                    (= x 2)",
        "#t",
    );
    assert_eval(
        "(define x 9)
                    (define x 10)
                    x",
        "10",
    );
    assert_eval(
        "(define x 9)
                    (define y x)
                    y",
        "9",
    );
}

#[test]
fn define_with_one_arg_lambda() {
    assert_eval(
        "(define f (lambda (x) 1))
                    (f 9)",
        "1",
    );
}

#[test]
fn define_procedure_with_fixed_args() {
    assert_eval_val("(define (x) 3)", unspecified());
}

#[test]
fn define_procedure_with_varargs() {
    assert_eval_val("(define (x a . b) 3)", unspecified());
}

#[test]
fn define_procedure_with_any() {
    assert_eval_val("(define (x . b) 3)", unspecified());
}

#[test]
fn define_bad_arity() {
    assert_eval_err("(define)", malformed_expr());
}
