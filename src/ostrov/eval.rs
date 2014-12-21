use ast::AST;
use env::Env;
use runtime::Error;
use values::Value;
use values::ArgumentsType;
use primitives;
use special_forms;
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
        return Ok(mem.empty_list());
    }

    let head = list.head().unwrap();
    let tail = list.tail();

    if is_quoted(head) {
        return Err(Error::UnappliableValue(Value::from_ast(head)));
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
            _        => (),
        }
    }

    let fun  = try!(eval(head, env, mem));
    let args = try!(eval_args(tail, env, mem));

    match fun.deref() {
        &Value::Fn(ref name, ref args_type, ref args_names, ref body) =>
            apply(name, args_type, args_names, args, body, env, mem),
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

fn eval_variable(name: &String, env: &mut Env) -> Result<Rc<Value>, Error> {
    match env.get(name) {
        Some(value) => Ok(value),
        None        => Err(Error::UnboundVariable(name.clone())),
    }
}

fn apply(name: &Option<String>, _args_type: &ArgumentsType, arg_names: &Vec<String>, arg_values: Vec<Rc<Value>>, body: &AST, env: &Env, mem: &mut Memory) -> Result<Rc<Value>, Error> {
    if arg_names.len() != arg_values.len() {
        return Err(Error::BadArity(name.clone()));
    }

    let mut inner_env = wrap_env(arg_names, arg_values, env);
    eval(body, &mut inner_env, mem)
}

fn wrap_env<'a>(arg_names: &Vec<String>, arg_values: Vec<Rc<Value>>, outer_env: &'a Env) -> Env<'a> {
    let mut env = Env::wraps(outer_env);

    for (name, value) in arg_names.iter().zip(arg_values.iter()) {
        env.set(name.clone(), value.clone());
    }

    env
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
