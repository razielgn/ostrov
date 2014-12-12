use std::fmt::Error;
use std::fmt::Formatter;
use std::fmt::Show;

#[deriving(PartialEq, Clone)]
pub enum AST {
    Atom(String),
    Bool(bool),
    DottedList(Vec<AST>, Box<AST>),
    Integer(i64),
    List(Vec<AST>),
    Fn(Option<String>, Vec<String>, Box<AST>),
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

fn fmt_list(items: &Vec<AST>, f: &mut Formatter) -> Result<(), Error> {
    try!("(".fmt(f));
    try!(fmt_join_with_spaces(items.as_slice(), f));
    try!(")".fmt(f));

    Ok(())
}

fn fmt_dotted_list(items: &Vec<AST>, right: &AST, f: &mut Formatter) -> Result<(), Error> {
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

impl Show for AST {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &AST::Atom(ref string) => string.fmt(f),
            &AST::Bool(false)      => "#f".fmt(f),
            &AST::Bool(true)       => "#t".fmt(f),
            &AST::Integer(ref i)   => i.fmt(f),
            &AST::List(ref list)   => fmt_list(list, f),
            &AST::DottedList(ref list, ref value) => fmt_dotted_list(list, &**value, f),
            &AST::Fn(ref name, ref args, ref _body) => fmt_procedure(name, args, f),
        }
    }
}
