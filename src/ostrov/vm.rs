use ast::AST;
use env::CellEnv;
use errors::Error;
use instructions::{Instruction, Bytecode};
use memory::Memory;
use primitives;
use values::{RcValue, Value};
use std::collections::LinkedList;
use std::rc::Rc;

pub type Rib = Vec<RcValue>;
pub type Stack = LinkedList<Frame>;

struct Frame {
    rib: Rib,
    env: CellEnv,
}

impl Frame {
    pub fn new(rib: &Rib, env: &CellEnv) -> Frame {
        Frame {
            rib: rib.clone(),
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
}

impl VM {
    pub fn new() -> VM {
        let memory = Memory::new();

        let mut vm = VM {
            acc: memory.unspecified(),
            memory: memory,
            stack: LinkedList::new(),
            rib: Vec::new(),
            env: CellEnv::new(),
        };

        vm.init_primitives();

        vm
    }

    pub fn execute<'a, I>(&mut self, mut instructions: I) -> Result<RcValue, Error>
        where I: Iterator<Item = &'a Instruction>
    {
        loop {
            match instructions.next() {
                Some(instr) =>
                    match instr {
                        &Instruction::LoadConstant { ref value } =>
                            self.load_constant(value),
                        &Instruction::Jump { offset } =>
                            VM::jump(&mut instructions, offset),
                        &Instruction::JumpOnFalse { offset } =>
                            self.jump_on_false(&mut instructions, offset),
                        &Instruction::JumpOnTrue { offset } =>
                            self.jump_on_true(&mut instructions, offset),
                        &Instruction::LoadReference { ref reference } =>
                            try!(self.load_reference(reference)),
                        &Instruction::Assignment { ref reference } =>
                            self.assignment(reference),
                        &Instruction::LoadUnspecified =>
                            self.load_unspecified(),
                        &Instruction::Apply =>
                            try!(self.apply()),
                        &Instruction::Argument =>
                            self.argument(),
                        &Instruction::Frame =>
                            self.push_frame(),
                        &Instruction::Close { ref args, ref body } =>
                            self.push_closure(args, body),
                    },
                None =>
                    break,
            }
        }

        Ok(self.acc.clone())
    }

    fn load_constant(&mut self, ast: &AST) {
        self.acc = Value::from_ast(ast, &mut self.memory);
    }

    fn jump_on_false<I>(&mut self, iter: &mut I, times: usize) where I: Iterator {
        if self.acc == self.memory.b_false() {
            VM::jump(iter, times);
        }
    }

    fn jump_on_true<I>(&mut self, iter: &mut I, times: usize) where I: Iterator {
        if self.acc != self.memory.b_false() {
            VM::jump(iter, times);
        }
    }

    fn jump<I>(iter: &mut I, times: usize) where I: Iterator {
        for _ in (0..times) {
            iter.next();
        }
    }

    fn load_reference(&mut self, reference: &String) -> Result<(), Error> {
        match self.env.get(reference) {
            Some(value) => Ok(self.acc = value),
            None        => Err(Error::UnboundVariable(reference.clone())),
        }
    }

    fn assignment(&mut self, reference: &String) {
        self.env.set(reference.clone(), self.acc.clone());
        self.load_unspecified();
    }

    fn load_unspecified(&mut self) {
        self.acc = self.memory.unspecified();
    }

    fn push_frame(&mut self) {
        self.stack.push_back(
            Frame::new(&self.rib, &self.env)
        );

        self.rib = Vec::new();
    }

    fn pop_frame(&mut self) -> Result<(), Error> {
        let frame = try!(
            self.stack
                .pop_back()
                .ok_or(Error::CannotPopLastFrame)
        );

        self.rib = frame.rib;
        self.env = frame.env;

        Ok(())
    }

    fn apply(&mut self) -> Result<(), Error> {
        match *self.acc.clone() {
            Value::PrimitiveFn(ref name) => {
                let result = try!(
                    primitives::apply(name, &self.rib, &mut self.memory)
                );

                try!(self.pop_frame());

                Ok(self.acc = result)
            }
            Value::Closure(ref args, ref body) => {
                Ok(())
            }
            _ =>
                return Err(Error::UnappliableValue(self.acc.clone()))
        }
    }

    fn push_closure(&mut self, args: &Vec<String>, body: &Bytecode) {
        self.acc = Rc::new(Value::Closure(args.clone(), body.clone()));
    }

    fn argument(&mut self) {
        self.rib.push(self.acc.clone());
    }

    fn init_primitives(&mut self) {
        for name in primitives::PRIMITIVES.iter() {
            let primitive = self.memory.primitive(name.to_string());
            self.env.set(name.to_string(), primitive);
        }
    }
}
