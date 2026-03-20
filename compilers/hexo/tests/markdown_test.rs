//! Markdown 解析和渲染测试

use hexo::markdown::{parse_markdown, render_markdown, render_markdown_file};
use std::path::Path;

#[test]
fn test_parse_markdown_with_front_matter() {
    let content = "---\ntitle: Test Post\ndate: 2026-03-20\ncategories:\n  - Test\ntags:\n  - Rust\n  - Hexo\ncustom_field: value\n---\n\n# Hello World\n\nThis is a test post.";

    let (front_matter, markdown_content) = parse_markdown(content).unwrap();

    assert!(front_matter.is_some());
    let fm = front_matter.unwrap();
    assert_eq!(fm.title, Some("Test Post".to_string()));
    assert_eq!(fm.date, Some("2026-03-20".to_string()));
    assert_eq!(fm.categories, Some(vec!["Test".to_string()]));
    assert_eq!(fm.tags, Some(vec!["Rust".to_string(), "Hexo".to_string()]));
    assert_eq!(markdown_content, "# Hello World\n\nThis is a test post.");
}

#[test]
fn test_parse_markdown_without_front_matter() {
    let content = "# Hello World\n\nThis is a test post.";

    let (front_matter, markdown_content) = parse_markdown(content).unwrap();

    assert!(front_matter.is_none());
    assert_eq!(markdown_content, "# Hello World\n\nThis is a test post.");
}

#[test]
fn test_render_markdown() {
    let content = "# Hello World\n\nThis is a **test** post with *italic* text.";

    let html = render_markdown(content);

    assert!(html.contains("<h1>Hello World</h1>"));
    assert!(html.contains("<p>This is a <strong>test</strong> post with <em>italic</em> text.</p>"));
}

#[test]
fn test_render_markdown_file() {
    let test_file = Path::new("../../examples/hexo-mvp/source/_posts/guide.md");

    let (front_matter, html) = render_markdown_file(test_file).unwrap();

    assert!(front_matter.is_some());
    let fm = front_matter.unwrap();
    assert!(fm.title.is_some());
    assert!(!html.is_empty());
}

#[test]
fn test_org_mode_support() {
    // 测试Org模式文档
    let org_mode_content = r#"#+TITLE: Org模式测试
#+DATE: 2023-01-01

* 一级标题
** 二级标题

这是一个 *粗体* 文本和 /斜体/ 文本。

- 列表项1
- 列表项2

#+BEGIN_SRC rust
fn main() {
    println!("Hello, Org Mode!");
}
#+END_SRC
"#;

    // 测试Hexo对Org模式的处理（作为普通文本）
    let (front_matter, content) = parse_markdown(org_mode_content).unwrap();
    assert!(front_matter.is_none());
    assert!(!content.is_empty());
    let html = render_markdown(&content);
    assert!(!html.is_empty());
}

#[test]
fn test_html_document_support() {
    // 测试HTML文档
    let html_content = r#"<!DOCTYPE html>
<html>
<head>
    <title>HTML测试</title>
</head>
<body>
    <h1>HTML标题</h1>
    <p>这是一个HTML段落。</p>
</body>
</html>"#;

    // 测试Hexo对HTML的处理
    let (front_matter, content) = parse_markdown(html_content).unwrap();
    assert!(front_matter.is_none());
    assert!(!content.is_empty());
    let html = render_markdown(&content);
    assert!(!html.is_empty());
}
