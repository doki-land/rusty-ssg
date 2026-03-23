use serde_json::Value;
use std::{collections::HashMap, fs, path::Path};
use tempfile::tempdir;

use jekyll::{Collection, CollectionConfig, CollectionItem, CollectionManager, JekyllConfig, JekyllStructure};

#[test]
fn test_collection_config() {
    let config = CollectionConfig::new("products".to_string()).with_output(true).with_permalink("/products/:name/");

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

#[test]
fn test_collection_loading() {
    let temp_dir = tempdir().unwrap();
    let collection_dir = temp_dir.path().join("_products");
    fs::create_dir_all(&collection_dir).unwrap();

    // 创建测试文件
    let product1_path = collection_dir.join("product1.md");
    fs::write(
        &product1_path,
        r#"---
title: Product 1
---
Product 1 content."#,
    )
    .unwrap();

    let product2_path = collection_dir.join("product2.md");
    fs::write(
        &product2_path,
        r#"---
title: Product 2
---
Product 2 content."#,
    )
    .unwrap();

    let config = CollectionConfig::new("products".to_string());
    let mut collection = Collection::new("products".to_string(), config, collection_dir);

    let count = collection.load_items().unwrap();
    assert_eq!(count, 2);
    assert_eq!(collection.items().len(), 2);
}

#[test]
fn test_collection_manager() {
    let temp_dir = tempdir().unwrap();

    // 创建 _products 目录
    let products_dir = temp_dir.path().join("_products");
    fs::create_dir_all(&products_dir).unwrap();

    // 创建测试文件
    let product_path = products_dir.join("product1.md");
    fs::write(
        &product_path,
        r#"---
title: Product 1
---
Product content."#,
    )
    .unwrap();

    // 创建配置
    let mut jekyll_config = JekyllConfig::new();
    let mut collections = HashMap::new();
    let mut product_config = serde_json::Map::new();
    product_config.insert("output".to_string(), Value::Bool(true));
    collections.insert("products".to_string(), Value::Object(product_config));
    jekyll_config.collections = Some(serde_json::Map::from_iter(collections));

    // 创建结构和管理器
    let structure = JekyllStructure::new(temp_dir.path()).unwrap();
    let mut manager = CollectionManager::new(structure, jekyll_config);

    let count = manager.load_collections().unwrap();
    assert_eq!(count, 1);
    assert!(manager.has_collection("products"));

    let collection = manager.get_collection("products").unwrap();
    assert_eq!(collection.items().len(), 1);
}
