use crate::rules::product::Product;

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
            .get(&self.attribute)
            .map_or(false, |v| v == &self.value)
    }
}
