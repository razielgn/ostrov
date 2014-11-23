use ast::AST;
use ast::atom_quote;

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
            if !list.is_empty() && list[0] == atom_quote() {
                Ok(list[1].clone())
            } else {
                Err(Error::IrreducibleValue(AST::List(list)))
            },
    }
}
