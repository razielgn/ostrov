use helpers::*;
use ostrov::parser::parse;

#[test]
fn multiple_expressions() {
    assert_eq!(
        Ok(vec!(integer(1), integer(2))),
        parse("1 2")
    );
}
