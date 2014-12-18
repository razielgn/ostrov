use helpers::*;
use helpers::ast::*;

#[test]
fn simple() {
    assert_parse("(1 . 2)", dotted_list(vec!(integer(1)), integer(2)));
    assert_parse("[1 . 2]", dotted_list(vec!(integer(1)), integer(2)));
}

#[test]
fn complex() {
    assert_parse("(1 . (2 . (3 . 4)))", dotted_list(vec!(integer(1), integer(2), integer(3)), integer(4)));
    assert_parse("[1 . [2 . [3 . 4]]]", dotted_list(vec!(integer(1), integer(2), integer(3)), integer(4)));
}

#[test]
fn whitespace() {
    assert_parse(" \n\r\t(1\t\n\r .\t\n\r 2) \n\r\t", dotted_list(vec!(integer(1)), integer(2)));
    assert_parse(" \n\r\t[1\t\n\r . \t\n\r 2] \n\r\t", dotted_list(vec!(integer(1)), integer(2)));
}
