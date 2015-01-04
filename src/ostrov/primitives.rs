use runtime::Error;
use values::Value;
use memory::{RcValue, Memory};

pub static PRIMITIVES: [&'static str; 18] = [
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

pub fn apply(name: &String, args: Vec<RcValue>, mem: &mut Memory) -> Result<RcValue, Error> {
    match name.as_slice() {
        "*"      => product(args, mem),
        "+"      => plus(args, mem),
        "-"      => minus(args, mem),
        "/"      => division(args, mem),
        "<"      => less_than(args, mem),
        "<="     => less_than_or_equal(args, mem),
        "="      => equals(args, mem),
        ">"      => greater_than(args, mem),
        ">="     => greater_than_or_equal(args, mem),
        "car"    => car(args, mem),
        "cdr"    => cdr(args, mem),
        "cons"   => cons(args, mem),
        "length" => length(args, mem),
        "list"   => list(args, mem),
        "list?"  => list_question_mark(args, mem),
        "not"    => not(args, mem),
        "null?"  => null(args, mem),
        "pair?"  => pair(args, mem),
        _        => Err(Error::PrimitiveFailed(name.to_string()))
    }
}

fn plus(args: Vec<RcValue>, mem: &mut Memory) -> Result<RcValue, Error> {
    let args_ = try!(list_of_integers(args));
    let sum = args_.iter().fold(0, |sum, n| sum + *n);

    Ok(mem.integer(sum))
}

fn minus(args_: Vec<RcValue>, mem: &mut Memory) -> Result<RcValue, Error> {
    let args = try!(list_of_integers(args_));

    if args.len() == 0 {
        return Err(Error::BadArity(Some("-".to_string())))
    }

    let head = args.first().unwrap();
    let tail = args.tail();

    if tail.is_empty() {
        return Ok(mem.integer(- *head))
    }

    let tail_sum = tail.iter().fold(0, |sum, n| sum + *n);
    Ok(mem.integer(*head - tail_sum))
}

fn division(args_: Vec<RcValue>, mem: &mut Memory) -> Result<RcValue, Error> {
    let args = try!(list_of_integers(args_));

    if args.len() == 0 {
        return Err(Error::BadArity(Some("/".to_string())))
    }

    let head = args.first().unwrap();
    let tail = args.tail();

    if tail.is_empty() {
        return Ok(mem.integer(1 / *head))
    }

    let div = tail.iter().fold(*head, |div, n| div / *n);
    Ok(mem.integer(div))
}

fn product(args_: Vec<RcValue>, mem: &mut Memory) -> Result<RcValue, Error> {
    let args = try!(list_of_integers(args_));
    let product = args.iter().fold(1, |product, n| product * *n);

    Ok(mem.integer(product))
}

fn equals(args_: Vec<RcValue>, mem: &mut Memory) -> Result<RcValue, Error> {
    if args_.len() < 2 {
        return Ok(mem.b_true());
    }

    let args = try!(list_of_integers(args_));
    let head = args.first().unwrap();
    let outcome = args.iter().skip(1).all(|n| *n == *head);

    Ok(mem.boolean(outcome))
}

fn less_than(args: Vec<RcValue>, mem: &mut Memory) -> Result<RcValue, Error> {
    ord(args, mem, |a, b| a < b)
}

fn less_than_or_equal(args: Vec<RcValue>, mem: &mut Memory) -> Result<RcValue, Error> {
    ord(args, mem, |a, b| a <= b)
}

fn greater_than(args: Vec<RcValue>, mem: &mut Memory) -> Result<RcValue, Error> {
    ord(args, mem, |a, b| a > b)
}

fn greater_than_or_equal(args: Vec<RcValue>, mem: &mut Memory) -> Result<RcValue, Error> {
    ord(args, mem, |a, b| a >= b)
}

fn not(args: Vec<RcValue>, mem: &mut Memory) -> Result<RcValue, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity(Some("not".to_string())))
    }

    let outcome = match **args.first().unwrap() {
        Value::Bool(false) => true,
        _                  => false,
    };

    Ok(mem.boolean(outcome))
}

