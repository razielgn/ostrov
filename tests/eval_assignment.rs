use helpers::*;
use helpers::values::*;

#[test]
fn returns_expression() {
    assert_eval_vm_val("(define x 0)
                        (set! x (+ x 1))", unspecified());
}

#[test]
fn overwrites_variables() {
    assert_eval_vm("(define x 0)
                    (set! x (+ x 1))
                    x", "1");
}

#[test]
fn overwrites_variables_on_upper_scopes() {
    assert_eval("(define x 0)
                 (define (f) (set! x (+ x 1)))
                 (f)
                 x", "1");
}

#[test]
fn overwrites_variables_in_captured_scopes() {
    assert_eval("(define incr 1)
                 (define (f)
                   (define x 0)
                   (lambda ()
                     (set! x (+ x incr))
                     x))
                 (define a (f))
                 (a)
                 (a)
                 (a)", "3");
}

#[test]
fn malformed_variable_name() {
    assert_eval_vm_err("(set! 3 3)", malformed_expr());
}

#[test]
fn unknown_variable() {
    assert_eval_vm_err("(set! x 3)", unbound_variable_error("x"));
}

#[test]
fn wrong_arguments_number() {
    assert_eval_vm_err("(set!)", bad_arity("set!"));
    assert_eval_vm_err("(set! x)", bad_arity("set!"));
    assert_eval_vm_err("(set! x 2 3)", bad_arity("set!"));
}
