use ast::AST;
use env::Env;
use runtime::Error;

pub fn eval(value: &AST, env: &mut Env) -> Result<AST, Error> {
    match value {
        &AST::Atom(ref atom) =>
            eval_variable(atom, env),
        &AST::Bool(_b) =>
            Ok(value.clone()),
        &AST::Integer(_i) =>
            Ok(value.clone()),
        &AST::List(ref list) =>
            eval_list(list.as_slice(), env),
        _ =>
            Err(Error::IrreducibleValue(value.clone())),
    }
}

fn eval_list(list: &[AST], env: &mut Env) -> Result<AST, Error> {
    if list.is_empty() {
        return Ok(AST::List(vec!()));
    }

    let fun = list.head().unwrap();
    let args = list.tail();

    match fun {
        &AST::Atom(ref atom) =>
            match atom.as_slice() {
                "and"   => eval_and(args, env),
                "if"    => eval_if(args, env),
                "or"    => eval_or(args, env),
                "quote" => eval_quote(args),
                "define" => eval_define(args, env),
                fun     => apply(fun, args, env),
            },
        _ =>
            Err(Error::UnappliableValue(fun.clone()))
    }
}

fn eval_args(args: &[AST], env: &mut Env) -> Result<Vec<AST>, Error> {
    let mut out = Vec::with_capacity(args.len());

    for arg in args.iter() {
        let evald_arg = try!(eval(arg, env));
        out.push(evald_arg);
    }

    Ok(out)
}

fn apply(fun: &str, args_: &[AST], env: &mut Env) -> Result<AST, Error> {
    let args = try!(eval_args(args_, env));

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
        _     => {
            let res = try!(eval_variable(&fun.to_string(), env));

            match res {
                AST::Fn(name, args_names, body) => {
                    eval_procedure(name, args_names, args, *body, env)
                },
                _ => Err(Error::UnappliableValue(res.clone()))
            }
        }
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

fn eval_and(args: &[AST], env: &mut Env) -> Result<AST, Error> {
    let mut last = AST::Bool(true);

    for val in args.iter() {
        let val = try!(eval(val, env));

        if val == AST::Bool(false) {
            return Ok(val)
        }

        last = val;
    }

    Ok(last)
}

fn eval_or(args: &[AST], env: &mut Env) -> Result<AST, Error> {
    let mut last = AST::Bool(false);

    for val in args.iter() {
        let val = try!(eval(val, env));

        if val != AST::Bool(false) {
            return Ok(val)
        }

        last = val;
    }

    Ok(last)
}

fn eval_if(args: &[AST], env: &mut Env) -> Result<AST, Error> {
    if args.len() < 1 || args.len() > 3 {
        return Err(Error::BadArity("if".to_string()))
    }

    let condition = try!(eval(&args[0], env));

    let result = if condition != AST::Bool(false) {
        try!(eval(&args[1], env))
    } else {
        if args.len() == 2 {
            AST::Bool(false)
        } else {
            try!(eval(&args[2], env))
        }
    };

    Ok(result)
}

fn eval_define(args: &[AST], env: &mut Env) -> Result<AST, Error> {
    if args.len() < 1 || args.len() > 2 {
        return Err(Error::BadArity("define".to_string()))
    }

    let ref atom = args[0];

    match atom {
        &AST::Atom(ref name) => eval_define_variable(name, args, env),
        &AST::List(ref list) if list.len() > 0 => eval_define_procedure(list.as_slice(), args, env),
        _ => Err(Error::WrongArgumentType(atom.clone()))
    }
}

fn eval_define_variable(name: &String, args: &[AST], env: &mut Env) -> Result<AST, Error> {
    if args.len() == 2 {
        let body = try!(eval(&args[1], env));
        env.set(name.to_string(), body.clone());

        Ok(body)
    } else {
        Ok(AST::Atom(name.to_string()))
    }
}

fn eval_define_procedure(list: &[AST], args: &[AST], env: &mut Env) -> Result<AST, Error> {
    let procedure_name = try!(atom_or_error(&list[0]));

    let tail = list.tail();
    let mut args_list: Vec<String> = Vec::with_capacity(tail.len());
    for arg in tail.iter() {
        let arg = try!(atom_or_error(arg));
        args_list.push(arg);
    }

    let procedure = AST::Fn(procedure_name.clone(), args_list, box args[1].clone());
    env.set(procedure_name.clone(), procedure);

    Ok(AST::Atom(procedure_name))
}

fn eval_variable(name: &String, env: &mut Env) -> Result<AST, Error> {
    match env.get(name) {
        Some(value) => Ok(value.clone()),
        None        => Err(Error::UnboundVariable(name.clone())),
    }
}

fn eval_procedure(name: String, arg_names: Vec<String>, args: Vec<AST>, body: AST, env: &Env) -> Result<AST, Error> {
    if arg_names.len() != args.len() {
        return Err(Error::BadArity(name));
    }

    let mut inner_env = Env::wraps(env);
    for (name, value) in arg_names.iter().zip(args.iter()) {
        inner_env.set(name.clone(), value.clone());
    }

    eval(&body, &mut inner_env)
}

fn atom_or_error(value: &AST) -> Result<String, Error> {
    if let &AST::Atom(ref atom) = value {
        Ok(atom.to_string())
    } else {
        Err(Error::WrongArgumentType(value.clone()))
    }
}
