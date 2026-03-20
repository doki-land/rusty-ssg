//! 配置解析器测试

use serde_json;
use std::io::Write;
use tempfile::NamedTempFile;
use vuepress::config::{ConfigParser, VuePressConfig};

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
    assert_eq!(config.base.unwrap(), "/test/");
    assert_eq!(config.lang.unwrap(), "zh-CN");
    assert_eq!(config.title.unwrap(), "Test Site");
    assert_eq!(config.description.unwrap(), "A test site");
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
    assert_eq!(config.base.unwrap(), "/test/");
    assert_eq!(config.lang.unwrap(), "zh-CN");
    assert_eq!(config.title.unwrap(), "Test Site");
    assert_eq!(config.description.unwrap(), "A test site");
    assert!(config.theme.is_some());
    assert!(config.bundler.is_some());
}

#[test]
fn test_default_config() {
    let config = VuePressConfig::new();
    assert_eq!(config.base.unwrap(), "/");
    assert_eq!(config.lang.unwrap(), "en-US");
    assert_eq!(config.title.unwrap(), "");
    assert_eq!(config.description.unwrap(), "");
    assert!(config.head.is_none());
    assert!(config.locales.is_none());
    assert!(config.theme.is_none());
    assert!(config.bundler.is_none());
    assert_eq!(config.dest.unwrap(), ".vuepress/dist");
    assert_eq!(config.temp.unwrap(), ".vuepress/.temp");
    assert_eq!(config.cache.unwrap(), ".vuepress/.cache");
    assert_eq!(config.public.unwrap(), ".vuepress/public");
    assert_eq!(config.debug.unwrap(), false);
    assert_eq!(config.page_patterns.unwrap(), vec!["**/*.md", ".vuepress", "node_modules"]);
    assert!(config.permalink_pattern.is_none());
    assert_eq!(config.host.unwrap(), "0.0.0.0");
    assert_eq!(config.port.unwrap(), 8080);
    assert_eq!(config.open.unwrap(), false);
    assert_eq!(config.template_dev.unwrap(), "@vuepress/client/templates/dev.html");
    assert!(config.should_preload.is_some());
    assert!(config.should_prefetch.is_some());
    assert_eq!(config.template_build.unwrap(), "@vuepress/client/templates/build.html");
    assert!(config.template_build_renderer.is_none());
    assert!(config.plugins.unwrap().is_empty());
}
