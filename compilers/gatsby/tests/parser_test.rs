//! 解析器测试

use gatsby::{MarkdownParser, Parser};

#[test]
fn test_markdown_parser() {
    let parser = MarkdownParser::new();
    let content = r#"
---
title: Test Page
date: 2024-01-01
---

# Test Page

This is a test page.
"#;
    let doc = parser.parse(content, "test-page.md").unwrap();
    assert_eq!(doc.title(), Some("Test Page"));
    assert!(!doc.content.is_empty());
    assert!(doc.rendered_content.is_some());
    let rendered_content = doc.rendered_content.as_ref().unwrap();
    assert!(rendered_content.contains("<h1>Test Page</h1>"));
    assert!(rendered_content.contains("<p>This is a test page.</p>"));
    println!("Title: {:?}", doc.title());
    println!("Content: '{}'", doc.content);
    println!("Rendered content: '{}'", rendered_content);
}

#[test]
fn test_markdown_parser_no_frontmatter() {
    let parser = MarkdownParser::new();
    let content = r#"
# Test Page

This is a test page without frontmatter.
"#;
    let doc = parser.parse(content, "test-page.md").unwrap();
    assert_eq!(doc.title(), None);
    assert!(!doc.content.is_empty());
    assert!(doc.rendered_content.is_some());
    let rendered_content = doc.rendered_content.as_ref().unwrap();
    assert!(rendered_content.contains("<h1>Test Page</h1>"));
    assert!(rendered_content.contains("<p>This is a test page without frontmatter.\n</p>"));
}

#[test]
fn test_markdown_parser_only_frontmatter() {
    let parser = MarkdownParser::new();
    let content = r#"
---
title: Test Page
date: 2024-01-01
---
"#;
    let doc = parser.parse(content, "test-page.md").unwrap();
    assert_eq!(doc.title(), Some("Test Page"));
    assert!(doc.content.is_empty());
    assert!(doc.rendered_content.is_some());
}

#[test]
fn test_markdown_parser_empty_content() {
    let parser = MarkdownParser::new();
    let content = "";
    let doc = parser.parse(content, "test-page.md").unwrap();
    assert_eq!(doc.title(), None);
    assert!(doc.content.is_empty());
    assert!(doc.rendered_content.is_some());
}

#[test]
fn test_markdown_parser_complex_frontmatter() {
    let parser = MarkdownParser::new();
    let content = r#"
---
title: Test Page
date: 2024-01-01
author: John Doe
description: This is a test page
tags:
  - test
  - gatsby
  - rust
categories:
  - blog
  - tutorial
---

# Test Page

This is a test page with complex frontmatter.
"#;
    let doc = parser.parse(content, "test-page.md").unwrap();
    assert_eq!(doc.title(), Some("Test Page"));
    assert!(!doc.content.is_empty());
    assert!(doc.rendered_content.is_some());
    let rendered_content = doc.rendered_content.as_ref().unwrap();
    assert!(rendered_content.contains("<h1>Test Page</h1>"));
    assert!(rendered_content.contains("<p>This is a test page with complex frontmatter.</p>"));
}