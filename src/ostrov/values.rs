use ast::AST;

use std::fmt::Error;
use std::fmt::Formatter;
use std::fmt::Show;

#[deriving(PartialEq, Clone)]
pub enum Value {
    Atom(String),
    Bool(bool),
    DottedList(Vec<Value>, Box<Value>),
    Fn(Option<String>, Vec<String>, AST),
    Integer(i64),
    List(Vec<Value>),
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

fn fmt_list(items: &Vec<Value>, f: &mut Formatter) -> Result<(), Error> {
    try!("(".fmt(f));
    try!(fmt_join_with_spaces(items.as_slice(), f));
    try!(")".fmt(f));

    Ok(())
}

fn fmt_dotted_list(items: &Vec<Value>, right: &Value, f: &mut Formatter) -> Result<(), Error> {
    try!("(".fmt(f));
    try!(fmt_join_with_spaces(items.as_slice(), f));
    try!(" . ".fmt(f));
    try!(right.fmt(f));
    try!(")".fmt(f));

    Ok(())
}

fn fmt_procedure(name: &Option<String>, args: &Vec<String>, f: &mut Formatter) -> Result<(), Error> {
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

    try!(" (".fmt(f));
    try!(fmt_join_with_spaces(args.as_slice(), f));
    try!(")>".fmt(f));

    Ok(())
}

impl Show for Value {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Value::Atom(ref string) => string.fmt(f),
            &Value::Bool(false) => "#f".fmt(f),
            &Value::Bool(true) => "#t".fmt(f),
            &Value::DottedList(ref list, ref value) => fmt_dotted_list(list, &**value, f),
            &Value::Fn(ref name, ref args, ref _body) => fmt_procedure(name, args, f),
            &Value::Integer(ref i) => i.fmt(f),
            &Value::List(ref list) => fmt_list(list, f),
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
            &AST::Fn(ref name, ref args, ref body) =>
                Value::Fn(name.clone(), args.clone(), *body.clone()),
        }
    }
}
