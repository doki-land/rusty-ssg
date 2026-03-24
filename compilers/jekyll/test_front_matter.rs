use jekyll::FrontMatterParser;

fn main() {
    let content = r#"---
title: Product 1
price: 99.99
---
This is product 1.
"#;

    let front_matter = FrontMatterParser::parse(content).unwrap();
    println!("Raw YAML: {}", front_matter.raw_yaml());
    println!("Variables: {:?}", front_matter.variables());
    println!("Has title: {}", front_matter.has("title"));
    println!("Title: {:?}", front_matter.get("title"));
    println!("Price: {:?}", front_matter.get("price"));
}