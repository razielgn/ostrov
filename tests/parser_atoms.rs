use parser_helpers::*;

#[test]
fn letters() {
    assert_parse("a", atom("a"));
    assert_parse("z", atom("z"));
    assert_parse("A", atom("A"));
    assert_parse("Z", atom("Z"));
    assert_parse("test", atom("test"));
}

#[test]
fn letters_and_numbers() {
    assert_parse("a25", atom("a25"));
    assert_parse("a25", atom("a25"));
    assert_parse("Zep0", atom("Zep0"));
}

#[test]
fn special_initials() {
    assert_parse("!", atom("!"));
    assert_parse("$", atom("$"));
    assert_parse("%", atom("%"));
    assert_parse("*", atom("*"));
    assert_parse("/", atom("/"));
    assert_parse("<", atom("<"));
    assert_parse("=", atom("="));
    assert_parse(">", atom(">"));
    assert_parse("?", atom("?"));
    assert_parse("^", atom("^"));
    assert_parse("_", atom("_"));
    assert_parse("~", atom("~"));
}

#[test]
fn subsequent() {
    assert_parse("str=>int", atom("str=>int"));
    assert_parse("true?", atom("true?"));
}

#[test]
fn special_subsequent() {
    assert_parse("what+is+this", atom("what+is+this"));
    assert_parse("what-is-this", atom("what-is-this"));
    assert_parse("what.is.this", atom("what.is.this"));
    assert_parse("what@is@this", atom("what@is@this"));
}

#[test]
fn peculiar_identifiers() {
    assert_parse("+", atom("+"));
    assert_parse("-", atom("-"));
    assert_parse("...", atom("..."));
    assert_parse("->", atom("->"));
    assert_parse("->test2", atom("->test2"));
}
