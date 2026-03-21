use gatsby::{
    MarkdownDataSource, SiteMetadataDataSource, FileDataSource, DataLayerManager,
    Node, NodeId, NodeType, ContentDigest,
};
use nargo_parser::parse_document;

#[test]
fn test_markdown_data_source_creation() {
    let source = MarkdownDataSource::new();
    let _ = source;
}

#[test]
fn test_create_node_from_markdown() {
    let source = MarkdownDataSource::new();
    
    let markdown = r#"---
title: "Test Page"
description: "Test Description"
layout: "default"
---

# Hello World

This is a test page.
"#;
    
    let result = source.create_node_from_markdown(markdown, "test.md");
    assert!(result.is_ok());
    
    let node = result.unwrap();
    assert_eq!(node.id, NodeId::new("test.md".to_string()));
    assert_eq!(node.internal.type_name, NodeType::new("MarkdownRemark".to_string()));
    
    let title = node.get_field("title");
    assert!(title.is_some());
    
    let description = node.get_field("description");
    assert!(description.is_some());
    
    let layout = node.get_field("layout");
    assert!(layout.is_some());
}

#[test]
fn test_create_node_from_document() {
    let source = MarkdownDataSource::new();
    
    let markdown = r#"---
title: "Test Page"
---

Content
"#;
    
    let doc = parse_document(markdown, "test.md").unwrap();
    let result = source.create_node_from_document(doc, markdown);
    
    assert!(result.is_ok());
    let node = result.unwrap();
    assert_eq!(node.id, NodeId::new("test.md".to_string()));
}

#[test]
fn test_site_metadata_data_source() {
    let source = SiteMetadataDataSource::new();
    
    let result = source.create_site_metadata_node(
        Some("My Site".to_string()),
        Some("My Description".to_string()),
        Some("https://example.com".to_string()),
    );
    
    assert!(result.is_ok());
    let node = result.unwrap();
    
    assert_eq!(node.id, NodeId::new("Site".to_string()));
    assert_eq!(node.internal.type_name, NodeType::new("Site".to_string()));
    
    let site_metadata = node.get_field("siteMetadata");
    assert!(site_metadata.is_some());
}

#[test]
fn test_file_data_source() {
    let source = FileDataSource::new();
    
    let result = source.create_file_node(
        "/path/to/file.txt",
        "file",
        "txt",
        1024,
    );
    
    assert!(result.is_ok());
    let node = result.unwrap();
    
    assert_eq!(node.id, NodeId::new("/path/to/file.txt".to_string()));
    assert_eq!(node.internal.type_name, NodeType::new("File".to_string()));
}

#[test]
fn test_data_layer_manager() {
    let manager = DataLayerManager::new();
    
    let _ = manager.markdown_source();
    let _ = manager.site_metadata_source();
    let _ = manager.file_source();
}

#[test]
fn test_content_digest_consistency() {
    let content = "test content";
    let digest1 = ContentDigest::generate(content);
    let digest2 = ContentDigest::generate(content);
    
    assert_eq!(digest1, digest2);
}

#[test]
fn test_content_digest_different_content() {
    let content1 = "content 1";
    let content2 = "content 2";
    
    let digest1 = ContentDigest::generate(content1);
    let digest2 = ContentDigest::generate(content2);
    
    assert_ne!(digest1, digest2);
}
