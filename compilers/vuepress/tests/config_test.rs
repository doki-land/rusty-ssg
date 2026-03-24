//! 配置解析器测试

use oak_toml;
use serde_json;
use std::io::Write;
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
    use tempfile::NamedTempFile;
    use vuepress::config::parser::ConfigParser;
    
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

    // 创建临时文件
    let mut temp_file = NamedTempFile::with_suffix(".toml").unwrap();
    temp_file.write_all(toml_content.as_bytes()).unwrap();
    
    // 使用 ConfigParser 解析
    let parser = ConfigParser::new(temp_file.path().to_str().unwrap());
    let config = parser.parse().unwrap();
    
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
