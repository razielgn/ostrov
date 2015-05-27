use ast::AST;
use values::RcValue;
use parser::ParseError;

#[derive(PartialEq, Debug)]
pub enum Error {
    BadArity(Option<String>),
    IrreducibleValue(AST),
    LoadError(String),
    MalformedExpression,
    ParseError(ParseError),
    PrimitiveFailed(String),
    UnappliableValue(RcValue),
    UnboundVariable(String),
    WrongArgumentType(RcValue),
}
