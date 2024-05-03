use crate::token::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FuncClosure {
    arg_len: usize,
    func: fn(&[Value]) -> Value
}

impl FuncClosure {
    pub(crate) fn new(f: fn(&[Value]) -> Value, n: usize) -> Self {
        Self {
            arg_len: n,
            func: f
        }
    }

    pub(crate) fn get_arg_len(&self) -> usize {
        self.arg_len
    }
    
    pub(crate) fn call(&self, x: &[Value]) -> Value {
        (self.func)(x)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Context {
    value_map: HashMap<String, Value>,
    func_map: HashMap<String, FuncClosure>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            value_map: HashMap::new(),
            func_map: HashMap::new()
        }
    }

    pub fn set_value(&mut self, key: &str, val: Value) -> &mut Self {
        self.value_map.insert(key.to_owned(), val);
        self
    }

    pub fn get_value(&self, key: &str) -> Option<&Value> {
        self.value_map.get(key)
    }

    pub fn set_func(&mut self, key: &str, n: usize, f: fn(&[Value]) -> Value) -> &mut Self {
        self.func_map.insert(key.to_owned(), FuncClosure::new(f, n));
        self
    }

    pub fn get_func(&self, key: &str) -> Option<&FuncClosure> {
        self.func_map.get(key)
    }

    
    pub fn ctx_merge(lhs: &Context, rhs: &Context) -> Self {
        let mut value_map = rhs.value_map.clone();
        value_map.extend(lhs.value_map.clone());
        let mut func_map = rhs.func_map.clone();
        func_map.extend(lhs.func_map.clone());
        Self {
            value_map,
            func_map,
        }
    }

    // fn set_value_from_str(&mut self, expr: &str) {
    // }
}