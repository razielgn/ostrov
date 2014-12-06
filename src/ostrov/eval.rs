use ast::AST;

#[deriving(Show, PartialEq)]
pub enum Error {
    UnboundVariable(String),
    UnappliableValue(AST),
    WrongArgumentType(AST),
    BadArity(String),
    IrreducibleValue(AST),
}

pub fn eval(value: &AST) -> Result<AST, Error> {
    match value {
        &AST::Atom(ref atom) =>
            Err(Error::UnboundVariable(atom.to_string())),
        &AST::Bool(_b) =>
            Ok(value.clone()),
        &AST::Integer(_i) =>
            Ok(value.clone()),
        &AST::List(ref list) =>
            eval_list(list.as_slice()),
        &AST::DottedList(ref _list, ref _val) =>
            Err(Error::IrreducibleValue(value.clone())),
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
            match atom.as_slice() {
                "and"   => eval_and(args),
                "if"    => eval_if(args),
                "or"    => eval_or(args),
                "quote" => eval_quote(args),
                fun     => apply(fun, args),
            },
        _ =>
            Err(Error::UnappliableValue(fun.clone()))
    }
}

fn eval_args(args: &[AST]) -> Result<Vec<AST>, Error> {
    let mut out = Vec::with_capacity(args.len());

    for arg in args.iter() {
        let evald_arg = try!(eval(arg));
        out.push(evald_arg);
    }

    Ok(out)
}

fn apply(fun: &str, args_: &[AST]) -> Result<AST, Error> {
    let args = try!(eval_args(args_));

    match fun {
        "+"   => eval_fun_plus(args),
        "-"   => eval_fun_minus(args),
        "*"   => eval_fun_product(args),
        "/"   => eval_fun_division(args),
        "="   => eval_fun_equals(args),
        "<"   => eval_fun_less_than(args),
        "<="  => eval_fun_less_than_or_equal(args),
        ">"   => eval_fun_greater_than(args),
        ">="  => eval_fun_greater_than_or_equal(args),
        "not" => eval_fun_not(args),
        _     => Err(Error::UnboundVariable(fun.to_string()))
    }
}

fn eval_fun_plus(args: Vec<AST>) -> Result<AST, Error> {
    let args_ = try!(list_of_integers(args));
    let sum = args_.iter().fold(0, |sum, n| sum + *n);
    Ok(AST::Integer(sum))
}

fn eval_fun_minus(args_: Vec<AST>) -> Result<AST, Error> {
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

fn eval_fun_division(args_: Vec<AST>) -> Result<AST, Error> {
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

fn eval_fun_product(args_: Vec<AST>) -> Result<AST, Error> {
    let args = try!(list_of_integers(args_));
    let product = args.iter().fold(1, |product, n| product * *n);
    Ok(AST::Integer(product))
}

fn eval_fun_equals(args_: Vec<AST>) -> Result<AST, Error> {
    if args_.len() < 2 {
        return Ok(AST::Bool(true))
    }

    let args = try!(list_of_integers(args_));
    let head = args.head().unwrap();
    let outcome = args.iter().skip(1).all(|n| *n == *head);
    Ok(AST::Bool(outcome))
}

fn eval_fun_less_than(args: Vec<AST>) -> Result<AST, Error> {
    eval_fun_ord(args, |a, b| a < b)
}

fn eval_fun_less_than_or_equal(args: Vec<AST>) -> Result<AST, Error> {
    eval_fun_ord(args, |a, b| a <= b)
}

fn eval_fun_greater_than(args: Vec<AST>) -> Result<AST, Error> {
    eval_fun_ord(args, |a, b| a > b)
}

fn eval_fun_greater_than_or_equal(args: Vec<AST>) -> Result<AST, Error> {
    eval_fun_ord(args, |a, b| a >= b)
}

fn eval_fun_ord(args_: Vec<AST>, cmp: |i64, i64| -> bool) -> Result<AST, Error> {
    if args_.len() < 2 {
        return Ok(AST::Bool(true))
    }

    let args = try!(list_of_integers(args_));
    let outcome = range(0, args.len() - 1).all(|i|
        cmp(args[i], args[i + 1u])
    );

    Ok(AST::Bool(outcome))
}

fn eval_fun_not(args: Vec<AST>) -> Result<AST, Error> {
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
    Ok(list[0].clone())
}

fn list_of_integers(list: Vec<AST>) -> Result<Vec<i64>, Error> {
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

fn eval_and(args: &[AST]) -> Result<AST, Error> {
    let mut last = AST::Bool(true);

    for val in args.iter() {
        let val = try!(eval(val));

        if val == AST::Bool(false) {
            return Ok(val)
        }

        last = val;
    }

    Ok(last)
}

fn eval_or(args: &[AST]) -> Result<AST, Error> {
    let mut last = AST::Bool(false);

    for val in args.iter() {
        let val = try!(eval(val));

        if val != AST::Bool(false) {
            return Ok(val)
        }

        last = val;
    }

    Ok(last)
}

fn eval_if(args: &[AST]) -> Result<AST, Error> {
    if args.len() < 1 || args.len() > 3 {
        return Err(Error::BadArity("if".to_string()))
    }

    let condition = try!(eval(&args[0]));

    let result = if condition != AST::Bool(false) {
        try!(eval(&args[1]))
    } else {
        if args.len() == 2 {
            AST::Bool(false)
        } else {
            try!(eval(&args[2]))
        }
    };

    Ok(result)
}
