use crate::rules::{
    TypeclassRuleBuilder,
    actions::{Action, SetValue},
    conditions::{Condition, Has},
};

pub fn typeclass(typeclass: &str) -> TypeclassRuleBuilder {
    TypeclassRuleBuilder::new(typeclass)
}

pub fn has(attribute: &str, value: &str) -> Box<dyn Condition> {
    Box::new(Has::new(attribute, value))
}

pub fn set(attribute: &str, value: &str) -> Box<dyn Action> {
    Box::new(SetValue::new(attribute, value))
}
