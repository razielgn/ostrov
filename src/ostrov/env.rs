use values::RcValue;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

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
        let CellEnv(ref cell) = *self;
        cell.borrow_mut().set(name, expr);
    }

    pub fn get(&self, name: &String) -> Option<RcValue> {
        let CellEnv(ref cell) = *self;
        cell.borrow().get(name)
    }

    pub fn replace(&self, name: String, expr: RcValue) -> Option<RcValue> {
        let CellEnv(ref cell) = *self;
        cell.borrow_mut().replace(name, expr)
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

    pub fn get(&self, name: &String) -> Option<RcValue> {
        match self.defs.get(name) {
            Some(value) => Some(value.clone()),
            None        => self.get_from_outer(name),
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

    fn get_from_outer(&self, name: &String) -> Option<RcValue> {
        self.outer.clone().and_then(|env| env.get(name))
    }

    fn replace_on_outer(&self, name: String, expr: RcValue) -> Option<RcValue> {
        match self.outer.clone() {
            Some(outer) => outer.replace(name, expr),
            None        => None,
        }
    }
}
