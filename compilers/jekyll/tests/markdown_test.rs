use crate::jekyll::markdown::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_markdown_processor_from_name() {
    assert_eq!(MarkdownProcessor::from_name("commonmark").unwrap(), MarkdownProcessor::CommonMark);
    assert_eq!(MarkdownProcessor::from_name("kramdown").unwrap(), MarkdownProcessor::Kramdown);
    assert_eq!(MarkdownProcessor::from_name("gfm").unwrap(), MarkdownProcessor::Gfm);
    assert_eq!(MarkdownProcessor::from_name("github").unwrap(), MarkdownProcessor::Gfm);
    assert!(MarkdownProcessor::from_name("invalid").is_err());
}

#[test]
fn test_markdown_options_builder() {
    let options = MarkdownOptions::new()
        .with_tables(true)
        .with_footnotes(true)
        .with_code_highlighting(true)
        .with_highlight_theme("github".to_string());

    assert!(options.tables);
    assert!(options.footnotes);
    assert!(options.code_highlighting);
    assert_eq!(options.highlight_theme, Some("github".to_string()));
}

#[test]
fn test_convert_basic_markdown() {
    let converter = MarkdownConverter::with_defaults(MarkdownProcessor::CommonMark);
    let markdown = "# Hello World\n\nThis is **bold** and this is *italic*";
    let html = converter.convert(markdown).unwrap();

    assert!(html.contains("<h1>Hello World</h1>"));
    assert!(html.contains("<p>This is <strong>bold</strong> and this is <em>italic</em></p>"));
}

#[test]
fn test_convert_with_tables() {
    let options = MarkdownOptions::new().with_tables(true);
    let converter = MarkdownConverter::commonmark(options);
    let markdown = r#"| Header 1 | Header 2 |
| -------- | -------- |
| Row 1    | Row 1    |
| Row 2    | Row 2    |"#;
    let html = converter.convert(markdown).unwrap();

    assert!(html.contains("<table"));
    assert!(html.contains("<th>Header 1</th>"));
    assert!(html.contains("<td>Row 1</td>"));
}

#[test]
fn test_convert_with_code_block() {
    let converter = MarkdownConverter::with_defaults(MarkdownProcessor::Gfm);
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
fn test_convert_file() {
    let temp_dir = tempdir().unwrap();
    let markdown_path = temp_dir.path().join("test.md");
    fs::write(&markdown_path, "# Test File\n\nThis is a test file").unwrap();

    let converter = MarkdownConverter::with_defaults(MarkdownProcessor::CommonMark);
    let html = converter.convert_file(&markdown_path).unwrap();

    assert!(html.contains("<h1>Test File</h1>"));
    assert!(html.contains("<p>This is a test file</p>"));
}

#[test]
fn test_from_jekyll_config() {
    let mut config = crate::jekyll::JekyllConfig::new();
    config.markdown = Some("kramdown".to_string());

    let converter = MarkdownConverter::from_jekyll_config(&config);
    assert_eq!(converter.processor(), MarkdownProcessor::Kramdown);
}
