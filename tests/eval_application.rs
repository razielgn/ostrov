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
fn lists() {
    assert_eval_err("('(1))", unappliable_value_error(list(vec!(atom("quote"),
                                                                list(vec!(integer(1)))))));
}

#[test]
fn expressions_in_first_position() {
    assert_eval("(define (x) 1)
                 ((if #t x))", integer(1));
}

#[test]
fn procedure_with_no_args() {
    assert_eval("(define (x) 1)
                 (x)", integer(1));
}

#[test]
fn procedure_with_one_arg() {
    assert_eval("(define (x y) y)
                 (x (+ 2 9))", integer(11));
}

#[test]
fn procedure_with_two_args() {
    assert_eval("(define (sum x y) (+ x y))
                 (sum 4 9)", integer(13));
}

#[test]
fn procedure_with_two_args_scoping() {
    assert_eval("(define x 10)
                 (define (sum x y) (+ x y))
                 (sum 4 9)", integer(13));
}

#[test]
fn procedure_with_two_args_previous_scoping_is_kept() {
    assert_eval("(define x 10)
                 (define (sum x y) (+ x y))
                 (sum 4 9)
                 x", integer(10));
}

#[test]
fn procedure_with_mismatched_arity() {
    assert_eval_err("(define (sum x y) (+ x y))
                     (sum 4)", bad_arity("sum"));
}
