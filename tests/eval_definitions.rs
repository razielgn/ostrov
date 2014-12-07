use helpers::*;

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
fn define_bad_arity() {
    assert_eval_err("(define)", bad_arity("define"));
    assert_eval_err("(define x 1 2)", bad_arity("define"));
}
