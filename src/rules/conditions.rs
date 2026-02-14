use std::collections::HashMap;

pub trait Condition {
    fn is_match(&self, product: &Product) -> bool;
}

pub struct Has {
    attribute: String,
    value: String,
}

impl Has {
    pub fn new(attribute: &str, value: &str) -> Self {
        Has {
            attribute: attribute.to_string(),
            value: value.to_string(),
        }
    }
}

impl Condition for Has {
    fn is_match(&self, product: &Product) -> bool {
        product
            .attributes
            .get(&self.attribute)
            .map_or(false, |v| v == &self.value)
    }
}

#[derive(Debug)]
pub struct Product {
    pub typeclass: String,
    attributes: HashMap<String, String>,
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
