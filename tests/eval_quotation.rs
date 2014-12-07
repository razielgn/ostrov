use helpers::*;

#[test]
fn quote() {
    assert_eval("'1", integer(1));
    assert_eval("'a", atom("a"));
    assert_eval("'#t", bool(true));
    assert_eval("'(1)", list(vec!(integer(1))));
    assert_eval("'[1]", list(vec!(integer(1))));
}
