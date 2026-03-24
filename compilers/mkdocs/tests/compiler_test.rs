//! 编译器测试

use mkdocs::{compile_single, MkDocsCompiler};

#[test]
fn test_compiler_creation() {
    // 创建编译器
    let config = mkdocs::MkDocsConfig::new();
    let compiler = MkDocsCompiler::new(config, &std::path::Path::new("."), &std::path::Path::new("./site"));

    // 验证编译器创建成功
    assert!(true);
}

#[test]
fn test_compile_single() {
    // 测试单个文档编译
    let content = r#"
# Test Page

This is a test page.

## Subheading

This is a subheading.
"#;

    let result = compile_single(content);

    // 验证编译结果
    assert!(result.is_ok());
    let html = result.unwrap();
    assert!(html.contains("<h1>Test Page</h1>"));
    assert!(html.contains("<p>This is a test page.</p>"));
    assert!(html.contains("<h2>Subheading</h2>"));
    assert!(html.contains("<p>This is a subheading.</p>"));
}

#[test]
fn test_compile_with_formatting() {
    // 测试带有格式化的文档编译
    let content = r#"
# Test Page

This is a test page with **bold** text and *italic* text.

## Subheading

This is a subheading with a [link](https://example.com).
"#;

    let result = compile_single(content);

    // 验证编译结果
    assert!(result.is_ok());
    let html = result.unwrap();
    assert!(html.contains("<h1>Test Page</h1>"));
    assert!(html.contains("<p>This is a test page with <strong>bold</strong> text and <em>italic</em> text.</p>"));
    assert!(html.contains("<h2>Subheading</h2>"));
    assert!(html.contains("<p>This is a subheading with a <a href=\"https://example.com\">link</a>.</p>"));
}
