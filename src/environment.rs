use std::collections::HashMap;
use ast::Identifier;
use interpreter::Value;
use std::convert::TryInto;
use std::borrow::Borrow;

pub struct Environment{map : HashMap<String, Value>}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            map: HashMap::new()
        }
    }

    pub fn put(&mut self, identifier: String, value: Value) -> () {
        self.map.insert(identifier, value);
        ()
    }

    pub fn get(&self, identifier: &String) -> Option<&Value> {
        self.map.get(identifier)
    }
}