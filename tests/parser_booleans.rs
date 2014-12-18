use helpers::*;
use helpers::ast::*;

#[test]
fn downcase() {
    assert_parse("#t", bool(true));
    assert_parse("#f", bool(false));
}

#[test]
fn upcase() {
    assert_parse("#T", bool(true));
    assert_parse("#F", bool(false));
}

#[test]
fn whitespace() {
    assert_parse(" \n\r\t#t \n\r\t", bool(true));
}
