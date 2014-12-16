use helpers::*;

#[test]
fn list_() {
    assert_eval("(list)", list(vec!()));
    assert_eval("(list (+ 0 1) 2 3)", list(vec!(integer(1), integer(2), integer(3))));
}

#[test]
fn length() {
    assert_eval("(length '())", integer(0));
    assert_eval("(length '(1 2 3))", integer(3));
    assert_eval("(length (list (+ 0 1)))", integer(1));
}

#[test]
fn length_bad_arity() {
    assert_eval_err("(length '() '())", bad_arity("length"));
}

#[test]
fn length_bad_arguments() {
    assert_eval_err("(length 1)", wrong_argument_type(integer(1)));
}

#[test]
fn pair() {
    assert_eval("(pair? 1)", bool(false));
    assert_eval("(pair? #t)", bool(false));
    assert_eval("(pair? '())", bool(false));
    assert_eval("(pair? (lambda (x) 0))", bool(false));
    assert_eval("(pair? '(1 2 3))", bool(true));
    assert_eval("(pair? '(1 2 . 3))", bool(true));
}

#[test]
fn pair_bad_arity() {
    assert_eval_err("(pair?)", bad_arity("pair?"));
    assert_eval_err("(pair? 1 2)", bad_arity("pair?"));
}

#[test]
fn cons() {
    assert_eval("(cons 'a '())", list(vec!(atom("a"))));
    assert_eval("(cons '(a) '(b c d))", list(vec!(list(vec!(atom("a"))),
                                                  atom("b"),
                                                  atom("c"),
                                                  atom("d"))));
    assert_eval("(cons 1 '(b c))", list(vec!(integer(1),
                                             atom("b"),
                                             atom("c"))));
    assert_eval("(cons 'a 3)", dotted_list(vec!(atom("a")), integer(3)));
    assert_eval("(cons '(a b) 'c)", dotted_list(vec!(list(vec!(atom("a"),
                                                               atom("b")))),
                                                atom("c")));
}

#[test]
fn cons_bad_arity() {
    assert_eval_err("(cons)", bad_arity("cons"));
    assert_eval_err("(cons 1)", bad_arity("cons"));
    assert_eval_err("(cons 1 2 3)", bad_arity("cons"));
}

#[test]
fn car() {
    assert_eval("(car '(a b c))", atom("a"));
    assert_eval("(car '(1 . 2))", integer(1));
}

#[test]
fn car_bad_arity() {
    assert_eval_err("(car)", bad_arity("car"));
    assert_eval_err("(car '(a) '(b))", bad_arity("car"));
}

#[test]
fn car_wrong_argument_type() {
    assert_eval_err("(car 12)", wrong_argument_type(integer(12)));
    assert_eval_err("(car '())", wrong_argument_type(empty_list()));
}
