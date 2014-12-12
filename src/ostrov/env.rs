use ast::AST;

use std::collections::HashMap;

pub struct Env<'a> {
    defs: HashMap<String, AST>,
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

    pub fn set(&mut self, name: String, expr: AST) {
        self.defs.insert(name, expr);
    }

    pub fn get(&'a self, name: &String) -> Option<&'a AST> {
        self.defs.get(name).or_else(|| self.get_from_outer(name) )
    }

    fn get_from_outer(&'a self, name: &String) -> Option<&'a AST> {
        self.outer.and_then(|env| env.get(name))
    }
}
