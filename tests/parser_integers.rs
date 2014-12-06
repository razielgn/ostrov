use helpers::*;

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

#[test]
fn whitespace() {
    assert_parse(" \t\n\r0 \t\n\r", integer(0));
}
