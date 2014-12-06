use ast::AST;

use std::collections::HashMap;

pub struct Env {
    defs: HashMap<String, AST>,
}

impl<'a> Env {
    pub fn new() -> Env {
        Env {
            defs: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: String, expr: AST) {
        self.defs.insert(name, expr);
    }

    pub fn get(&'a self, name: &String) -> Option<&'a AST> {
        self.defs.get(name)
    }
}
