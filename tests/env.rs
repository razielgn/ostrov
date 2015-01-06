use ostrov::memory::Memory;
use ostrov::env::Env;

#[test]
fn set_and_get_ok() {
    let mut mem = Memory::new();
    let env = Env::new();
    let name = "foo".to_string();

    let val = mem.integer(3);
    env.borrow_mut().set(name.clone(), val.clone());

    assert_eq!(Some(val), env.borrow().get(&name));
}

#[test]
fn set_and_get_none() {
    let env = Env::new();

    assert_eq!(None, env.borrow().get(&"foo".to_string()));
}

#[test]
fn wraps() {
    let mut mem = Memory::new();
    let outer = Env::new();
    let outer_foo = mem.integer(5);
    let bar = mem.intern("bar".to_string());

    outer.borrow_mut().set("foo".to_string(), outer_foo.clone());
    outer.borrow_mut().set("bar".to_string(), bar.clone());

    let inner = Env::wraps(outer);
    let inner_foo = mem.integer(25);

    inner.borrow_mut().set("foo".to_string(), inner_foo.clone());

    assert_eq!(Some(inner_foo), inner.borrow().get(&"foo".to_string()));
    assert_eq!(Some(bar), inner.borrow().get(&"bar".to_string()));
}
