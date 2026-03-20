use serde_json::Value;
use std::{collections::HashMap, fs, path::Path};
use tempfile::tempdir;

use jekyll::{Collection, CollectionConfig, CollectionItem, CollectionManager, JekyllConfig, JekyllStructure};

#[test]
fn test_collection_config() {
    let config = CollectionConfig::new("products".to_string()).with_output(true).with_permalink("/products/:name/".to_string());

    assert_eq!(config.name, "products");
    assert!(config.output);
    assert_eq!(config.permalink, Some("/products/:name/".to_string()));
}

#[test]
fn test_collection_item_creation() {
    let temp_dir = tempdir().unwrap();
    let item_path = temp_dir.path().join("product1.md");

    let content = r#"---
title: Product 1
price: 99.99
---
This is product 1.
"#;
    fs::write(&item_path, content).unwrap();

    let config = CollectionConfig::new("products".to_string());
    let item = CollectionItem::from_file(&item_path, "products", &config).unwrap();

    assert_eq!(item.name, "product1");
    assert_eq!(item.collection, "products");
    assert!(item.permalink.contains("/products/product1/"));
    assert_eq!(item.front_matter.get("title").unwrap().as_str().unwrap(), "Product 1");
    assert_eq!(item.front_matter.get("price").unwrap().as_f64().unwrap(), 99.99);
}
