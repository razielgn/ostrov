use parser_helpers::*;

#[test]
fn unsigned_integers() {
    assert_parse("0", integer(0));
    assert_parse("1", integer(1));
    assert_parse("2", integer(2));
}

#[test]
fn signed_integers() {
    assert_parse("+0", integer(0));
    assert_parse("+1", integer(1));
    assert_parse("+2", integer(2));

    assert_parse("-0", integer(0));
    assert_parse("-1", integer(-1));
    assert_parse("-2", integer(-2));
}
