use helpers::*;
use helpers::values::*;

#[test]
fn define_with_one_arg() {
    assert_eval("(define x)", atom("x")); // unspecified behaviour
}

#[test]
fn define_with_two_args() {
    assert_eval("(define x 2)", integer(2)); // unspecified behaviour

    assert_eval("(define x 2)
                 (= x 2)", bool(true));
    assert_eval("(define x 9)
                 (define x 10)
                 x", integer(10));
    assert_eval("(define x 9)
                 (define y x)
                 y", integer(9));
}

#[test]
fn define_with_one_arg_lambda() {
    assert_eval("(define f (lambda (x) 1))
                 f", func("f", vec!("x"), ast::integer(1)));
    assert_eval("(define f (lambda (x) 1))
                 (f 9)", integer(1));
}

#[test]
fn define_procedure_with_fixed_args() {
    assert_eval("(define (x) 3)", atom("x")); // unspecified behaviour

    assert_eval("(define (x) 3)
                 x", func("x", vec!(), ast::integer(3)));
}

#[test]
fn define_procedure_with_varargs() {
    assert_eval("(define (x a . b) 3)", atom("x")); // unspecified behaviour

    assert_eval("(define (x a . b) 3)
                 x", func_var("x", vec!("a", "b"), ast::integer(3)));
}

#[test]
fn define_procedure_with_any() {
    assert_eval("(define (x . b) 3)", atom("x")); // unspecified behaviour

    assert_eval("(define (x . b) 3)
                 x", func_any("x", "b", ast::integer(3)));
}

#[test]
fn define_bad_arity() {
    assert_eval_err("(define)", bad_arity("define"));
    assert_eval_err("(define x 1 2)", bad_arity("define"));
}
