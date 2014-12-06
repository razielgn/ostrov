use helpers::*;

#[test]
fn quote() {
    assert_eval("'1", integer(1));
    assert_eval("'a", atom("a"));
    assert_eval("'#t", bool(true));
    assert_eval("'(1)", list(vec!(integer(1))));
    assert_eval("'[1]", list(vec!(integer(1))));
}

#[test]
fn and() {
    assert_eval("(and)", bool(true));
    assert_eval("(and (+ 2 3))", integer(5));
    assert_eval("(and #t 2)", integer(2));
    assert_eval("(and 1 #f a)", bool(false));
}

#[test]
fn or() {
    assert_eval("(or)", bool(false));
    assert_eval("(or (+ 2 3))", integer(5));
    assert_eval("(or #f 2)", integer(2));
    assert_eval("(or 1 a)", integer(1));
}
