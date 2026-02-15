use crate::rules::{TypeclassRuleBuilder, actions::ActionExpr, conditions::ConditionExpr};

pub fn typeclass(typeclass: &str) -> TypeclassRuleBuilder {
    TypeclassRuleBuilder::new(typeclass)
}

pub fn has(attribute: &str, value: &str) -> ConditionExpr {
    ConditionExpr::has(attribute, value)
}

pub fn is(part: &str) -> ConditionExpr {
    ConditionExpr::is(part)
}

pub fn set(attribute: &str, value: &str) -> ActionExpr {
    ActionExpr::set(attribute, value)
}
