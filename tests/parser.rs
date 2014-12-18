use helpers::*;
use helpers::ast::*;

#[test]
fn multiple_expressions() {
    let runtime = Runtime::new();

    assert_eq!(
        Ok(vec!(integer(1), integer(2))),
        runtime.parse_str("1 2")
    );
}
