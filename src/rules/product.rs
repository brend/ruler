use std::collections::HashMap;

#[derive(Debug)]
pub struct Product {
    pub typeclass: String,
    pub attributes: HashMap<String, String>,
}

impl Product {
    pub fn new(typeclass: &str) -> Self {
        Product {
            typeclass: typeclass.to_string(),
            attributes: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.attributes.insert(key.to_string(), value.to_string());
    }
}
