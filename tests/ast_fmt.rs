use helpers::*;

#[test]
fn integers() {
    assert_fmt("1", integer(1));
    assert_fmt("-213", integer(-213));
}

#[test]
fn booleans() {
    assert_fmt("#t", bool(true));
    assert_fmt("#f", bool(false));
}

#[test]
fn atoms() {
    assert_fmt("->", atom("->"));
}

#[test]
fn lists() {
    assert_fmt("()", empty_list());
    assert_fmt("(+ 1 2 #f (1 2))", list(vec!(atom("+"),
                                             integer(1),
                                             integer(2),
                                             bool(false),
                                             list(vec!(integer(1),
                                                       integer(2))))));
}