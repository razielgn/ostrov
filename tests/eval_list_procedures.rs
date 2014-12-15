use helpers::*;

#[test]
fn list_() {
    assert_eval("(list)", list(vec!()));
    assert_eval("(list (+ 0 1) 2 3)", list(vec!(integer(1), integer(2), integer(3))));
}
