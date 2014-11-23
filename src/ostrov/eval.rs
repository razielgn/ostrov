use parser::AST;

#[deriving(Show, PartialEq)]
pub enum Error {
    IrreducibleValue(AST),
}

pub fn eval(value: AST) -> Result<AST, Error> {
    match value {
        AST::Atom(atom) =>
            Err(Error::IrreducibleValue(AST::Atom(atom))),
        AST::Bool(_b) =>
            Ok(value),
        AST::Integer(_i) =>
            Ok(value),
        AST::List(list) =>
            Err(Error::IrreducibleValue(AST::List(list))),
    }
}
