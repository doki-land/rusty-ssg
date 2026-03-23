use std::{fs, path::Path};
use tempfile::tempdir;

use jekyll::{FrontMatterParser, JekyllConfig, JekyllConfigLoader, JekyllDirectory, JekyllStructure};

#[test]
fn test_jekyll_directory_name() {
    assert_eq!(JekyllDirectory::Posts.name(), "_posts");
    assert_eq!(JekyllDirectory::Layouts.name(), "_layouts");
    assert_eq!(JekyllDirectory::Includes.name(), "_includes");
    assert_eq!(JekyllDirectory::Data.name(), "_data");
    assert_eq!(JekyllDirectory::Drafts.name(), "_drafts");
    assert_eq!(JekyllDirectory::Sass.name(), "_sass");
    assert_eq!(JekyllDirectory::Site.name(), "_site");
    assert_eq!(JekyllDirectory::Assets.name(), "assets");
}

#[test]
fn test_jekyll_directory_from_name() {
    assert_eq!(JekyllDirectory::from_name("_posts"), Some(JekyllDirectory::Posts));
    assert_eq!(JekyllDirectory::from_name("_layouts"), Some(JekyllDirectory::Layouts));
    assert_eq!(JekyllDirectory::from_name("_includes"), Some(JekyllDirectory::Includes));
    assert_eq!(JekyllDirectory::from_name("_data"), Some(JekyllDirectory::Data));
    assert_eq!(JekyllDirectory::from_name("_drafts"), Some(JekyllDirectory::Drafts));
    assert_eq!(JekyllDirectory::from_name("_sass"), Some(JekyllDirectory::Sass));
    assert_eq!(JekyllDirectory::from_name("_site"), Some(JekyllDirectory::Site));
    assert_eq!(JekyllDirectory::from_name("assets"), Some(JekyllDirectory::Assets));
    assert_eq!(JekyllDirectory::from_name("unknown"), None);
}

#[test]
fn test_jekyll_structure_creation() {
    let temp_dir = tempdir().unwrap();
    let structure = JekyllStructure::new(temp_dir.path());
    assert!(structure.is_ok());
}

#[test]
fn test_directories_discovery() {
    let temp_dir = tempdir().unwrap();
    let root = temp_dir.path();

    // 创建一些 Jekyll 目录
    fs::create_dir(root.join("_posts")).unwrap();
    fs::create_dir(root.join("_layouts")).unwrap();
    fs::create_dir(root.join("_includes")).unwrap();

    let structure = JekyllStructure::new(root).unwrap();

    assert!(structure.has_directory(JekyllDirectory::Posts));
    assert!(structure.has_directory(JekyllDirectory::Layouts));
    assert!(structure.has_directory(JekyllDirectory::Includes));
    assert!(!structure.has_directory(JekyllDirectory::Data));
}

#[test]
fn test_front_matter_simple_parse() {
    let content = r#"---
title: Hello World
layout: post
date: 2024-01-01
---

This is the content.
"#;

    let front_matter = FrontMatterParser::parse(content).unwrap();

    assert_eq!(front_matter.get("title").unwrap(), "Hello World");
    assert_eq!(front_matter.get("layout").unwrap(), "post");
    assert!(front_matter.content().contains("This is the content."));
}

#[test]
fn test_front_matter_complex_parse() {
    let content = r#"---
title: Complex Document
tags:
  - programming
  - rust
  - yaml
author:
  name: John Doe
  email: john@example.com
  social:
    github: johndoe
    twitter: johndoe123
metadata:
  published: true
  views: 1234
  rating: 4.5
---

Complex content here.
"#;

    let front_matter = FrontMatterParser::parse(content).unwrap();

    assert_eq!(front_matter.get("title").unwrap(), "Complex Document");

    let tags = front_matter.get_array("tags").unwrap();
    assert_eq!(tags.len(), 3);
    assert_eq!(tags[0], "programming");
    assert_eq!(tags[1], "rust");
    assert_eq!(tags[2], "yaml");

    let author = front_matter.get_object("author").unwrap();
    assert_eq!(author.get("name").unwrap(), "John Doe");
    assert_eq!(author.get("email").unwrap(), "john@example.com");

    let social = author.get("social").unwrap().as_object().unwrap();
    assert_eq!(social.get("github").unwrap(), "johndoe");
    assert_eq!(social.get("twitter").unwrap(), "johndoe123");

    let metadata = front_matter.get_object("metadata").unwrap();
    assert_eq!(metadata.get("published").unwrap(), true);
    assert_eq!(metadata.get("views").unwrap(), 1234);
    assert_eq!(metadata.get("rating").unwrap(), 4.5);
}

#[test]
fn test_front_matter_no_front_matter() {
    let content = r#"Just plain content without any front matter.
Another line here.
"#;

    let front_matter = FrontMatterParser::parse(content).unwrap();

    assert!(front_matter.variables().is_empty());
    assert!(front_matter.raw_yaml().is_empty());
    assert!(front_matter.content().contains("Just plain content"));
}

#[test]
fn test_front_matter_empty() {
    let content = r#"---
---

Content after empty front matter.
"#;

    let front_matter = FrontMatterParser::parse(content).unwrap();

    assert!(front_matter.variables().is_empty());
    assert!(front_matter.content().contains("Content after empty front matter"));
}

