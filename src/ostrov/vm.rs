use ast::AST;
use env::CellEnv;
use errors::Error;
use instructions::Instruction;
use memory::Memory;
use primitives;
use values::{RcValue, Value};

pub struct VM {
    pub acc: RcValue,
    pub memory: Memory,
    pub env: CellEnv,
    pub rib: Vec<RcValue>,
}

impl VM {
    pub fn new() -> VM {
        let memory = Memory::new();

        let mut vm = VM {
            acc: memory.unspecified(),
            memory: memory,
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

    fn apply(&mut self) -> Result<(), Error> {
        match *self.acc.clone() {
            Value::PrimitiveFn(ref name) => {
                let result = try!(
                    primitives::apply(name, &self.rib, &mut self.memory)
                );

                self.rib.clear();

                Ok(self.acc = result)
            }
            _ =>
                return Err(Error::UnappliableValue(self.acc.clone()))
        }
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
