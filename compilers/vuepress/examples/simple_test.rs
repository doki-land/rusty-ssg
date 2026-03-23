//! 简单测试程序验证 VuePressCompiler 功能

use std::collections::HashMap;
use vuepress::VuePressCompiler;

fn main() {
    println!("=== 测试 VuePressCompiler ===\n");

    let mut compiler = VuePressCompiler::new();

    let test_source = r#"---
title: 测试文档
description: 这是一个测试文档
tags: [test, markdown]
---

# 欢迎使用 VuTeX

这是一个 **简单** 的测试文档。

## 主要特性

1. 支持 Markdown 解析
2. 支持 Frontmatter
3. 支持 HTML 渲染

## 代码示例

```rust
fn main() {
    println!("Hello, World!");
}
```

感谢使用！
"#;

    println!("1. 测试单个文档编译...");
    match compiler.compile_document(test_source, "test.md") {
        Ok(doc) => {
            println!("   ✓ 文档解析成功！");
            println!("   文档路径: {:?}", doc.meta.path);
            println!("   文档标题: {:?}", doc.title());
            println!("   内容长度: {} 字符", doc.content.len());
            println!("   渲染内容: {:?}", doc.rendered_content.is_some());

            if let Some(rendered) = &doc.rendered_content {
                println!("   渲染结果长度: {} 字符", rendered.len());
            }
        }
        Err(e) => {
            eprintln!("   ✗ 文档解析失败: {:?}", e);
        }
    }

    println!("\n2. 测试批量编译...");
    let mut documents = HashMap::new();
    documents.insert("doc1.md".to_string(), test_source.to_string());
    documents.insert("doc2.md".to_string(), "# 第二份文档\n这是另一份文档的内容。".to_string());

    let result = compiler.compile_batch(&documents);
    if result.success {
        println!("   ✓ 批量编译成功！");
        println!("   编译文档数: {}", result.documents.len());
        println!("   编译时间: {}ms", result.compile_time_ms);
    }
    else {
        eprintln!("   ✗ 批量编译失败: {:?}", result.errors);
    }

    println!("\n=== 测试完成 ===");
}
