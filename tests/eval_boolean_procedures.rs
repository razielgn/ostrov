use helpers::*;

#[test]
fn and() {
    assert_eval_vm("(and)", "#t");
    assert_eval_vm("(and (+ 2 3))", "5");
    assert_eval_vm("(and #f)", "#f");
    assert_eval_vm("(and #f 2)", "#f");
    assert_eval_vm("(and #t 2)", "2");
    assert_eval_vm("(and 1 #f a)", "#f");
}

#[test]
fn or() {
    assert_eval_vm("(or)", "#f");
    assert_eval_vm("(or (+ 2 3))", "5");
    assert_eval_vm("(or #f 2)", "2");
    assert_eval_vm("(or 1 a)", "1");
}

#[test]
fn not() {
    assert_eval_vm("(not #f)", "#t");
    assert_eval_vm("(not #t)", "#f");
    assert_eval_vm("(not 2)", "#f");
    assert_eval_vm("(not 'a)", "#f");
}

#[test]
fn not_bad_arity() {
    assert_eval_vm_err("(not)", bad_arity("not"));
    assert_eval_vm_err("(not 2 3)", bad_arity("not"));
}
