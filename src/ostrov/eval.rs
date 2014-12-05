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
        "+"     => eval_fun_plus(args),
        "-"     => eval_fun_minus(args),
        "*"     => eval_fun_product(args),
        "/"     => eval_fun_division(args),
        _       => Err(Error::UnboundVariable(name.to_string()))
    }
}

fn eval_fun_plus(args: &[AST]) -> Result<AST, Error> {
    let mut sum: i64 = 0;

    for val in args.iter() {
        if let &AST::Integer(n) = val {
            sum += n
        } else {
            return Err(Error::WrongArgumentType(val.clone()))
        };
    }

    Ok(AST::Integer(sum))
}

fn eval_fun_minus(args: &[AST]) -> Result<AST, Error> {
    if args.len() == 0 {
        return Err(Error::BadArity("-".to_string()))
    }

    let head = args.head().unwrap();
    let tail = args.tail();

    let start =
        if let &AST::Integer(n) = head {
            n
        } else {
            return Err(Error::WrongArgumentType(head.clone()))
        };

    if tail.is_empty() {
        return Ok(AST::Integer(-start));
    }

    let mut sum: i64 = 0;

    for val in tail.iter() {
        if let &AST::Integer(n) = val {
            sum += n;
        } else {
            return Err(Error::WrongArgumentType(val.clone()))
        };
    }

    Ok(AST::Integer(start - sum))
}

fn eval_fun_division(args: &[AST]) -> Result<AST, Error> {
    if args.len() == 0 {
        return Err(Error::BadArity("/".to_string()))
    }

    let head = args.head().unwrap();
    let tail = args.tail();

    let mut div =
        if let &AST::Integer(n) = head {
            n
        } else {
            return Err(Error::WrongArgumentType(head.clone()))
        };

    if tail.is_empty() {
        return Ok(AST::Integer(1 / div))
    }

    for val in tail.iter() {
        if let &AST::Integer(n) = val {
            div /= n;
        } else {
            return Err(Error::WrongArgumentType(val.clone()))
        };
    }

    Ok(AST::Integer(div))
}

fn eval_fun_product(args: &[AST]) -> Result<AST, Error> {
    let mut product: i64 = 1;

    for val in args.iter() {
        if let &AST::Integer(n) = val {
            product *= n;
        } else {
            return Err(Error::WrongArgumentType(val.clone()))
        };
    }

    Ok(AST::Integer(product))
}

fn eval_quote(list: &[AST]) -> Result<AST, Error> {
    Ok(list.tail()[0].clone())
}
