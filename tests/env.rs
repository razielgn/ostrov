use ostrov::memory::Memory;
use ostrov::env::CellEnv;

#[test]
fn set_and_get_ok() {
    let mut mem = Memory::new();
    let env = CellEnv::new();
    let name = "foo".to_owned();

    let val = mem.integer(3);
    env.set(name.clone(), val.clone());

    assert_eq!(Some(val), env.get(&name));
}

#[test]
fn set_and_get_none() {
    let env = CellEnv::new();

    assert_eq!(None, env.get(&"foo".to_owned()));
}

#[test]
fn wraps() {
    let mut mem = Memory::new();
    let outer = CellEnv::new();
    let outer_foo = mem.integer(5);
    let bar = mem.intern("bar".to_owned());

    outer.set("foo".to_owned(), outer_foo.clone());
    outer.set("bar".to_owned(), bar.clone());

    let inner = CellEnv::wraps(outer);
    let inner_foo = mem.integer(25);

    inner.set("foo".to_owned(), inner_foo.clone());

    assert_eq!(Some(inner_foo), inner.get(&"foo".to_owned()));
    assert_eq!(Some(bar), inner.get(&"bar".to_owned()));
}
