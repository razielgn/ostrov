use ast::AST;
use env::CellEnv;
use memory::Memory;
use instructions::Bytecode;

use std::fmt::Error;
use std::fmt::Formatter;
use std::fmt::Debug;
use std::fmt::Display;
use std::rc::Rc;

pub type RcValue = Rc<Value>;

#[derive(Clone)]
pub enum Value {
    Atom(String),
    Bool(bool),
    Nil,
    Unspecified,
    Pair(RcValue, RcValue),
    Closure {
        name: Option<String>,
        args_type: ArgumentsType,
        args: Vec<String>,
        closure: CellEnv,
        code: Bytecode,
    },
    PrimitiveFn(String),
    Integer(i64),
}

#[derive(Copy, PartialEq, Clone, Debug)]
pub enum ArgumentsType {
    Fixed,
    Variable,
    Any,
}

impl Value {
    pub fn is_list(&self) -> bool {
        match self {
            &Value::Nil =>
                true,
            &Value::Pair(ref _left, ref right) =>
                right.is_list(),
            _ =>
                false,
        }
    }

    pub fn is_pair(&self) -> bool {
        match self {
            &Value::Pair(..) => true,
            _                => false,
        }
    }

    pub fn pair_len(&self) -> Option<i64> {
        fn pair_len_rec(val: &Value, acc: i64) -> Option<i64> {
            match val {
                &Value::Nil =>
                    Some(acc),
                &Value::Pair(ref _left, ref right) =>
                    pair_len_rec(right, acc + 1),
                _ =>
                    None,
            }
        }

        pair_len_rec(self, 0)
    }
}

fn fmt_join_with_spaces<T: Display>(items: &[T], f: &mut Formatter) -> Result<(), Error> {
    for (i, item) in items.iter().enumerate() {
        try!(write!(f, "{}", item));

        if i + 1 != items.len() {
            try!(write!(f, " "));
        }
    }

    Ok(())
}

fn fmt_list<T: Display>(items: &Vec<T>, f: &mut Formatter) -> Result<(), Error> {
    try!(write!(f, "("));
    try!(fmt_join_with_spaces(items.as_ref(), f));
    write!(f, ")")
}

fn fmt_pair(left: &RcValue, right: &RcValue, f: &mut Formatter) -> Result<(), Error> {
    try!(write!(f, "{}", left));

    match **right {
        Value::Nil =>
            Ok(()),
        Value::Pair(ref left, ref right) => {
            try!(write!(f, " "));
            fmt_pair(left, right, f)
        }
        _ =>
            write!(f, " . {}", right),
    }
}

fn fmt_dotted_list<T: Display>(items: &[T], right: &T, f: &mut Formatter) -> Result<(), Error> {
    try!(write!(f, "("));
    try!(fmt_join_with_spaces(items.as_ref(), f));
    write!(f, " . {})", right)
}

fn fmt_primitive(name: &String, f: &mut Formatter) -> Result<(), Error> {
    write!(f, "<primitive procedure {}>", name)
}

fn fmt_procedure(name: &Option<String>, args_type: &ArgumentsType, args: &Vec<String>, f: &mut Formatter) -> Result<(), Error> {
    try!(write!(f, "<"));

    match name {
        &Some(ref n) => {
            try!(write!(f, "procedure {}", n));
        }
        _ => {
            try!(write!(f, "lambda"));
        }
    };

    try!(write!(f, " "));

    match args_type {
        &ArgumentsType::Any =>
            try!(write!(f, "{}", args[0])),
        &ArgumentsType::Fixed =>
            try!(fmt_list(args, f)),
        &ArgumentsType::Variable => {
            let head = &args[0 .. args.len() - 1];
            let tail = args.last().unwrap();
            try!(fmt_dotted_list(head, tail, f));
        }
    }

    write!(f, ">")
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Value::Atom(ref string) =>
                write!(f, "{}", string),
            &Value::Bool(false) =>
                write!(f, "#f"),
            &Value::Bool(true) =>
                write!(f, "#t"),
            &Value::Nil(..) =>
                write!(f, "()"),
            &Value::Unspecified(..) =>
                write!(f, "<unspecified>"),
            &Value::Pair(ref left, ref right) => {
                try!(write!(f, "("));
                try!(fmt_pair(left, right, f));
                write!(f, ")")
            }
            &Value::Closure { ref name, ref args_type, ref args, .. } =>
                fmt_procedure(name, args_type, args, f),
            &Value::Integer(ref i) =>
                write!(f, "{}", i),
            &Value::PrimitiveFn(ref name) =>
                fmt_primitive(name, f),
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let t = match self {
            &Value::Atom(..)        => "Atom",
            &Value::Bool(..)        => "Bool",
            &Value::Integer(..)     => "Integer",
            &Value::Nil(..)         => "Nil",
            &Value::Unspecified     => "Unspecified",
            &Value::Pair(..)        => "Pair",
            &Value::PrimitiveFn(..) => "PrimitiveFn",
            &Value::Closure { .. }  => "Closure",
        };

        write!(f, "{}({})", t, self)
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match self {
            &Value::Atom(ref a) =>
                if let &Value::Atom(ref b) = other { a == b } else { false },
            &Value::Bool(ref a) =>
                if let &Value::Bool(ref b) = other { a == b } else { false },
            &Value::Integer(ref a) =>
                if let &Value::Integer(ref b) = other { a == b } else { false },
            &Value::Nil =>
                if let &Value::Nil = other { true } else { false },
            &Value::Unspecified =>
                if let &Value::Unspecified = other { true } else { false },
            &Value::Pair(ref left, ref right) =>
                if let &Value::Pair(ref left2, ref right2) = other { (left, right) == (left2, right2) } else { false },
            &Value::PrimitiveFn(ref a) =>
                if let &Value::PrimitiveFn(ref b) = other { a == b } else { false },
            &Value::Closure { .. } =>
                false
        }
    }
}

impl Value {
    pub fn from_ast(ast: &AST, mem: &mut Memory) -> RcValue {
        match ast {
            &AST::Atom(ref string) =>
                mem.intern(string.to_owned()),
            &AST::Bool(b) =>
                mem.boolean(b),
            &AST::Integer(i) =>
                mem.integer(i),
            &AST::List(ref list) => {
                let values = list.iter().map(|ast| Value::from_ast(ast, mem)).collect();
                mem.list(&values)
            }
            &AST::DottedList(ref list, ref value) => {
                let values: Vec<RcValue> = list.iter().map(|ast| Value::from_ast(ast, mem)).collect();
                let value = Value::from_ast(&*value.clone(), mem);
                values
                    .iter()
                    .rev()
                    .fold(value, |cdr, car| {
                        mem.pair(car.clone(), cdr)
                    })
            }
        }
    }
}
