//! Liquid 模板引擎测试
//! 测试 Jekyll Liquid 模板引擎功能

use jekyll::{FrontMatterParser, JekyllConfig, JekyllStructure, LiquidEngine};
use serde_json::Value;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_render_template() {
    let temp_dir = tempdir().unwrap();
    let structure = JekyllStructure::new(temp_dir.path()).unwrap();
    let config = JekyllConfig::new();
    let mut engine = LiquidEngine::new(structure, config);

    let template = "Hello {{ name }}!";
    let mut context = serde_json::Map::new();
    context.insert("name".to_string(), Value::String("World".to_string()));

    let result = engine.render_template(template, &Value::Object(context)).unwrap();
    assert_eq!(result, "Hello World!");
}

#[test]
fn test_render_template_with_filters() {
    let temp_dir = tempdir().unwrap();
    let structure = JekyllStructure::new(temp_dir.path()).unwrap();
    let config = JekyllConfig::new();
    let mut engine = LiquidEngine::new(structure, config);

    let template = "Hello {{ name | upcase }}!";
    let mut context = serde_json::Map::new();
    context.insert("name".to_string(), Value::String("world".to_string()));

    let result = engine.render_template(template, &Value::Object(context)).unwrap();
    assert_eq!(result, "Hello WORLD!");
}

#[test]
fn test_create_jekyll_context() {
    let temp_dir = tempdir().unwrap();
    let structure = JekyllStructure::new(temp_dir.path()).unwrap();
    let config = JekyllConfig::new()
        .with_title("Test Site".to_string())
        .with_description("A test site".to_string())
        .with_url("https://example.com".to_string());
    let mut engine = LiquidEngine::new(structure, config);

    let content = r#"---
title: Test Page
layout: post
date: 2024-01-01
---
Content here."#;
    let front_matter = FrontMatterParser::parse(content).unwrap();

    let context = engine.create_jekyll_context(&front_matter, "test-page.md");

    let site = context.get("site").unwrap().as_object().unwrap();
    assert_eq!(site.get("title").unwrap().as_str().unwrap(), "Test Site");

    let page = context.get("page").unwrap().as_object().unwrap();
    assert_eq!(page.get("title").unwrap().as_str().unwrap(), "Test Page");
}

#[test]
fn test_render_layout() {
    let temp_dir = tempdir().unwrap();

    let layouts_dir = temp_dir.path().join("_layouts");
    fs::create_dir_all(&layouts_dir).unwrap();

    let layout_content = r#"<!DOCTYPE html>
<html>
<head>
    <title>{{ page.title }}</title>
</head>
<body>
    {{ content }}
</body>
</html>"#;
    fs::write(layouts_dir.join("default.html"), layout_content).unwrap();

    let structure = JekyllStructure::new(temp_dir.path()).unwrap();
    let config = JekyllConfig::new();
    let mut engine = LiquidEngine::new(structure, config);

    let content = "<h1>Hello World</h1><p>This is the content.</p>";
    let mut context = serde_json::Map::new();
    let mut page = serde_json::Map::new();
    page.insert("title".to_string(), Value::String("Test Page".to_string()));
    context.insert("page".to_string(), Value::Object(page));

    let result = engine.render_layout("default", content, &Value::Object(context)).unwrap();
    assert!(result.contains("<!DOCTYPE html>"));
    assert!(result.contains("<title>Test Page</title>"));
    assert!(result.contains("<h1>Hello World</h1>"));
}

#[test]
fn test_render_include() {
    let temp_dir = tempdir().unwrap();

    let includes_dir = temp_dir.path().join("_includes");
    fs::create_dir_all(&includes_dir).unwrap();

    let include_content = r#"<div class="footer">
    <p>Footer content</p>
</div>"#;
    fs::write(includes_dir.join("footer.html"), include_content).unwrap();

    let structure = JekyllStructure::new(temp_dir.path()).unwrap();
    let config = JekyllConfig::new();
    let mut engine = LiquidEngine::new(structure, config);

    let context = serde_json::Map::new();
    let result = engine.render_include("footer.html", &Value::Object(context)).unwrap();
    assert!(result.contains("<div class=\"footer\">"));
}
