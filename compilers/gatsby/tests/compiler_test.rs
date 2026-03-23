//! 编译器测试

use gatsby::compiler::{Compiler, Parser, Renderer};

#[test]
fn test_compiler_creation() {
    // 创建编译器
    let compiler = Compiler::new();

    // 验证编译器组件
    assert!(compiler.parser().is_some());
    assert!(compiler.renderer().is_some());
}

#[test]
fn test_compiler_parse() {
    // 创建编译器
    let compiler = Compiler::new();

    // 解析内容
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

    let parsed = compiler.parse(content).unwrap();

    // 验证解析结果
    assert_eq!(parsed["title"], "Test Page");
    assert_eq!(parsed["date"], "2024-01-01");
    assert!(parsed["tags"].as_array().unwrap().contains(&serde_json::Value::String("test".to_string())));
    assert!(parsed["content"].as_str().unwrap().contains("This is a test page"));
}

#[test]
fn test_compiler_render() {
    // 创建编译器
    let compiler = Compiler::new();

    // 解析内容
    let content = r#"
---
title: Test Page
date: 2024-01-01
---

# Test Page

This is a test page.
"#;

    let parsed = compiler.parse(content).unwrap();

    // 渲染内容
    let rendered = compiler.render(&parsed).unwrap();

    // 验证渲染结果
    assert!(rendered.contains("<h1>Test Page</h1>"));
    assert!(rendered.contains("<p>This is a test page.</p>"));
}

#[test]
fn test_compiler_full_process() {
    // 创建编译器
    let compiler = Compiler::new();

    // 完整编译过程
    let content = r#"
---
title: Test Page
date: 2024-01-01
tags:
  - test
  - gatsby
---

# Test Page

This is a test page with **bold** text and *italic* text.
"#;

    let result = compiler.compile(content).unwrap();

    // 验证编译结果
    assert!(result.contains("<h1>Test Page</h1>"));
    assert!(result.contains("<p>This is a test page with <strong>bold</strong> text and <em>italic</em> text.</p>"));
}
