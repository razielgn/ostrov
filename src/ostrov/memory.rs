use values::Value;
use std::rc::Rc;

pub struct Memory {
    heap: Vec<Rc<Value>>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            heap: Vec::new(),
        }
    }

    pub fn new_integer(&mut self, n: i64) -> Rc<Value> {
        let value = Value::Integer(n);

        self.store(value)
    }

    pub fn new_boolean(&mut self, b: bool) -> Rc<Value> {
        let value = Value::Bool(b);

        self.store(value)
    }

    pub fn new_list(&mut self, values: Vec<Value>) -> Rc<Value> {
        let value = Value::List(values);

        self.store(value)
    }

    pub fn new_dotted_list(&mut self, values: Vec<Value>, tail: Value) -> Rc<Value> {
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
}
