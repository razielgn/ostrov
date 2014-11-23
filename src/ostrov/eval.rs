use parser::AST;

#[deriving(Show, PartialEq)]
pub enum Error {
    IrreducibleValue(AST),
}

pub fn eval(value: AST) -> Result<AST, Error> {
    if value.is_reducible() {
        Ok(value)
    } else {
        Err(Error::IrreducibleValue(value))
    }
}
