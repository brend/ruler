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
            .or_default()
            .push(rule);
    }

    pub fn apply_rules(&self, product: Product) -> Product {
        let mut result = product;

        if let Some(rules) = self.rules.get(&result.typeclass) {
            for rule in rules {
                if Self::is_rule_applicable(rule, &result) {
                    result = Self::apply_rule(rule, result);
                }
            }
        }

        result
    }

    fn is_rule_applicable(rule: &Rule, product: &Product) -> bool {
        for condition in &rule.conditions {
            if !condition.is_match(product) {
                return false;
            }
        }

        true
    }

    fn apply_rule(rule: &Rule, product: Product) -> Product {
        let mut result = product;

        for action in &rule.actions {
            action.apply(&mut result);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::dsl::{has, set, typeclass};

    #[test]
    fn applies_matching_rule_actions() {
        let mut registry = RuleRegistry::new();
        typeclass("W600")
            .when(has("TYP", "W600"))
            .then(set("TYP", "514"))
            .and_then(set("GEHAEUSEFORM", "M"))
            .create(&mut registry);

        let mut product = Product::new("W600");
        product.set("TYP", "W600");

        let result = registry.apply_rules(product);
        assert_eq!(result.get("TYP").map(String::as_str), Some("514"));
        assert_eq!(result.get("GEHAEUSEFORM").map(String::as_str), Some("M"));
    }

    #[test]
    fn skips_rules_for_different_typeclass() {
        let mut registry = RuleRegistry::new();
        typeclass("W600")
            .when(has("TYP", "W600"))
            .then(set("TYP", "514"))
            .create(&mut registry);

        let mut product = Product::new("W700");
        product.set("TYP", "W600");

        let result = registry.apply_rules(product);
        assert_eq!(result.get("TYP").map(String::as_str), Some("W600"));
    }

    #[test]
    fn skips_rule_when_condition_does_not_match() {
        let mut registry = RuleRegistry::new();
        typeclass("W600")
            .when(has("GEHAEUSEFORM", "S"))
            .then(set("GEHAEUSEFORM", "M"))
            .create(&mut registry);

        let mut product = Product::new("W600");
        product.set("GEHAEUSEFORM", "X");

        let result = registry.apply_rules(product);
        assert_eq!(result.get("GEHAEUSEFORM").map(String::as_str), Some("X"));
    }

    #[test]
    fn applies_rules_in_registration_order() {
        let mut registry = RuleRegistry::new();
        typeclass("W600")
            .when(has("TYP", "W600"))
            .then(set("TYP", "514"))
            .create(&mut registry);
        typeclass("W600")
            .when(has("TYP", "514"))
            .then(set("STATUS", "DERIVED"))
            .create(&mut registry);

        let mut product = Product::new("W600");
        product.set("TYP", "W600");

        let result = registry.apply_rules(product);
        assert_eq!(result.get("TYP").map(String::as_str), Some("514"));
        assert_eq!(result.get("STATUS").map(String::as_str), Some("DERIVED"));
    }

    #[test]
    fn applies_typeclass_rule_without_conditions() {
        let mut registry = RuleRegistry::new();
        typeclass("W600")
            .then(set("FLAG", "ALWAYS"))
            .create(&mut registry);

        let product = Product::new("W600");
        let result = registry.apply_rules(product);
        assert_eq!(result.get("FLAG").map(String::as_str), Some("ALWAYS"));
    }

    #[test]
    fn table_driven_rule_application() {
        struct TestCase {
            name: &'static str,
            typeclass: &'static str,
            initial: Vec<(&'static str, &'static str)>,
            expected: Vec<(&'static str, Option<&'static str>)>,
        }

        let cases = vec![
            TestCase {
                name: "matching rule updates typ",
                typeclass: "W600",
                initial: vec![("TYP", "W600")],
                expected: vec![("TYP", Some("514"))],
            },
            TestCase {
                name: "missing attribute does not match has",
                typeclass: "W600",
                initial: vec![],
                expected: vec![("TYP", None)],
            },
        ];

        for case in cases {
            let mut registry = RuleRegistry::new();
            typeclass("W600")
                .when(has("TYP", "W600"))
                .then(set("TYP", "514"))
                .create(&mut registry);

            let mut product = Product::new(case.typeclass);
            for (key, value) in case.initial {
                product.set(key, value);
            }

            let result = registry.apply_rules(product);

            for (key, expected) in case.expected {
                assert_eq!(
                    result.get(key).map(String::as_str),
                    expected,
                    "case: {} key: {}",
                    case.name,
                    key
                );
            }
        }
    }
}
