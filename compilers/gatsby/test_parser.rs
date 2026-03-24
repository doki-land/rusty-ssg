use gatsby::MarkdownParser;

fn main() {
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
    println!("Title: {:?}", doc.title());
    println!("Content: '{}'", doc.content);
    println!("Rendered content: '{}'", doc.rendered_content.unwrap());
}