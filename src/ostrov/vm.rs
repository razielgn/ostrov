use crate::{
    ast::AST,
    env::CellEnv,
    errors::RuntimeError,
    instructions::{Bytecode, Instruction},
    memory::Memory,
    primitives,
    values::{ArgumentsType, RcValue, Value},
};
use std::{collections::LinkedList, iter::FromIterator, mem};

pub type Rib = Vec<RcValue>;
pub type Stack = LinkedList<Frame>;

pub struct Frame {
    rib: Rib,
    env: CellEnv,
}

impl Frame {
    pub fn new(rib: &[RcValue], env: &CellEnv) -> Frame {
        Frame {
            rib: rib.to_vec(),
            env: env.clone(),
        }
    }
}

pub struct VM {
    pub acc: RcValue,
    pub memory: Memory,
    pub rib: Rib,
    pub env: CellEnv,
    pub stack: Stack,
    instructions: Bytecode,
    pc: usize,
    code: Vec<(Bytecode, usize)>,
}

impl VM {
    pub fn new() -> VM {
        let memory = Memory::new();

        let mut vm = VM {
            acc: memory.unspecified(),
            memory,
            stack: Default::default(),
            rib: Default::default(),
            env: CellEnv::new(),
            instructions: Default::default(),
            pc: 0,
            code: Default::default(),
        };

        vm.init_primitives();

        vm
    }

    pub fn execute(
        &mut self,
        instructions: Bytecode,
    ) -> Result<RcValue, RuntimeError> {
        self.instructions = instructions;
        self.pc = 0;

        loop {
            use crate::instructions::Instruction::*;

            match self.next_instruction() {
                Some(instr) => match instr {
                    LoadConstant(ref value) => self.load_constant(value),
                    Jump(offset) => self.jump(offset),
                    JumpOnFalse(offset) => self.jump_on_false(offset),
                    JumpOnTrue(offset) => self.jump_on_true(offset),
                    LoadReference(ref reference) => {
                        self.load_reference(reference)?
                    }
                    Assignment(ref reference) => self.assignment(reference),
                    Replace(ref reference) => self.replace(reference)?,
                    LoadUnspecified => self.load_unspecified(),
                    Apply => self.apply()?,
                    Argument => self.argument(),
                    Frame => self.push_frame(),
                    Close {
                        ref args,
                        ref args_type,
                        ref body,
                    } => self.push_closure(args, *args_type, body),
                },
                None => match self.pop_frame(true) {
                    Ok(()) => (),
                    Err(_) => break,
                },
            }
        }

        Ok(self.acc.clone())
    }

    fn next_instruction(&mut self) -> Option<Instruction> {
        let instr = self.instructions.get(self.pc).cloned();
        self.pc += 1;
        instr
    }

    fn load_constant(&mut self, ast: &AST) {
        self.acc = Value::from_ast(ast, &mut self.memory);
    }

    fn jump_on_false(&mut self, times: usize) {
        if self.acc == self.memory.b_false() {
            self.jump(times);
        }
    }

    fn jump_on_true(&mut self, times: usize) {
        if self.acc != self.memory.b_false() {
            self.jump(times);
        }
    }

    fn jump(&mut self, times: usize) {
        self.pc += times;
    }

    fn load_reference(&mut self, reference: &str) -> Result<(), RuntimeError> {
        match self.env.get(reference) {
            Some(value) => {
                self.acc = value;
                Ok(())
            }
            None => Err(RuntimeError::UnboundVariable(reference.into())),
        }
    }

    fn assignment(&mut self, reference: &str) {
        self.env.set(reference.into(), self.acc.clone());
        self.load_unspecified();
    }

    fn replace(&mut self, reference: &str) -> Result<(), RuntimeError> {
        match self.env.replace(reference.into(), self.acc.clone()) {
            Some(_) => {
                self.load_unspecified();
                Ok(())
            }
            None => Err(RuntimeError::UnboundVariable(reference.into())),
        }
    }

    fn load_unspecified(&mut self) {
        self.acc = self.memory.unspecified();
    }

    fn push_frame(&mut self) {
        self.stack.push_back(Frame::new(&self.rib, &self.env));

        self.rib = Vec::new();
    }

