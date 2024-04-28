use crate::token::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Context {
    value_map: HashMap<String, Value>,
    //func_map: HashMap<String, FunctionClosure>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            value_map: HashMap::new()
        }
    }

    pub fn set_value(&mut self, key: &str, val: Value) -> &mut Self {
        self.value_map.insert(key.to_owned(), val);
        self
    }

    pub fn get_value(&self, key: &str) -> Option<&Value> {
        self.value_map.get(key)
    }

    // fn set_value_from_str(&mut self, expr: &str) {
    // }
}