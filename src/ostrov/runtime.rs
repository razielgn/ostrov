use ast::AST;
use env::CellEnv;
use eval::eval;
use parser::{parse, ParseError};
use primitives;
use memory::Memory;
use values::RcValue;

use std::io;
use std::io::Read;
use std::fs::File;
use std::path::Path;

#[derive(PartialEq, Debug)]
pub enum Error {
    BadArity(Option<String>),
    IrreducibleValue(AST),
    ParseError(ParseError),
    UnappliableValue(RcValue),
    MalformedExpression,
    UnboundVariable(String),
    WrongArgumentType(RcValue),
    LoadError(String),
    PrimitiveFailed(String),
}

pub struct Runtime {
    env: CellEnv,
    memory: Memory,
}

impl Runtime {
    pub fn new() -> Runtime {
        let mut runtime = Runtime {
            env: CellEnv::new(),
            memory: Memory::new(),
        };

        runtime.init_primitives();

        runtime
    }

    pub fn parse_str(&self, input: &str) -> Result<Vec<AST>, Error> {
        parse(input)
    }

    pub fn eval_str(&mut self, input: &str) -> Result<Vec<RcValue>, Error> {
        let exprs = try!(self.parse_str(input));

        let mut evalued_exprs = Vec::new();
        for expr in exprs.iter() {
            let evalued_expr = try!(eval(expr, self.env.clone(), &mut self.memory));
            evalued_exprs.push(evalued_expr);
        }

        Ok(evalued_exprs)
    }

    pub fn eval_file(&mut self, path: &Path) -> Result<Vec<RcValue>, Error> {
        let mut file = try!(Runtime::open_file(path));
        let mut content = String::new();
        try!(Runtime::handle_io_error(file.read_to_string(&mut content), path));

        self.eval_str(content.as_slice())
    }

    pub fn dump_heap(&self) {
        self.memory.dump();
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

    fn init_primitives(&mut self) {
        for name in primitives::PRIMITIVES.iter() {
            let primitive = self.memory.primitive(name.to_string());
            self.env.set(name.to_string(), primitive);
        }
    }
}
