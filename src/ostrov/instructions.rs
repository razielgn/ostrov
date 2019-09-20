use crate::ast::AST;
pub use crate::values::ArgumentsType;

pub type Bytecode = Vec<Instruction>;

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    Apply,
    Argument,
    Assignment(String),
    Close {
        args: Vec<String>,
        args_type: ArgumentsType,
        body: Bytecode,
    },
    Frame,
    LoadConstant(AST),
    LoadReference(String),
    LoadUnspecified,
    JumpOnFalse(usize),
    JumpOnTrue(usize),
    Jump(usize),
    Replace(String),
}
