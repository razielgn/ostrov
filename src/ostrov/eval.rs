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
        &AST::Fn(ref _name, ref _args, ref _body) =>
            Ok(value.clone()),
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
                "lambda" => eval_lambda(args, env),
                fun     => apply(fun, args, env),
            },
        &AST::Bool(ref _b) =>
            Err(Error::UnappliableValue(fun.clone())),
        &AST::Integer(ref _i) =>
            Err(Error::UnappliableValue(fun.clone())),
        &AST::Fn(ref name, ref args_names, ref body) =>
            apply_lambda(name.clone(), args_names.clone(), args, *body.clone(), env),
        &AST::List(ref list) if list.head() == Some(&AST::Atom("quote".to_string())) =>
            Err(Error::UnappliableValue(fun.clone())),
        head => {
            let mut value = args.clone().to_vec();
            let evald_head = try!(eval(head, env));
            value.insert(0, evald_head);

            eval_list(value.as_slice(), env)
        }
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
        "list" => eval_fun_list(args),
        "length" => eval_fun_length(args),
        "pair?" => eval_fun_pair(args),
        "cons" => eval_fun_cons(args),
        "car" => eval_fun_car(args),
        "cdr" => eval_fun_cdr(args),
        "null?" => eval_fun_null(args),
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

fn apply_lambda(name: Option<String>, args_names: Vec<String>, args:&[AST], body: AST, env: &mut Env) -> Result<AST, Error> {
    let args = try!(eval_args(args, env));

    eval_procedure(name, args_names, args, body, env)
}

fn eval_fun_plus(args: Vec<AST>) -> Result<AST, Error> {
    let args_ = try!(list_of_integers(args));
    let sum = args_.iter().fold(0, |sum, n| sum + *n);
    Ok(AST::Integer(sum))
}

fn eval_fun_minus(args_: Vec<AST>) -> Result<AST, Error> {
    let args = try!(list_of_integers(args_));

    if args.len() == 0 {
        return Err(Error::BadArity(Some("-".to_string())))
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
        return Err(Error::BadArity(Some("/".to_string())))
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
        return Err(Error::BadArity(Some("not".to_string())))
    }

    let outcome = match args.head().unwrap() {
        &AST::Bool(false) => true,
        _                 => false,
    };

    Ok(AST::Bool(outcome))
}

fn eval_fun_list(args: Vec<AST>) -> Result<AST, Error> {
    Ok(AST::List(args))
}

fn eval_fun_length(args: Vec<AST>) -> Result<AST, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity(Some("length".to_string())));
    }

    if let AST::List(ref list) = args[0] {
        Ok(AST::Integer(list.len() as i64))
    } else {
        Err(Error::WrongArgumentType(args[0].clone()))
    }
}

fn eval_fun_pair(args: Vec<AST>) -> Result<AST, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity(Some("pair?".to_string())));
    }

    if let AST::List(ref list) = args[0] {
        Ok(AST::Bool(!list.is_empty()))
    } else if let AST::DottedList(ref _list, ref _el) = args[0] {
        Ok(AST::Bool(true))
    } else {
        Ok(AST::Bool(false))
    }
}

fn eval_fun_cons(args: Vec<AST>) -> Result<AST, Error> {
    if args.len() != 2 {
        return Err(Error::BadArity(Some("cons".to_string())));
    }

    let mut list: Vec<AST> = Vec::new();

    if let AST::List(ref l) = args[1] {
        list.push(args[0].clone());
        list.push_all(l.clone().as_slice());

        Ok(AST::List(list))
    } else {
        list.push(args[0].clone());
        Ok(AST::DottedList(list, box args[1].clone()))
    }
}

fn eval_fun_car(args: Vec<AST>) -> Result<AST, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity(Some("car".to_string())));
    }

    match args[0] {
        AST::List(ref l) if !l.is_empty() => {
            Ok(l.head().unwrap().clone())
        }
        AST::DottedList(ref l, ref _t) => {
            Ok(l.head().unwrap().clone())
        }
        ref bad_arg => {
            Err(Error::WrongArgumentType(bad_arg.clone()))
        }
    }
}

fn eval_fun_cdr(args: Vec<AST>) -> Result<AST, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity(Some("cdr".to_string())));
    }

    match args[0] {
        AST::List(ref l) if !l.is_empty() => {
            Ok(AST::List(l.tail().to_vec()))
        }
        AST::DottedList(ref _l, ref t) => {
            Ok(*t.clone())
        }
        ref bad_arg => {
            Err(Error::WrongArgumentType(bad_arg.clone()))
        }
    }
}

fn eval_fun_null(args: Vec<AST>) -> Result<AST, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity(Some("null?".to_string())));
    }

    let out = match args[0] {
        AST::List(ref l) if l.is_empty() => true,
        _ => false
    };

    Ok(AST::Bool(out))
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
        return Err(Error::BadArity(Some("if".to_string())))
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
        return Err(Error::BadArity(Some("define".to_string())))
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
        let mut value = try!(eval(&args[1], env));

        if let AST::Fn(_name, args, box body) = value {
            value = AST::Fn(Some(name.clone()), args, box body);
        }

        env.set(name.clone(), value.clone());

        Ok(value)
    } else {
        Ok(AST::Atom(name.clone()))
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

    let procedure = AST::Fn(Some(procedure_name.clone()), args_list, box args[1].clone());
    env.set(procedure_name.clone(), procedure);

    Ok(AST::Atom(procedure_name))
}

fn eval_lambda(list: &[AST], _env: &Env) -> Result<AST, Error> {
    if list.len() != 2 {
        return Err(Error::BadArity(Some("lambda".to_string())));
    }

    let ref args = list[0];
    let ref body = list[1];

    match args {
        &AST::List(ref args) if list.len() > 0 => {
            let mut args_list: Vec<String> = Vec::with_capacity(args.len());
            for arg in args.iter() {
                let arg = try!(atom_or_error(arg));
                args_list.push(arg);
            }

            Ok(AST::Fn(None, args_list, box body.clone()))
        }
        value => Err(Error::WrongArgumentType(value.clone()))
    }
}

fn eval_variable(name: &String, env: &mut Env) -> Result<AST, Error> {
    match env.get(name) {
        Some(value) => Ok(value.clone()),
        None        => Err(Error::UnboundVariable(name.clone())),
    }
}

fn eval_procedure(name: Option<String>, arg_names: Vec<String>, args: Vec<AST>, body: AST, env: &Env) -> Result<AST, Error> {
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
