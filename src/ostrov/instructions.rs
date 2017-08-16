use ast::AST;
use std::collections::LinkedList;
pub use values::ArgumentsType;

pub type Bytecode = LinkedList<Instruction>;
pub type PackedBytecode = Vec<Instruction>;

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    Apply,
    Argument,
    Assignment { reference: String },
    Close {
        args: Vec<String>,
        args_type: ArgumentsType,
        body: Bytecode,
    },
    Frame,
    LoadConstant { value: AST },
    LoadReference { reference: String },
    LoadUnspecified,
    JumpOnFalse { offset: usize },
    JumpOnTrue { offset: usize },
    Jump { offset: usize },
    Replace { reference: String },
}

impl Instruction {
    pub fn apply() -> Instruction {
        Instruction::Apply
    }

    pub fn argument() -> Instruction {
        Instruction::Argument
    }

    pub fn assignment(reference: String) -> Instruction {
        Instruction::Assignment {
            reference: reference,
        }
    }

    pub fn close(
        args: Vec<String>,
        args_type: ArgumentsType,
        body: Bytecode,
    ) -> Instruction {
        Instruction::Close {
            args: args,
            args_type: args_type,
            body: body,
        }
    }

    pub fn frame() -> Instruction {
        Instruction::Frame
    }

    pub fn load_constant(value: AST) -> Instruction {
        Instruction::LoadConstant { value: value }
    }

    pub fn load_reference(reference: String) -> Instruction {
        Instruction::LoadReference {
            reference: reference,
        }
    }

    pub fn load_unspecified() -> Instruction {
        Instruction::LoadUnspecified
    }

    pub fn jump_on_false(offset: usize) -> Instruction {
        Instruction::JumpOnFalse { offset: offset }
    }

    pub fn jump_on_true(offset: usize) -> Instruction {
        Instruction::JumpOnTrue { offset: offset }
    }

    pub fn jump(offset: usize) -> Instruction {
        Instruction::Jump { offset: offset }
    }

    pub fn replace(reference: String) -> Instruction {
        Instruction::Replace {
            reference: reference,
        }
    }
}
