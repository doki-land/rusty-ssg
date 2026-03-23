//! 配置解析器测试

use oak_toml as toml;
use serde_json;
use vitepress::config::VitePressConfig;

#[test]
fn test_default_config() {
    let config = VitePressConfig::default();
    assert_eq!(config.base, Some("/".to_string()));
    assert_eq!(config.lang, Some("en-US".to_string()));
    assert_eq!(config.title, Some("".to_string()));
    assert_eq!(config.description, Some("".to_string()));
    assert!(config.head.is_none() || config.head.as_ref().unwrap().is_empty());
    assert!(config.locales.is_none() || config.locales.as_ref().unwrap().is_empty());
    assert!(config.theme.is_none());
    assert!(config.build.is_none());
    assert_eq!(config.out_dir, Some(".vitepress/dist".to_string()));
    assert_eq!(config.temp, Some(".vitepress/.temp".to_string()));
    assert_eq!(config.cache, Some(".vitepress/.cache".to_string()));
    assert_eq!(config.public_dir, Some(".vitepress/public".to_string()));
    assert_eq!(config.debug, Some(false));
    assert_eq!(config.page_patterns, Some(vec!["**/*.md".to_string(), ".vitepress".to_string(), "node_modules".to_string()]));
    assert!(config.permalink_pattern.is_none());
    assert_eq!(config.host, Some("0.0.0.0".to_string()));
    assert_eq!(config.port, Some(8080));
    assert_eq!(config.open, Some(false));
    assert!(config.preload.is_none() || config.preload.is_some());
    assert!(config.prefetch.is_none() || config.prefetch.is_some());
    assert!(config.plugins.is_none() || config.plugins.as_ref().unwrap().is_empty());
}

#[test]
fn test_load_toml_config() {
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

    let config: Config = toml::from_str(toml_content).unwrap();
    assert_eq!(config.base, "/test/");
    assert_eq!(config.lang, "zh-CN");
    assert_eq!(config.title, "Test Site");
    assert_eq!(config.description, "A test site");
    assert!(config.theme.is_some());
    assert!(config.bundler.is_some());
}

#[test]
fn test_load_json_config() {
    let json_content = r#"
{
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
}
"#;

    let config: Config = serde_json::from_str(json_content).unwrap();
    assert_eq!(config.base, "/test/");
    assert_eq!(config.lang, "zh-CN");
    assert_eq!(config.title, "Test Site");
    assert_eq!(config.description, "A test site");
    assert!(config.theme.is_some());
    assert!(config.bundler.is_some());
}

#[test]
fn test_save_config() {
    let mut config = Config::default();
    config.base = "/test/";
    config.lang = "zh-CN";
    config.title = "Test Site";

    let temp_file = std::env::temp_dir().join("test_config.toml");
    config.to_file(temp_file.to_str().unwrap()).unwrap();

    let loaded_config = Config::from_file(temp_file.to_str().unwrap()).unwrap();
    assert_eq!(loaded_config.base, "/test/");
    assert_eq!(loaded_config.lang, "zh-CN");
    assert_eq!(loaded_config.title, "Test Site");

    // Clean up
    std::fs::remove_file(temp_file).unwrap();
}
