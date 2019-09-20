use crate::{ast::AST, env::CellEnv, instructions::Bytecode, memory::Memory};
use std::{
    fmt::{Debug, Display, Error, Formatter},
    rc::Rc,
};

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

use self::Value::*;

#[derive(Copy, PartialEq, Clone, Debug)]
pub enum ArgumentsType {
    Fixed,
    Variable,
    Any,
}

use self::ArgumentsType::*;

impl Value {
    pub fn is_list(&self) -> bool {
        match *self {
            Nil => true,
            Pair(ref _left, ref right) => right.is_list(),
            _ => false,
        }
    }

    pub fn is_pair(&self) -> bool {
        if let Pair(..) = *self {
            true
        } else {
            false
        }
    }

    pub fn pair_len(&self) -> Option<i64> {
        fn pair_len_rec(val: &Value, acc: i64) -> Option<i64> {
            match *val {
                Nil => Some(acc),
                Pair(ref _left, ref right) => pair_len_rec(right, acc + 1),
                _ => None,
            }
        }

        pair_len_rec(self, 0)
    }
}

fn fmt_join_with_spaces<T: Display>(
    items: &[T],
    f: &mut Formatter,
) -> Result<(), Error> {
    for (i, item) in items.iter().enumerate() {
        write!(f, "{}", item)?;

        if i + 1 != items.len() {
            write!(f, " ")?;
        }
    }

    Ok(())
}

fn fmt_list<T: Display>(items: &[T], f: &mut Formatter) -> Result<(), Error> {
    write!(f, "(")?;
    fmt_join_with_spaces(items, f)?;
    write!(f, ")")
}

fn fmt_pair(
    left: &RcValue,
    right: &RcValue,
    f: &mut Formatter,
) -> Result<(), Error> {
    write!(f, "{}", left)?;

    match **right {
        Nil => Ok(()),
        Pair(ref left, ref right) => {
            write!(f, " ")?;
            fmt_pair(left, right, f)
        }
        _ => write!(f, " . {}", right),
    }
}

fn fmt_dotted_list<T: Display>(
    items: &[T],
    right: &T,
    f: &mut Formatter,
) -> Result<(), Error> {
    write!(f, "(")?;
    fmt_join_with_spaces(items, f)?;
    write!(f, " . {})", right)
}

fn fmt_primitive(name: &str, f: &mut Formatter) -> Result<(), Error> {
    write!(f, "<primitive procedure {}>", name)
}

fn fmt_procedure(
    name: &Option<String>,
    args_type: &ArgumentsType,
    args: &[String],
    f: &mut Formatter,
) -> Result<(), Error> {
    write!(f, "<")?;

    match *name {
        Some(ref n) => {
            write!(f, "procedure {}", n)?;
        }
        _ => {
            write!(f, "lambda")?;
        }
    };

    write!(f, " ")?;

    match *args_type {
        Any => write!(f, "{}", args[0])?,
        Fixed => fmt_list(args, f)?,
        Variable => {
            let head = &args[0..args.len() - 1];
            let tail = args.last().unwrap();
            fmt_dotted_list(head, tail, f)?;
        }
    }

    write!(f, ">")
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Atom(ref string) => write!(f, "{}", string),
            Bool(false) => write!(f, "#f"),
            Bool(true) => write!(f, "#t"),
            Nil => write!(f, "()"),
            Unspecified => write!(f, "<unspecified>"),
            Pair(ref left, ref right) => {
                write!(f, "(")?;
                fmt_pair(left, right, f)?;
                write!(f, ")")
            }
            Closure {
                ref name,
                ref args_type,
                ref args,
                ..
            } => fmt_procedure(name, args_type, args, f),
            Integer(ref i) => write!(f, "{}", i),
            PrimitiveFn(ref name) => fmt_primitive(name, f),
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let t = match *self {
            Atom(..) => "Atom",
            Bool(..) => "Bool",
            Integer(..) => "Integer",
            Nil => "Nil",
            Unspecified => "Unspecified",
            Pair(..) => "Pair",
            PrimitiveFn(..) => "PrimitiveFn",
            Closure { .. } => "Closure",
        };

        write!(f, "{}({})", t, self)
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match (self, other) {
            (&Atom(ref a), &Atom(ref b)) if a == b => true,
            (&Bool(ref a), &Bool(ref b)) if a == b => true,
            (&Integer(ref a), &Integer(ref b)) if a == b => true,
            (&Nil, &Nil) => true,
            (&Unspecified, &Unspecified) => true,
            (&Pair(ref left1, ref right1), &Pair(ref left2, ref right2))
                if (left1, right1) == (left2, right2) =>
            {
                true
            }
            (&PrimitiveFn(ref a), &PrimitiveFn(ref b)) if a == b => true,
            _ => false,
        }
    }
}

