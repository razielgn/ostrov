use ast::AST;
use ast::AST::*;
use nom::types::CompleteStr;
use nom::{alpha1, digit1, multispace0, multispace1, Err};
use nom_locate::LocatedSpan;

pub type Span<'a> = LocatedSpan<CompleteStr<'a>>;

#[derive(PartialEq, Debug)]
pub struct ParseError<'a>(Err<Span<'a>>);

named!(boolean(Span) -> AST,
    do_parse!(
        tag!("#") >>
        c: one_of!("ftFT") >>
        (Bool(c == 't' || c == 'T'))
    )
);

named!(integer(Span) -> AST,
    do_parse!(
        sign: opt!(one_of!("-+")) >>
        n: map_res!(digit1, |d: Span| d.fragment.parse::<i64>()) >>
        (Integer(if sign == Some('-') { -n } else { n }))
    )
);

named!(initial(Span) -> Span, alt!(constituent | special_initial));
named!(constituent(Span) -> Span, recognize!(count!(alpha1, 1)));
named!(special_initial(Span) -> Span, recognize!(one_of!("!$%&*/<=>?^_~")));
named!(one_digit(Span) -> Span, recognize!(count!(digit1, 1)));
named!(subsequent(Span) -> Span, alt!(initial | one_digit | special_subsequent));
named!(special_subsequent(Span) -> Span, recognize!(one_of!("+-.@")));
named!(peculiar_identifier(Span) -> Span,
    alt!(
        recognize!(preceded!(tag!("->"), many0!(subsequent))) |
        tag!("...") |
        tag!("+") |
        tag!("-")
    )
);
named!(atom(Span) -> AST,
    map!(
        alt!(
            recognize!(preceded!(initial, many0!(subsequent))) |
            peculiar_identifier
        ),
        |span| Atom(span.fragment.to_string())
    )
);

named!(quoted(Span) -> AST,
    map!(
        preceded!(char!('\''), value),
        |ast| List(vec![Atom("quote".into()), ast])
    )
);

named_args!(list(op: char, cl: char) <Span, AST>,
    delimited!(
        char!(op),
        map!(separated_list_complete!(multispace1, value), List),
        char!(cl)
    )
);

named_args!(dotted(op: char, cl: char) <Span, AST>,
    delimited!(
        char!(op),
        map!(
            do_parse!(
                left: separated_nonempty_list_complete!(multispace1, value) >>
                ws!(char!('.')) >>
                right: value >>
                (left, right)
            ),
            |(mut left, right)| {
                match right {
                    List(mut list) => {
                        left.append(&mut list);
                        List(left)
                    }
                    DottedList(mut list, right) => {
                        left.append(&mut list);
                        DottedList(left, right)
                    }
                    _ => DottedList(left, Box::new(right))
                }
            }
        ),
        char!(cl)
    )
);

named!(list_or_dotted(Span) -> AST,
    alt!(
        apply!(list, '(', ')')
      | apply!(list, '[', ']')
      | apply!(dotted, '(', ')')
      | apply!(dotted, '[', ']')
    )
);

named!(value(Span) -> AST,
    alt!(boolean | integer | atom | quoted | list_or_dotted)
);

named!(exprs(Span) -> Vec<AST>,
    do_parse!(
        values: delimited!(
            multispace0,
            separated_list_complete!(multispace1, value),
            multispace0
        ) >>
        eof!() >>
        (values)
    )
);

pub fn parse<'a>(input: &'a str) -> Result<Vec<AST>, ParseError<'a>> {
    let input = Span::new(CompleteStr(input));

    exprs(input).map(|(_, v)| v).map_err(ParseError)
}

#[cfg(test)]
mod test {
    use ast::AST;
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
    fn no_expressions() {
        assert_parse!([] as [AST; 0], " \r\n\t \t\n\r");
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
