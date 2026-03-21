//! 配置解析器测试

use oak_toml as toml;
use serde_json;
use tempfile::NamedTempFile;
use vutex::config::{ConfigParser, VuePressConfig};

#[test]
fn test_parse_json_config() {
    // 创建临时 JSON 配置文件
    let json_content = r#"{
  "base": "/test/",
  "lang": "zh-CN",
  "title": "Test Site",
  "description": "A test site",
  "theme": {
    "name": "default"
  },
  "bundler": {
    "name": "vite"
  }
}"#;

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(json_content.as_bytes()).unwrap();
    let temp_path = temp_file.path().to_str().unwrap();

    // 解析配置文件
    let parser = ConfigParser::new(temp_path);
    let config = parser.parse().unwrap();

    // 验证配置
    assert_eq!(config.base, "/test/");
    assert_eq!(config.lang, "zh-CN");
    assert_eq!(config.title, "Test Site");
    assert_eq!(config.description, "A test site");
    assert!(config.theme.is_some());
    assert!(config.bundler.is_some());
}

#[test]
fn test_parse_toml_config() {
    // 创建临时 TOML 配置文件
    let toml_content = r#"
base = "/test/"
lang = "zh-CN"
title = "Test Site"
description = "A test site"

[theme]
name = "default"

[bundler]
name = "vite"
"#;

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(toml_content.as_bytes()).unwrap();
    let temp_path = temp_file.path().to_str().unwrap();

    // 解析配置文件
    let parser = ConfigParser::new(temp_path);
    let config = parser.parse_toml().unwrap();

    // 验证配置
    assert_eq!(config.base, "/test/");
    assert_eq!(config.lang, "zh-CN");
    assert_eq!(config.title, "Test Site");
    assert_eq!(config.description, "A test site");
    assert!(config.theme.is_some());
    assert!(config.bundler.is_some());
}

#[test]
fn test_default_config() {
    let config = VuePressConfig::new();
    assert_eq!(config.base, "/");
    assert_eq!(config.lang, "en-US");
    assert_eq!(config.title, "");
    assert_eq!(config.description, "");
    assert!(config.head.is_empty());
    assert!(config.locales.is_empty());
    assert!(config.theme.is_none());
    assert!(config.bundler.is_none());
    assert_eq!(config.dest, ".vuepress/dist");
    assert_eq!(config.temp, ".vuepress/.temp");
    assert_eq!(config.cache, ".vuepress/.cache");
    assert_eq!(config.public, ".vuepress/public");
    assert_eq!(config.debug, false);
    assert_eq!(config.page_patterns, vec!["**/*.md", ".vuepress", "node_modules"]);
    assert!(config.permalink_pattern.is_none());
    assert_eq!(config.host, "0.0.0.0");
    assert_eq!(config.port, 8080);
    assert_eq!(config.open, false);
    assert_eq!(config.template_dev, "@vuepress/client/templates/dev.html");
    assert!(config.should_preload.is_some());
    assert!(config.should_prefetch.is_some());
    assert_eq!(config.template_build, "@vuepress/client/templates/build.html");
    assert!(config.template_build_renderer.is_none());
    assert!(config.plugins.is_empty());
}
