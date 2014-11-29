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
        "-"     => eval_fun_minus(args),
        "*"     => eval_fun_product(args),
        "/"     => eval_fun_division(args),
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

fn eval_fun_minus(args: &[AST]) -> Result<AST, Error> {
    if args.len() == 0 {
        Err(Error::BadArity("-".to_string()))
    } else {
        let head = args.head().unwrap();
        let tail = args.tail();

        let evald_head = try!(eval(head.clone()));

        let start = match evald_head {
            AST::Integer(n) => n,
            _               => return Err(Error::WrongArgumentType(evald_head))
        };

        if tail.is_empty() {
            Ok(AST::Integer(-start))
        } else {
            let mut sum: i64 = 0;

            for val in tail.iter() {
                let evald_val = try!(eval(val.clone()));

                match evald_val {
                    AST::Integer(n) =>
                        sum = sum + n,
                    _ =>
                        return Err(Error::WrongArgumentType(evald_val))
                };
            }

            Ok(AST::Integer(start - sum))
        }
    }
}

fn eval_fun_division(args: &[AST]) -> Result<AST, Error> {
    if args.len() == 0 {
        Err(Error::BadArity("/".to_string()))
    } else {
        let head = args.head().unwrap();
        let tail = args.tail();

        let evald_head = try!(eval(head.clone()));

        let mut div = match evald_head {
            AST::Integer(n) => n,
            _               => return Err(Error::WrongArgumentType(evald_head))
        };

        if tail.is_empty() {
            Ok(AST::Integer(1 / div))
        } else {
            for val in tail.iter() {
                let evald_val = try!(eval(val.clone()));

                match evald_val {
                    AST::Integer(n) =>
                        div = div / n,
                    _ =>
                        return Err(Error::WrongArgumentType(evald_val))
                };
            }

            Ok(AST::Integer(div))
        }
    }
}

fn eval_fun_product(args: &[AST]) -> Result<AST, Error> {
    let mut product: i64 = 1;

    for val in args.iter() {
        let evald_val = try!(eval(val.clone()));

        match evald_val {
            AST::Integer(n) =>
                product = product * n,
            _ =>
                return Err(Error::WrongArgumentType(evald_val))
        };
    }

    Ok(AST::Integer(product))
}

fn eval_fun_quote(args: &[AST]) -> Result<AST, Error> {
    Ok(args[0].clone())
}
