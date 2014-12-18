use helpers::*;

#[test]
fn integers() {
    assert_fmt("1", values::integer(1));
    assert_fmt("-213", values::integer(-213));
}

#[test]
fn booleans() {
    assert_fmt("#t", values::bool(true));
    assert_fmt("#f", values::bool(false));
}

#[test]
fn atoms() {
    assert_fmt("->", values::atom("->"));
}

#[test]
fn lists() {
    assert_fmt("()", values::empty_list());
    assert_fmt("(+ 1 2 #f (1 2))", values::list(vec!(values::atom("+"),
                                                     values::integer(1),
                                                     values::integer(2),
                                                     values::bool(false),
                                                     values::list(vec!(values::integer(1),
                                                                       values::integer(2))))));
}

#[test]
fn dotted_lists() {
    assert_fmt("(+ (1 2) . a)", values::dotted_list(vec!(values::atom("+"),
                                                         values::list(vec!(values::integer(1),
                                                                           values::integer(2)))),
                                                         values::atom("a")));
}

#[test]
fn procedures() {
    assert_fmt("<procedure foo (bar baz)>",
               values::func("foo", vec!("bar", "baz"), integer(1)));
}

#[test]
fn lambdas() {
    assert_fmt("<lambda (bar baz)>",
               values::lambda(vec!("bar", "baz"), integer(1)));
}
