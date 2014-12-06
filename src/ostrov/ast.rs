use std::fmt::Error;
use std::fmt::Formatter;
use std::fmt::Show;

#[deriving(PartialEq, Clone)]
pub enum AST {
    Atom(String),
    Bool(bool),
    Integer(i64),
    List(Vec<AST>),
}

#[inline]
pub fn atom_quote() -> AST {
    AST::Atom("quote".to_string())
}

fn fmt_list(items: &Vec<AST>, f: &mut Formatter) -> Result<(), Error> {
    try!("(".fmt(f));

    let mut i = 0;
    for item in items.iter() {
        try!(item.fmt(f));

        i += 1;
        if i != items.len() {
            try!(" ".fmt(f));
        }
    }

    ")".fmt(f)
}

impl Show for AST {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &AST::Atom(ref string) => string.fmt(f),
            &AST::Bool(false)      => "#f".fmt(f),
            &AST::Bool(true)       => "#t".fmt(f),
            &AST::Integer(ref i)   => i.fmt(f),
            &AST::List(ref list)   => fmt_list(list, f),
        }
    }
}
