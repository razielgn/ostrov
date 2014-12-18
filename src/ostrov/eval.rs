use ast::AST;
use env::Env;
use runtime::Error;
use values::Value;

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

    if let Value::Fn(name, args_names, body) = fun {
        apply(name, args_names, args, body, env)
    } else {
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

fn eval_fun_plus(args: Vec<Value>) -> Result<Value, Error> {
    let args_ = try!(list_of_integers(args));
    let sum = args_.iter().fold(0, |sum, n| sum + *n);
    Ok(Value::Integer(sum))
}

fn eval_fun_minus(args_: Vec<Value>) -> Result<Value, Error> {
    let args = try!(list_of_integers(args_));

    if args.len() == 0 {
        return Err(Error::BadArity(Some("-".to_string())))
    }

    let head = args.head().unwrap();
    let tail = args.tail();

    if tail.is_empty() {
        return Ok(Value::Integer(- *head))
    }

    let tail_sum = tail.iter().fold(0, |sum, n| sum + *n);
    Ok(Value::Integer(*head - tail_sum))
}

fn eval_fun_division(args_: Vec<Value>) -> Result<Value, Error> {
    let args = try!(list_of_integers(args_));

    if args.len() == 0 {
        return Err(Error::BadArity(Some("/".to_string())))
    }

    let head = args.head().unwrap();
    let tail = args.tail();

    if tail.is_empty() {
        return Ok(Value::Integer(1 / *head))
    }

    let div = tail.iter().fold(*head, |div, n| div / *n);
    Ok(Value::Integer(div))
}

fn eval_fun_product(args_: Vec<Value>) -> Result<Value, Error> {
    let args = try!(list_of_integers(args_));
    let product = args.iter().fold(1, |product, n| product * *n);
    Ok(Value::Integer(product))
}

fn eval_fun_equals(args_: Vec<Value>) -> Result<Value, Error> {
    if args_.len() < 2 {
        return Ok(Value::Bool(true))
    }

    let args = try!(list_of_integers(args_));
    let head = args.head().unwrap();
    let outcome = args.iter().skip(1).all(|n| *n == *head);
    Ok(Value::Bool(outcome))
}

fn eval_fun_less_than(args: Vec<Value>) -> Result<Value, Error> {
    eval_fun_ord(args, |a, b| a < b)
}

fn eval_fun_less_than_or_equal(args: Vec<Value>) -> Result<Value, Error> {
    eval_fun_ord(args, |a, b| a <= b)
}

fn eval_fun_greater_than(args: Vec<Value>) -> Result<Value, Error> {
    eval_fun_ord(args, |a, b| a > b)
}

fn eval_fun_greater_than_or_equal(args: Vec<Value>) -> Result<Value, Error> {
    eval_fun_ord(args, |a, b| a >= b)
}

fn eval_fun_ord(args_: Vec<Value>, cmp: |i64, i64| -> bool) -> Result<Value, Error> {
    if args_.len() < 2 {
        return Ok(Value::Bool(true))
    }

    let args = try!(list_of_integers(args_));
    let outcome = range(0, args.len() - 1).all(|i|
        cmp(args[i], args[i + 1u])
    );

    Ok(Value::Bool(outcome))
}

fn eval_fun_not(args: Vec<Value>) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity(Some("not".to_string())))
    }

    let outcome = match args.head().unwrap() {
        &Value::Bool(false) => true,
        _                 => false,
    };

    Ok(Value::Bool(outcome))
}

fn eval_fun_list(args: Vec<Value>) -> Result<Value, Error> {
    Ok(Value::List(args))
}

fn eval_fun_length(args: Vec<Value>) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity(Some("length".to_string())));
    }

    if let Value::List(ref list) = args[0] {
        Ok(Value::Integer(list.len() as i64))
    } else {
        Err(Error::WrongArgumentType(args[0].clone()))
    }
}

fn eval_fun_pair(args: Vec<Value>) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity(Some("pair?".to_string())));
    }

    if let Value::List(ref list) = args[0] {
        Ok(Value::Bool(!list.is_empty()))
    } else if let Value::DottedList(ref _list, ref _el) = args[0] {
        Ok(Value::Bool(true))
    } else {
        Ok(Value::Bool(false))
    }
}

fn eval_fun_cons(args: Vec<Value>) -> Result<Value, Error> {
    if args.len() != 2 {
        return Err(Error::BadArity(Some("cons".to_string())));
    }

    let mut list: Vec<Value> = Vec::new();

    if let Value::List(ref l) = args[1] {
        list.push(args[0].clone());
        list.push_all(l.clone().as_slice());

        Ok(Value::List(list))
    } else {
        list.push(args[0].clone());
        Ok(Value::DottedList(list, box args[1].clone()))
    }
}

fn eval_fun_car(args: Vec<Value>) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity(Some("car".to_string())));
    }

    match args[0] {
        Value::List(ref l) if !l.is_empty() => {
            Ok(l.head().unwrap().clone())
        }
        Value::DottedList(ref l, ref _t) => {
            Ok(l.head().unwrap().clone())
        }
        ref bad_arg => {
            Err(Error::WrongArgumentType(bad_arg.clone()))
        }
    }
}

fn eval_fun_cdr(args: Vec<Value>) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity(Some("cdr".to_string())));
    }

    match args[0] {
        Value::List(ref l) if !l.is_empty() => {
            Ok(Value::List(l.tail().to_vec()))
        }
        Value::DottedList(ref _l, ref t) => {
            Ok(*t.clone())
        }
        ref bad_arg => {
            Err(Error::WrongArgumentType(bad_arg.clone()))
        }
    }
}

fn eval_fun_null(args: Vec<Value>) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity(Some("null?".to_string())));
    }

    let out = match args[0] {
        Value::List(ref l) if l.is_empty() => true,
        _ => false
    };

    Ok(Value::Bool(out))
}

fn eval_fun_list_question_mark(args: Vec<Value>) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity(Some("list?".to_string())));
    }

    let out = if let Value::List(ref _l) = args[0] {
        true
    } else {
        false
    };

    Ok(Value::Bool(out))
}

fn eval_quote(list: &[AST]) -> Result<Value, Error> {
    Ok(Value::from_ast(&list[0]))
}

fn list_of_integers(list: Vec<Value>) -> Result<Vec<i64>, Error> {
    let mut integers = Vec::with_capacity(list.len());

    for val in list.iter() {
        if let &Value::Integer(n) = val {
            integers.push(n);
        } else {
            return Err(Error::WrongArgumentType(val.clone()))
        };
    }

    Ok(integers)
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
