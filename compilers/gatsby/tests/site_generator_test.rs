//! 站点生成器测试

use gatsby::tools::site_generator::SiteGenerator;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_site_generator_creation() {
    // 创建临时目录
    let temp_dir = tempdir().unwrap();
    let output_dir = temp_dir.path().join("public");

    // 创建站点生成器
    let generator = SiteGenerator::new(output_dir.to_str().unwrap());

    // 验证生成器配置
    assert_eq!(generator.output_dir(), output_dir.to_str().unwrap());
}

#[test]
fn test_site_generator_generate() {
    // 创建临时目录
    let temp_dir = tempdir().unwrap();
    let output_dir = temp_dir.path().join("public");

    // 创建站点生成器
    let mut generator = SiteGenerator::new(output_dir.to_str().unwrap());

    // 生成站点
    let content = r#"
---
title: Test Page
date: 2024-01-01
---

# Test Page

This is a test page.
"#;

    generator.generate("test-page", content).unwrap();

    // 验证文件是否生成
    let output_file = output_dir.join("test-page.html");
    assert!(output_file.exists());

    // 验证文件内容
    let file_content = fs::read_to_string(output_file).unwrap();
    assert!(file_content.contains("<h1>Test Page</h1>"));
    assert!(file_content.contains("<p>This is a test page.</p>"));
}

#[test]
fn test_site_generator_generate_multiple_pages() {
    // 创建临时目录
    let temp_dir = tempdir().unwrap();
    let output_dir = temp_dir.path().join("public");

    // 创建站点生成器
    let mut generator = SiteGenerator::new(output_dir.to_str().unwrap());

    // 生成多个页面
    let pages = vec![
        (
            "page1",
            r#"
---
title: Page 1
date: 2024-01-01
---

# Page 1

This is page 1.
"#,
        ),
        (
            "page2",
            r#"
---
title: Page 2
date: 2024-01-02
---

# Page 2

This is page 2.
"#,
        ),
    ];

    for (path, content) in pages {
        generator.generate(path, content).unwrap();
    }

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
fn test_site_generator_clean() {
    // 创建临时目录
    let temp_dir = tempdir().unwrap();
    let output_dir = temp_dir.path().join("public");
    fs::create_dir_all(&output_dir).unwrap();

    // 创建测试文件
    let test_file = output_dir.join("test.html");
    fs::write(test_file, "test content").unwrap();

    // 创建站点生成器
    let mut generator = SiteGenerator::new(output_dir.to_str().unwrap());

    // 清理输出目录
    generator.clean().unwrap();

    // 验证文件是否被删除
    assert!(!test_file.exists());
}
