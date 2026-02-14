use crate::rules::{dsl::*, engine::RuleRegistry, product::Product};

mod rules;

fn main() {
    let mut rules = RuleRegistry::new();

    typeclass("W600")
        .when(has("TYP", "W600"))
        .and(has("GEHAEUSEFORM", "S"))
        .then(set("TYP", "514"))
        .and_then(set("GEHAEUSEFORM", "M"))
        .create(&mut rules);

    let mut product = Product::new("W600");

    product.set("TYP", "W600");
    product.set("GEHAEUSEFORM", "S");

    println!("before: {:?}", product);
    product = rules.apply_rules(product);
    println!(" after: {:?}", product);

    typeclass("P600").then(set("DN", "15"));
}
