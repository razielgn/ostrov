use helpers::*;
use helpers::values::*;

#[test]
fn if_with_one_arg() {
    assert_eval_vm("(if #t 3)", "3");
    assert_eval_vm_val("(if #f 3)", unspecified());
}

#[test]
fn if_with_two_args() {
    assert_eval_vm("(if #t (+ 1) a)", "1");
    assert_eval_vm("(if #f a (+ 1))", "1");
    assert_eval_vm("(if (and #t #f) a 1)", "1");
}

#[test]
fn if_bad_arity() {
    assert_eval_vm_err("(if)", bad_arity("if"));
    assert_eval_vm_err("(if a b c d)", bad_arity("if"));
}
