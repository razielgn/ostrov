use ast::AST;
use env::CellEnv;
use values::{RcValue, Value, ArgumentsType};

use std::rc::Rc;

pub struct Memory {
    heap: Vec<RcValue>,
    reserved: Vec<RcValue>,
}

impl Memory {
    pub fn new() -> Memory {
        let mut memory = Memory {
            heap: Vec::new(),
            reserved: Vec::new(),
        };

        memory.init();

        memory
    }

    pub fn integer(&mut self, n: i64) -> RcValue {
        let value = Value::Integer(n);

        self.store(value)
    }

    pub fn b_true(&self) -> RcValue {
        self.reserved[0].clone()
    }

    pub fn b_false(&self) -> RcValue {
        self.reserved[1].clone()
    }

    pub fn boolean(&self, b: bool) -> RcValue {
        if b {
            self.b_true()
        } else {
            self.b_false()
        }
    }

    pub fn empty_list(&self) -> RcValue {
        self.reserved[2].clone()
    }

    pub fn list(&mut self, values: Vec<RcValue>) -> RcValue {
        let value = Value::List(values);
        self.store(value)
    }

    pub fn intern(&mut self, atom: String) -> RcValue {
        let value = Value::Atom(atom);

        self.store(value)
    }

    pub fn dotted_list(&mut self, values: Vec<RcValue>, tail: RcValue) -> RcValue {
        let value = Value::DottedList(values, tail);

        self.store(value)
    }

    pub fn lambda(&mut self, name: Option<String>, args_type: ArgumentsType, args: Vec<String>, closure: CellEnv, body: Vec<AST>) -> RcValue {
        let value = Value::Fn(name, args_type, args, closure, body);

        self.store(value)
    }

    pub fn primitive(&self, name: String) -> RcValue {
        Rc::new(Value::PrimitiveFn(name))
    }

    pub fn dump(&self) {
        for (i, value) in self.heap.iter().enumerate() {
            println!("{:04}: {:p} {}", i, value, value);
        }
    }

    pub fn store(&mut self, value: Value) -> RcValue {
        self.heap.push(Rc::new(value));
        self.heap.last().unwrap().clone()
    }

    fn init(&mut self) {
        self.store_reserved(Value::Bool(true));
        self.store_reserved(Value::Bool(false));
        self.store_reserved(Value::List(vec!()));
    }

    fn store_reserved(&mut self, value: Value) {
        self.reserved.push(Rc::new(value));
    }
}
