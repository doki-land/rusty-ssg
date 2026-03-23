//! 编译器测试

use hugo::compiler::VutexCompiler;

#[test]
fn test_compiler_creation() {
    // 创建编译器
    let compiler = VutexCompiler::new();

    // 验证编译器配置
    assert!(compiler.config().is_some());
}

#[test]
fn test_compiler_compile_document() {
    // 创建编译器
    let mut compiler = VutexCompiler::new();

    // 编译内容
    let content = r#"
---
title: Test Page
date: 2024-01-01T10:00:00Z
draft: false
categories:
  - test
  - hugo
tags:
  - test
  - hugo
---

# Test Page

This is a test page.
"#;

    let result = compiler.compile_document(content, "test.md").unwrap();

    // 验证编译结果
    assert_eq!(result.frontmatter.title, Some("Test Page".to_string()));
    assert!(result.rendered_content.is_some());
    let rendered_content = result.rendered_content.as_ref().unwrap();
    assert!(!rendered_content.is_empty());
    // 检查内容是否包含预期的文本
    assert!(rendered_content.contains("Test Page"));
    assert!(rendered_content.contains("This is a test page"));
}

#[test]
fn test_compiler_compile_batch() {
    // 创建编译器
    let mut compiler = VutexCompiler::new();

    // 准备多个文档
    use std::collections::HashMap;
    let mut documents = HashMap::new();
    documents.insert("test1.md".to_string(), r#"
---
title: Test Page 1
---

# Test Page 1

This is test page 1.
"#.to_string());
    documents.insert("test2.md".to_string(), r#"
---
title: Test Page 2
---

# Test Page 2

This is test page 2.
"#.to_string());

    let result = compiler.compile_batch(&documents);

    // 验证编译结果
    assert!(result.success);
    assert_eq!(result.documents.len(), 2);
    assert!(result.documents.contains_key("test1.md"));
    assert!(result.documents.contains_key("test2.md"));
}

#[test]
fn test_compiler_cache() {
    // 创建编译器
    let mut compiler = VutexCompiler::new();

    // 编译内容
    let content = r#"
---
title: Test Page
---

# Test Page

This is a test page.
"#;

    // 第一次编译
    let result1 = compiler.compile_document(content, "test.md").unwrap();
    assert!(compiler.get_cached("test.md").is_some());

    // 第二次编译应该使用缓存
    let result2 = compiler.compile_document(content, "test.md").unwrap();
    assert_eq!(result1, result2);

    // 清除缓存
    compiler.clear_cache();
    assert!(compiler.get_cached("test.md").is_none());
}