    fn pop_frame(&mut self, a: bool) -> Result<(), RuntimeError> {
        let frame = self
            .stack
            .pop_back()
            .ok_or(RuntimeError::CannotPopLastFrame)?;

        self.rib = frame.rib;
        self.env = frame.env;

        if a {
            let (instr, pc) =
                self.code.pop().ok_or(RuntimeError::CannotPopLastFrame)?;

            self.instructions = instr;
            self.pc = pc;
        }

        Ok(())
    }

    fn apply(&mut self) -> Result<(), RuntimeError> {
        match *self.acc.clone() {
            Value::PrimitiveFn(ref name) => {
                let result =
                    primitives::apply(name, &self.rib, &mut self.memory)?;

                self.acc = result;

                self.pop_frame(false)?;

                Ok(())
            }
            Value::Closure {
                ref name,
                ref args_type,
                args: ref arg_names,
                ref closure,
                code: ref body,
            } => {
                let mut instructions = Vec::from_iter(body.clone());
                mem::swap(&mut self.instructions, &mut instructions);

                self.code.push((instructions, self.pc));
                self.pc = 0;
                self.env = closure.clone();

                match *args_type {
                    ArgumentsType::Fixed => {
                        if arg_names.len() != self.rib.len() {
                            return Err(RuntimeError::BadArity(name.clone()));
                        }

                        for (name, value) in arg_names.iter().zip(self.rib.iter())
                        {
                            self.env.set(name.clone(), value.clone());
                        }
                    }
                    ArgumentsType::Variable => {
                        let fixed_arg_names = &arg_names[0..arg_names.len() - 1];

                        if fixed_arg_names.len() > self.rib.len() {
                            return Err(RuntimeError::BadArity(name.clone()));
                        }

                        for (name, value) in
                            fixed_arg_names.iter().zip(self.rib.iter())
                        {
                            self.env.set(name.clone(), value.clone());
                        }

                        let var_args = self
                            .rib
                            .iter()
                            .skip(fixed_arg_names.len())
                            .cloned()
                            .collect();

                        self.env.set(
                            arg_names.last().unwrap().clone(),
                            self.memory.list(var_args),
                        );
                    }
                    ArgumentsType::Any => {
                        self.env.set(
                            arg_names[0].clone(),
                            self.memory.list(self.rib.clone()),
                        );
                    }
                }

                Ok(())
            }
            _ => Err(RuntimeError::UnappliableValue(self.acc.clone())),
        }
    }

    fn push_closure(
        &mut self,
        args: &[String],
        args_type: ArgumentsType,
        body: &Bytecode,
    ) {
        self.acc = self.memory.closure(
            args_type,
            args.to_vec(),
            CellEnv::wraps(self.env.clone()),
            body.clone(),
        );
    }

    fn argument(&mut self) {
        self.rib.push(self.acc.clone());
    }

