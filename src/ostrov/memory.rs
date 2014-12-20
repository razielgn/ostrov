use values::Value;
use std::rc::Rc;

pub struct Memory {
    heap: Vec<Rc<Value>>,
}

impl Memory {
    pub fn new() -> Memory {
        let mut memory = Memory {
            heap: Vec::new(),
        };

        memory.init();

        memory
    }

    pub fn integer(&mut self, n: i64) -> Rc<Value> {
        let value = Value::Integer(n);

        self.store(value)
    }

    pub fn b_true(&self) -> Rc<Value> {
        self.heap[0].clone()
    }

    pub fn b_false(&self) -> Rc<Value> {
        self.heap[1].clone()
    }

    pub fn boolean(&self, b: bool) -> Rc<Value> {
        if b {
            self.b_true()
        } else {
            self.b_false()
        }
    }

    pub fn empty_list(&self) -> Rc<Value> {
        self.heap[2].clone()
    }

    pub fn list(&mut self, values: Vec<Value>) -> Rc<Value> {
        let value = Value::List(values);

        self.store(value)
    }

    pub fn intern(&mut self, atom: String) -> Rc<Value> {
        let value = Value::Atom(atom);

        self.store(value)
    }

    pub fn dotted_list(&mut self, values: Vec<Value>, tail: Value) -> Rc<Value> {
        let value = Value::DottedList(values, box tail);

        self.store(value)
    }

    pub fn dump(&self) {
        for (i, value) in self.heap.iter().enumerate() {
            println!("{:04}: {:p} {}", i, value, value);
        }
    }

    pub fn store(&mut self, value: Value) -> Rc<Value> {
        self.heap.push(Rc::new(value));
        self.heap.last().unwrap().clone()
    }

    fn init(&mut self) {
        self.store(Value::Bool(true));
        self.store(Value::Bool(false));
        self.store(Value::List(vec!()));
    }
}
