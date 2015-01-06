use ast::AST;
use env::CellEnv;
use memory::Memory;

use std::fmt::Error;
use std::fmt::Formatter;
use std::fmt::Show;
use std::rc::Rc;

pub type RcValue = Rc<Value>;

#[derive(Clone)]
pub enum Value {
    Atom(String),
    Bool(bool),
    DottedList(Vec<RcValue>, RcValue),
    Fn(Option<String>, ArgumentsType, Vec<String>, CellEnv, Vec<AST>),
    PrimitiveFn(String),
    Integer(i64),
    List(Vec<RcValue>),
}

#[derive(Copy, PartialEq, Clone)]
pub enum ArgumentsType {
    Fixed,
    Variable,
    Any,
}

fn fmt_join_with_spaces<T: Show>(items: &[T], f: &mut Formatter) -> Result<(), Error> {
    for (i, item) in items.iter().enumerate() {
        try!(item.fmt(f));

        if i + 1 != items.len() {
            try!(" ".fmt(f));
        }
    }

    Ok(())
}

fn fmt_list<T: Show>(items: &Vec<T>, f: &mut Formatter) -> Result<(), Error> {
    try!("(".fmt(f));
    try!(fmt_join_with_spaces(items.as_slice(), f));
    try!(")".fmt(f));

    Ok(())
}

fn fmt_dotted_list<T: Show>(items: &[T], right: &T, f: &mut Formatter) -> Result<(), Error> {
    try!("(".fmt(f));
    try!(fmt_join_with_spaces(items.as_slice(), f));
    try!(" . ".fmt(f));
    try!(right.fmt(f));
    try!(")".fmt(f));

    Ok(())
}

fn fmt_primitive(name: &String, f: &mut Formatter) -> Result<(), Error> {
    try!("<primitive procedure ".fmt(f));
    try!(name.fmt(f));
    try!(">".fmt(f));

    Ok(())
}

fn fmt_procedure(name: &Option<String>, args_type: &ArgumentsType, args: &Vec<String>, f: &mut Formatter) -> Result<(), Error> {
    try!("<".fmt(f));

    match name {
        &Some(ref n) => {
            try!("procedure ".fmt(f));
            try!(n.fmt(f));
        }
        _ => {
            try!("lambda".fmt(f));
        }
    };

    try!(" ".fmt(f));

    match args_type {
        &ArgumentsType::Any =>
            try!(args[0].fmt(f)),
        &ArgumentsType::Fixed =>
            try!(fmt_list(args, f)),
        &ArgumentsType::Variable => {
            let head = args.slice(0, args.len() - 1);
            let tail = args.last().unwrap();
            try!(fmt_dotted_list(head, tail, f));
        }
    }

    try!(">".fmt(f));

    Ok(())
}

impl Show for Value {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Value::Atom(ref string) => string.fmt(f),
            &Value::Bool(false) => "#f".fmt(f),
            &Value::Bool(true) => "#t".fmt(f),
            &Value::DottedList(ref list, ref value) => fmt_dotted_list(list.as_slice(), &*value, f),
            &Value::Fn(ref name, ref args_type, ref args, ref _closure, ref _body) => fmt_procedure(name, args_type, args, f),
            &Value::Integer(ref i) => i.fmt(f),
            &Value::List(ref list) => fmt_list(list, f),
            &Value::PrimitiveFn(ref name) => fmt_primitive(name, f),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match self {
            &Value::Atom(ref a) =>
                if let &Value::Atom(ref b) = other { a == b } else { false },
            &Value::Bool(ref a) =>
                if let &Value::Bool(ref b) = other { a == b } else { false },
            &Value::DottedList(ref a, ref a1) =>
                if let &Value::DottedList(ref b, ref b1) = other { (a, a1) == (b, b1) } else { false },
            &Value::Fn(ref a, ref a1, ref a2, ref _closure, ref a3) =>
                if let &Value::Fn(ref b, ref b1, ref b2, ref _closure, ref b3) = other { (a, a1, a2, a3) == (b, b1, b2, b3) } else { false },
            &Value::Integer(ref a) =>
                if let &Value::Integer(ref b) = other { a == b } else { false },
            &Value::List(ref a) =>
                if let &Value::List(ref b) = other { a == b } else { false },
            &Value::PrimitiveFn(ref a) =>
                if let &Value::PrimitiveFn(ref b) = other { a == b } else { false },
        }
    }
}

impl Value {
    pub fn from_ast(ast: &AST, mem: &mut Memory) -> RcValue {
        match ast {
            &AST::Atom(ref string) =>
                mem.intern(string.to_string()),
            &AST::Bool(b) =>
                mem.boolean(b),
            &AST::Integer(i) =>
                mem.integer(i),
            &AST::List(ref list) => {
                let values = list.iter().map(|ast| Value::from_ast(ast, mem)).collect();
                mem.list(values)
            }
            &AST::DottedList(ref list, ref value) => {
                let values = list.iter().map(|ast| Value::from_ast(ast, mem)).collect();
                let value = Value::from_ast(&*value.clone(), mem);
                mem.dotted_list(values, value)
            }
        }
    }
}