    fn init_primitives(&mut self) {
        for &name in &primitives::PRIMITIVES {
            let primitive = self.memory.primitive(name.to_owned());
            self.env.set(name.to_owned(), primitive);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        ast::AST::*, errors::RuntimeError, instructions::Instruction::*, vm::VM,
    };

    #[test]
    fn execute_load_constant() {
        {
            let mut vm = VM::new();
            let instr = vec![
                LoadConstant(Integer(1)),
                LoadConstant(Integer(2)),
                LoadConstant(Integer(3)),
            ];

            assert_eq!(Ok(vm.memory.integer(3)), vm.execute(instr));
        }

        {
            let mut vm = VM::new();
            let instr = vec![
                LoadConstant(Bool(false)),
                LoadConstant(Bool(true)),
                LoadConstant(Bool(false)),
            ];

            assert_eq!(Ok(vm.memory.b_false()), vm.execute(instr));
        }
    }

    #[test]
    fn execute_jump() {
        {
            let mut vm = VM::new();
            let instr = vec![
                LoadConstant(Integer(23)),
                Jump(1),
                LoadConstant(Integer(42)),
            ];

            assert_eq!(Ok(vm.memory.integer(23)), vm.execute(instr));
        }
    }

    #[test]
    fn execute_jump_on_false() {
        {
            let mut vm = VM::new();
            let instr = vec![
                LoadConstant(Bool(false)),
                JumpOnFalse(1),
                LoadConstant(Integer(23)),
            ];

            assert_eq!(Ok(vm.memory.b_false()), vm.execute(instr));
        }

        {
            let mut vm = VM::new();
            let instr = vec![
                LoadConstant(Bool(false)),
                JumpOnFalse(2),
                LoadConstant(Integer(1)),
                Jump(1),
                LoadConstant(Bool(true)),
                JumpOnFalse(2),
                LoadConstant(Integer(1)),
                Jump(1),
                LoadConstant(Integer(2)),
            ];

            assert_eq!(Ok(vm.memory.integer(1)), vm.execute(instr));
        }
    }

    #[test]
    fn execute_jump_on_true() {
        {
            let mut vm = VM::new();
            let instr = vec![
                LoadConstant(Bool(false)),
                JumpOnTrue(1),
                LoadConstant(Integer(23)),
            ];

            assert_eq!(Ok(vm.memory.integer(23)), vm.execute(instr));
        }

        {
            let mut vm = VM::new();
            let instr = vec![
                LoadConstant(Bool(true)),
                JumpOnTrue(1),
                LoadConstant(Integer(23)),
            ];

            assert_eq!(Ok(vm.memory.b_true()), vm.execute(instr));
        }
    }

    #[test]
    fn execute_load_reference() {
        {
            let mut vm = VM::new();
            let instr = vec![LoadReference("a".to_owned())];

            assert_eq!(
                Err(RuntimeError::UnboundVariable("a".into())),
                vm.execute(instr)
            );
        }

        {
            let mut vm = VM::new();
            let instr = vec![LoadReference("a".to_owned())];

            vm.env.set("a".to_owned(), vm.memory.integer(1));

            assert_eq!(Ok(vm.memory.integer(1)), vm.execute(instr));
        }
    }

    #[test]
    fn execute_assignment() {
        let mut vm = VM::new();
        let instr = vec![
            LoadConstant(Integer(1)),
            Assignment("x".to_owned()),
            LoadConstant(Integer(2)),
            LoadReference("x".to_owned()),
        ];

        assert_eq!(Ok(vm.memory.integer(1)), vm.execute(instr));
    }

    #[test]
    fn execute_load_unspecified() {
        let mut vm = VM::new();
        let instr = vec![LoadConstant(Integer(1)), LoadUnspecified];

        assert_eq!(Ok(vm.memory.unspecified()), vm.execute(instr));
    }

    #[test]
    fn execute_apply() {
        let mut vm = VM::new();
        let instr = vec![Frame, LoadReference("+".to_owned()), Apply];

        assert_eq!(Ok(vm.memory.integer(0)), vm.execute(instr));
    }

    #[test]
    fn execute_argument() {
        let mut vm = VM::new();
        let instr = vec![
            Frame,
            LoadConstant(Integer(2)),
            Argument,
            LoadReference("+".to_owned()),
            Apply,
        ];

        assert_eq!(Ok(vm.memory.integer(2)), vm.execute(instr));
    }

    #[test]
    fn execute_nested_arguments() {
        let mut vm = VM::new();
        let instr = vec![
            Frame,
            Frame,
            LoadConstant(Integer(1)),
            Argument,
            LoadConstant(Integer(2)),
            Argument,
            LoadReference("+".to_owned()),
            Apply,
            Argument,
            Frame,
            LoadConstant(Integer(4)),
            Argument,
            LoadConstant(Integer(3)),
            Argument,
            LoadReference("-".to_owned()),
            Apply,
            Argument,
            LoadReference("+".to_owned()),
            Apply,
        ];

        assert_eq!(Ok(vm.memory.integer(4)), vm.execute(instr));
    }

    #[test]
    fn illegal_frame_apply() {
        let mut vm = VM::new();
        let instr = vec![LoadReference("+".to_owned()), Apply];

        assert_eq!(Err(RuntimeError::CannotPopLastFrame), vm.execute(instr));
    }
}
