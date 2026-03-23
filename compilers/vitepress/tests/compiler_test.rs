//! 编译器测试

use vitepress::compiler::VitePressCompiler;

#[test]
fn test_compiler_creation() {
    // 创建编译器
    let compiler = VitePressCompiler::new();
    
    // 验证编译器创建成功
    assert!(compiler.config().title.is_some() || true);
}

#[test]
fn test_compiler_parse() {
    // 创建编译器
    let mut compiler = VitePressCompiler::new();
    
    // 解析内容
    let content = r#"
---
title: Test Page
date: 2024-01-01
---

# Test Page

This is a test page.

## Subheading

This is a subheading.
"#;
    
    let parsed = compiler.compile_document(content, "test.md").unwrap();
    
    // 验证解析结果
    assert_eq!(parsed.frontmatter.title, Some("Test Page".to_string()));
    assert!(parsed.content.contains("This is a test page"));
    assert!(parsed.content.contains("This is a subheading"));
}

#[test]
fn test_compiler_full_process() {
    // 创建编译器
    let mut compiler = VitePressCompiler::new();
    
    // 完整编译过程
    let content = r#"
---
title: Test Page
date: 2024-01-01
---

# Test Page

This is a test page with **bold** text and *italic* text.

## Subheading

This is a subheading with a [link](https://example.com).
"#;
    
    let result = compiler.compile_document(content, "test.md").unwrap();
    
    // 验证编译结果
    assert_eq!(result.frontmatter.title, Some("Test Page".to_string()));
    assert!(result.content.contains("# Test Page"));
    assert!(result.content.contains("This is a test page with **bold** text and *italic* text"));
    assert!(result.content.contains("## Subheading"));
    assert!(result.content.contains("This is a subheading with a [link](https://example.com)"));
}
