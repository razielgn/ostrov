use helpers::*;

#[test]
fn empty_list() {
    assert_parse("()", list(vec!()));
}

#[test]
fn empty_list_with_brackets() {
    assert_parse("[]", list(vec!()));
}

#[test]
fn list_of_integers() {
    assert_parse("(1 2 3 4 5)", list(vec!(integer(1), integer(2), integer(3), integer(4), integer(5))));
}

#[test]
fn list_of_integers_with_brackets() {
    assert_parse("[1 2 3 4 5]", list(vec!(integer(1), integer(2), integer(3), integer(4), integer(5))));
}

#[test]
fn list_of_atom() {
    assert_parse("(a !a lol? a=>3)", list(vec!(atom("a"), atom("!a"), atom("lol?"), atom("a=>3"))));
}

#[test]
fn list_of_atom_with_brackets() {
    assert_parse("[a !a lol? a=>3]", list(vec!(atom("a"), atom("!a"), atom("lol?"), atom("a=>3"))));
}

#[test]
fn list_of_lists() {
    assert_parse(
        "(1 (a b) ((c d e)))",
        list(vec!(integer(1),
            list(vec!(atom("a"), atom("b"))),
            list(vec!(
                list(vec!(atom("c"), atom("d"), atom("e")))
            ))
        ))
    );
}

#[test]
fn list_of_lists_with_brackets() {
    assert_parse(
        "[1 [a b] [[c d e]]]",
        list(vec!(integer(1),
            list(vec!(atom("a"), atom("b"))),
            list(vec!(
                list(vec!(atom("c"), atom("d"), atom("e")))
            ))
        ))
    );
}

#[test]
fn list_with_special_spaces() {
    assert_parse("(1  2)", list(vec!(integer(1), integer(2))));
    assert_parse("(1\t2)", list(vec!(integer(1), integer(2))));
    assert_parse("(1\t\t2)", list(vec!(integer(1), integer(2))));
    assert_parse("(1\r2)", list(vec!(integer(1), integer(2))));
    assert_parse("(1\r\r2)", list(vec!(integer(1), integer(2))));
    assert_parse("(1\n2)", list(vec!(integer(1), integer(2))));
    assert_parse("(1\n\n2)", list(vec!(integer(1), integer(2))));
    assert_parse("(1 \t \r \n 2)", list(vec!(integer(1), integer(2))));
}

#[test]
fn list_with_special_spaces_and_brackets() {
    assert_parse("[1  2]", list(vec!(integer(1), integer(2))));
    assert_parse("[1\t2]", list(vec!(integer(1), integer(2))));
    assert_parse("[1\t\t2]", list(vec!(integer(1), integer(2))));
    assert_parse("[1\r2]", list(vec!(integer(1), integer(2))));
    assert_parse("[1\r\r2]", list(vec!(integer(1), integer(2))));
    assert_parse("[1\n2]", list(vec!(integer(1), integer(2))));
    assert_parse("[1\n\n2]", list(vec!(integer(1), integer(2))));
    assert_parse("[1 \t \r \n 2]", list(vec!(integer(1), integer(2))));
}