impl Value {
    pub fn from_ast(ast: &AST, mem: &mut Memory) -> RcValue {
        match *ast {
            AST::Atom(ref string) => mem.intern(string.to_owned()),
            AST::Bool(b) => mem.boolean(b),
            AST::Integer(i) => mem.integer(i),
            AST::List(ref list) => {
                let values =
                    list.iter().map(|ast| Value::from_ast(ast, mem)).collect();
                mem.list(values)
            }
            AST::DottedList(ref list, ref value) => {
                let values: Vec<RcValue> =
                    list.iter().map(|ast| Value::from_ast(ast, mem)).collect();
                let value = Value::from_ast(&*value.clone(), mem);
                values
                    .iter()
                    .rev()
                    .fold(value, |cdr, car| mem.pair(car.clone(), cdr))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{ArgumentsType::*, Value::*};
    use crate::env::CellEnv;
    use std::rc::Rc;

    macro_rules! assert_fmt {
        ($s:expr, $v:expr) => {
            assert_eq!($s, format!("{}", $v));
        };
    }

    #[test]
    fn integers() {
        assert_fmt!("1", Integer(1));
        assert_fmt!("-213", Integer(-213));
    }

    #[test]
    fn booleans() {
        assert_fmt!("#t", Bool(true));
        assert_fmt!("#f", Bool(false));
    }

    #[test]
    fn atoms() {
        assert_fmt!("->", Atom("->".into()));
    }

    #[test]
    fn nil() {
        assert_fmt!("()", Nil);
    }

    #[test]
    fn unspecified() {
        assert_fmt!("<unspecified>", Unspecified);
    }

    #[test]
    fn pairs() {
        assert_fmt!(
            "(+ 1 2 #f (1 2))",
            Pair(
                Rc::new(Atom("+".into())),
                Rc::new(Pair(
                    Rc::new(Integer(1)),
                    Rc::new(Pair(
                        Rc::new(Integer(2)),
                        Rc::new(Pair(
                            Rc::new(Bool(false)),
                            Rc::new(Pair(
                                Rc::new(Pair(
                                    Rc::new(Integer(1)),
                                    Rc::new(Pair(
                                        Rc::new(Integer(2)),
                                        Rc::new(Nil)
                                    ))
                                )),
                                Rc::new(Nil)
                            ))
                        ))
                    ))
                ))
            )
        );

        assert_fmt!(
            "(+ (1 2) . a)",
            Pair(
                Rc::new(Atom("+".into())),
                Rc::new(Pair(
                    Rc::new(Pair(
                        Rc::new(Integer(1)),
                        Rc::new(Pair(Rc::new(Integer(2)), Rc::new(Nil))),
                    )),
                    Rc::new(Atom("a".into()))
                ))
            )
        );
    }

    #[test]
    fn closures() {
        assert_fmt!(
            "<procedure foo (bar baz)>",
            Closure {
                name: Some("foo".into()),
                args_type: Fixed,
                args: vec!["bar".into(), "baz".into()],
                code: Default::default(),
                closure: CellEnv::new(),
            }
        );

        assert_fmt!(
            "<lambda (bar baz)>",
            Closure {
                name: None,
                args_type: Fixed,
                args: vec!["bar".into(), "baz".into()],
                code: Default::default(),
                closure: CellEnv::new(),
            }
        );

        assert_fmt!(
            "<lambda (bar . baz)>",
            Closure {
                name: None,
                args_type: Variable,
                args: vec!["bar".into(), "baz".into()],
                code: Default::default(),
                closure: CellEnv::new(),
            }
        );

        assert_fmt!(
            "<lambda bar>",
            Closure {
                name: None,
                args_type: Any,
                args: vec!["bar".into()],
                code: Default::default(),
                closure: CellEnv::new(),
            }
        );
    }

    #[test]
    fn primitive_fns() {
        assert_fmt!("<primitive procedure +>", PrimitiveFn("+".into()));
    }
}
