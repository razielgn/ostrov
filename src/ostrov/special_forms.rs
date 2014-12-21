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
    if args.len() < 1 || args.len() > 2 {
        return Err(Error::BadArity(Some("define".to_string())))
    }

    let ref atom = args[0];

    match atom {
        &AST::Atom(ref name) =>
            define_variable(name, args, env, mem),
        &AST::List(ref list) if list.len() > 0 =>
            define_procedure(list.as_slice(), args, env, mem),
        _ =>
            Err(Error::WrongArgumentType(Value::from_ast(atom)))
    }
}

pub fn lambda(list: &[AST], name: Option<String>, mem: &mut Memory) -> Result<Rc<Value>, Error> {
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

            Ok(mem.lambda(name, ArgumentsType::Fixed, args_list, body.clone()))
        },
        &AST::DottedList(ref args, ref extra) => {
            let mut args_list: Vec<String> = Vec::with_capacity(args.len());
            for arg in args.iter() {
                let arg = try!(atom_or_error(arg));
                args_list.push(arg);
            }

            let extra_arg = try!(atom_or_error(&**extra));
            args_list.push(extra_arg);

            Ok(mem.lambda(name, ArgumentsType::Variable, args_list, body.clone()))
        }
        value => Err(Error::WrongArgumentType(Value::from_ast(value)))
    }
}

fn define_variable(name: &String, args: &[AST], env: &mut Env, mem: &mut Memory) -> Result<Rc<Value>, Error> {
    if args.len() == 1 {
        return Ok(mem.intern(name.clone()));
    }

    let value = match args[1] {
        AST::List(ref l) if l[0] == AST::Atom("lambda".to_string()) =>
            try!(lambda(l.tail(), Some(name.clone()), mem)),
        ref value =>
            try!(eval(value, env, mem))
    };

    env.set(name.clone(), value.clone());

    Ok(value)
}

fn define_procedure(list: &[AST], args: &[AST], env: &mut Env, mem: &mut Memory) -> Result<Rc<Value>, Error> {
    let procedure_name = try!(atom_or_error(&list[0]));

    let tail = list.tail();
    let mut args_list: Vec<String> = Vec::with_capacity(tail.len());
    for arg in tail.iter() {
        let arg = try!(atom_or_error(arg));
        args_list.push(arg);
    }

    let procedure = mem.lambda(Some(procedure_name.clone()), ArgumentsType::Fixed, args_list, args[1].clone());
    env.set(procedure_name.clone(), procedure);

    Ok(mem.intern(procedure_name))
}

fn atom_or_error(value: &AST) -> Result<String, Error> {
    if let &AST::Atom(ref atom) = value {
        Ok(atom.to_string())
    } else {
        Err(Error::WrongArgumentType(Value::from_ast(value)))
    }
}
