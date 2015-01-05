use values::RcValue;

use std::collections::HashMap;

pub struct Env<'a> {
    defs: HashMap<String, RcValue>,
    outer: Option<&'a Env<'a>>,
}

impl<'a> Env<'a> {
    pub fn new() -> Env<'a> {
        Env {
            defs: HashMap::new(),
            outer: None,
        }
    }

    pub fn wraps(outer: &'a Env) -> Env<'a> {
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

    fn get_from_outer(&self, name: &String) -> Option<RcValue> {
        self.outer.and_then(|env| env.get(name))
    }

    pub fn remove(&mut self, name: &String) {
        self.defs.remove(name);
    }
}
