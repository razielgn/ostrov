use ostrov::parser::AST;
use ostrov::parser::parse;

#[test]
fn unsigned_integers() {
    assert_eq!(AST::Integer(0), parse("0"));
    assert_eq!(AST::Integer(1), parse("1"));
    assert_eq!(AST::Integer(2), parse("2"));
}

#[test]
fn signed_integers() {
    assert_eq!(AST::Integer(0), parse("+0"));
    assert_eq!(AST::Integer(1), parse("+1"));
    assert_eq!(AST::Integer(2), parse("+2"));

    assert_eq!(AST::Integer(0), parse("-0"));
    assert_eq!(AST::Integer(-1), parse("-1"));
    assert_eq!(AST::Integer(-2), parse("-2"));
}
