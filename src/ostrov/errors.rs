use values::RcValue;
use parser::ParseError;

#[derive(PartialEq, Debug)]
pub enum Error<'a> {
    ParseError(ParseError<'a>),
    RuntimeError(RuntimeError),
}

impl<'a> From<RuntimeError> for Error<'a> {
    fn from(e: RuntimeError) -> Error<'a> { Error::RuntimeError(e) }
}

impl<'a> From<ParseError<'a>> for Error<'a> {
    fn from(e: ParseError<'a>) -> Error<'a> { Error::ParseError(e) }
}

#[derive(PartialEq, Debug)]
pub enum RuntimeError {
    BadArity(Option<String>),
    CannotPopLastFrame,
    MalformedExpression,
    PrimitiveFailed(String),
    UnappliableValue(RcValue),
    UnboundVariable(String),
    WrongArgumentType(RcValue),
}
