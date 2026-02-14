use crate::rules::conditions::Product;

pub trait Action {
    fn apply(&self, product: Product) -> Product;
}

pub struct SetValue {
    attribute: String,
    value: String,
}

impl SetValue {
    pub fn new(attribute: &str, value: &str) -> Self {
        SetValue {
            attribute: attribute.to_string(),
            value: value.to_string(),
        }
    }
}

impl Action for SetValue {
    fn apply(&self, product: Product) -> Product {
        let mut product = product;
        product.set(&self.attribute, &self.value);
        product
    }
}
