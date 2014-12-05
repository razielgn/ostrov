use ast::AST;

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

    match fun {
        &AST::Atom(ref atom) if atom.as_slice() == "quote" =>
            eval_quote(list),
        &AST::Atom(ref atom) => {
            let args = try!(eval_args(list.tail()));
            apply(atom.as_slice(), args.as_slice())
        },
        _ =>
            Err(Error::UnappliableValue(fun.clone()))
    }
}

fn eval_args(args: &[AST]) -> Result<Vec<AST>, Error> {
    let mut out = Vec::with_capacity(args.len());

    for arg in args.iter() {
        let evald_arg = try!(eval(arg.clone()));
        out.push(evald_arg);
    }

    Ok(out)
}

fn apply(name: &str, args: &[AST]) -> Result<AST, Error> {
    match name {
        "+" => eval_fun_plus(args),
        "-" => eval_fun_minus(args),
        "*" => eval_fun_product(args),
        "/" => eval_fun_division(args),
        "=" => eval_fun_equals(args),
        "<" => eval_fun_less_than(args),
        ">" => eval_fun_greater_than(args),
        "not" => eval_fun_not(args),
        _   => Err(Error::UnboundVariable(name.to_string()))
    }
}

fn eval_fun_plus(args: &[AST]) -> Result<AST, Error> {
    let args_ = try!(list_of_integers(args));
    let sum = args_.iter().fold(0, |sum, n| sum + *n);
    Ok(AST::Integer(sum))
}

fn eval_fun_minus(args_: &[AST]) -> Result<AST, Error> {
    let args = try!(list_of_integers(args_));

    if args.len() == 0 {
        return Err(Error::BadArity("-".to_string()))
    }

    let head = args.head().unwrap();
    let tail = args.tail();

    if tail.is_empty() {
        return Ok(AST::Integer(- *head))
    }

    let tail_sum = tail.iter().fold(0, |sum, n| sum + *n);
    Ok(AST::Integer(*head - tail_sum))
}

fn eval_fun_division(args_: &[AST]) -> Result<AST, Error> {
    let args = try!(list_of_integers(args_));

    if args.len() == 0 {
        return Err(Error::BadArity("/".to_string()))
    }

    let head = args.head().unwrap();
    let tail = args.tail();

    if tail.is_empty() {
        return Ok(AST::Integer(1 / *head))
    }

    let div = tail.iter().fold(*head, |div, n| div / *n);
    Ok(AST::Integer(div))
}

fn eval_fun_product(args_: &[AST]) -> Result<AST, Error> {
    let args = try!(list_of_integers(args_));
    let product = args.iter().fold(1, |product, n| product * *n);
    Ok(AST::Integer(product))
}

fn eval_fun_equals(args_: &[AST]) -> Result<AST, Error> {
    if args_.len() < 2 {
        return Ok(AST::Bool(true))
    }

    let args = try!(list_of_integers(args_));
    let head = args.head().unwrap();
    let outcome = args.iter().skip(1).all(|n| *n == *head);
    Ok(AST::Bool(outcome))
}

fn eval_fun_less_than(args_: &[AST]) -> Result<AST, Error> {
    if args_.len() < 2 {
        return Ok(AST::Bool(true))
    }

    let args = try!(list_of_integers(args_));
    let outcome = range(0, args.len() - 1).all(|i| args[i] < args[i + 1u]);
    Ok(AST::Bool(outcome))
}

fn eval_fun_greater_than(args_: &[AST]) -> Result<AST, Error> {
    if args_.len() < 2 {
        return Ok(AST::Bool(true))
    }

    let args = try!(list_of_integers(args_));
    let outcome = range(0, args.len() - 1).all(|i| args[i] > args[i + 1u]);
    Ok(AST::Bool(outcome))
}
fn eval_fun_not(args: &[AST]) -> Result<AST, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity("not".to_string()))
    }

    let outcome = match args.head().unwrap() {
        &AST::Bool(false) => true,
        _                 => false,
    };

    Ok(AST::Bool(outcome))
}

fn eval_quote(list: &[AST]) -> Result<AST, Error> {
    Ok(list.tail()[0].clone())
}

fn list_of_integers(list: &[AST]) -> Result<Vec<i64>, Error> {
    let mut integers = Vec::with_capacity(list.len());

    for val in list.iter() {
        if let &AST::Integer(n) = val {
            integers.push(n);
        } else {
            return Err(Error::WrongArgumentType(val.clone()))
        };
    }

    Ok(integers)
}
