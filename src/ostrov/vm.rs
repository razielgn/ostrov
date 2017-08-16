use ast::AST;
use env::CellEnv;
use errors::Error;
use instructions::{Bytecode, Instruction, PackedBytecode};
use memory::Memory;
use primitives;
use std::collections::LinkedList;
use std::iter::FromIterator;
use std::mem;
use values::{ArgumentsType, RcValue, Value};

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
    instructions: PackedBytecode,
    pc: usize,
    code: Vec<(PackedBytecode, usize)>,
}

impl VM {
    pub fn new() -> VM {
        let memory = Memory::new();

        let mut vm = VM {
            acc: memory.unspecified(),
            memory: memory,
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

    pub fn execute<I>(&mut self, instructions: I) -> Result<RcValue, Error>
    where
        I: Iterator<Item = Instruction>,
    {
        self.instructions = Vec::from_iter(instructions.into_iter());
        self.pc = 0;

        loop {
            match self.next_instruction() {
                Some(instr) => match instr {
                    Instruction::LoadConstant { ref value } => {
                        self.load_constant(value)
                    }
                    Instruction::Jump { offset } => self.jump(offset),
                    Instruction::JumpOnFalse { offset } => {
                        self.jump_on_false(offset)
                    }
                    Instruction::JumpOnTrue { offset } => {
                        self.jump_on_true(offset)
                    }
                    Instruction::LoadReference { ref reference } => {
                        try!(self.load_reference(reference))
                    }
                    Instruction::Assignment { ref reference } => {
                        self.assignment(reference)
                    }
                    Instruction::Replace { ref reference } => {
                        try!(self.replace(reference))
                    }
                    Instruction::LoadUnspecified => self.load_unspecified(),
                    Instruction::Apply => try!(self.apply()),
                    Instruction::Argument => self.argument(),
                    Instruction::Frame => self.push_frame(),
                    Instruction::Close {
                        ref args,
                        ref args_type,
                        ref body,
                    } => self.push_closure(args, args_type, body),
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

    fn load_reference(&mut self, reference: &str) -> Result<(), Error> {
        match self.env.get(reference) {
            Some(value) => Ok(self.acc = value),
            None => Err(Error::UnboundVariable(reference.into())),
        }
    }

    fn assignment(&mut self, reference: &str) {
        self.env.set(reference.into(), self.acc.clone());
        self.load_unspecified();
    }

    fn replace(&mut self, reference: &str) -> Result<(), Error> {
        match self.env.replace(reference.into(), self.acc.clone()) {
            Some(_) => Ok(self.load_unspecified()),
            None => Err(Error::UnboundVariable(reference.into())),
        }
    }

    fn load_unspecified(&mut self) {
        self.acc = self.memory.unspecified();
    }

    fn push_frame(&mut self) {
        self.stack.push_back(Frame::new(&self.rib, &self.env));

        self.rib = Vec::new();
    }

    fn pop_frame(&mut self, a: bool) -> Result<(), Error> {
        let frame = try!(self.stack.pop_back().ok_or(Error::CannotPopLastFrame));

        self.rib = frame.rib;
        self.env = frame.env;

        if a {
            let (instr, pc) =
                try!(self.code.pop().ok_or(Error::CannotPopLastFrame));

            self.instructions = instr;
            self.pc = pc;
        }

        Ok(())
    }

    fn apply(&mut self) -> Result<(), Error> {
        match *self.acc.clone() {
            Value::PrimitiveFn(ref name) => {
                let result =
                    try!(primitives::apply(name, &self.rib, &mut self.memory));

                self.acc = result;

                try!(self.pop_frame(false));

                Ok(())
            }
            Value::Closure {
                ref name,
                ref args_type,
                args: ref arg_names,
                ref closure,
                code: ref body,
            } => {
                let mut instructions = Vec::from_iter(body.clone().into_iter());
                mem::swap(&mut self.instructions, &mut instructions);

                self.code.push((instructions, self.pc));
                self.pc = 0;
                self.env = closure.clone();

                match *args_type {
                    ArgumentsType::Fixed => {
                        if arg_names.len() != self.rib.len() {
                            return Err(Error::BadArity(name.clone()));
                        }

                        for (name, value) in
                            arg_names.iter().zip(self.rib.iter())
                        {
                            self.env.set(name.clone(), value.clone());
                        }
                    }
                    ArgumentsType::Variable => {
                        let fixed_arg_names = &arg_names[0..arg_names.len() - 1];

                        if fixed_arg_names.len() > self.rib.len() {
                            return Err(Error::BadArity(name.clone()));
                        }

                        for (name, value) in
                            fixed_arg_names.iter().zip(self.rib.iter())
                        {
                            self.env.set(name.clone(), value.clone());
                        }

                        let var_args = self.rib
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
            _ => Err(Error::UnappliableValue(self.acc.clone())),
        }
    }

    fn push_closure(
        &mut self,
        args: &[String],
        args_type: &ArgumentsType,
        body: &Bytecode,
    ) {
        self.acc = self.memory.closure(
            *args_type,
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
