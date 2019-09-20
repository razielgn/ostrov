use crate::values::RcValue;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone)]
pub struct CellEnv(Rc<RefCell<Env>>);

impl CellEnv {
    pub fn new() -> CellEnv {
        CellEnv::build(Env::new())
    }

    pub fn wraps(outer: CellEnv) -> CellEnv {
        CellEnv::build(Env::wraps(outer))
    }

    pub fn set(&self, name: String, expr: RcValue) {
        self.0.borrow_mut().set(name, expr);
    }

    pub fn get(&self, name: &str) -> Option<RcValue> {
        self.0.borrow().get(name)
    }

    pub fn replace(&self, name: String, expr: RcValue) -> Option<RcValue> {
        self.0.borrow_mut().replace(name, expr)
    }

    fn build(env: Env) -> CellEnv {
        CellEnv(Rc::new(RefCell::new(env)))
    }
}

struct Env {
    defs: HashMap<String, RcValue>,
    outer: Option<CellEnv>,
}

impl Env {
    pub fn new() -> Env {
        Env {
            defs: HashMap::new(),
            outer: None,
        }
    }

    pub fn wraps(outer: CellEnv) -> Env {
        Env {
            defs: HashMap::new(),
            outer: Some(outer),
        }
    }

    pub fn set(&mut self, name: String, expr: RcValue) {
        self.defs.insert(name, expr);
    }

    pub fn get(&self, name: &str) -> Option<RcValue> {
        match self.defs.get(name) {
            Some(value) => Some(value.clone()),
            None => self.get_from_outer(name),
        }
    }

    pub fn replace(&mut self, name: String, expr: RcValue) -> Option<RcValue> {
        match self.defs.get(&name) {
            Some(_) => {
                self.set(name, expr.clone());
                Some(expr)
            }
            None => self.replace_on_outer(name, expr),
        }
    }

    fn get_from_outer(&self, name: &str) -> Option<RcValue> {
        self.outer.clone().and_then(|env| env.get(name))
    }

    fn replace_on_outer(&self, name: String, expr: RcValue) -> Option<RcValue> {
        match self.outer.clone() {
            Some(outer) => outer.replace(name, expr),
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::CellEnv;
    use crate::memory::Memory;

    #[test]
    fn set_and_get_ok() {
        let mut mem = Memory::new();
        let env = CellEnv::new();
        let name: String = "foo".into();

        let val = mem.integer(3);
        env.set(name.clone(), val.clone());

        assert_eq!(Some(val), env.get(&name));
    }

    #[test]
    fn set_and_get_none() {
        let env = CellEnv::new();
        assert_eq!(None, env.get("foo"));
    }

    #[test]
    fn wraps() {
        let mut mem = Memory::new();
        let outer = CellEnv::new();
        let outer_foo = mem.integer(5);
        let bar = mem.intern("bar".into());

        outer.set("foo".into(), outer_foo.clone());
        outer.set("bar".into(), bar.clone());

        let inner = CellEnv::wraps(outer);
        let inner_foo = mem.integer(25);

        inner.set("foo".into(), inner_foo.clone());

        assert_eq!(Some(inner_foo), inner.get("foo"));
        assert_eq!(Some(bar), inner.get("bar"));
    }
}
