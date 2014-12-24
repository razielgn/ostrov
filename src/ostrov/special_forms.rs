use ast::AST;
use env::Env;
use memory::Memory;
use runtime::Error;
use values::Value;
use values::ArgumentsType;
use eval::eval;

use std::rc::Rc;

pub fn quote(list: &[AST], mem: &mut Memory) -> Result<Rc<Value>, Error> {
    match Value::from_ast(&list[0]) {
        Value::Bool(b) =>
            Ok(mem.boolean(b)),
        Value::List(ref l) if l.is_empty() =>
            Ok(mem.empty_list()),
        value =>
            Ok(mem.store(value)),
    }
}

pub fn and(args: &[AST], env: &mut Env, mem: &mut Memory) -> Result<Rc<Value>, Error> {
    let mut last = mem.b_true();

    for val in args.iter() {
        let val = try!(eval(val, env, mem));

        if val == mem.b_false() {
            return Ok(val)
        }

        last = val;
    }

    Ok(last)
}

pub fn or(args: &[AST], env: &mut Env, mem: &mut Memory) -> Result<Rc<Value>, Error> {
    let mut last = mem.b_false();

    for val in args.iter() {
        let val = try!(eval(val, env, mem));

        if val != mem.b_false() {
            return Ok(val)
        }

        last = val;
    }

    Ok(last)
}

pub fn if_(args: &[AST], env: &mut Env, mem: &mut Memory) -> Result<Rc<Value>, Error> {
    if args.len() < 1 || args.len() > 3 {
        return Err(Error::BadArity(Some("if".to_string())))
    }

    let condition = try!(eval(&args[0], env, mem));

    let result = if condition.deref() != &Value::Bool(false) {
        try!(eval(&args[1], env, mem))
    } else {
        if args.len() == 2 {
            mem.b_false()
        } else {
            try!(eval(&args[2], env, mem))
        }
    };

    Ok(result)
}

pub fn define(args: &[AST], env: &mut Env, mem: &mut Memory) -> Result<Rc<Value>, Error> {
    if args.len() < 1 {
        return Err(Error::BadArity(Some("define".to_string())))
    }

    match &args[0] {
        &AST::Atom(ref name) => {
            let body = if args.len() == 1 { None } else { Some(&args[1]) };
            define_variable(name, body, env, mem)
        }
        &AST::List(ref list) if list.len() > 0 => {
            let body = args.tail().to_vec();
            define_procedure(list.as_slice(), &body, env, mem)
        }
        &AST::DottedList(ref list, ref extra) => {
            let body = args.tail().to_vec();
            define_procedure_var(list.as_slice(), &**extra, &body, env, mem)
        }
        value =>
            Err(Error::WrongArgumentType(Value::from_ast(value)))
    }
}

pub fn lambda(list: &[AST], name: Option<String>, mem: &mut Memory) -> Result<Rc<Value>, Error> {
    if list.len() < 2 {
        return Err(Error::BadArity(Some("lambda".to_string())));
    }

    let args = list.head().unwrap();
    let body = list.tail().to_vec();
    create_fn(args, &body, name, mem)
}

fn define_variable(name: &String, body: Option<&AST>, env: &mut Env, mem: &mut Memory) -> Result<Rc<Value>, Error> {
    if body.is_none() {
        return Ok(mem.intern(name.clone()));
    }

    let value = match body.unwrap() {
        &AST::List(ref list) if list[0] == AST::Atom("lambda".to_string()) =>
            try!(lambda(list.tail(), Some(name.clone()), mem)),
        value =>
            try!(eval(value, env, mem))
    };

    env.set(name.clone(), value.clone());

    Ok(value)
}

fn define_procedure(args: &[AST], body: &Vec<AST>, env: &mut Env, mem: &mut Memory) -> Result<Rc<Value>, Error> {
    let procedure_name = try!(atom_or_error(&args[0]));

    let args = AST::List(args.tail().to_vec());
    let procedure = try!(create_fn(&args, body, Some(procedure_name.clone()), mem));
    env.set(procedure_name.clone(), procedure);

    Ok(mem.intern(procedure_name))
}

fn define_procedure_var(args: &[AST], extra_arg: &AST, body: &Vec<AST>, env: &mut Env, mem: &mut Memory) -> Result<Rc<Value>, Error> {
    let procedure_name = try!(atom_or_error(&args[0]));

    let procedure = if args.len() == 1 {
        try!(create_fn(extra_arg, body, Some(procedure_name.clone()), mem))
    } else {
        let args = AST::DottedList(args.tail().to_vec(), box extra_arg.clone());
        try!(create_fn(&args, body, Some(procedure_name.clone()), mem))
    };
    env.set(procedure_name.clone(), procedure);

    Ok(mem.intern(procedure_name))
}

fn create_fn(list: &AST, body: &Vec<AST>, name: Option<String>, mem: &mut Memory) -> Result<Rc<Value>, Error> {
    match list {
        &AST::List(ref args) => {
            let args_list = try!(compose_args_list(args.as_slice(), None));
            Ok(mem.lambda(name, ArgumentsType::Fixed, args_list, body.clone()))
        },
        &AST::DottedList(ref args, ref extra) => {
            let args_list = try!(compose_args_list(args.as_slice(), Some(&**extra)));
            Ok(mem.lambda(name, ArgumentsType::Variable, args_list, body.clone()))
        }
        &AST::Atom(ref arg) =>
            Ok(mem.lambda(name, ArgumentsType::Any, vec!(arg.clone()), body.clone())),
        value =>
            Err(Error::WrongArgumentType(Value::from_ast(value)))
    }
}

fn compose_args_list(args: &[AST], extra: Option<&AST>) -> Result<Vec<String>, Error> {
    let length = args.len() + if extra.is_some() { 1 } else { 0 };
    let mut list = Vec::with_capacity(length);

    for arg in args.iter() {
        let arg = try!(atom_or_error(arg));
        list.push(arg);
    }

    if extra.is_some() {
        let arg = try!(atom_or_error(extra.unwrap()));
        list.push(arg);
    }

    Ok(list)
}

fn atom_or_error(value: &AST) -> Result<String, Error> {
    if let &AST::Atom(ref atom) = value {
        Ok(atom.to_string())
    } else {
        Err(Error::WrongArgumentType(Value::from_ast(value)))
    }
}
