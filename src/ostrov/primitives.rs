use runtime::Error;
use values::Value;

pub static PRIMITIVES: [&'static str, ..18] = [
    "*",
    "+",
    "-",
    "/",
    "<",
    "<=",
    "=",
    ">",
    ">=",
    "car",
    "cdr",
    "cons",
    "length",
    "list",
    "list?",
    "not",
    "null?",
    "pair?",
];

pub fn apply(name: &str, args: Vec<Value>) -> Result<Value, Error> {
    match name {
        "*"      => product(args),
        "+"      => plus(args),
        "-"      => minus(args),
        "/"      => division(args),
        "<"      => less_than(args),
        "<="     => less_than_or_equal(args),
        "="      => equals(args),
        ">"      => greater_than(args),
        ">="     => greater_than_or_equal(args),
        "car"    => car(args),
        "cdr"    => cdr(args),
        "cons"   => cons(args),
        "length" => length(args),
        "list"   => list(args),
        "list?"  => list_question_mark(args),
        "not"    => not(args),
        "null?"  => null(args),
        "pair?"  => pair(args),
        _        => Err(Error::PrimitiveFailed(name.to_string()))
    }
}

fn plus(args: Vec<Value>) -> Result<Value, Error> {
    let args_ = try!(list_of_integers(args));
    let sum = args_.iter().fold(0, |sum, n| sum + *n);
    Ok(Value::Integer(sum))
}

fn minus(args_: Vec<Value>) -> Result<Value, Error> {
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

fn division(args_: Vec<Value>) -> Result<Value, Error> {
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

fn product(args_: Vec<Value>) -> Result<Value, Error> {
    let args = try!(list_of_integers(args_));
    let product = args.iter().fold(1, |product, n| product * *n);
    Ok(Value::Integer(product))
}

fn equals(args_: Vec<Value>) -> Result<Value, Error> {
    if args_.len() < 2 {
        return Ok(Value::Bool(true))
    }

    let args = try!(list_of_integers(args_));
    let head = args.head().unwrap();
    let outcome = args.iter().skip(1).all(|n| *n == *head);
    Ok(Value::Bool(outcome))
}

fn less_than(args: Vec<Value>) -> Result<Value, Error> {
    ord(args, |a, b| a < b)
}

fn less_than_or_equal(args: Vec<Value>) -> Result<Value, Error> {
    ord(args, |a, b| a <= b)
}

fn greater_than(args: Vec<Value>) -> Result<Value, Error> {
    ord(args, |a, b| a > b)
}

fn greater_than_or_equal(args: Vec<Value>) -> Result<Value, Error> {
    ord(args, |a, b| a >= b)
}

fn not(args: Vec<Value>) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity(Some("not".to_string())))
    }

    let outcome = match args.head().unwrap() {
        &Value::Bool(false) => true,
        _                 => false,
    };

    Ok(Value::Bool(outcome))
}

fn list(args: Vec<Value>) -> Result<Value, Error> {
    Ok(Value::List(args))
}

fn length(args: Vec<Value>) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity(Some("length".to_string())));
    }

    if let Value::List(ref list) = args[0] {
        Ok(Value::Integer(list.len() as i64))
    } else {
        Err(Error::WrongArgumentType(args[0].clone()))
    }
}

fn pair(args: Vec<Value>) -> Result<Value, Error> {
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

fn cons(args: Vec<Value>) -> Result<Value, Error> {
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

fn car(args: Vec<Value>) -> Result<Value, Error> {
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

fn cdr(args: Vec<Value>) -> Result<Value, Error> {
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

fn null(args: Vec<Value>) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity(Some("null?".to_string())));
    }

    let out = match args[0] {
        Value::List(ref l) if l.is_empty() => true,
        _ => false
    };

    Ok(Value::Bool(out))
}

fn list_question_mark(args: Vec<Value>) -> Result<Value, Error> {
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

fn ord(args_: Vec<Value>, cmp: |i64, i64| -> bool) -> Result<Value, Error> {
    if args_.len() < 2 {
        return Ok(Value::Bool(true))
    }

    let args = try!(list_of_integers(args_));
    let outcome = range(0, args.len() - 1).all(|i|
        cmp(args[i], args[i + 1u])
    );

    Ok(Value::Bool(outcome))
}