fn list(args: Vec<RcValue>, mem: &mut Memory) -> Result<RcValue, Error> {
    Ok(mem.list(args))
}

fn length(args: Vec<RcValue>, mem: &mut Memory) -> Result<RcValue, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity(Some("length".to_string())));
    }

    match *args[0] {
        Value::List(ref list) =>
            Ok(mem.integer(list.len() as i64)),
        ref value =>
            Err(Error::WrongArgumentType(value.clone())),
    }
}

fn pair(args: Vec<RcValue>, mem: &mut Memory) -> Result<RcValue, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity(Some("pair?".to_string())));
    }

    let outcome = if let Value::List(ref list) = *args[0] {
        !list.is_empty()
    } else if let Value::DottedList(ref _list, ref _el) = *args[0] {
        true
    } else {
        false
    };

    Ok(mem.boolean(outcome))
}

fn cons(args: Vec<RcValue>, mem: &mut Memory) -> Result<RcValue, Error> {
    if args.len() != 2 {
        return Err(Error::BadArity(Some("cons".to_string())));
    }

    let mut list: Vec<RcValue> = Vec::new();
    list.push(args[0].clone());

    if let Value::List(ref l) = *args[1] {
        for item in l.iter() {
            list.push(mem.store(item.clone()));
        }

        Ok(mem.list(list))
    } else {
        Ok(mem.dotted_list(list, (*args[1]).clone()))
    }
}

fn car(args: Vec<RcValue>, mem: &mut Memory) -> Result<RcValue, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity(Some("car".to_string())));
    }

    match *args[0] {
        Value::List(ref l) if !l.is_empty() =>
            Ok(mem.store(l.first().unwrap().clone())),
        Value::DottedList(ref l, ref _t) =>
            Ok(mem.store(l.first().unwrap().clone())),
        ref value =>
            Err(Error::WrongArgumentType(value.clone())),
    }
}

fn cdr(args: Vec<RcValue>, mem: &mut Memory) -> Result<RcValue, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity(Some("cdr".to_string())));
    }

    match *args[0] {
        Value::List(ref l) if !l.is_empty() =>
            match l.tail() {
                tail if tail.is_empty() =>
                    Ok(mem.empty_list()),
                tail => {
                    let mut list = Vec::with_capacity(tail.len());

                    for item in tail.iter() {
                        list.push(mem.store(item.clone()));
                    }

                    Ok(mem.list(list))
                }
            },
        Value::DottedList(ref _l, ref t) =>
            Ok(mem.store(*t.clone())),
        ref value =>
            Err(Error::WrongArgumentType(value.clone())),
    }
}

fn null(args: Vec<RcValue>, mem: &mut Memory) -> Result<RcValue, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity(Some("null?".to_string())));
    }

    let outcome = *args[0] == *mem.empty_list();
    Ok(mem.boolean(outcome))
}

fn list_question_mark(args: Vec<RcValue>, mem: &mut Memory) -> Result<RcValue, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity(Some("list?".to_string())));
    }

    let out = if let Value::List(ref _l) = *args[0] {
        true
    } else {
        false
    };

    Ok(mem.boolean(out))
}

fn list_of_integers(list: Vec<RcValue>) -> Result<Vec<i64>, Error> {
    let mut integers = Vec::with_capacity(list.len());

    for val in list.into_iter() {
        match *val {
            Value::Integer(n) =>
                integers.push(n),
            ref value =>
                return Err(Error::WrongArgumentType(value.clone())),
        }
    }

    Ok(integers)
}

fn ord(args_: Vec<RcValue>, mem: &mut Memory, cmp: |i64, i64| -> bool) -> Result<RcValue, Error> {
    if args_.len() < 2 {
        return Ok(mem.b_true())
    }

    let args = try!(list_of_integers(args_));
    let outcome = range(0, args.len() - 1).all(|i|
        cmp(args[i], args[i + 1u])
    );

    return Ok(mem.boolean(outcome))
}
