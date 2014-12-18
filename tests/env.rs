use helpers::ast::*;

use ostrov::env::Env;

#[test]
fn set_and_get_ok() {
    let mut env = Env::new();
    let name = "foo".to_string();

    env.set(name.clone(), integer(3));

    assert_eq!(Some(&integer(3)), env.get(&name));
}

#[test]
fn set_and_get_none() {
    let env = Env::new();
    let name = "foo".to_string();

    assert_eq!(None, env.get(&name));
}

#[test]
fn wraps() {
    let mut outer = Env::new();
    let outer_foo = integer(5);
    let bar = atom("bar");

    outer.set("foo".to_string(), outer_foo);
    outer.set("bar".to_string(), bar);

    let mut inner = Env::wraps(&outer);
    let inner_foo = integer(25);

    inner.set("foo".to_string(), inner_foo);

    assert_eq!(Some(&integer(25)), inner.get(&"foo".to_string()));
    assert_eq!(Some(&atom("bar")), inner.get(&"bar".to_string()));
}
