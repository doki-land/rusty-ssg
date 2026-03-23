//! 配置解析器测试
//! 测试 VitePress 配置解析器的功能

use vitepress::{types::config::{ConfigError, ConfigValidation, VitePressConfig}, types::VitePressError};
use std::fs;
use std::path::PathBuf;

#[test]
fn test_config_parser_toml() {
    // 创建临时 TOML 配置文件
    let config_content = r#"# VitePress 配置文件

title = "测试站点"
description = "这是一个测试站点"
base = "/test/"

[theme]
name = "default"

[build]
out_dir = "dist"
"#;

    let temp_path = PathBuf::from(".vitepress").join("vitepress.config.toml");

    // 创建目录并写入配置文件
    if let Some(parent) = temp_path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(&temp_path, config_content).unwrap();

    // 解析配置文件
    let result = VitePressConfig::load_from_file(temp_path);

    // 检查解析结果
    println!("TOML 解析结果: {:?}", result);
    assert!(result.is_ok());
    let config = result.unwrap();
    println!("TOML 配置内容: {:?}", config);
    assert_eq!(config.title, Some("测试站点".to_string()));
    assert_eq!(config.description, Some("这是一个测试站点".to_string()));
    assert_eq!(config.base, Some("/test/".to_string()));

    // 清理临时文件
    fs::remove_file(".vitepress/vitepress.config.toml").unwrap();

    println!("TOML 配置解析测试通过！");
}

#[test]
fn test_config_parser_json() {
    // 创建临时 JSON 配置文件
    let config_content = r#"{
  "title": "测试站点",
  "description": "这是一个测试站点",
  "base": "/test/",
  "theme": {
    "name": "default"
  },
  "build": {
    "out_dir": "dist"
  }
}"#;

    let temp_path = PathBuf::from(".vitepress").join("vitepress.config.json");

    // 创建目录并写入配置文件
    if let Some(parent) = temp_path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(&temp_path, config_content).unwrap();

    // 解析配置文件
    let result = VitePressConfig::load_from_file(temp_path);

    // 检查解析结果
    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.title, Some("测试站点".to_string()));
    assert_eq!(config.description, Some("这是一个测试站点".to_string()));
    assert_eq!(config.base, Some("/test/".to_string()));

    // 清理临时文件
    fs::remove_file(".vitepress/vitepress.config.json").unwrap();

    println!("JSON 配置解析测试通过！");
}

#[test]
fn test_config_parser_validation() {
    // 创建一个有效的配置
    let config = VitePressConfig::new()
        .with_title("测试站点".to_string())
        .with_description("这是一个测试站点".to_string())
        .with_base("/test/".to_string());

    // 验证配置
    let result = config.validate();
    assert!(result.is_ok());

    println!("配置验证测试通过！");
}
