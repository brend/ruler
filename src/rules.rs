use crate::rules::{actions::Action, conditions::Condition, engine::RuleRegistry};

pub mod actions;
pub mod conditions;
pub mod dsl;
pub mod engine;
pub mod product;

pub struct Rule {
    pub typeclass: String,
    pub conditions: Vec<Box<dyn Condition>>,
    pub actions: Vec<Box<dyn Action>>,
}

pub struct TypeclassRuleBuilder {
    typeclass: String,
}

impl TypeclassRuleBuilder {
    pub fn new(name: &str) -> Self {
        TypeclassRuleBuilder {
            typeclass: name.to_string(),
        }
    }

    pub fn when(self, condition: Box<dyn Condition>) -> ConditionalRuleBuilder {
        ConditionalRuleBuilder {
            typeclass: self.typeclass,
            conditions: vec![condition],
        }
    }

    pub fn then(self, action: Box<dyn Action>) -> ActionRuleBuilder {
        ActionRuleBuilder {
            typeclass: self.typeclass,
            conditions: vec![],
            actions: vec![action],
        }
    }
}

pub struct ConditionalRuleBuilder {
    typeclass: String,
    conditions: Vec<Box<dyn Condition>>,
}

impl ConditionalRuleBuilder {
    pub fn and(mut self, condition: Box<dyn Condition>) -> Self {
        self.conditions.push(condition);
        self
    }

    pub fn then(self, action: Box<dyn Action>) -> ActionRuleBuilder {
        ActionRuleBuilder {
            typeclass: self.typeclass,
            conditions: self.conditions,
            actions: vec![action],
        }
    }
}

pub struct ActionRuleBuilder {
    typeclass: String,
    conditions: Vec<Box<dyn Condition>>,
    actions: Vec<Box<dyn Action>>,
}

impl ActionRuleBuilder {
    pub fn and_then(mut self, action: Box<dyn Action>) -> Self {
        self.actions.push(action);
        self
    }

    pub fn create(self, registry: &mut RuleRegistry) {
        registry.register(self.compile())
    }

    fn compile(self) -> Rule {
        Rule {
            typeclass: self.typeclass,
            conditions: self.conditions,
            actions: self.actions,
        }
    }
}
