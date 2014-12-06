use helpers::*;

#[test]
fn empty_list_() {
    assert_parse("(1 . 2)", dotted_list(vec!(integer(1)), integer(2)));
    assert_parse("[1 . 2]", dotted_list(vec!(integer(1)), integer(2)));
    assert_parse("[1 . #t]", dotted_list(vec!(integer(1)), bool(true)));
}

#[test]
fn whitespace() {
    assert_parse(" \n\r\t(1\t\n\r .\t\n\r 2) \n\r\t", dotted_list(vec!(integer(1)), integer(2)));
    assert_parse(" \n\r\t[1\t\n\r . \t\n\r 2] \n\r\t", dotted_list(vec!(integer(1)), integer(2)));
}
