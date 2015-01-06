use values::RcValue;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type CellEnv = Rc<RefCell<Env>>;

pub struct Env {
    defs: HashMap<String, RcValue>,
    outer: Option<CellEnv>,
}

impl Env {
    pub fn new() -> CellEnv {
        let env = Env {
            defs: HashMap::new(),
            outer: None,
        };

        Rc::new(RefCell::new(env))
    }

    pub fn wraps(outer: CellEnv) -> CellEnv {
        let env = Env {
            defs: HashMap::new(),
            outer: Some(outer),
        };

        Rc::new(RefCell::new(env))
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
        self.outer.clone().and_then(|env| env.borrow().get(name))
    }

    fn replace_on_outer(&self, name: String, expr: RcValue) -> Option<RcValue> {
        match self.outer.clone() {
            Some(outer) => outer.borrow_mut().replace(name, expr),
            None        => None,
        }
    }
}
