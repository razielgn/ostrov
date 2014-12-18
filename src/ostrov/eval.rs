use ast::AST;
use env::Env;
use runtime::Error;
use values::Value;
use primitives;

pub fn eval(value: &AST, env: &mut Env) -> Result<Value, Error> {
    match value {
        &AST::Atom(ref atom) =>
            eval_variable(atom, env),
        &AST::Bool(_b) =>
            Ok(Value::from_ast(value)),
        &AST::Integer(_i) =>
            Ok(Value::from_ast(value)),
        &AST::List(ref list) =>
            eval_list(list, env),
        _ =>
            Err(Error::IrreducibleValue(Value::from_ast(value))),
    }
}

fn eval_list(list: &Vec<AST>, env: &mut Env) -> Result<Value, Error> {
    if list.is_empty() {
        return Ok(Value::List(vec!()));
    }

    let head = list.head().unwrap();
    let tail = list.tail();

    if let &AST::List(ref list) = head {
        if list.head().unwrap() == &AST::Atom("quote".to_string()) {
            return Err(Error::UnappliableValue(Value::from_ast(head)));
        }
    }

    if let &AST::Atom(ref special_form) = head {
        let args = tail;

        match special_form.as_slice() {
            "and"    => return eval_and(args, env),
            "define" => return eval_define(args, env),
            "if"     => return eval_if(args, env),
            "lambda" => return eval_lambda(args, env),
            "or"     => return eval_or(args, env),
            "quote"  => return eval_quote(args),
            _        => (),
        }
    }

    let fun  = try!(eval(head, env));
    let args = try!(eval_args(tail, env));

    match fun {
        Value::Fn(name, args_names, body) =>
            apply(name, args_names, args, body, env),
        Value::PrimitiveFn(name) =>
            primitives::apply(name.as_slice(), args),
        _ =>
            Err(Error::UnappliableValue(fun.clone()))
    }
}

fn eval_args(args: &[AST], env: &mut Env) -> Result<Vec<Value>, Error> {
    let mut out = Vec::with_capacity(args.len());

    for arg in args.iter() {
        let evald_arg = try!(eval(arg, env));
        out.push(evald_arg);
    }

    Ok(out)
}

fn eval_quote(list: &[AST]) -> Result<Value, Error> {
    Ok(Value::from_ast(&list[0]))
}

fn eval_and(args: &[AST], env: &mut Env) -> Result<Value, Error> {
    let mut last = Value::Bool(true);

    for val in args.iter() {
        let val = try!(eval(val, env));

        if val == Value::Bool(false) {
            return Ok(val)
        }

        last = val;
    }

    Ok(last)
}

fn eval_or(args: &[AST], env: &mut Env) -> Result<Value, Error> {
    let mut last = Value::Bool(false);

    for val in args.iter() {
        let val = try!(eval(val, env));

        if val != Value::Bool(false) {
            return Ok(val)
        }

        last = val;
    }

    Ok(last)
}

fn eval_if(args: &[AST], env: &mut Env) -> Result<Value, Error> {
    if args.len() < 1 || args.len() > 3 {
        return Err(Error::BadArity(Some("if".to_string())))
    }

    let condition = try!(eval(&args[0], env));

    let result = if condition != Value::Bool(false) {
        try!(eval(&args[1], env))
    } else {
        if args.len() == 2 {
            Value::Bool(false)
        } else {
            try!(eval(&args[2], env))
        }
    };

    Ok(result)
}

fn eval_define(args: &[AST], env: &mut Env) -> Result<Value, Error> {
    if args.len() < 1 || args.len() > 2 {
        return Err(Error::BadArity(Some("define".to_string())))
    }

    let ref atom = args[0];

    match atom {
        &AST::Atom(ref name) => eval_define_variable(name, args, env),
        &AST::List(ref list) if list.len() > 0 => eval_define_procedure(list.as_slice(), args, env),
        _ => Err(Error::WrongArgumentType(Value::from_ast(atom)))
    }
}

fn eval_define_variable(name: &String, args: &[AST], env: &mut Env) -> Result<Value, Error> {
    if args.len() == 2 {
        let mut value = try!(eval(&args[1], env));

        if let Value::Fn(_name, args, body) = value {
            value = Value::Fn(Some(name.clone()), args, body);
        }

        env.set(name.clone(), value.clone());

        Ok(value)
    } else {
        Ok(Value::Atom(name.clone()))
    }
}

fn eval_define_procedure(list: &[AST], args: &[AST], env: &mut Env) -> Result<Value, Error> {
    let procedure_name = try!(atom_or_error(&list[0]));

    let tail = list.tail();
    let mut args_list: Vec<String> = Vec::with_capacity(tail.len());
    for arg in tail.iter() {
        let arg = try!(atom_or_error(arg));
        args_list.push(arg);
    }

    let procedure = Value::Fn(Some(procedure_name.clone()), args_list, args[1].clone());
    env.set(procedure_name.clone(), procedure);

    Ok(Value::Atom(procedure_name))
}

fn eval_lambda(list: &[AST], _env: &Env) -> Result<Value, Error> {
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

            Ok(Value::Fn(None, args_list, body.clone()))
        }
        value => Err(Error::WrongArgumentType(Value::from_ast(value)))
    }
}

fn eval_variable(name: &String, env: &mut Env) -> Result<Value, Error> {
    match env.get(name) {
        Some(value) => Ok(value.clone()),
        None        => Err(Error::UnboundVariable(name.clone())),
    }
}

fn apply(name: Option<String>, arg_names: Vec<String>, arg_values: Vec<Value>, body: AST, env: &Env) -> Result<Value, Error> {
    if arg_names.len() != arg_values.len() {
        return Err(Error::BadArity(name));
    }

    let mut inner_env = Env::wraps(env);
    for (name, value) in arg_names.iter().zip(arg_values.iter()) {
        inner_env.set(name.clone(), value.clone());
    }

    eval(&body, &mut inner_env)
}

fn atom_or_error(value: &AST) -> Result<String, Error> {
    if let &AST::Atom(ref atom) = value {
        Ok(atom.to_string())
    } else {
        Err(Error::WrongArgumentType(Value::from_ast(value)))
    }
}
