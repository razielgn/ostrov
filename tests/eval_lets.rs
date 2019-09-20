use crate::helpers::*;
use ostrov::errors::RuntimeError::*;

#[test]
fn one_definition() {
    assert_eval(
        "(let ((x 2)
               (y 3))
           (* x y))",
        "6",
    );

    assert_eval(
        "(let ((x 2)
               (y 3))
           (let ((x 7)
             (z (+ x y)))
           (* z x)))",
        "35",
    );
}

#[test]
fn bad_arity_() {
    assert_eval_err("(let)", BadArity(Some("let".into())));
    assert_eval_err("(let 1)", BadArity(Some("let".into())));
}

#[test]
fn malformed() {
    assert_eval_err("(let 1 2)", MalformedExpression);
    assert_eval_err("(let a 2)", MalformedExpression);
    assert_eval_err("(let (a) 2)", MalformedExpression);
    assert_eval_err("(let ((a)) 2)", MalformedExpression);
    assert_eval_err("(let ((a) b) 2)", MalformedExpression);
}
