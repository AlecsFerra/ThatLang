use crate::execution::value::Value;
use std::collections::HashMap;

pub struct Memory {
    memory_frame: Vec<HashMap<String, Value>>
}

impl Memory {

    pub fn new() -> Self {
        Self {
            memory_frame: vec![HashMap::new()]
        }
    }

    pub fn create_frame(&mut self) {
        self.memory_frame.push(HashMap::new());
    }

    pub fn remove_frame(&mut self) {
        self.memory_frame.pop();
    }

    pub fn declare(&mut self, key: String) {
        let mut current_frame = self.memory_frame.pop().unwrap();
        current_frame.insert(key, Value::Unit);
        self.memory_frame.push(current_frame);
    }

    pub fn declare_assign(&mut self, key: String, value: Value) {
        let mut current_frame = self.memory_frame.pop().unwrap();
        current_frame.insert(key, value);
        self.memory_frame.push(current_frame);
    }

    pub fn assign(&mut self, key: String, value: Value) {
        for frame in self.memory_frame.iter_mut().rev() {
            if frame.contains_key(&key) {
                frame.insert(key.to_string(), value);
                break;
            }
        }
    }

    pub fn retrieve_val(&self, key: String) -> Value {
        for frame in self.memory_frame.iter() {
            match frame.get(&key) {
                Some(value) => return *value,
                _ => ()
            }
        }
        unreachable!("Unable to find key: {} in memory", key)
    }

}