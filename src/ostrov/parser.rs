#[phase(plugin)]
extern crate peg_syntax_ext;

use ast::AST;
use runtime::Error;

#[derive(PartialEq)]
enum IntegerSign {
    Positive,
    Negative,
}

fn parse_decimal(str: &str, sign: &IntegerSign) -> AST {
    let integer: i64 = str.parse().unwrap();

    AST::Integer(
        if *sign == IntegerSign::Negative {
            -integer
        } else {
            integer
        }
    )
}

fn parse_sign(str: &str) -> IntegerSign {
    match str {
        "-" => IntegerSign::Negative,
        _   => IntegerSign::Positive,
    }
}

fn parse_atom(str: &str) -> AST {
    AST::Atom(str.to_string())
}

fn parse_list(values: Vec<AST>) -> AST {
    AST::List(values)
}

fn parse_dotted_list(mut left: Vec<AST>, right: AST) -> AST {
    match right {
        AST::List(list) => {
            left.push_all(list.as_slice());
            AST::List(left)
        }
        AST::DottedList(list, right) => {
            left.push_all(list.as_slice());
            AST::DottedList(left, right)
        }
        _ => AST::DottedList(left, box right)
    }
}

fn parse_bool(str: &str) -> AST {
    AST::Bool(str == "t" || str == "T")
}

fn parse_quoted(val: AST) -> AST {
    AST::List(vec!(AST::Atom("quote".to_string()), val))
}

peg! ast(r#"

use ast::AST;

#[pub]
grammar -> Vec<AST> =
    expression*

expression -> AST =
    __ ast:value {
        ast
    }

value -> AST =
    integer
    / boolean
    / identifier
    / quoted
    / list

identifier -> AST =
    identifier:(
        initial subsequent*   { match_str }
        / peculiar_identifier { match_str }
    ) __ {
        super::parse_atom(identifier)
    }

initial -> &'input str =
    constituent
    / special_initial

constituent -> &'input str =
    letter

letter -> &'input str =
    [a-zA-Z] { match_str }

special_initial -> &'input str =
    [!$%&*/<=>?^_~] { match_str }

subsequent -> &'input str =
    initial
    / digit
    / special_subsequent

digit -> &'input str =
    [0-9] { match_str }

peculiar_identifier -> &'input str =
    (
        "->" subsequent*
        / "..."
        / "+"
        / "-"
    )
    { match_str }

special_subsequent -> &'input str =
    [+-.@] { match_str }

integer -> AST =
    sign:sign digits:digits __ {
        super::parse_decimal(digits, &sign)
    }

list -> AST =
    "(" values:(value ** __) ")" __ {
        super::parse_list(values)
    }
    / "[" values:(value ** __) "]" __ {
        super::parse_list(values)
    }
    / "(" left:(value ++ __) "." __ right:value ")" __ {
        super::parse_dotted_list(left, right)
    }
    / "[" left:(value ++ __) "." __ right:value "]" __ {
        super::parse_dotted_list(left, right)
    }

boolean -> AST =
    "\#" value:boolean_char __ {
        super::parse_bool(value)
    }

boolean_char -> &'input str =
    [tfTF] { match_str }

digits -> &'input str =
    digit+ { match_str }

sign -> super::IntegerSign =
    [-+]? {
        super::parse_sign(match_str)
    }

quoted -> AST =
    "'" value:value {
        super::parse_quoted(value)
    }

__ = (whitespace)*

whitespace =
    [ \t\r\n]

"#);

pub fn parse(input: &str) -> Result<Vec<AST>, Error> {
    match ast::grammar(input) {
        Ok(exprs)  => Ok(exprs),
        Err(error) => Err(Error::ParseError(error)),
    }
}
