//! 配置解析器测试

use serde_json;
use std::io::Write;
use tempfile::NamedTempFile;
use vitepress::config::VitePressConfig;

#[test]
fn test_load_json_config() {
    // 创建临时 JSON 配置文件
    let json_content = r#"{
  "base": "/test/",
  "lang": "zh-CN",
  "title": "Test Site",
  "description": "A test site",
  "theme": {
    "name": "default"
  },
  "build": {
    "outDir": ".vitepress/dist"
  }
}"#;

    let temp_file = tempfile::Builder::new().suffix(".json").tempfile().unwrap();
    temp_file.as_file().write_all(json_content.as_bytes()).unwrap();
    let temp_path = temp_file.path().to_path_buf();

    // 加载配置文件
    let config = VitePressConfig::load_from_file(&temp_path).unwrap();

    // 验证配置
    assert_eq!(config.base, Some("/test/".to_string()));
    assert_eq!(config.lang, Some("zh-CN".to_string()));
    assert_eq!(config.title, Some("Test Site".to_string()));
    assert_eq!(config.description, Some("A test site".to_string()));
    assert!(config.theme.is_some());
    assert!(config.build.is_some());
}

#[test]
fn test_default_config() {
    let config = VitePressConfig::default();
    assert_eq!(config.base, None);
    assert_eq!(config.lang, None);
    assert_eq!(config.title, None);
    assert_eq!(config.description, None);
    assert!(config.head.is_none() || config.head.as_ref().unwrap().is_empty());
    assert!(config.locales.is_none() || config.locales.as_ref().unwrap().is_empty());
    assert!(config.theme.is_none());
    assert!(config.build.is_none());
    assert_eq!(config.out_dir, None);
    assert_eq!(config.temp, None);
    assert_eq!(config.cache_dir, None);
    assert_eq!(config.public, None);
    assert_eq!(config.debug, None);
    assert!(config.page_patterns.is_none() || config.page_patterns.as_ref().unwrap().is_empty());
    assert!(config.permalink_pattern.is_none());
    assert_eq!(config.host, None);
    assert_eq!(config.port, None);
    assert_eq!(config.open, None);
    assert!(config.plugins.is_none() || config.plugins.as_ref().unwrap().is_empty());
}
