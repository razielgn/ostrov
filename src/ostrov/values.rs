use ast::AST;

use std::fmt::Error;
use std::fmt::Formatter;
use std::fmt::Show;

#[deriving(PartialEq, Clone)]
pub enum Value {
    Atom(String),
    Bool(bool),
    DottedList(Vec<Value>, Box<Value>),
    Fn(Option<String>, ArgumentsType, Vec<String>, AST),
    PrimitiveFn(String),
    Integer(i64),
    List(Vec<Value>),
}

#[deriving(Copy, PartialEq, Clone)]
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
            &Value::DottedList(ref list, ref value) => fmt_dotted_list(list.as_slice(), &**value, f),
            &Value::Fn(ref name, ref args_type, ref args, ref _body) => fmt_procedure(name, args_type, args, f),
            &Value::Integer(ref i) => i.fmt(f),
            &Value::List(ref list) => fmt_list(list, f),
            &Value::PrimitiveFn(ref name) => fmt_primitive(name, f),
        }
    }
}

impl Value {
    pub fn from_ast(ast: &AST) -> Value {
        match ast {
            &AST::Atom(ref string) =>
                Value::Atom(string.clone()),
            &AST::Bool(b) =>
                Value::Bool(b),
            &AST::Integer(i) =>
                Value::Integer(i),
            &AST::List(ref list) => {
                let values = list.iter().map(|ast| Value::from_ast(ast)).collect();
                Value::List(values)
            }
            &AST::DottedList(ref list, ref value) => {
                let values = list.iter().map(|ast| Value::from_ast(ast)).collect();
                Value::DottedList(values, box Value::from_ast(&*value.clone()))
            }
        }
    }
}
