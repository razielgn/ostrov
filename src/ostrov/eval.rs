use ast::AST;
use ast::atom_quote;

#[deriving(Show, PartialEq)]
pub enum Error {
    IrreducibleValue(AST),
    UnboundVariable(String),
    UnappliableValue(AST),
    WrongArgumentType(AST),
}

pub fn eval(value: AST) -> Result<AST, Error> {
    match value {
        AST::Atom(atom) =>
            Err(Error::UnboundVariable(atom)),
        AST::Bool(_b) =>
            Ok(value),
        AST::Integer(_i) =>
            Ok(value),
        AST::List(ref list) =>
            eval_list(list.as_slice()),
    }
}

fn eval_list(list: &[AST]) -> Result<AST, Error> {
    if list.is_empty() {
        return Ok(AST::List(vec!()));
    }

    let fun = list.head().unwrap();
    let args = list.tail();

    match fun {
        &AST::Atom(ref atom) =>
            eval_fun(atom.as_slice(), args),
        _ =>
            Err(Error::UnappliableValue(fun.clone()))
    }
}

fn eval_fun(name: &str, args: &[AST]) -> Result<AST, Error> {
    match name {
        "quote" => eval_fun_quote(args),
        _       => Err(Error::UnboundVariable(name.to_string()))
    }
}

fn eval_fun_quote(args: &[AST]) -> Result<AST, Error> {
    Ok(args[0].clone())
}
