//! Front Matter 增强器测试
//! 测试 Front Matter 增强器的功能

use hugo::types::document::{FrontMatterEnhancer, HugoFrontMatter};
use std::path::PathBuf;

#[test]
fn test_extract_date_from_filename() {
    let path = PathBuf::from("2024-01-01-my-post.md");
    assert_eq!(FrontMatterEnhancer::extract_date_from_filename(&path), Some("2024-01-01".to_string()));

    let path = PathBuf::from("my-post.md");
    assert_eq!(FrontMatterEnhancer::extract_date_from_filename(&path), None);
}

#[test]
fn test_is_valid_date() {
    assert!(FrontMatterEnhancer::is_valid_date("2024-01-01"));
    assert!(FrontMatterEnhancer::is_valid_date("2024-01-01 12:00:00"));
    assert!(FrontMatterEnhancer::is_valid_date("2024-01-01T12:00:00"));
    assert!(FrontMatterEnhancer::is_valid_date("2024-01-01T12:00:00Z"));
    assert!(!FrontMatterEnhancer::is_valid_date("invalid-date"));
}

#[test]
fn test_enhance() {
    let frontmatter = HugoFrontMatter::new();
    let path = PathBuf::from("2024-01-01-test.md");

    let enhanced = FrontMatterEnhancer::enhance(frontmatter, &path).unwrap();
    assert_eq!(enhanced.date, Some("2024-01-01".to_string()));
    assert!(enhanced.lastmod.is_some());
    assert_eq!(enhanced.publish_date, Some("2024-01-01".to_string()));
}

#[test]
fn test_validate() {
    let mut frontmatter = HugoFrontMatter::new();
    frontmatter.date = Some("2024-01-01".to_string());
    frontmatter.lastmod = Some("2024-01-02".to_string());

    assert!(FrontMatterEnhancer::validate(&frontmatter).is_ok());

    frontmatter.date = Some("invalid-date".to_string());
    assert!(FrontMatterEnhancer::validate(&frontmatter).is_err());
}
