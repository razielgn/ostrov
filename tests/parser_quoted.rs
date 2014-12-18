use helpers::*;
use helpers::ast::*;

#[test]
fn any_value() {
    assert_parse("'1", list(vec!(atom("quote"), integer(1))));
    assert_parse("''atom", list(vec!(atom("quote"),
                               list(vec!(atom("quote"), atom("atom"))))));
}

#[test]
fn whitespace() {
    assert_parse(" \n\r\t'1 \n\r\t", list(vec!(atom("quote"), integer(1))));
}
