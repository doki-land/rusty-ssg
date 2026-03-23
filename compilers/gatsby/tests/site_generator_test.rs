use gatsby::{GatsbyConfig, StaticSiteGenerator};
use nargo_types::Document;
use std::{collections::HashMap, fs};
use tempfile::tempdir;

#[test]
fn test_site_generator_creation() {
    let config = GatsbyConfig::new();
    let result = StaticSiteGenerator::new(config);
    assert!(result.is_ok());
}

#[test]
fn test_site_generator_generate() {
    let config = GatsbyConfig::new();
    let mut generator = StaticSiteGenerator::new(config).unwrap();

    let mut documents = HashMap::new();

    let mut doc1 = Document::default();
    doc1.frontmatter.title = Some("Test Page 1".to_string());
    doc1.rendered_content = Some("<h1>Test Page 1</h1><p>Content 1</p>".to_string());
    documents.insert("page1.md".to_string(), doc1);

    let mut doc2 = Document::default();
    doc2.frontmatter.title = Some("Test Page 2".to_string());
    doc2.rendered_content = Some("<h1>Test Page 2</h1><p>Content 2</p>".to_string());
    documents.insert("page2.md".to_string(), doc2);

    let temp_dir = tempdir().unwrap();
    let output_dir = temp_dir.path().join("output");

    let result = generator.generate(&documents, &output_dir);
    assert!(result.is_ok());

    // Check if output directory was created
    assert!(output_dir.exists());

    // Check if root index.html was created
    let root_index = output_dir.join("index.html");
    assert!(root_index.exists());

    // Check if 404.html was created
    let not_found = output_dir.join("404.html");
    assert!(not_found.exists());

    // Check if sitemap.xml was created
    let sitemap = output_dir.join("sitemap.xml");
    assert!(sitemap.exists());

    // Check if robots.txt was created
    let robots = output_dir.join("robots.txt");
    assert!(robots.exists());
}

#[test]
fn test_site_generator_generate_with_language() {
    let config = GatsbyConfig::new();
    let mut generator = StaticSiteGenerator::new(config).unwrap();

    let mut documents = HashMap::new();

    let mut doc = Document::default();
    doc.frontmatter.title = Some("Test Page".to_string());
    doc.rendered_content = Some("<h1>Test Page</h1><p>Content</p>".to_string());
    documents.insert("en/page.md".to_string(), doc);

    let temp_dir = tempdir().unwrap();
    let output_dir = temp_dir.path().join("output");

    let result = generator.generate(&documents, &output_dir);
    assert!(result.is_ok());

    // Check if language-specific directory was created
    let en_dir = output_dir.join("en");
    assert!(en_dir.exists());

    // Check if page.html was created in language directory
    let page_html = en_dir.join("page.html");
    assert!(page_html.exists());
}
