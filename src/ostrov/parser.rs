use ast::AST;
use errors::Error;
use grammar;

pub type ParseError = grammar::ParseError;

#[derive(PartialEq)]
pub enum IntegerSign {
    Positive,
    Negative,
}

pub fn parse_decimal(str: &str, sign: &IntegerSign) -> AST {
    let integer: i64 = str.parse().unwrap();

    AST::Integer(if *sign == IntegerSign::Negative {
        -integer
    } else {
        integer
    })
}

pub fn parse_sign(str: &str) -> IntegerSign {
    match str {
        "-" => IntegerSign::Negative,
        _ => IntegerSign::Positive,
    }
}

pub fn parse_atom(str: &str) -> AST {
    AST::Atom(str.to_owned())
}

pub fn parse_list(values: Vec<AST>) -> AST {
    AST::List(values)
}

pub fn parse_dotted_list(mut left: Vec<AST>, right: AST) -> AST {
    match right {
        AST::List(list) => {
            for x in list {
                left.push(x);
            }
            AST::List(left)
        }
        AST::DottedList(list, right) => {
            for x in list {
                left.push(x);
            }
            AST::DottedList(left, right)
        }
        _ => AST::DottedList(left, Box::new(right)),
    }
}

pub fn parse_bool(str: &str) -> AST {
    AST::Bool(str == "t" || str == "T")
}

pub fn parse_quoted(val: AST) -> AST {
    AST::List(vec![AST::Atom("quote".to_owned()), val])
}

pub fn parse(input: &str) -> Result<Vec<AST>, Error> {
    match grammar::grammar(input) {
        Ok(exprs) => Ok(exprs),
        Err(error) => Err(Error::ParseError(error)),
    }
}

#[cfg(test)]
mod test {
    use ast::AST::*;

    macro_rules! assert_parse {
        ($expected:expr, $str:expr) => {
            match super::parse($str) {
                Ok(v) => assert_eq!(
                    $expected.as_ref(),
                    v.as_slice(),
                    "{:?} => {:?}",
                    $str,
                    $expected
                ),
                Err(err) => panic!("Failed to parse {:?}: {:?}", $str, err),
            }
        };
    }

    #[test]
    fn multiple_expressions() {
        assert_parse!([Bool(true), Integer(1), Integer(-2)], "  #t 1 -2  ");
    }

    #[test]
    fn booleans() {
        assert_parse!([Bool(true)], "#t");
        assert_parse!([Bool(true)], "#T");
        assert_parse!([Bool(false)], "#f");
        assert_parse!([Bool(false)], "#F");
        assert_parse!([Bool(false)], " \n\r\t#F\t\r\n ");
    }

    #[test]
    fn integers() {
        assert_parse!([Integer(1231923)], "1231923");
        assert_parse!([Integer(1231923)], "+1231923");
        assert_parse!([Integer(-1231923)], "-1231923");
        assert_parse!([Integer(-1231923)], " \n\r\t-1231923\t\r\n ");
    }

    #[test]
    fn atoms() {
        assert_parse!([Atom("a".into())], "a");
        assert_parse!([Atom("z".into())], "z");
        assert_parse!([Atom("A".into())], "A");
        assert_parse!([Atom("Z".into())], "Z");
        assert_parse!([Atom("test".into())], "test");

        assert_parse!([Atom("a25".into())], "a25");
        assert_parse!([Atom("Zep0".into())], "Zep0");

        assert_parse!([Atom("!".into())], "!");
        assert_parse!([Atom("$".into())], "$");
        assert_parse!([Atom("%".into())], "%");
        assert_parse!([Atom("*".into())], "*");
        assert_parse!([Atom("/".into())], "/");
        assert_parse!([Atom("<".into())], "<");
        assert_parse!([Atom("=".into())], "=");
        assert_parse!([Atom(">".into())], ">");
        assert_parse!([Atom("?".into())], "?");
        assert_parse!([Atom("^".into())], "^");
        assert_parse!([Atom("_".into())], "_");
        assert_parse!([Atom("~".into())], "~");

        assert_parse!([Atom("str=>int".into())], "str=>int");
        assert_parse!([Atom("true?".into())], "true?");

        assert_parse!([Atom("what+is+this".into())], "what+is+this");
        assert_parse!([Atom("what-is-this".into())], "what-is-this");
        assert_parse!([Atom("what.is.this".into())], "what.is.this");
        assert_parse!([Atom("what@is@this".into())], "what@is@this");

        assert_parse!([Atom("+".into())], "+");
        assert_parse!([Atom("-".into())], "-");
        assert_parse!([Atom("...".into())], "...");
        assert_parse!([Atom("->".into())], "->");
        assert_parse!([Atom("->test2".into())], "->test2");

        assert_parse!([Atom("lisp".into())], " \n\r\tlisp \n\r\t");
        assert_parse!([Atom("->".into())], " \n\r\t-> \n\r\t");
    }

