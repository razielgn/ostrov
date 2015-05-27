use ast::AST;
use grammar;
use runtime::Error;

pub type ParseError = grammar::ParseError;

#[derive(PartialEq)]
pub enum IntegerSign {
    Positive,
    Negative,
}

pub fn parse_decimal(str: &str, sign: &IntegerSign) -> AST {
    let integer: i64 = str.parse().unwrap();

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

pub fn parse_atom(str: &str) -> AST {
    AST::Atom(str.to_string())
}

pub fn parse_list(values: Vec<AST>) -> AST {
    AST::List(values)
}

pub fn parse_dotted_list(mut left: Vec<AST>, right: AST) -> AST {
    match right {
        AST::List(list) => {
            for x in list { left.push(x); }
            AST::List(left)
        }
        AST::DottedList(list, right) => {
            for x in list { left.push(x); }
            AST::DottedList(left, right)
        }
        _ => AST::DottedList(left, Box::new(right))
    }
}

pub fn parse_bool(str: &str) -> AST {
    AST::Bool(str == "t" || str == "T")
}

pub fn parse_quoted(val: AST) -> AST {
    AST::List(vec!(AST::Atom("quote".to_string()), val))
}

pub fn parse(input: &str) -> Result<Vec<AST>, Error> {
    match grammar::grammar(input) {
        Ok(exprs)  => Ok(exprs),
        Err(error) => Err(Error::ParseError(error)),
    }
}
