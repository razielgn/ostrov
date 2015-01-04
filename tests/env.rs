use ostrov::env::Env;
use ostrov::memory::Memory;

#[test]
fn set_and_get_ok() {
    let mut mem = Memory::new();
    let mut env = Env::new();
    let name = "foo".to_string();

    let val = mem.integer(3);
    env.set(name.clone(), val.clone());

    assert_eq!(Some(val), env.get(&name));
}

#[test]
fn set_and_get_none() {
    let env = Env::new();

    assert_eq!(None, env.get(&"foo".to_string()));
}

#[test]
fn wraps() {
    let mut mem = Memory::new();
    let mut outer = Env::new();
    let outer_foo = mem.integer(5);
    let bar = mem.intern("bar".to_string());

    outer.set("foo".to_string(), outer_foo.clone());
    outer.set("bar".to_string(), bar.clone());

    let mut inner = Env::wraps(&outer);
    let inner_foo = mem.integer(25);

    inner.set("foo".to_string(), inner_foo.clone());

    assert_eq!(Some(inner_foo), inner.get(&"foo".to_string()));
    assert_eq!(Some(bar), inner.get(&"bar".to_string()));
}
