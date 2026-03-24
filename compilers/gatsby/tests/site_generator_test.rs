//! 站点生成器测试

use gatsby::{tools::StaticSiteGenerator, GatsbyConfig, Parser};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_static_site_generator_creation() {
    // 创建配置
    let config = GatsbyConfig::new();

    // 创建站点生成器
    let generator = StaticSiteGenerator::new(config);

    // 验证生成器创建成功
    assert!(generator.is_ok());
}

#[test]
fn test_static_site_generator_generate() {
    // 创建临时目录
    let temp_dir = tempdir().unwrap();
    let output_dir = temp_dir.path().join("public");

    // 创建配置
    let config = GatsbyConfig::new();

    // 创建站点生成器
    let mut generator = StaticSiteGenerator::new(config).unwrap();

    // 准备文档
    let mut documents = std::collections::HashMap::new();
    let content = r#"
---
title: Test Page
date: 2024-01-01
---

# Test Page

This is a test page.
"#;
    let doc = gatsby::MarkdownParser::new().parse(content, "test-page.md").unwrap();
    documents.insert("test-page.md".to_string(), doc);

    // 生成站点
    generator.generate(&documents, &output_dir).unwrap();

    // 验证文件是否生成
    let output_file = output_dir.join("test-page.html");
    assert!(output_file.exists());

    // 验证文件内容
    let file_content = fs::read_to_string(output_file).unwrap();
    assert!(file_content.contains("<h1>Test Page</h1>"));
    assert!(file_content.contains("This is a test page"));
}

#[test]
fn test_static_site_generator_generate_multiple_pages() {
    // 创建临时目录
    let temp_dir = tempdir().unwrap();
    let output_dir = temp_dir.path().join("public");

    // 创建配置
    let config = GatsbyConfig::new();

    // 创建站点生成器
    let mut generator = StaticSiteGenerator::new(config).unwrap();

    // 准备文档
    let mut documents = std::collections::HashMap::new();

    // 添加第一个页面
    let content1 = r#"
---
title: Page 1
date: 2024-01-01
---

# Page 1

This is page 1.
"#;
    let doc1 = gatsby::MarkdownParser::new().parse(content1, "page1.md").unwrap();
    documents.insert("page1.md".to_string(), doc1);

    // 添加第二个页面
    let content2 = r#"
---
title: Page 2
date: 2024-01-02
---

# Page 2

This is page 2.
"#;
    let doc2 = gatsby::MarkdownParser::new().parse(content2, "page2.md").unwrap();
    documents.insert("page2.md".to_string(), doc2);

    // 生成站点
    generator.generate(&documents, &output_dir).unwrap();

    // 验证文件是否生成
    let page1_file = output_dir.join("page1.html");
    let page2_file = output_dir.join("page2.html");

    assert!(page1_file.exists());
    assert!(page2_file.exists());

    // 验证文件内容
    let page1_content = fs::read_to_string(page1_file).unwrap();
    let page2_content = fs::read_to_string(page2_file).unwrap();

    assert!(page1_content.contains("<h1>Page 1</h1>"));
    assert!(page2_content.contains("<h1>Page 2</h1>"));
}

#[test]
fn test_static_site_generator_generate_index() {
    // 创建临时目录
    let temp_dir = tempdir().unwrap();
    let output_dir = temp_dir.path().join("public");

    // 创建配置
    let config = GatsbyConfig::new()
        .with_site_metadata(
            gatsby::config::SiteMetadata::new()
                .with_title("Test Site".to_string())
                .with_description("A test site".to_string())
        );

    // 创建站点生成器
    let mut generator = StaticSiteGenerator::new(config).unwrap();

    // 准备文档
    let mut documents = std::collections::HashMap::new();

    // 生成站点
    generator.generate(&documents, &output_dir).unwrap();

    // 验证索引文件是否生成
    let index_file = output_dir.join("index.html");
    assert!(index_file.exists());

    // 验证文件内容
    let index_content = fs::read_to_string(index_file).unwrap();
    assert!(index_content.contains("Test Site"));
    assert!(index_content.contains("A test site"));
}

#[test]
fn test_static_site_generator_generate_404() {
    // 创建临时目录
    let temp_dir = tempdir().unwrap();
    let output_dir = temp_dir.path().join("public");

    // 创建配置
    let config = GatsbyConfig::new();

    // 创建站点生成器
    let mut generator = StaticSiteGenerator::new(config).unwrap();

    // 准备文档
    let mut documents = std::collections::HashMap::new();

    // 生成站点
    generator.generate(&documents, &output_dir).unwrap();

    // 验证 404 文件是否生成
    let not_found_file = output_dir.join("404.html");
    assert!(not_found_file.exists());

    // 验证文件内容
    let not_found_content = fs::read_to_string(not_found_file).unwrap();
    assert!(not_found_content.contains("404"));
    assert!(not_found_content.contains("Page Not Found"));
}
