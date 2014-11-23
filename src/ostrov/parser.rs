#[phase(plugin)]
extern crate peg_syntax_ext;

#[deriving(Show, PartialEq)]
pub enum AST {
    Atom(String),
    Bool(bool),
    Integer(i64),
    List(Vec<AST>),
}

#[deriving(PartialEq)]
pub enum IntegerSign {
    Positive,
    Negative,
}

pub mod integer {
    use parser::AST;
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
    use parser::AST;

    pub fn parse(str: &str) -> AST {
        AST::Atom(str.to_string())
    }
}

pub mod list {
    use parser::AST;

    pub fn parse(values: Vec<AST>) -> AST {
        AST::List(values)
    }
}

pub mod bool {
    use parser::AST;

    pub fn parse(str: &str) -> AST {
        AST::Bool(str == "t" || str == "T")
    }
}

peg! ast(r#"

use parser::*;

#[pub]
value -> AST =
    integer
    / boolean
    / identifier
    / list

identifier -> AST =
    initial subsequent* {
        atom::parse(match_str)
    }
    / peculiar_identifier {
        atom::parse(match_str)
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
    sign:sign digits:digits {
        integer::parse_decimal(digits, &sign)
    }

list -> AST =
    "(" values:(value ** whitespace) ")" {
        list::parse(values)
    }

boolean -> AST =
    "\#" value:boolean_char {
        bool::parse(value)
    }

boolean_char -> &'input str =
    [tfTF] { match_str }

whitespace -> &'input str =
    [ \t\r\n]+ {
        match_str
    }

digits -> &'input str =
    [0-9]+ { match_str }

sign -> IntegerSign =
    [-+]? {
        integer::parse_sign(match_str)
    }

"#)

pub fn parse(input: &str) -> Result<AST, String> {
    ast::value(input)
}
