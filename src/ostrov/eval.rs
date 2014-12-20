use ast::AST;
use env::Env;
use runtime::Error;
use values::Value;
use primitives;
use memory::Memory;

use std::rc::Rc;

pub fn eval(value: &AST, env: &mut Env, memory: &mut Memory) -> Result<Rc<Value>, Error> {
    match value {
        &AST::Atom(ref atom) =>
            eval_variable(atom, env),
        &AST::Bool(b) =>
            Ok(memory.boolean(b)),
        &AST::Integer(i) =>
            Ok(memory.integer(i)),
        &AST::List(ref list) =>
            eval_list(list, env, memory),
        _ =>
            Err(Error::IrreducibleValue(value.clone())),
    }
}

fn eval_list(list: &Vec<AST>, env: &mut Env, mem: &mut Memory) -> Result<Rc<Value>, Error> {
    if list.is_empty() {
        return Ok(mem.new_list(vec!()));
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
            "and"    => return eval_and(args, env, mem),
            "define" => return eval_define(args, env, mem),
            "if"     => return eval_if(args, env, mem),
            "lambda" => return eval_lambda(args, env, mem),
            "or"     => return eval_or(args, env, mem),
            "quote"  => return eval_quote(args, mem),
            _        => (),
        }
    }

    let fun  = try!(eval(head, env, mem));
    let args = try!(eval_args(tail, env, mem));

    match fun.deref() {
        &Value::Fn(ref name, ref args_names, ref body) =>
            apply(name, args_names, args, body, env, mem),
        &Value::PrimitiveFn(ref name) =>
            primitives::apply(name, args, mem),
        fun =>
            Err(Error::UnappliableValue(fun.clone()))
    }
}

fn eval_args(args: &[AST], env: &mut Env, mem: &mut Memory) -> Result<Vec<Rc<Value>>, Error> {
    let mut out = Vec::with_capacity(args.len());

    for arg in args.iter() {
        let evald_arg = try!(eval(arg, env, mem));
        out.push(evald_arg);
    }

    Ok(out)
}

fn eval_quote(list: &[AST], mem: &mut Memory) -> Result<Rc<Value>, Error> {
    match Value::from_ast(&list[0]) {
        Value::Bool(b) =>
            Ok(mem.boolean(b)),
        value =>
            Ok(mem.store(value)),
    }
}

fn eval_and(args: &[AST], env: &mut Env, mem: &mut Memory) -> Result<Rc<Value>, Error> {
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

fn eval_or(args: &[AST], env: &mut Env, mem: &mut Memory) -> Result<Rc<Value>, Error> {
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

fn eval_if(args: &[AST], env: &mut Env, mem: &mut Memory) -> Result<Rc<Value>, Error> {
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

fn eval_define(args: &[AST], env: &mut Env, mem: &mut Memory) -> Result<Rc<Value>, Error> {
    if args.len() < 1 || args.len() > 2 {
        return Err(Error::BadArity(Some("define".to_string())))
    }

    let ref atom = args[0];

    match atom {
        &AST::Atom(ref name) => eval_define_variable(name, args, env, mem),
        &AST::List(ref list) if list.len() > 0 => eval_define_procedure(list.as_slice(), args, env, mem),
        _ => Err(Error::WrongArgumentType(Value::from_ast(atom)))
    }
}

fn eval_define_variable(name: &String, args: &[AST], env: &mut Env, mem: &mut Memory) -> Result<Rc<Value>, Error> {
    if args.len() == 2 {
        let value = try!(eval(&args[1], env, mem));

        if let &Value::Fn(ref _name, ref args, ref body) = value.deref() {
            env.set(
                name.clone(),
                mem.store(Value::Fn(Some(name.clone()), args.clone(), body.clone()))
            );
        } else {
            env.set(name.clone(), value.clone());
        }

        Ok(value)
    } else {
        Ok(mem.store(Value::Atom(name.clone())))
    }
}

fn eval_define_procedure(list: &[AST], args: &[AST], env: &mut Env, mem: &mut Memory) -> Result<Rc<Value>, Error> {
    let procedure_name = try!(atom_or_error(&list[0]));

    let tail = list.tail();
    let mut args_list: Vec<String> = Vec::with_capacity(tail.len());
    for arg in tail.iter() {
        let arg = try!(atom_or_error(arg));
        args_list.push(arg);
    }

    let procedure = mem.store(Value::Fn(Some(procedure_name.clone()), args_list, args[1].clone()));
    env.set(procedure_name.clone(), procedure);

    Ok(mem.store(Value::Atom(procedure_name)))
}

fn eval_lambda(list: &[AST], _env: &Env, mem: &mut Memory) -> Result<Rc<Value>, Error> {
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

            Ok(mem.store(Value::Fn(None, args_list, body.clone())))
        }
        value => Err(Error::WrongArgumentType(Value::from_ast(value)))
    }
}

fn eval_variable(name: &String, env: &mut Env) -> Result<Rc<Value>, Error> {
    match env.get(name) {
        Some(value) => Ok(value),
        None        => Err(Error::UnboundVariable(name.clone())),
    }
}

fn apply(name: &Option<String>, arg_names: &Vec<String>, arg_values: Vec<Rc<Value>>, body: &AST, env: &Env, mem: &mut Memory) -> Result<Rc<Value>, Error> {
    if arg_names.len() != arg_values.len() {
        return Err(Error::BadArity(name.clone()));
    }

    let mut inner_env = Env::wraps(env);
    for (name, value) in arg_names.iter().zip(arg_values.iter()) {
        inner_env.set(name.clone(), value.clone());
    }

    eval(body, &mut inner_env, mem)
}

fn atom_or_error(value: &AST) -> Result<String, Error> {
    if let &AST::Atom(ref atom) = value {
        Ok(atom.to_string())
    } else {
        Err(Error::WrongArgumentType(Value::from_ast(value)))
    }
}
