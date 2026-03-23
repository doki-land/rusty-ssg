//! 编译器测试
//! 测试 VitePressCompiler 的核心功能

use vitepress::{compiler::VitePressCompiler, types::VitePressConfig};

#[test]
fn test_compiler_basic() {
    // 创建编译器
    let mut compiler = VitePressCompiler::new();

    // 测试基本的 Markdown 文档编译
    let markdown = r#"---
title: 测试文档
description: 这是一个测试文档
---

# 测试标题

这是测试内容。
"#;

    let result = compiler.compile_document(markdown, "test.md");
    assert!(result.is_ok());

    let doc = result.unwrap();
    assert_eq!(doc.frontmatter.title, Some("测试文档".to_string()));
    assert_eq!(doc.frontmatter.description, Some("这是一个测试文档".to_string()));
    assert!(doc.content.contains("测试标题"));
    assert!(doc.rendered_content.is_some());

    println!("编译器基本功能测试通过！");
}

#[test]
fn test_compiler_batch() {
    // 创建编译器
    let mut compiler = VitePressCompiler::new();

    // 准备多个文档
    let mut documents = std::collections::HashMap::new();
    documents.insert("doc1.md".to_string(), "# 文档 1\n内容 1".to_string());
    documents.insert("doc2.md".to_string(), "# 文档 2\n内容 2".to_string());

    // 批量编译
    let result = compiler.compile_batch(&documents);
    assert!(result.success);
    assert_eq!(result.documents.len(), 2);

    println!("编译器批量编译测试通过！");
}

#[test]
fn test_compiler_with_config() {
    // 创建配置
    let config = VitePressConfig::new().with_title("测试站点").with_lang("zh-CN");

    // 创建带配置的编译器
    let mut compiler = VitePressCompiler::with_config(config);

    // 测试编译
    let markdown = "# 测试标题";
    let result = compiler.compile_document(markdown, "test.md");
    assert!(result.is_ok());

    println!("编译器配置测试通过！");
}
