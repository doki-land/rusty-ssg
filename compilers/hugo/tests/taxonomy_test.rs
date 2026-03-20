//! 分类法（Taxonomies）系统测试

use std::path::PathBuf;
use hugo::types::document::hugo_content::{HugoPage, HugoFrontMatter};
use hugo::types::taxonomies::{TaxonomyBuilder, Taxonomy, TaxonomyTerm, TaxonomyIndex};

fn create_test_page(title: &str, tags: Vec<String>, categories: Vec<String>) -> HugoPage {
    let mut page = HugoPage::new(
        PathBuf::from(format!("/content/test/{}.md", title.to_lowercase())),
        PathBuf::from(format!("test/{}.md", title.to_lowercase())),
    );
    
    let mut frontmatter = HugoFrontMatter::new();
    frontmatter.title = Some(title.to_string());
    frontmatter.tags = Some(tags);
    frontmatter.categories = Some(categories);
    
    page.frontmatter = frontmatter;
    page
}

#[test]
fn test_taxonomy_builder_default() {
    let builder = TaxonomyBuilder::new().with_default_taxonomies();
    let index = builder.finish();
    
    assert!(index.get_taxonomy("tags").is_some());
    assert!(index.get_taxonomy("categories").is_some());
}

#[test]
fn test_taxonomy_builder_custom() {
    let mut builder = TaxonomyBuilder::new();
    builder.register_taxonomy("colors".to_string(), "color".to_string());
    let index = builder.finish();
    
    assert!(index.get_taxonomy("colors").is_some());
    assert_eq!(index.get_taxonomy("colors").unwrap().singular, "color");
}

#[test]
fn test_build_from_pages() {
    let page1 = create_test_page(
        "First Post",
        vec!["rust".to_string(), "web".to_string()],
        vec!["programming".to_string()]
    );
    
    let page2 = create_test_page(
        "Second Post",
        vec!["rust".to_string(), "testing".to_string()],
        vec!["programming".to_string(), "tutorial".to_string()]
    );
    
    let mut builder = TaxonomyBuilder::new().with_default_taxonomies();
    let index = builder.build_from_pages(&[page1, page2]);
    
    let tags = index.get_taxonomy("tags").unwrap();
    assert_eq!(tags.terms.len(), 3);
    assert!(tags.get_term("rust").is_some());
    assert_eq!(tags.get_term("rust").unwrap().pages.len(), 2);
    
    let categories = index.get_taxonomy("categories").unwrap();
    assert_eq!(categories.terms.len(), 2);
    assert!(categories.get_term("programming").is_some());
    assert_eq!(categories.get_term("programming").unwrap().pages.len(), 2);
}

#[test]
fn test_taxonomy_term() {
    let mut term = TaxonomyTerm::new("Rust".to_string(), "rust".to_string());
    assert_eq!(term.name, "Rust");
    assert_eq!(term.slug, "rust");
    assert!(term.pages.is_empty());
    
    let page = create_test_page("Test", vec![], vec![]);
    term.add_page(page);
    assert_eq!(term.pages.len(), 1);
}

#[test]
fn test_taxonomy() {
    let mut taxonomy = Taxonomy::new("tags".to_string(), "tag".to_string());
    assert_eq!(taxonomy.name, "tags");
    assert_eq!(taxonomy.singular, "tag");
    assert!(taxonomy.terms.is_empty());
    
    let term = TaxonomyTerm::new("Rust".to_string(), "rust".to_string());
    taxonomy.add_term(term);
    assert_eq!(taxonomy.terms.len(), 1);
    assert!(taxonomy.get_term("rust").is_some());
}

#[test]
fn test_sorted_terms() {
    let mut taxonomy = Taxonomy::new("tags".to_string(), "tag".to_string());
    
    taxonomy.add_term(TaxonomyTerm::new("Zebra".to_string(), "zebra".to_string()));
    taxonomy.add_term(TaxonomyTerm::new("Apple".to_string(), "apple".to_string()));
    taxonomy.add_term(TaxonomyTerm::new("Banana".to_string(), "banana".to_string()));
    
    let sorted = taxonomy.get_sorted_terms();
    assert_eq!(sorted.len(), 3);
    assert_eq!(sorted[0].name, "Apple");
    assert_eq!(sorted[1].name, "Banana");
    assert_eq!(sorted[2].name, "Zebra");
}

#[test]
fn test_slugify() {
    let mut builder = TaxonomyBuilder::new().with_default_taxonomies();
    
    let page = create_test_page(
        "Test",
        vec!["Hello World".to_string(), "Rust!".to_string(), "Test 123".to_string()],
        vec![]
    );
    
    builder.build_from_pages(&[page]);
    let index = builder.finish();
    
    let tags = index.get_taxonomy("tags").unwrap();
    assert!(tags.get_term("hello-world").is_some());
    assert!(tags.get_term("rust").is_some());
    assert!(tags.get_term("test-123").is_some());
}
