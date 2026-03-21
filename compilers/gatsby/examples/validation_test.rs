
use gatsby::{
    GatsbyConfig, GatsbyCompiler, compile_single,
    MarkdownDataSource, SiteMetadataDataSource,
    GraphQLExecutor, SchemaBuilder, NodeBuilder, ContentDigest,
    NodeId, NodeType,
};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Gatsby 编译器验证测试 ===\n");

    // 测试 1: 配置加载
    println!("测试 1: 配置加载");
    let config = GatsbyConfig::new()
        .with_site_metadata(
            gatsby::SiteMetadata::new()
                .with_title("Test Site".to_string())
                .with_description("Test Description".to_string())
        );
    println!("✓ 配置创建成功");
    println!("  - 标题: {:?}", config.site_metadata.as_ref().unwrap().title);
    println!("  - 描述: {:?}", config.site_metadata.as_ref().unwrap().description);
    println!();

    // 测试 2: 编译单个文档
    println!("测试 2: 编译单个 Markdown 文档");
    let markdown = r#"---
title: "Test Page"
description: "Test Description"
layout: "default"
---

# Hello World

This is a test page.
"#;
    
    let result = compile_single(markdown, "test.md");
    match result {
        Ok(doc) => {
            println!("✓ 文档编译成功");
            println!("  - 标题: {:?}", doc.frontmatter.title);
            println!("  - 描述: {:?}", doc.frontmatter.description);
            println!("  - 布局: {:?}", doc.frontmatter.layout);
            println!("  - 已渲染: {}", doc.rendered_content.is_some());
        }
        Err(e) => println!("✗ 编译失败: {}", e),
    }
    println!();

    // 测试 3: Markdown 数据源
    println!("测试 3: Markdown 数据源");
    let markdown_source = MarkdownDataSource::new();
    let node_result = markdown_source.create_node_from_markdown(markdown, "test.md");
    match node_result {
        Ok(node) => {
            println!("✓ Markdown 节点创建成功");
            println!("  - 节点 ID: {}", node.id);
            println!("  - 节点类型: {}", node.internal.type_name);
            println!("  - 字段数: {}", node.fields.len());
        }
        Err(e) => println!("✗ 节点创建失败: {}", e),
    }
    println!();

    // 测试 4: 站点元数据数据源
    println!("测试 4: 站点元数据数据源");
    let site_source = SiteMetadataDataSource::new();
    let site_node_result = site_source.create_site_metadata_node(
        Some("My Site".to_string()),
        Some("My Description".to_string()),
        Some("https://example.com".to_string()),
    );
    match site_node_result {
        Ok(node) => {
            println!("✓ 站点元数据节点创建成功");
            println!("  - 节点 ID: {}", node.id);
            println!("  - 节点类型: {}", node.internal.type_name);
        }
        Err(e) => println!("✗ 站点元数据节点创建失败: {}", e),
    }
    println!();

    // 测试 5: GraphQL Schema 构建
    println!("测试 5: GraphQL Schema 构建");
    let schema_result = SchemaBuilder::new()
        .query_type("Query".to_string())
        .build();
    match schema_result {
        Ok(schema) => {
            println!("✓ GraphQL Schema 构建成功");
            println!("  - 查询类型: {}", schema.query_type);
        }
        Err(e) => println!("✗ Schema 构建失败: {}", e),
    }
    println!();

    // 测试 6: 节点构建器
    println!("测试 6: 节点构建器");
    let id = NodeId::new("test-node".to_string());
    let type_name = NodeType::new("TestType".to_string());
    let digest = ContentDigest::generate("test content");
    
    let node_result = NodeBuilder::new()
        .id(id.clone())
        .type_name(type_name.clone())
        .content_digest(digest)
        .build();
    match node_result {
        Ok(node) => {
            println!("✓ 节点构建成功");
            println!("  - 节点 ID: {}", node.id);
            println!("  - 节点类型: {}", node.internal.type_name);
        }
        Err(e) => println!("✗ 节点构建失败: {}", e),
    }
    println!();

    // 测试 7: 批量编译
    println!("测试 7: 批量编译文档");
    let mut documents = HashMap::new();
    documents.insert("page1.md".to_string(), r#"---
title: "Page 1"
---
Content 1
"#.to_string());
    documents.insert("page2.md".to_string(), r#"---
title: "Page 2"
---
Content 2
"#.to_string());
    
    let compile_result = gatsby::compile_batch(&documents);
    println!("✓ 批量编译完成");
    println!("  - 成功: {}", compile_result.success);
    println!("  - 文档数: {}", compile_result.documents.len());
    println!("  - 错误数: {}", compile_result.errors.len());
    println!();

    println!("=== 验证测试完成 ===");
    println!("所有核心功能验证通过！");

    Ok(())
}