    #[test]
    fn quoted_literals() {
        assert_parse!([List(vec![Atom("quote".into()), Integer(1)])], "'1");
        assert_parse!(
            [List(vec![Atom("quote".into()), Integer(1)])],
            " \n\r\t'1 \n\r\t"
        );
        assert_parse!(
            [List(vec![
                Atom("quote".into()),
                List(vec![Atom("quote".into()), Atom("atom".into())]),
            ])],
            "''atom"
        );
    }

    #[test]
    fn lists() {
        let empty = [List(vec![])];
        assert_parse!(empty, "()");
        assert_parse!(empty, "[]");

        let l1 = [List(vec![Integer(1)])];
        assert_parse!(l1, "(1 . ())");
        assert_parse!(l1, "[1 . []]");

        let l2 = [List(vec![Integer(1), Integer(2)])];
        assert_parse!(l2, "(1  2)");
        assert_parse!(l2, "(1\t2)");
        assert_parse!(l2, "(1\t\t2)");
        assert_parse!(l2, "(1\r2)");
        assert_parse!(l2, "(1\r\r2)");
        assert_parse!(l2, "(1\n2)");
        assert_parse!(l2, "(1\n\n2)");
        assert_parse!(l2, "(1 \t \r \n 2)");
        assert_parse!(l2, "[1  2]");
        assert_parse!(l2, "[1\t2]");
        assert_parse!(l2, "[1\t\t2]");
        assert_parse!(l2, "[1\r2]");
        assert_parse!(l2, "[1\r\r2]");
        assert_parse!(l2, "[1\n2]");
        assert_parse!(l2, "[1\n\n2]");
        assert_parse!(l2, "[1 \t \r \n 2]");
        assert_parse!(l2, " \n\r\t(1 2) \n\r\t");
        assert_parse!(l2, " \n\r\t[1  2] \n\r\t");
        assert_parse!(l2, "(1 2 . ())");
        assert_parse!(l2, "[1 2 . []]");

        let l3 = [List(vec![Integer(1), Integer(2), Integer(3)])];
        assert_parse!(l3, "(1 2 . (3))");
        assert_parse!(l3, "(1 . (2 . (3 . ())))");
        assert_parse!(l3, "[1 2 . [3]]");
        assert_parse!(l3, "[1 . [2 . [3 . []]]]");
    }

    #[test]
    fn dotted_list() {
        let l2 = [DottedList(vec![Integer(1)], Box::new(Integer(2)))];
        assert_parse!(l2, "(1 . 2)");
        assert_parse!(l2, "[1 . 2]");
        assert_parse!(l2, " \n\r\t(1\t\n\r .\t\n\r 2) \n\r\t");
        assert_parse!(l2, " \n\r\t[1\t\n\r . \t\n\r 2] \n\r\t");

        let l4 = [DottedList(
            vec![Integer(1), Integer(2), Integer(3)],
            Box::new(Integer(4)),
        )];
        assert_parse!(l4, "(1 . (2 . (3 . 4)))");
        assert_parse!(l4, "[1 . [2 . [3 . 4]]]");
    }
}
