//! 配置解析测试

use mkdocs::types::{MkDocsConfig, ThemeConfig, PaletteConfig};
use tempfile::NamedTempFile;

#[test]
fn test_parse_yaml_config() {
    let yaml_content = r#"
site_name: Test Site
site_description: A test documentation site
site_author: Test Author
site_url: https://example.com
repo_url: https://github.com/example/test
repo_name: example/test
copyright: Copyright © 2024 Test
docs_dir: docs
site_dir: site

theme:
  name: material
  language: zh
  features:
    - navigation.tabs
    - navigation.sections
  palette:
    primary: indigo
    accent: amber
    scheme: default

nav:
  - Home: index.md
  - Getting Started:
    - Installation: getting-started/installation.md
    - Quick Start: getting-started/quick-start.md
  - About: about.md

markdown_extensions:
  - admonition
  - codehilite
  - toc:
      permalink: true

plugins:
  - search
  - minify:
      enabled: true

extra:
  social:
    - icon: fontawesome/brands/github
      link: https://github.com/example
  version:
    provider: mike
"#;

    let config = MkDocsConfig::from_yaml(yaml_content).unwrap();
    
    assert_eq!(config.site_name, "Test Site");
    assert_eq!(config.site_description, Some("A test documentation site".to_string()));
    assert_eq!(config.site_author, Some("Test Author".to_string()));
    assert_eq!(config.site_url, Some("https://example.com".to_string()));
    assert_eq!(config.repo_url, Some("https://github.com/example/test".to_string()));
    assert_eq!(config.repo_name, Some("example/test".to_string()));
    assert_eq!(config.copyright, Some("Copyright © 2024 Test".to_string()));
    assert_eq!(config.docs_dir, "docs");
    assert_eq!(config.site_dir, "site");
    
    assert_eq!(config.theme.name, "material");
    assert_eq!(config.theme.language, "zh");
    assert_eq!(config.theme.features, vec!["navigation.tabs", "navigation.sections"]);
    
    assert!(config.theme.palette.is_some());
    let palette = config.theme.palette.as_ref().unwrap();
    assert_eq!(palette.primary, Some("indigo".to_string()));
    assert_eq!(palette.accent, Some("amber".to_string()));
    assert_eq!(palette.scheme, Some("default".to_string()));
    
    assert_eq!(config.markdown_extensions.len(), 3);
    assert_eq!(config.plugins.len(), 2);
}

#[test]
fn test_default_config() {
    let config = MkDocsConfig::default();
    
    assert_eq!(config.site_name, "");
    assert_eq!(config.site_description, None);
    assert_eq!(config.site_author, None);
    assert_eq!(config.site_url, None);
    assert_eq!(config.repo_url, None);
    assert_eq!(config.repo_name, None);
    assert_eq!(config.copyright, None);
    assert_eq!(config.docs_dir, "docs");
    assert_eq!(config.site_dir, "site");
    
    assert_eq!(config.theme.name, "material");
    assert_eq!(config.theme.language, "en");
    assert!(config.theme.features.is_empty());
    assert!(config.theme.palette.is_none());
    
    assert!(config.nav.is_empty());
    assert!(config.markdown_extensions.is_empty());
    assert!(config.plugins.is_empty());
    assert!(config.extra.is_empty());
    assert!(config.extra_css.is_empty());
    assert!(config.extra_javascript.is_empty());
}

#[test]
fn test_config_validation() {
    let valid_config = MkDocsConfig {
        site_name: "Valid Site".to_string(),
        ..Default::default()
    };
    assert!(valid_config.validate().is_ok());
    
    let invalid_config = MkDocsConfig {
        site_name: "".to_string(),
        ..Default::default()
    };
    assert!(invalid_config.validate().is_err());
}

#[test]
fn test_minimal_config() {
    let yaml_content = r#"
site_name: Minimal Site
"#;
    
    let config = MkDocsConfig::from_yaml(yaml_content).unwrap();
    assert_eq!(config.site_name, "Minimal Site");
    assert!(config.validate().is_ok());
}

#[test]
fn test_load_from_file() {
    let yaml_content = r#"
site_name: File Test Site
site_description: Testing file loading
"#;
    
    let mut temp_file = NamedTempFile::new().unwrap();
    std::io::Write::write_all(&mut temp_file, yaml_content.as_bytes()).unwrap();
    let temp_path = temp_file.path().to_path_buf();
    
    let config = MkDocsConfig::load_from_file(&temp_path).unwrap();
    assert_eq!(config.site_name, "File Test Site");
    assert_eq!(config.site_description, Some("Testing file loading".to_string()));
}
