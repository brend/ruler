use std::collections::HashMap;

#[derive(Debug)]
pub struct Product {
    pub typeclass: String,
    part: String,
    attributes: HashMap<String, String>,
}

impl Product {
    pub fn new(typeclass: &str, part: &str) -> Self {
        Product {
            typeclass: typeclass.to_string(),
            part: part.to_string(),
            attributes: HashMap::new(),
        }
    }

    pub fn part(&self) -> &str {
        &self.part
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.attributes.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.attributes.get(key)
    }
}
