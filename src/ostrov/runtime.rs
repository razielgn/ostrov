use ast::AST;
use compiler::compile_single;
use errors::Error;
use parser::parse;
use values::RcValue;
use vm::VM;

use std::io;
use std::io::Read;
use std::fs::File;
use std::path::Path;

pub struct Runtime {
    vm: VM,
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            vm: VM::new(),
        }
    }

    pub fn parse_str(&self, input: &str) -> Result<Vec<AST>, Error> {
        parse(input)
    }

    pub fn eval_str(&mut self, input: &str) -> Result<Vec<RcValue>, Error> {
        let exprs = try!(self.parse_str(input));

        let mut evalued_exprs = Vec::new();
        for expr in &exprs {
            let bytecode = try!(compile_single(expr));
            let evalued_expr = try!(self.vm.execute(bytecode.into_iter()));
            evalued_exprs.push(evalued_expr);
        }

        Ok(evalued_exprs)
    }

    pub fn eval_file(&mut self, path: &Path) -> Result<Vec<RcValue>, Error> {
        let mut file = try!(Runtime::open_file(path));
        let mut content = String::new();
        try!(Runtime::handle_io_error(file.read_to_string(&mut content), path));

        self.eval_str(&*content)
    }

    pub fn dump_heap(&self) {
        self.vm.memory.dump();
    }

    fn open_file(path: &Path) -> Result<File, Error> {
        Runtime::handle_io_error(File::open(path), path)
    }

    fn handle_io_error<T>(result: io::Result<T>, path: &Path) -> Result<T, Error> {
        match result {
            Ok(value) => Ok(value),
            Err(_err) => {
                let str_path = path.display().to_string();
                Err(Error::LoadError(str_path))
            }
        }
    }
}
