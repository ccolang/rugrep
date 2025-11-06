use std::{collections::HashMap};

#[derive(Clone)]
pub enum Value {
    Int(i32),
    Float(f64),
    Text(String),
    Bool(bool)
}

pub struct ArgManager {
    pub arguments: Vec<String>,
    options: HashMap<String, Value>,
    pub options_size: usize,
    pub length: usize
}

impl ArgManager {
    pub fn new(arguments: Vec<String>) -> Self {
        let length = arguments.len();
        ArgManager { arguments, options: HashMap::new(), options_size: 0, length }
    }

    pub fn scan(&mut self) {
        for (_i, argument) in self.arguments.iter().enumerate() {
            if argument.starts_with("-") {

                if let Some(value) = self.options.get(&argument[1..]) {

                    self.options_size += 1;
                    match value {
                        Value::Bool(b) => {
                            self.options.insert(argument[1..].to_string(), Value::Bool(!b));
                        }
                        _ => {
                            self.options.insert(argument[1..].to_string(), value.clone());
                        }
                    }
                }
            } 
        }
    }

    pub fn print(&self) {
        for (i, argument) in self.arguments.iter().enumerate() {
            println!("{} {}", i, argument);
        }
    }

    pub fn add_option(&mut self, key: &str, value: Value) {
        self.options.insert(key.to_string(), value);
    }

    pub fn get_option(&self, key: &str) -> Option<&Value> {
        self.options.get(key)
    }

    pub fn is_true(&self, val: &Value) -> bool {
        matches!(val, Value::Bool(true))
    }

    pub fn has_options(&self) -> bool {
        self.options_size > 0
    }

}