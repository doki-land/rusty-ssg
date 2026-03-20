//! 独立的配置解析器测试

use std::{fs::File, io::Write, path::Path};

use vuepress::config::{ConfigParser, VuePressConfig};

fn main() {
    // 测试默认配置
    println!("=== 测试默认配置 ===");
    let default_config = VuePressConfig::new();
    println!("默认配置: {:?}", default_config);
    println!();

    // 测试 JSON 配置解析
    println!("=== 测试 JSON 配置解析 ===");
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

    let json_path = "test_config.json";
    let mut json_file = File::create(json_path).unwrap();
    json_file.write_all(json_content.as_bytes()).unwrap();

    let json_parser = ConfigParser::new(json_path);
    match json_parser.parse() {
        Ok(config) => println!("JSON 配置解析成功: {:?}", config),
        Err(e) => println!("JSON 配置解析失败: {:?}", e),
    }
    println!();

    // 测试 TOML 配置解析
    println!("=== 测试 TOML 配置解析 ===");
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

    let toml_path = "test_config.toml";
    let mut toml_file = File::create(toml_path).unwrap();
    toml_file.write_all(toml_content.as_bytes()).unwrap();

    let toml_parser = ConfigParser::new(toml_path);
    match toml_parser.parse() {
        Ok(config) => println!("TOML 配置解析成功: {:?}", config),
        Err(e) => println!("TOML 配置解析失败: {:?}", e),
    }
    println!();

    // 清理临时文件
    std::fs::remove_file(json_path).unwrap();
    std::fs::remove_file(toml_path).unwrap();

    println!("测试完成!");
}
