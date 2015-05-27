use ast::AST;
use env::CellEnv;
use memory::Memory;
use values::{Value, RcValue, ArgumentsType};
use runtime::Error;
use eval::{eval, eval_sequence};

pub fn quote(list: &[AST], mem: &mut Memory) -> Result<RcValue, Error> {
    Ok(Value::from_ast(&list[0], mem))
}

pub fn and(args: &[AST], env: CellEnv, mem: &mut Memory) -> Result<RcValue, Error> {
    let mut last = mem.b_true();

    for val in args.iter() {
        let val = try!(eval(val, env.clone(), mem));

        if val == mem.b_false() {
            return Ok(val)
        }

        last = val;
    }

    Ok(last)
}

pub fn or(args: &[AST], env: CellEnv, mem: &mut Memory) -> Result<RcValue, Error> {
    let mut last = mem.b_false();

    for val in args.iter() {
        let val = try!(eval(val, env.clone(), mem));

        if val != mem.b_false() {
            return Ok(val)
        }

        last = val;
    }

    Ok(last)
}

pub fn if_(args: &[AST], env: CellEnv, mem: &mut Memory) -> Result<RcValue, Error> {
    if args.len() < 1 || args.len() > 3 {
        return Err(Error::BadArity(Some("if".to_string())))
    }

    let condition = try!(eval(&args[0], env.clone(), mem));

    let result = if condition != mem.b_false() {
        try!(eval(&args[1], env.clone(), mem))
    } else {
        if args.len() == 2 {
            mem.unspecified()
        } else {
            try!(eval(&args[2], env.clone(), mem))
        }
    };

    Ok(result)
}

pub fn define(args: &[AST], env: CellEnv, mem: &mut Memory) -> Result<RcValue, Error> {
    if args.len() < 1 {
        return Err(Error::BadArity(Some("define".to_string())))
    }

    match args[0] {
        AST::Atom(ref name) => {
            let body = if args.len() == 1 { None } else { Some(&args[1]) };
            try!(define_variable(name, body, env, mem));
        }
        AST::List(ref list) if list.len() > 0 => {
            let body = args[1..].to_vec();
            try!(define_procedure(list.as_ref(), &body, env, mem));
        }
        AST::DottedList(ref list, ref extra) => {
            let body = args[1..].to_vec();
            try!(define_procedure_var(list.as_ref(), &**extra, &body, env, mem));
        }
        _ =>
            return Err(Error::MalformedExpression),
    }

    Ok(mem.unspecified())
}

pub fn lambda(list: &[AST], name: Option<String>, closure: CellEnv, mem: &mut Memory) -> Result<RcValue, Error> {
    if list.len() < 2 {
        return Err(Error::BadArity(Some("lambda".to_string())));
    }

    let args = &list[0];
    let body = &list[1..].to_vec();
    create_fn(args, &body, name, closure, mem)
}

pub fn set(args: &[AST], env: CellEnv, mem: &mut Memory) -> Result<RcValue, Error> {
    if args.len() != 2 {
        return Err(Error::BadArity(Some("set!".to_string())));
    }

    let variable_name = try!(atom_or_error(&args[0], mem));
    let expr = try!(eval(&args[1], env.clone(), mem));

    match env.replace(variable_name.clone(), expr.clone()) {
        Some(_) => Ok(mem.unspecified()),
        None    => Err(Error::UnboundVariable(variable_name)),
    }
}

pub fn let_(args: &[AST], env: CellEnv, mem: &mut Memory) -> Result<RcValue, Error> {
    if args.len() < 2 {
        return Err(Error::BadArity(Some("let".to_string())));
    }

    let inner_env = CellEnv::wraps(env.clone());

    if let &AST::List(ref bindings) = args.first().unwrap() {
        for binding in bindings.iter() {
            match binding {
                &AST::List(ref binding) if binding.len() == 2 => {
                    let name = try!(atom_or_error(&binding[0], mem));
                    let expr = try!(eval(&binding[1], env.clone(), mem));
                    inner_env.set(name, expr);
                }
                _ =>
                    return Err(Error::MalformedExpression)
            }
        }
    } else {
        return Err(Error::MalformedExpression);
    };

    let body = &args[1..];
    eval_sequence(body, inner_env, mem)
}

fn define_variable(name: &String, body: Option<&AST>, env: CellEnv, mem: &mut Memory) -> Result<(), Error> {
    let value = match body {
        Some(&AST::List(ref list)) if list[0] == AST::Atom("lambda".to_string()) =>
            try!(lambda(&list[1..], Some(name.clone()), env.clone(), mem)),
        Some(value) =>
            try!(eval(value, env.clone(), mem)),
        None =>
            mem.unspecified(),
    };

    env.set(name.clone(), value.clone());

    Ok(())
}

fn define_procedure(args: &[AST], body: &Vec<AST>, env: CellEnv, mem: &mut Memory) -> Result<(), Error> {
    let procedure_name = try!(atom_or_error(&args[0], mem));

    let args = AST::List(args[1..].to_vec());
    let procedure = try!(create_fn(&args, body, Some(procedure_name.clone()), env.clone(), mem));
    env.set(procedure_name.clone(), procedure);

    Ok(())
}

fn define_procedure_var(args: &[AST], extra_arg: &AST, body: &Vec<AST>, env: CellEnv, mem: &mut Memory) -> Result<(), Error> {
    let procedure_name = try!(atom_or_error(&args[0], mem));

    let procedure = if args.len() == 1 {
        try!(create_fn(extra_arg, body, Some(procedure_name.clone()), env.clone(), mem))
    } else {
        let args = AST::DottedList(args[1..].to_vec(), Box::new(extra_arg.clone()));
        try!(create_fn(&args, body, Some(procedure_name.clone()), env.clone(), mem))
    };
    env.set(procedure_name.clone(), procedure);

    Ok(())
}

fn create_fn(args: &AST, body: &Vec<AST>, name: Option<String>, closure: CellEnv, mem: &mut Memory) -> Result<RcValue, Error> {
    match args {
        &AST::List(ref list) => {
            let args_list = try!(compose_args_list(list.as_ref(), None, mem));
            Ok(mem.lambda(name, ArgumentsType::Fixed, args_list, closure, body.clone()))
        },
        &AST::DottedList(ref list, ref extra) => {
            let args_list = try!(compose_args_list(list.as_ref(), Some(&**extra), mem));
            Ok(mem.lambda(name, ArgumentsType::Variable, args_list, closure, body.clone()))
        }
        &AST::Atom(ref atom) =>
            Ok(mem.lambda(name, ArgumentsType::Any, vec!(atom.clone()), closure, body.clone())),
        _ => {
            let value = Value::from_ast(args, mem);
            Err(Error::WrongArgumentType(value))
        }
    }
}

fn compose_args_list(args: &[AST], extra: Option<&AST>, mem: &mut Memory) -> Result<Vec<String>, Error> {
    let length = args.len() + if extra.is_some() { 1 } else { 0 };
    let mut list = Vec::with_capacity(length);

    for arg in args.iter() {
        let arg = try!(atom_or_error(arg, mem));
        list.push(arg);
    }

    if extra.is_some() {
        let arg = try!(atom_or_error(extra.unwrap(), mem));
        list.push(arg);
    }

    Ok(list)
}

fn atom_or_error(value: &AST, mem: &mut Memory) -> Result<String, Error> {
    if let &AST::Atom(ref atom) = value {
        Ok(atom.to_string())
    } else {
        let value = Value::from_ast(value, mem);
        Err(Error::WrongArgumentType(value))
    }
}
