use ast::AST;
use env::CellEnv;
use runtime::Error;
use primitives;
use special_forms;
use memory::Memory;
use values::{Value, RcValue, ArgumentsType};

pub fn eval(value: &AST, env: CellEnv, memory: &mut Memory) -> Result<RcValue, Error> {
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

fn eval_list(list: &Vec<AST>, env: CellEnv, mem: &mut Memory) -> Result<RcValue, Error> {
    if list.is_empty() {
        return Ok(mem.empty_list());
    }

    let head = list.first().unwrap();
    let tail = list.tail();

    if is_quoted(head) {
        let value = Value::from_ast(head, mem);
        return Err(Error::UnappliableValue(value));
    }

    if let &AST::Atom(ref special_form) = head {
        let args = tail;

        match special_form.as_slice() {
            "and"    => return special_forms::and(args, env, mem),
            "define" => return special_forms::define(args, env, mem),
            "if"     => return special_forms::if_(args, env, mem),
            "lambda" => return special_forms::lambda(args, None, mem),
            "or"     => return special_forms::or(args, env, mem),
            "quote"  => return special_forms::quote(args, mem),
            "set!"   => return special_forms::set(args, env, mem),
            _        => (),
        }
    }

    let fun  = try!(eval(head, env.clone(), mem));
    let args = try!(eval_args(tail, env.clone(), mem));

    match *fun {
        Value::Fn(ref name, args_type, ref args_names, ref body) =>
            apply(name, args_type, args_names, args, body, env.clone(), mem),
        Value::PrimitiveFn(ref name) =>
            primitives::apply(name, args, mem),
        _ =>
            Err(Error::UnappliableValue(fun.clone()))
    }
}

fn eval_args(args: &[AST], env: CellEnv, mem: &mut Memory) -> Result<Vec<RcValue>, Error> {
    let mut out = Vec::with_capacity(args.len());

    for arg in args.iter() {
        let evald_arg = try!(eval(arg, env.clone(), mem));
        out.push(evald_arg);
    }

    Ok(out)
}

fn eval_variable(name: &String, env: CellEnv) -> Result<RcValue, Error> {
    match env.get(name) {
        Some(value) => Ok(value),
        None        => Err(Error::UnboundVariable(name.clone())),
    }
}

fn apply(name: &Option<String>, args_type: ArgumentsType, arg_names: &Vec<String>, arg_values: Vec<RcValue>, body: &Vec<AST>, env: CellEnv, mem: &mut Memory) -> Result<RcValue, Error> {
    let inner_env = CellEnv::wraps(env);

    match args_type {
        ArgumentsType::Any => {
            inner_env.set(arg_names[0].clone(), mem.list(arg_values));
        }
        ArgumentsType::Fixed => {
            if arg_names.len() != arg_values.len() {
                return Err(Error::BadArity(name.clone()));
            }

            for (name, value) in arg_names.iter().zip(arg_values.iter()) {
                inner_env.set(name.clone(), value.clone());
            }
        }
        ArgumentsType::Variable => {
            let fixed_arg_names = arg_names.slice(0, arg_names.len() - 1);

            if fixed_arg_names.len() > arg_values.len() {
                return Err(Error::BadArity(name.clone()));
            }

            for (name, value) in fixed_arg_names.iter().zip(arg_values.iter()) {
                inner_env.set(name.clone(), value.clone());
            }

            let var_args = arg_values.into_iter()
                .skip(fixed_arg_names.len())
                .collect();

            inner_env.set(
                arg_names.last().unwrap().clone(),
                mem.list(var_args)
            );
        }
    };

    eval_sequence(body, inner_env, mem)
}

fn eval_sequence(seq: &Vec<AST>, env: CellEnv, mem: &mut Memory) -> Result<RcValue, Error> {
    let mut result = mem.empty_list();

    for expr in seq.iter() {
        result = try!(eval(expr, env.clone(), mem));
    }

    Ok(result)
}

fn is_quoted(value: &AST) -> bool {
    match value {
        &AST::List(ref list) =>
            match list[0] {
                AST::Atom(ref a) if a.as_slice() == "quote" =>
                    true,
                _ =>
                    false
            },
        _ =>
            false
    }
}
