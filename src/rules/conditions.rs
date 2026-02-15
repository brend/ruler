use crate::rules::product::Product;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConditionExpr {
    Has { attribute: String, value: String },
    Is { part: String },
}

impl ConditionExpr {
    pub fn has(attribute: &str, value: &str) -> Self {
        Self::Has {
            attribute: attribute.to_string(),
            value: value.to_string(),
        }
    }

    pub fn is(part: &str) -> Self {
        Self::Is {
            part: part.to_string(),
        }
    }

    pub fn matches(&self, product: &Product) -> bool {
        match self {
            Self::Has { attribute, value } => product.get(attribute) == Some(value),
            Self::Is { part } => product.part() == part,
        }
    }
}
