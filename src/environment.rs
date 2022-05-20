use std::collections::HashMap;
use crate::interpreter::Object;

pub struct Environment {
    values: HashMap<String, Object>
}

impl Environment {
    pub fn new() -> Self {
        Self {values: HashMap::<String, Object>::new()}
    }

    pub fn assign(&mut self, k: String, v: Object) {
        self.values.insert(k, v);
    }

    pub fn get(&mut self, k: String) -> Option<&Object> {
        self.values.get(&k)
    }
}