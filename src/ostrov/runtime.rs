use ast::AST;
use env::Env;
use eval::eval;
use parser::parse;
use values::Value;
use primitives;
use memory::Memory;

use std::io::BufferedReader;
use std::io::File;
use std::io::IoResult;
use std::rc::Rc;

#[derive(Show, PartialEq)]
pub enum Error {
    BadArity(Option<String>),
    IrreducibleValue(AST),
    ParseError(String),
    UnappliableValue(Value),
    UnboundVariable(String),
    WrongArgumentType(Value),
    LoadError(String),
    PrimitiveFailed(String),
}

pub struct Runtime<'a> {
    env: Env<'a>,
    memory: Memory,
}

impl<'a> Runtime<'a> {
    pub fn new() -> Runtime<'a> {
        let mut runtime = Runtime {
            env: Env::new(),
            memory: Memory::new(),
        };

        runtime.init_primitives();

        runtime
    }

    pub fn parse_str(&self, input: &str) -> Result<Vec<AST>, Error> {
        parse(input)
    }

    pub fn eval_str(&mut self, input: &str) -> Result<Vec<Rc<Value>>, Error> {
        let exprs = try!(self.parse_str(input));

        let mut evalued_exprs = Vec::new();
        for expr in exprs.iter() {
            let evalued_expr = try!(eval(expr, &mut self.env, &mut self.memory));
            evalued_exprs.push(evalued_expr);
        }

        Ok(evalued_exprs)
    }

    pub fn eval_file(&mut self, path: &Path) -> Result<Vec<Rc<Value>>, Error> {
        let file = try!(Runtime::open_file(path));
        let mut reader = BufferedReader::new(file);

        let content = try!(Runtime::handle_io_error(reader.read_to_string(), path));
        self.eval_str(content.as_slice())
    }

    pub fn dump_heap(&self) {
        self.memory.dump();
    }

    fn open_file(path: &Path) -> Result<File, Error> {
        Runtime::handle_io_error(File::open(path), path)
    }

    fn handle_io_error<T>(result: IoResult<T>, path: &Path) -> Result<T, Error> {
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
            let primitive = Value::PrimitiveFn(name.to_string());

            self.env.set(name.to_string(), Rc::new(primitive));
        }
    }
}
