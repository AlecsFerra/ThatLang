use std::collections::HashMap;

use crate::parsing::ast::Type;

pub struct SymbolTable {
    memory_frame: Vec<HashMap<String, Type>>
}

impl SymbolTable {
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


    pub fn declare(&mut self, key: String, value: Type) -> bool {
        if self.retrieve_type(key.clone()).is_some() {
            return false;
        }
        let mut current_frame = self.memory_frame.pop().unwrap();
        current_frame.insert(key.clone(), value);
        self.memory_frame.push(current_frame);
        true
    }


    pub fn retrieve_type(&self, key: String) -> Option<Type> {
        for frame in self.memory_frame.iter() {
            match frame.get(&key) {
                Some(value) => return Some(value.clone()),
                _ => ()
            }
        }
        None
    }
}