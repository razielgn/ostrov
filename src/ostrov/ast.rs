#[deriving(Show, PartialEq, Clone)]
pub enum AST {
    Atom(String),
    Bool(bool),
    Integer(i64),
    List(Vec<AST>),
}

#[inline]
pub fn atom_quote() -> AST {
    AST::Atom("quote".to_string())
}
