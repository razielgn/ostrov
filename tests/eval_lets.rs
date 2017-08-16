use helpers::*;

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
    assert_eval_err("(let)", bad_arity("let"));
    assert_eval_err("(let 1)", bad_arity("let"));
}

#[test]
fn malformed() {
    assert_eval_err("(let 1 2)", malformed_expr());
    assert_eval_err("(let a 2)", malformed_expr());
    assert_eval_err("(let (a) 2)", malformed_expr());
    assert_eval_err("(let ((a)) 2)", malformed_expr());
    assert_eval_err("(let ((a) b) 2)", malformed_expr());
}
