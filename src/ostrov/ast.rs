#[derive(Debug, PartialEq, Clone)]
pub enum AST {
    Atom(String),
    Bool(bool),
    DottedList(Vec<AST>, Box<AST>),
    Integer(i64),
    List(Vec<AST>),
}
