use std::{collections::HashMap};

#[derive(Clone)]
pub enum Value {
    Int(i32),
    Float(f64),
    Text(String),
    Bool(bool),
    None(bool)
}

pub struct ArgManager {
    arguments: Vec<String>,
    options: HashMap<String, Value>,
    has_options: bool
}

impl ArgManager {
    pub fn new(arguments: Vec<String>) -> Self {
        ArgManager { arguments, options: HashMap::new(), has_options: false }
    }

    pub fn scan(&mut self) {
        for (i, argument) in self.arguments.iter().enumerate() {
            if argument.starts_with("-") {
                self.has_options = true;
                if let Some(value) = self.options.get(&argument[1..]) {
                    if let Value::None(_) = value {
                        self.options.insert(argument[1..].to_string(), Value::None(true));
                    } else {
                        self.options.insert(argument[1..].to_string(), value.clone());
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

    pub fn has_options(&self) -> bool {
        self.has_options
    }

}