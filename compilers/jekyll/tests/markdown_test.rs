use jekyll::MarkdownConverter;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_convert_basic_markdown() {
    let converter = MarkdownConverter::new();
    let markdown = "# Hello World\n\nThis is **bold** and this is *italic*";
    let html = converter.convert(markdown).unwrap();

    assert!(html.contains("<h1>Hello World</h1>"));
    assert!(html.contains("<p>This is <strong>bold</strong> and this is <em>italic</em></p>"));
}

#[test]
fn test_convert_with_code_block() {
    let converter = MarkdownConverter::new();
    let markdown = r#"```rust
fn main() {
    println!("Hello, world!");
}
```"#;
    let html = converter.convert(markdown).unwrap();

    assert!(html.contains("<pre"));
    assert!(html.contains("<code"));
    assert!(html.contains("fn main()"));
}

#[test]
fn test_plugin_registration() {
    let mut converter = MarkdownConverter::new();
    assert_eq!(converter.plugin_registry().plugin_count(), 3); // KaTeX, Mermaid, Prism
}

#[test]
fn test_convert_with_katex() {
    let converter = MarkdownConverter::new();
    let markdown = "$$E = mc^2$$";
    let html = converter.convert(markdown).unwrap();
    
    // 验证数学公式是否被处理
    assert!(!html.contains("$$E = mc^2$$"));
}

#[test]
fn test_convert_with_mermaid() {
    let converter = MarkdownConverter::new();
    let markdown = r#"```mermaid
graph TD
    A --> B
    B --> C
```"#;
    let html = converter.convert(markdown).unwrap();
    
    // 验证 Mermaid 图表是否被处理
    assert!(!html.contains("```mermaid"));
}