#[test]
fn test_front_matter_invalid_format() {
    let content = r#"---
title: Unclosed front matter
This is missing the closing ---
"#;

    let result = FrontMatterParser::parse(content);
    assert!(result.is_err());
}

#[test]
fn test_front_matter_has_and_get() {
    let content = r#"---
key1: value1
key2: value2
---
"#;

    let front_matter = FrontMatterParser::parse(content).unwrap();

    assert!(front_matter.has("key1"));
    assert!(front_matter.has("key2"));
    assert!(!front_matter.has("key3"));

    assert_eq!(front_matter.get("key1").unwrap(), "value1");
    assert_eq!(front_matter.get("key2").unwrap(), "value2");
    assert!(front_matter.get("key3").is_none());
}

#[test]
fn test_jekyll_config_default() {
    let config = JekyllConfig::new();
    assert!(config.title.is_none());
    assert!(config.description.is_none());
    assert!(config.custom.is_empty());
}

#[test]
fn test_jekyll_config_builder() {
    let config = JekyllConfig::new()
        .with_title("My Site".to_string())
        .with_description("A test site".to_string())
        .with_author("Test Author".to_string())
        .with_url("https://example.com".to_string());

    assert_eq!(config.title, Some("My Site".to_string()));
    assert_eq!(config.description, Some("A test site".to_string()));
    assert_eq!(config.author, Some("Test Author".to_string()));
    assert_eq!(config.url, Some("https://example.com".to_string()));
}

#[test]
fn test_jekyll_config_from_yaml() {
    let yaml = r#"
title: My Jekyll Site
description: This is a Jekyll site
permalink: /:categories/:year/:month/:day/:title/
markdown: kramdown
exclude:
  - Gemfile
  - Gemfile.lock
include:
  - .htaccess
plugins:
  - jekyll-feed
  - jekyll-seo-tag
"#;

    let config = JekyllConfig::from_yaml_str(yaml).unwrap();

    assert_eq!(config.title, Some("My Jekyll Site".to_string()));
    assert_eq!(config.description, Some("This is a Jekyll site".to_string()));
    assert_eq!(config.permalink, Some("/:categories/:year/:month/:day/:title/".to_string()));
    assert_eq!(config.markdown, Some("kramdown".to_string()));
    assert_eq!(config.exclude, Some(vec!["Gemfile".to_string(), "Gemfile.lock".to_string()]));
    assert_eq!(config.include, Some(vec![".htaccess".to_string()]));
    assert_eq!(config.plugins, Some(vec!["jekyll-feed".to_string(), "jekyll-seo-tag".to_string()]));
}

#[test]
fn test_jekyll_config_merge() {
    let mut config1 =
        JekyllConfig::new().with_title("Original Title".to_string()).with_description("Original Description".to_string());
    config1.exclude = Some(vec!["file1.txt".to_string()]);

    let mut config2 = JekyllConfig::new().with_title("New Title".to_string()).with_author("New Author".to_string());
    config2.exclude = Some(vec!["file2.txt".to_string()]);

    let merged = config1.merge(&config2);

    assert_eq!(merged.title, Some("New Title".to_string()));
    assert_eq!(merged.description, Some("Original Description".to_string()));
    assert_eq!(merged.author, Some("New Author".to_string()));
    assert_eq!(merged.exclude, Some(vec!["file1.txt".to_string(), "file2.txt".to_string()]));
}

#[test]
fn test_jekyll_config_from_file() {
    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("_config.yml");

    let yaml_content = r#"
title: Test Site from File
description: This is loaded from a file
"#;

    fs::write(&config_path, yaml_content).unwrap();

    let config = JekyllConfig::from_file(&config_path).unwrap();

    assert_eq!(config.title, Some("Test Site from File".to_string()));
    assert_eq!(config.description, Some("This is loaded from a file".to_string()));
}

#[test]
fn test_jekyll_config_loader_from_dir() {
    let temp_dir = tempdir().unwrap();

    let main_config_path = temp_dir.path().join("_config.yml");
    let main_config_content = r#"
title: Main Config
description: From main config
"#;
    fs::write(&main_config_path, main_config_content).unwrap();

    let local_config_path = temp_dir.path().join("_config.local.yml");
    let local_config_content = r#"
title: Local Config
author: Local Author
"#;
    fs::write(&local_config_path, local_config_content).unwrap();

    let config = JekyllConfigLoader::load_from_dir(temp_dir.path()).unwrap();

    assert_eq!(config.title, Some("Local Config".to_string()));
    assert_eq!(config.description, Some("From main config".to_string()));
    assert_eq!(config.author, Some("Local Author".to_string()));
}

#[test]
fn test_jekyll_config_custom_fields() {
    let yaml = r#"
title: My Site
custom_field: custom_value
another_field:
  - item1
  - item2
"#;

    let config = JekyllConfig::from_yaml_str(yaml).unwrap();

    assert_eq!(config.title, Some("My Site".to_string()));
    assert_eq!(config.get_custom("custom_field"), Some(&serde_json::Value::String("custom_value".to_string())));
    assert!(config.get_custom("another_field").is_some());
}
