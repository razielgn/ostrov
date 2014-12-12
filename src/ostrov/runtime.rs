use ast::AST;
use env::Env;
use eval::eval;
use parser::parse;

#[deriving(Show, PartialEq)]
pub enum Error {
    BadArity(String),
    IrreducibleValue(AST),
    ParseError(String),
    UnappliableValue(AST),
    UnboundVariable(String),
    WrongArgumentType(AST),
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

    pub fn eval_str(&mut self, input: &str) -> Result<Vec<AST>, Error> {
        let exprs = try!(self.parse_str(input));

        let mut evalued_exprs = Vec::new();
        for expr in exprs.iter() {
            let evalued_expr = try!(eval(expr, &mut self.env));
            evalued_exprs.push(evalued_expr);
        }

        Ok(evalued_exprs)
    }
}
