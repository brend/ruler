use crate::rules::product::Product;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionExpr {
    Set { attribute: String, value: String },
    Delete { attribute: String },
}

impl ActionExpr {
    pub fn set(attribute: &str, value: &str) -> Self {
        Self::Set {
            attribute: attribute.to_string(),
            value: value.to_string(),
        }
    }

    pub fn delete(attribute: &str) -> Self {
        Self::Delete {
            attribute: attribute.to_string(),
        }
    }

    pub fn apply(&self, product: &mut Product) {
        match self {
            Self::Set { attribute, value } => product.set(attribute, value),
            Self::Delete { attribute } => product.delete(attribute),
        }
    }
}
