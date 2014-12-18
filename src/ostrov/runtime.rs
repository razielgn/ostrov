use ast::AST;
use env::Env;
use eval::eval;
use parser::parse;
use values::Value;

use std::io::BufferedReader;
use std::io::File;
use std::io::IoResult;

#[deriving(Show, PartialEq)]
pub enum Error {
    BadArity(Option<String>),
    IrreducibleValue(Value),
    ParseError(String),
    UnappliableValue(Value),
    UnboundVariable(String),
    WrongArgumentType(Value),
    LoadError(String),
}

pub struct Runtime<'a> {
    env: Env<'a>,
}

impl<'a> Runtime<'a> {
    pub fn new() -> Runtime<'a> {
        Runtime {
            env: Env::new(),
        }
    }

    pub fn parse_str(&self, input: &str) -> Result<Vec<AST>, Error> {
        parse(input)
    }

    pub fn eval_str(&mut self, input: &str) -> Result<Vec<Value>, Error> {
        let exprs = try!(self.parse_str(input));

        let mut evalued_exprs = Vec::new();
        for expr in exprs.iter() {
            let evalued_expr = try!(eval(expr, &mut self.env));
            evalued_exprs.push(evalued_expr);
        }

        Ok(evalued_exprs)
    }

    pub fn eval_file(&mut self, path: &Path) -> Result<Vec<Value>, Error> {
        let file = try!(Runtime::open_file(path));
        let mut reader = BufferedReader::new(file);

        let content = try!(Runtime::handle_io_error(reader.read_to_string(), path));
        self.eval_str(content.as_slice())
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
}
