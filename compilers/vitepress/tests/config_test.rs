//! 配置解析器测试

use serde_json;
use oak_toml as toml;
use vutex::types::config::Config;

#[test]
fn test_default_config() {
    let config = Config::default();
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
