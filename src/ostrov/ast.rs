use std::fmt::Error;
use std::fmt::Formatter;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum AST {
    Atom(String),
    Bool(bool),
    DottedList(Vec<AST>, Box<AST>),
    Integer(i64),
    List(Vec<AST>),
}

use self::AST::*;

fn fmt_join_with_spaces<T: Display>(items: &[T], f: &mut Formatter) -> Result<(), Error> {
    for (i, item) in items.iter().enumerate() {
        try!(write!(f, "{}", item));

        if i + 1 != items.len() {
            try!(write!(f, " "));
        }
    }

    Ok(())
}

fn fmt_list<T: Display>(items: &[T], f: &mut Formatter) -> Result<(), Error> {
    try!(write!(f, "("));
    try!(fmt_join_with_spaces(items.as_ref(), f));
    write!(f, ")")
}

fn fmt_dotted_list<T: Display>(items: &[T], right: &T, f: &mut Formatter) -> Result<(), Error> {
    try!(write!(f, "("));
    try!(fmt_join_with_spaces(items.as_ref(), f));
    write!(f, " . {})", right)
}

impl Display for AST {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Atom(ref string) =>
                write!(f, "{}", string),
            Bool(false) =>
                write!(f, "#f"),
            Bool(true) =>
                write!(f, "#t"),
            Integer(ref i) =>
                write!(f, "{}", i),
            List(ref list) =>
                fmt_list(list, f),
            DottedList(ref list, ref value) =>
                fmt_dotted_list(list.as_ref(), &**value, f),
        }
    }
}
