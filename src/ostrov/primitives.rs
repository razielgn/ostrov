use errors::Error;
use memory::Memory;
use values::{RcValue, Value};

pub static PRIMITIVES: [&'static str; 20] = [
    "*", "+", "-", "/", "<", "<=", "=", ">", ">=", "car", "cdr", "cons",
    "length", "list", "list?", "not", "null?", "pair?", "display", "newline",
];

pub fn apply(
    name: &str,
    args: &[RcValue],
    mem: &mut Memory,
) -> Result<RcValue, Error> {
    match name {
        "*" => product(args, mem),
        "+" => plus(args, mem),
        "-" => minus(args, mem),
        "/" => division(args, mem),
        "<" => less_than(args, mem),
        "<=" => less_than_or_equal(args, mem),
        "=" => equals(args, mem),
        ">" => greater_than(args, mem),
        ">=" => greater_than_or_equal(args, mem),
        "car" => car(args),
        "cdr" => cdr(args),
        "cons" => cons(args, mem),
        "length" => length(args, mem),
        "list" => list(args, mem),
        "list?" => is_list(args, mem),
        "not" => not(args, mem),
        "null?" => null(args, mem),
        "pair?" => pair(args, mem),
        "display" => display(args, mem),
        "newline" => newline(args, mem),
        _ => Err(Error::PrimitiveFailed(name.to_owned())),
    }
}

fn plus(args: &[RcValue], mem: &mut Memory) -> Result<RcValue, Error> {
    let integers = try!(list_of_integers(args));
    let sum = integers.into_iter().sum();
    Ok(mem.integer(sum))
}

fn minus(args: &[RcValue], mem: &mut Memory) -> Result<RcValue, Error> {
    if args.is_empty() {
        return Err(Error::BadArity(Some("-".to_owned())));
    }

    let integers = try!(list_of_integers(args));
    let first = integers[0];

    if integers.len() == 1 {
        Ok(mem.integer(-first))
    } else {
        let sum_of_the_rest = integers.into_iter().skip(1).sum::<i64>();
        Ok(mem.integer(first - sum_of_the_rest))
    }
}

fn division(args: &[RcValue], mem: &mut Memory) -> Result<RcValue, Error> {
    if args.is_empty() {
        return Err(Error::BadArity(Some("/".to_owned())));
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

fn product(args: &[RcValue], mem: &mut Memory) -> Result<RcValue, Error> {
    let integers = try!(list_of_integers(args));
    let product = integers.into_iter().product();
    Ok(mem.integer(product))
}

fn equals(args: &[RcValue], mem: &mut Memory) -> Result<RcValue, Error> {
    if args.len() < 2 {
        return Ok(mem.b_true());
    }

    let integers = try!(list_of_integers(args));
    let first = integers[0];

    let equality = integers.into_iter().skip(1).all(|n| n == first);
    Ok(mem.boolean(equality))
}

fn less_than(args: &[RcValue], mem: &mut Memory) -> Result<RcValue, Error> {
    ord(args, mem, |a, b| a < b)
}

fn less_than_or_equal(
    args: &[RcValue],
    mem: &mut Memory,
) -> Result<RcValue, Error> {
    ord(args, mem, |a, b| a <= b)
}

fn greater_than(args: &[RcValue], mem: &mut Memory) -> Result<RcValue, Error> {
    ord(args, mem, |a, b| a > b)
}

fn greater_than_or_equal(
    args: &[RcValue],
    mem: &mut Memory,
) -> Result<RcValue, Error> {
    ord(args, mem, |a, b| a >= b)
}

fn not(args: &[RcValue], mem: &mut Memory) -> Result<RcValue, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity(Some("not".to_owned())));
    }

    Ok(mem.boolean(args[0] == mem.b_false()))
}

fn list(args: &[RcValue], mem: &mut Memory) -> Result<RcValue, Error> {
    Ok(mem.list(args.to_vec()))
}

fn length(args: &[RcValue], mem: &mut Memory) -> Result<RcValue, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity(Some("length".to_owned())));
    }

    args[0].pair_len().map_or_else(
        || Err(Error::WrongArgumentType(args[0].clone())),
        |length| Ok(mem.integer(length)),
    )
}

fn pair(args: &[RcValue], mem: &mut Memory) -> Result<RcValue, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity(Some("pair?".to_owned())));
    }

    let outcome = args[0].is_pair();
    Ok(mem.boolean(outcome))
}

fn cons(args: &[RcValue], mem: &mut Memory) -> Result<RcValue, Error> {
    if args.len() != 2 {
        return Err(Error::BadArity(Some("cons".to_owned())));
    }

    let left = args[0].clone();
    let right = args[1].clone();
    Ok(mem.pair(left, right))
}

fn car(args: &[RcValue]) -> Result<RcValue, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity(Some("car".to_owned())));
    }

    match *args[0] {
        Value::Pair(ref left, ref _right) => Ok(left.clone()),
        _ => Err(Error::WrongArgumentType(args[0].clone())),
    }
}

fn cdr(args: &[RcValue]) -> Result<RcValue, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity(Some("cdr".to_owned())));
    }

    match *args[0] {
        Value::Pair(ref _left, ref right) => Ok(right.clone()),
        _ => Err(Error::WrongArgumentType(args[0].clone())),
    }
}

fn null(args: &[RcValue], mem: &mut Memory) -> Result<RcValue, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity(Some("null?".to_owned())));
    }

    Ok(mem.boolean(args[0] == mem.nil()))
}

fn is_list(args: &[RcValue], mem: &mut Memory) -> Result<RcValue, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity(Some("list?".to_owned())));
    }

    let outcome = args[0].is_list();
    Ok(mem.boolean(outcome))
}

fn list_of_integers(list: &[RcValue]) -> Result<Vec<i64>, Error> {
    let mut integers = Vec::with_capacity(list.len());

    for val in list.iter() {
        match **val {
            Value::Integer(n) => integers.push(n),
            _ => return Err(Error::WrongArgumentType(val.clone())),
        }
    }

    Ok(integers)
}

fn ord<F>(args: &[RcValue], mem: &mut Memory, cmp: F) -> Result<RcValue, Error>
where
    F: Fn(i64, i64) -> bool,
{
    if args.len() < 2 {
        return Ok(mem.b_true());
    }

    let integers = try!(list_of_integers(args));
    let outcome =
        (0..integers.len() - 1).all(|i| cmp(integers[i], integers[i + 1]));

    Ok(mem.boolean(outcome))
}

fn display(args: &[RcValue], mem: &mut Memory) -> Result<RcValue, Error> {
    if args.len() != 1 {
        return Err(Error::BadArity(Some("display".to_owned())));
    }

    print!("{}", args.first().unwrap());

    Ok(mem.b_true())
}

fn newline(args: &[RcValue], mem: &mut Memory) -> Result<RcValue, Error> {
    if !args.is_empty() {
        return Err(Error::BadArity(Some("newline".to_owned())));
    }

    println!();

    Ok(mem.b_true())
}
