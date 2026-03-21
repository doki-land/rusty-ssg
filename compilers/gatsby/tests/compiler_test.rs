use gatsby::{GatsbyCompiler, compile_batch, compile_single};
use std::collections::HashMap;

#[test]
fn test_compiler_creation() {
    let compiler = GatsbyCompiler::new();
    assert!(compiler.config().site_metadata.is_none());
}

#[test]
fn test_compile_single_document() {
    let markdown = r#"---
title: "Test Page"
description: "Test Description"
layout: "default"
---

# Hello World

This is a test page.
"#;

    let result = compile_single(markdown, "test.md");
    assert!(result.is_ok());

    let doc = result.unwrap();
    assert_eq!(doc.frontmatter.title, Some("Test Page".to_string()));
    assert_eq!(doc.frontmatter.description, Some("Test Description".to_string()));
    assert_eq!(doc.frontmatter.layout, Some("default".to_string()));
    assert!(doc.rendered_content.is_some());
}

#[test]
fn test_compile_batch_documents() {
    let mut documents = HashMap::new();

    let markdown1 = r#"---
title: "Page 1"
---

# Page 1
"#;

    let markdown2 = r#"---
title: "Page 2"
---

# Page 2
"#;

    documents.insert("page1.md".to_string(), markdown1.to_string());
    documents.insert("page2.md".to_string(), markdown2.to_string());

    let result = compile_batch(&documents);
    assert!(result.success);
    assert_eq!(result.documents.len(), 2);
    assert!(result.errors.is_empty());
}

#[test]
fn test_compiler_caching() {
    let mut compiler = GatsbyCompiler::new();

    let markdown = r#"---
title: "Test"
---

Content
"#;

    let doc1 = compiler.compile_document(markdown, "test.md").unwrap();
    let doc2 = compiler.get_cached("test.md");

    assert!(doc2.is_some());
    assert_eq!(doc1.meta.path, doc2.unwrap().meta.path);
}

#[test]
fn test_compiler_clear_cache() {
    let mut compiler = GatsbyCompiler::new();

    let markdown = r#"---
title: "Test"
---

Content
"#;

    compiler.compile_document(markdown, "test.md").unwrap();
    assert!(compiler.get_cached("test.md").is_some());

    compiler.clear_cache();
    assert!(compiler.get_cached("test.md").is_none());
}
