use ast::AST;
use compiler::compile_single;
use errors::Error;
use parser::{parse, ParseError};
use std::fs;
use std::path::Path;
use values::RcValue;
use vm::VM;

pub struct Runtime {
    vm: VM,
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime { vm: VM::new() }
    }

    pub fn parse_str<'a>(
        &self,
        input: &'a str,
    ) -> Result<Vec<AST>, ParseError<'a>> {
        parse(input)
    }

    pub fn eval_str<'a>(
        &mut self,
        input: &'a str,
    ) -> Result<Vec<RcValue>, Error<'a>> {
        let exprs = try!(self.parse_str(input));

        let mut evalued_exprs = Vec::new();
        for expr in &exprs {
            let bytecode = try!(compile_single(expr));
            let evalued_expr = try!(self.vm.execute(bytecode));
            evalued_exprs.push(evalued_expr);
        }

        Ok(evalued_exprs)
    }

    pub fn eval_file(&mut self, path: &Path) {
        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Failed loading {:?}: {:?}", path, e);
                return;
            }
        };

        match self.eval_str(&content) {
            Ok(_) => (),
            Err(e) => eprintln!("{:?}", e),
        }
    }

    pub fn dump_heap(&self) {
        self.vm.memory.dump();
    }
}
