//! 配置解析器测试

use oak_toml;
use serde_json;
use vuepress::config::types::VuePressConfig;

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
    assert!(config.markdown.is_some());
    assert!(config.plugins.unwrap().is_empty());
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

    let config: VuePressConfig = oak_toml::from_str(toml_content).unwrap();
    assert_eq!(config.base.unwrap(), "/test/");
    assert_eq!(config.lang.unwrap(), "zh-CN");
    assert_eq!(config.title.unwrap(), "Test Site");
    assert_eq!(config.description.unwrap(), "A test site");
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

    let config: VuePressConfig = serde_json::from_str(json_content).unwrap();
    assert_eq!(config.base.unwrap(), "/test/");
    assert_eq!(config.lang.unwrap(), "zh-CN");
    assert_eq!(config.title.unwrap(), "Test Site");
    assert_eq!(config.description.unwrap(), "A test site");
    assert!(config.theme.is_some());
    assert!(config.bundler.is_some());
}
