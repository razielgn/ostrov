use ast::AST;
use ast::atom_quote;

#[deriving(Show, PartialEq)]
pub enum Error {
    UnboundVariable(String),
    UnappliableValue(AST),
    WrongArgumentType(AST),
    BadArity(String),
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
        "+"     => eval_fun_plus(args),
        _       => Err(Error::UnboundVariable(name.to_string()))
    }
}

fn eval_fun_plus(args: &[AST]) -> Result<AST, Error> {
    let mut sum: i64 = 0;

    for val in args.iter() {
        let evald_val = try!(eval(val.clone()));

        match evald_val {
            AST::Integer(n) =>
                sum = sum + n,
            _ =>
                return Err(Error::WrongArgumentType(evald_val))
        };
    }

    Ok(AST::Integer(sum))
}

fn eval_fun_quote(args: &[AST]) -> Result<AST, Error> {
    Ok(args[0].clone())
}
