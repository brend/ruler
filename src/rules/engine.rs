use std::collections::HashMap;

use crate::rules::{Rule, product::Product};

pub struct RuleRegistry {
    rules: HashMap<String, Vec<Rule>>,
}

impl RuleRegistry {
    pub fn new() -> Self {
        RuleRegistry {
            rules: HashMap::new(),
        }
    }

    pub fn register(&mut self, rule: Rule) {
        self.rules
            .entry(rule.typeclass.clone())
            .or_insert_with(Vec::new)
            .push(rule);
    }

    pub fn apply_rules(&self, product: Product) -> Product {
        let mut result = product;

        if let Some(rules) = self.rules.get(&result.typeclass) {
            for rule in rules {
                if Self::is_rule_applicable(&rule, &result) {
                    result = Self::apply_rule(&rule, result);
                }
            }
        }

        result
    }

    fn is_rule_applicable(rule: &Rule, product: &Product) -> bool {
        for condition in &rule.conditions {
            if !condition.is_match(&product) {
                return false;
            }
        }

        true
    }

    fn apply_rule(rule: &Rule, product: Product) -> Product {
        let mut result = product;

        for action in &rule.actions {
            result = action.apply(result);
        }

        result
    }
}
