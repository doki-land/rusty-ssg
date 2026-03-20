use crate::jekyll::liquid::*;
use liquid::{Object, Value};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_render_template() {
    let temp_dir = tempdir().unwrap();
    let structure = JekyllStructure::new(temp_dir.path()).unwrap();
    let config = JekyllConfig::new();
    let engine = LiquidEngine::new(structure, config);

    let template = "Hello {{ name }}!";
    let mut context = Object::new();
    context.insert("name".to_string(), Value::scalar("World"));

    let result = engine.render_template(template, &context).unwrap();
    assert_eq!(result, "Hello World!");
}

#[test]
fn test_render_template_with_filters() {
    let temp_dir = tempdir().unwrap();
    let structure = JekyllStructure::new(temp_dir.path()).unwrap();
    let config = JekyllConfig::new();
    let engine = LiquidEngine::new(structure, config);

    let template = "Hello {{ name | upcase }}!";
    let mut context = Object::new();
    context.insert("name".to_string(), Value::scalar("world"));

    let result = engine.render_template(template, &context).unwrap();
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
    let engine = LiquidEngine::new(structure, config);

    let content = r#"---
title: Test Page
layout: post
date: 2024-01-01
---
Content here."#;
    let front_matter = crate::jekyll::FrontMatterParser::parse(content).unwrap();

    let context = engine.create_jekyll_context(&front_matter, "test-page.md");

    // 检查 site 变量
    let site = context.get("site").unwrap().as_object().unwrap();
    assert_eq!(site.get("title").unwrap().as_scalar().unwrap(), "Test Site");
    assert_eq!(site.get("description").unwrap().as_scalar().unwrap(), "A test site");
    assert_eq!(site.get("url").unwrap().as_scalar().unwrap(), "https://example.com");

    // 检查 page 变量
    let page = context.get("page").unwrap().as_object().unwrap();
    assert_eq!(page.get("title").unwrap().as_scalar().unwrap(), "Test Page");
    assert_eq!(page.get("layout").unwrap().as_scalar().unwrap(), "post");
    assert_eq!(page.get("path").unwrap().as_scalar().unwrap(), "test-page.md");
}

#[test]
fn test_render_layout() {
    let temp_dir = tempdir().unwrap();

    // 创建 layouts 目录和布局文件
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
    let mut context = Object::new();
    let mut page = Object::new();
    page.insert("title".to_string(), Value::scalar("Test Page"));
    context.insert("page".to_string(), Value::Object(page));

    let result = engine.render_layout("default", content, &context).unwrap();
    assert!(result.contains("<!DOCTYPE html>"));
    assert!(result.contains("<title>Test Page</title>"));
    assert!(result.contains("<h1>Hello World</h1>"));
    assert!(result.contains("<p>This is the content.</p>"));
}

#[test]
fn test_render_include() {
    let temp_dir = tempdir().unwrap();

    // 创建 includes 目录和包含文件
    let includes_dir = temp_dir.path().join("_includes");
    fs::create_dir_all(&includes_dir).unwrap();

    let include_content = r#"<div class="footer">
    <p>© {{ site.title }} {{ "now" | date: "%Y" }}</p>
</div>"#;
    fs::write(includes_dir.join("footer.html"), include_content).unwrap();

    let structure = JekyllStructure::new(temp_dir.path()).unwrap();
    let config = JekyllConfig::new().with_title("Test Site".to_string());
    let mut engine = LiquidEngine::new(structure, config);

    let mut context = Object::new();
    let mut site = Object::new();
    site.insert("title".to_string(), Value::scalar("Test Site"));
    context.insert("site".to_string(), Value::Object(site));

    let result = engine.render_include("footer.html", &context).unwrap();
    assert!(result.contains("<div class=\"footer\">"));
    assert!(result.contains("© Test Site"));
}
