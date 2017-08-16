use ast::AST;
use parser::ParseError;
use values::RcValue;

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
    CannotPopLastFrame,
}
