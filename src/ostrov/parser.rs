#[phase(plugin)]
extern crate peg_syntax_ext;

use ast::AST;
use runtime::Error;

#[deriving(PartialEq)]
pub enum IntegerSign {
    Positive,
    Negative,
}

pub mod integer {
    use ast::AST;
    use parser::IntegerSign;

    pub fn parse_decimal(str: &str, sign: &IntegerSign) -> AST {
        let integer: i64 = from_str(str).unwrap();

        AST::Integer(
            if *sign == IntegerSign::Negative {
                -integer
            } else {
                integer
            }
        )
    }

    pub fn parse_sign(str: &str) -> IntegerSign {
        match str {
            "-" => IntegerSign::Negative,
            _   => IntegerSign::Positive,
        }
    }
}

pub mod atom {
    use ast::AST;

    pub fn parse(str: &str) -> AST {
        AST::Atom(str.to_string())
    }
}

pub mod list {
    use ast::AST;

    pub fn parse(values: Vec<AST>) -> AST {
        AST::List(values)
    }

    pub fn parse_dotted(mut left: Vec<AST>, right: AST) -> AST {
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
}

pub mod bool {
    use ast::AST;

    pub fn parse(str: &str) -> AST {
        AST::Bool(str == "t" || str == "T")
    }
}

pub mod quoted {
    use ast::AST;
    use ast::atom_quote;

    pub fn parse(val: AST) -> AST {
        AST::List(vec!(atom_quote(), val))
    }
}

peg! ast(r#"

use ast::AST;
use parser::*;

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
        atom::parse(identifier)
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
        integer::parse_decimal(digits, &sign)
    }

list -> AST =
    "(" values:(value ** __) ")" __ {
        list::parse(values)
    }
    / "[" values:(value ** __) "]" __ {
        list::parse(values)
    }
    / "(" left:(value ++ __) "." __ right:value ")" __ {
        list::parse_dotted(left, right)
    }
    / "[" left:(value ++ __) "." __ right:value "]" __ {
        list::parse_dotted(left, right)
    }

boolean -> AST =
    "\#" value:boolean_char __ {
        bool::parse(value)
    }

boolean_char -> &'input str =
    [tfTF] { match_str }

digits -> &'input str =
    [0-9]+ { match_str }

sign -> IntegerSign =
    [-+]? {
        integer::parse_sign(match_str)
    }

quoted -> AST =
    "'" value:value {
        quoted::parse(value)
    }

__ = (whitespace)*

whitespace =
    [ \t\r\n]

"#)

pub fn parse(input: &str) -> Result<Vec<AST>, Error> {
    match ast::grammar(input) {
        Ok(exprs)  => Ok(exprs),
        Err(error) => Err(Error::ParseError(error)),
    }
}
