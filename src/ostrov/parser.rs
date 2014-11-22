#[phase(plugin)]
extern crate peg_syntax_ext;

#[deriving(Show, PartialEq)]
pub enum AST {
    Atom(String),
    Integer(i64),
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

peg! ast(r#"

use parser::*;

#[pub]
value -> AST =
    integer
    / identifier

identifier -> AST =
    initial subsequent* {
        AST::Atom(match_str.to_string())
    }
    / peculiar_identifier {
        AST::Atom(match_str.to_string())
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
