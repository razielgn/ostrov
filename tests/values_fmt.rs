use helpers::*;
use helpers::values::*;

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
    assert_fmt("()", nil());
    assert_fmt(
        "(+ 1 2 #f (1 2))",
        pair(
            atom("+"),
            pair(
                integer(1),
                pair(
                    integer(2),
                    pair(bool(false),
                         pair(
                             pair(
                                 integer(1),
                                 pair(
                                     integer(2),
                                     nil())),
                             nil()))))));
}

#[test]
fn dotted_lists() {
    assert_fmt(
        "(+ (1 2) . a)",
        pair(
            atom("+"),
            pair(
                pair(
                    integer(1),
                    pair(
                        integer(2),
                        nil())
                ),
                atom("a"))));
}

#[test]
fn procedures() {
    assert_fmt("<procedure foo (bar baz)>",
               func("foo", vec!("bar", "baz")));
}

#[test]
fn lambdas() {
    assert_fmt("<lambda (bar baz)>",
               lambda(vec!("bar", "baz")));
    assert_fmt("<lambda (bar . baz)>",
               lambda_var(vec!("bar", "baz")));
    assert_fmt("<lambda bar>",
               lambda_any("bar"));
}

#[test]
fn primitive() {
    assert_fmt("<primitive procedure +>", primitive_func("+"));
}
