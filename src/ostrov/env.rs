use values::Value;

use std::collections::HashMap;

pub struct Env<'a> {
    defs: HashMap<String, Value>,
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

    pub fn set(&mut self, name: String, expr: Value) {
        self.defs.insert(name, expr);
    }

    pub fn get(&'a self, name: &String) -> Option<&'a Value> {
        self.defs.get(name).or_else(|| self.get_from_outer(name) )
    }

    fn get_from_outer(&'a self, name: &String) -> Option<&'a Value> {
        self.outer.and_then(|env| env.get(name))
    }

    pub fn remove(&mut self, name: &String) {
        self.defs.remove(name);
    }
}
