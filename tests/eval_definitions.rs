use helpers::*;
use helpers::values::*;

#[test]
fn define_with_one_arg() {
    assert_eval("(define x)", "'x"); // unspecified behaviour
}

#[test]
fn define_with_two_args() {
    assert_eval("(define x 2)", "2"); // unspecified behaviour

    assert_eval("(define x 2)
                 (= x 2)", "#t");
    assert_eval("(define x 9)
                 (define x 10)
                 x", "10");
    assert_eval("(define x 9)
                 (define y x)
                 y", "9");
}

#[test]
fn define_with_one_arg_lambda() {
    assert_eval_val("(define f (lambda (x) 1))
                     f", func("f", vec!("x"), vec!(ast::integer(1))));
    assert_eval("(define f (lambda (x) 1))
                 (f 9)", "1");
}

#[test]
fn define_procedure_with_fixed_args() {
    assert_eval("(define (x) 3)", "'x"); // unspecified behaviour

    assert_eval_val("(define (x) 3)
                     x", func("x", vec!(), vec!(ast::integer(3))));
}

#[test]
fn define_procedure_with_varargs() {
    assert_eval("(define (x a . b) 3)", "'x"); // unspecified behaviour

    assert_eval_val("(define (x a . b) 3)
                     x", func_var("x", vec!("a", "b"), vec!(ast::integer(3))));
}

#[test]
fn define_procedure_with_any() {
    assert_eval("(define (x . b) 3)", "'x"); // unspecified behaviour

    assert_eval_val("(define (x . b) 3)
                     x", func_any("x", "b", vec!(ast::integer(3))));
}

#[test]
fn define_procedure_with_multiple_expressions() {
    assert_eval_val("(define (foo . x) 3 2 1)
                     foo", func_any("foo", "x", vec!(ast::integer(3),
                                                     ast::integer(2),
                                                     ast::integer(1))));
}

#[test]
fn define_bad_arity() {
    assert_eval_err("(define)", bad_arity("define"));
}
