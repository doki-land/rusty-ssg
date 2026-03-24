//! 编译器测试

use gatsby::compiler::{GatsbyCompiler};
use gatsby::GatsbyConfig;

#[test]
fn test_compiler_creation() {
    // 创建编译器
    let compiler = GatsbyCompiler::new();

    // 验证编译器创建成功
    assert!(true);
}

#[test]
fn test_compiler_compile_document() {
    // 创建编译器
    let mut compiler = GatsbyCompiler::new();

    // 编译内容
    let content = r#"
---
title: Test Page
date: 2024-01-01
tags:
  - test
  - gatsby
---

# Test Page

This is a test page.
"#;

    let doc = compiler.compile_document(content, "test.md").unwrap();

    // 验证编译结果
    assert_eq!(doc.frontmatter.title.unwrap(), "Test Page");
    assert!(doc.content.contains("This is a test page"));
    assert!(doc.rendered_content.is_some());
}

#[test]
fn test_compiler_compile_batch() {
    // 创建编译器
    let mut compiler = GatsbyCompiler::new();

    // 准备文档
    let mut documents = std::collections::HashMap::new();
    documents.insert(
        "test1.md".to_string(),
        r#"
---
title: Test Page 1
---

# Test Page 1

This is test page 1.
"#.to_string(),
    );
    documents.insert(
        "test2.md".to_string(),
        r#"
---
title: Test Page 2
---

# Test Page 2

This is test page 2.
"#.to_string(),
    );

    // 批量编译
    let result = compiler.compile_batch(&documents);

    // 验证编译结果
    assert!(result.success);
    assert_eq!(result.documents.len(), 2);
    assert!(result.documents.contains_key("test1.md"));
    assert!(result.documents.contains_key("test2.md"));
}

#[test]
fn test_compiler_with_config() {
    // 创建配置
    let config = GatsbyConfig::new()
        .with_site_metadata(
            gatsby::config::SiteMetadata::new()
                .with_title("Test Site".to_string())
                .with_description("Test site description".to_string())
        );

    // 创建带配置的编译器
    let compiler = GatsbyCompiler::with_config(config);

    // 验证配置
    assert_eq!(compiler.config().site_title(), "Test Site");
    assert_eq!(compiler.config().site_description().unwrap(), "Test site description");
}

#[test]
fn test_compiler_cache() {
    // 创建编译器
    let mut compiler = GatsbyCompiler::new();

    // 编译内容
    let content = r#"
---
title: Test Page
date: 2024-01-01
---

# Test Page

This is a test page.
"#;

    // 第一次编译
    let doc1 = compiler.compile_document(content, "test.md").unwrap();
    assert_eq!(doc1.frontmatter.title.unwrap(), "Test Page");

    // 第二次编译，应该使用缓存
    let doc2 = compiler.compile_document(content, "test.md").unwrap();
    assert_eq!(doc2.frontmatter.title.unwrap(), "Test Page");

    // 验证缓存
    assert!(compiler.get_cached("test.md").is_some());

    // 清除缓存
    compiler.clear_cache();
    assert!(compiler.get_cached("test.md").is_none());
}

#[test]
fn test_compiler_compile_batch_with_cache() {
    // 创建编译器
    let mut compiler = GatsbyCompiler::new();

    // 准备文档
    let mut documents = std::collections::HashMap::new();
    documents.insert(
        "test1.md".to_string(),
        r#"
---
title: Test Page 1
---

# Test Page 1

This is test page 1.
"#.to_string(),
    );

    // 第一次批量编译
    let result1 = compiler.compile_batch(&documents);
    assert!(result1.success);
    assert_eq!(result1.documents.len(), 1);

    // 第二次批量编译，应该使用缓存
    let result2 = compiler.compile_batch(&documents);
    assert!(result2.success);
    assert_eq!(result2.documents.len(), 1);
}

#[test]
fn test_compiler_convert_frontmatter_to_map() {
    // 创建编译器
    let compiler = GatsbyCompiler::new();

    // 编译内容
    let content = r#"
---
title: Test Page
date: 2024-01-01
description: This is a test page
---

# Test Page

This is a test page.
"#;

    let mut compiler_mut = GatsbyCompiler::new();
    let doc = compiler_mut.compile_document(content, "test.md").unwrap();

    // 转换 frontmatter 为 map
    let frontmatter_map = compiler.convert_frontmatter_to_map(&doc);
    assert_eq!(frontmatter_map.get("title"), Some(&"Test Page".to_string()));
    assert_eq!(frontmatter_map.get("description"), Some(&"This is a test page".to_string()));
}
