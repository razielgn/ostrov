use runtime::Error;
use memory::Memory;
use values::{RcValue, Value};

pub static PRIMITIVES: [&'static str; 20] = [
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
    "display",
    "newline",
];

pub fn apply(name: &String, args: Vec<RcValue>, mem: &mut Memory) -> Result<RcValue, Error> {
    match &**name {
        "*"      => product(args, mem),
        "+"      => plus(args, mem),
        "-"      => minus(args, mem),
        "/"      => division(args, mem),
        "<"      => less_than(args, mem),
        "<="     => less_than_or_equal(args, mem),
        "="      => equals(args, mem),
        ">"      => greater_than(args, mem),
        ">="     => greater_than_or_equal(args, mem),
        "car"    => car(args),
        "cdr"    => cdr(args),
        "cons"   => cons(args, mem),
        "length" => length(args, mem),
        "list"   => list(args, mem),
        "list?"  => is_list(args, mem),
        "not"    => not(args, mem),
        "null?"  => null(args, mem),
        "pair?"  => pair(args, mem),
        "display"  => display(args, mem),
        "newline"  => newline(args, mem),
        _        => Err(Error::PrimitiveFailed(name.to_string()))
    }
}

fn plus(args: Vec<RcValue>, mem: &mut Memory) -> Result<RcValue, Error> {
    let integers = try!(list_of_integers(args));
    let sum = integers.into_iter().fold(0, |sum, n| sum + n);
    Ok(mem.integer(sum))
}

fn minus(args: Vec<RcValue>, mem: &mut Memory) -> Result<RcValue, Error> {
    if args.len() == 0 {
        return Err(Error::BadArity(Some("-".to_string())))
    }

    let integers = try!(list_of_integers(args));
    let first = integers[0];

    if integers.len() == 1 {
        Ok(mem.integer(-first))
    } else {
        let sum_of_the_rest = integers.into_iter().skip(1).fold(0, |sum, n| sum + n);
        Ok(mem.integer(first - sum_of_the_rest))
    }
}

fn division(args: Vec<RcValue>, mem: &mut Memory) -> Result<RcValue, Error> {
    if args.len() == 0 {
        return Err(Error::BadArity(Some("/".to_string())))
    }

    let integers = try!(list_of_integers(args));
    let first = integers[0];

    if integers.len() == 1 {
        Ok(mem.integer(1 / first))
    } else {
        let div = integers.into_iter().skip(1).fold(first, |div, n| div / n);
        Ok(mem.integer(div))
    }
}

fn product(args: Vec<RcValue>, mem: &mut Memory) -> Result<RcValue, Error> {
    let integers = try!(list_of_integers(args));
    let product = integers.into_iter().fold(1, |product, n| product * n);
    Ok(mem.integer(product))
}

fn equals(args: Vec<RcValue>, mem: &mut Memory) -> Result<RcValue, Error> {
    if args.len() < 2 {
        return Ok(mem.b_true());
    }

    let integers = try!(list_of_integers(args));
    let first = integers[0];

    let equality = integers.into_iter().skip(1).all(|n| n == first);
    Ok(mem.boolean(equality))
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

    Ok(mem.boolean(args[0] == mem.b_false()))
}

fn list(args: Vec<RcValue>, mem: &mut Memory) -> Result<RcValue, Error> {
    Ok(mem.list(args))
}

fn length(args: Vec<RcValue>, mem: &mut Memory) -> Result<RcValue, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity(Some("length".to_string())));
    }

    args[0].pair_len().map_or_else(
        || Err(Error::WrongArgumentType(args[0].clone())),
        |length| Ok(mem.integer(length)),
    )
}

fn pair(args: Vec<RcValue>, mem: &mut Memory) -> Result<RcValue, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity(Some("pair?".to_string())));
    }

    let outcome = args[0].is_pair();
    Ok(mem.boolean(outcome))
}

fn cons(args: Vec<RcValue>, mem: &mut Memory) -> Result<RcValue, Error> {
    if args.len() != 2 {
        return Err(Error::BadArity(Some("cons".to_string())));
    }

    let left = args[0].clone();
    let right = args[1].clone();
    Ok(mem.pair(left, right))
}

fn car(args: Vec<RcValue>) -> Result<RcValue, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity(Some("car".to_string())));
    }

    match *args[0] {
        Value::Pair(ref left, ref _right) =>
            Ok(left.clone()),
        _ =>
            Err(Error::WrongArgumentType(args[0].clone())),
    }
}

fn cdr(args: Vec<RcValue>) -> Result<RcValue, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity(Some("cdr".to_string())));
    }

    match *args[0] {
        Value::Pair(ref _left, ref right) =>
            Ok(right.clone()),
        _ =>
            Err(Error::WrongArgumentType(args[0].clone())),
    }
}

fn null(args: Vec<RcValue>, mem: &mut Memory) -> Result<RcValue, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity(Some("null?".to_string())));
    }

    Ok(mem.boolean(args[0] == mem.nil()))
}

fn is_list(args: Vec<RcValue>, mem: &mut Memory) -> Result<RcValue, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity(Some("list?".to_string())));
    }

    let outcome = args[0].is_list();
    Ok(mem.boolean(outcome))
}

fn list_of_integers(list: Vec<RcValue>) -> Result<Vec<i64>, Error> {
    let mut integers = Vec::with_capacity(list.len());

    for val in list.into_iter() {
        match *val {
            Value::Integer(n) =>
                integers.push(n),
            _ =>
                return Err(Error::WrongArgumentType(val.clone())),
        }
    }

    Ok(integers)
}

fn ord<F>(args: Vec<RcValue>, mem: &mut Memory, cmp: F) -> Result<RcValue, Error> where F: Fn(i64, i64) -> bool {
    if args.len() < 2 {
        return Ok(mem.b_true())
    }

    let integers = try!(list_of_integers(args));
    let outcome = (0..integers.len() - 1).all(|i|
        cmp(integers[i], integers[i + 1])
    );

    return Ok(mem.boolean(outcome))
}

fn display(args: Vec<RcValue>, mem: &mut Memory) -> Result<RcValue, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity(Some("display".to_string())));
    }

    print!("{}", args.first().unwrap());

    Ok(mem.b_true())
}

fn newline(args: Vec<RcValue>, mem: &mut Memory) -> Result<RcValue, Error> {
    if args.len() != 0 {
        return Err(Error::BadArity(Some("newline".to_string())));
    }

    println!("");

    Ok(mem.b_true())
}
