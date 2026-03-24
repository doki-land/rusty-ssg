//! 配置测试
//! 
//! 测试 MkDocs 配置加载和处理功能

use mkdocs::MkDocsConfig;
use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn test_load_config() {
    // 创建临时目录
    let temp_dir = tempdir().unwrap();
    let config_file = temp_dir.path().join("mkdocs.yml");

    // 创建测试配置文件
    let config_content = r#"
site_name: Test Site
site_description: A test site
docs_dir: docs
site_dir: site

nav:
  - Home: index.md
  - About: about.md

plugins:
  - search
  - navigation
"#;
    fs::write(&config_file, config_content).unwrap();

    // 加载配置
    let result = MkDocsConfig::load_from_file(&config_file);
    assert!(result.is_ok());
    let config = result.unwrap();

    // 验证配置内容
    assert_eq!(config.site_name, "Test Site");
    assert_eq!(config.site_description, Some("A test site".to_string()));
    assert_eq!(config.docs_dir, "docs");
    assert_eq!(config.site_dir, "site");
    assert_eq!(config.nav.len(), 2);
    assert_eq!(config.plugins.len(), 2);
}

#[test]
fn test_load_config_from_dir() {
    // 创建临时目录
    let temp_dir = tempdir().unwrap();
    let config_file = temp_dir.path().join("mkdocs.yml");

    // 创建测试配置文件
    let config_content = r#"
site_name: Test Site
docs_dir: docs
site_dir: site
"#;
    fs::write(&config_file, config_content).unwrap();

    // 从目录加载配置
    let result = MkDocsConfig::load_from_dir(&temp_dir.path().to_path_buf());
    assert!(result.is_ok());
    let config = result.unwrap();

    // 验证配置内容
    assert_eq!(config.site_name, "Test Site");
    assert_eq!(config.docs_dir, "docs");
    assert_eq!(config.site_dir, "site");
}

#[test]
fn test_default_config() {
    // 创建默认配置
    let config = MkDocsConfig::new();

    // 验证默认值
    assert_eq!(config.docs_dir, "docs");
    assert_eq!(config.site_dir, "site");
    assert_eq!(config.dev_addr, "127.0.0.1:8000");
    assert_eq!(config.remote_branch, "gh-pages");
    assert_eq!(config.remote_name, "origin");
    assert!(config.use_directory_urls);
}

#[test]
fn test_config_validation() {
    // 创建临时目录
    let temp_dir = tempdir().unwrap();
    let config_file = temp_dir.path().join("mkdocs.yml");

    // 创建无效配置文件（缺少 site_name）
    let config_content = r#"
docs_dir: docs
site_dir: site
"#;
    fs::write(&config_file, config_content).unwrap();

    // 加载配置
    let result = MkDocsConfig::load_from_file(&config_file);
    assert!(result.is_err());
}
