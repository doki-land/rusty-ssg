#![warn(missing_docs)]

//! Front Matter 解析器
//!
//! 解析 Jekyll 文件中的 YAML Front Matter

use crate::errors::{JekyllError, Result};
use oak_yaml;
use serde_json::Value;

/// Front Matter 解析结果
#[derive(Debug, Clone)]
pub struct FrontMatter {
    /// 原始 YAML 内容
    raw_yaml: String,
    /// 解析后的变量
    variables: Value,
    /// 文档内容（front matter 之后的内容）
    content: String,
}

impl FrontMatter {
    /// 创建新的 Front Matter
    fn new(raw_yaml: String, variables: Value, content: String) -> Self {
        Self { raw_yaml, variables, content }
    }

    /// 获取原始 YAML 内容
    pub fn raw_yaml(&self) -> &str {
        &self.raw_yaml
    }

    /// 获取所有变量
    pub fn variables(&self) -> &Value {
        &self.variables
    }

    /// 获取文档内容
    pub fn content(&self) -> &str {
        &self.content
    }

    /// 检查是否包含指定的键
    ///
    /// # Arguments
    ///
    /// * `key` - 要检查的键
    ///
    /// # Returns
    ///
    /// 如果键存在返回 true
    pub fn has(&self, key: &str) -> bool {
        self.variables.get(key).is_some()
    }

    /// 获取指定键的值
    ///
    /// # Arguments
    ///
    /// * `key` - 要获取的键
    ///
    /// # Returns
    ///
    /// 返回值，如果键不存在返回 None
    pub fn get(&self, key: &str) -> Option<Value> {
        self.variables.get(key).cloned()
    }

    /// 获取指定键的字符串值
    ///
    /// # Arguments
    ///
    /// * `key` - 要获取的键
    ///
    /// # Returns
    ///
    /// 返回字符串值，如果键不存在或不是字符串返回 None
    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.variables.get(key).and_then(|v| v.as_str())
    }

    /// 获取指定键的数组值
    ///
    /// # Arguments
    ///
    /// * `key` - 要获取的键
    ///
    /// # Returns
    ///
    /// 返回数组值，如果键不存在或不是数组返回 None
    pub fn get_array(&self, key: &str) -> Option<&Vec<Value>> {
        self.variables.get(key).and_then(|v| v.as_array())
    }

    /// 获取指定键的对象值
    ///
    /// # Arguments
    ///
    /// * `key` - 要获取的键
    ///
    /// # Returns
    ///
    /// 返回对象值，如果键不存在或不是对象返回 None
    pub fn get_object(&self, key: &str) -> Option<&serde_json::Map<String, Value>> {
        self.variables.get(key).and_then(|v| v.as_object())
    }
}

/// Front Matter 解析器
pub struct FrontMatterParser;

impl FrontMatterParser {
    /// 解析包含 Front Matter 的内容
    ///
    /// # Arguments
    ///
    /// * `content` - 要解析的内容
    ///
    /// # Returns
    ///
    /// 返回解析后的 Front Matter 或错误
    pub fn parse(content: &str) -> Result<FrontMatter> {
        let trimmed = content.trim_start();

        if !trimmed.starts_with("---") {
            return Ok(FrontMatter::new(String::new(), Value::Object(serde_json::Map::new()), content.to_string()));
        }

        let lines: Vec<&str> = trimmed.lines().collect();

        if lines.len() < 2 {
            return Err(JekyllError::InvalidFrontMatterFormat.into());
        }

        let mut yaml_end = 1;
        let mut found_end = false;

        for (i, line) in lines.iter().enumerate().skip(1) {
            if line.trim() == "---" {
                yaml_end = i;
                found_end = true;
                break;
            }
        }

        if !found_end {
            return Err(JekyllError::InvalidFrontMatterFormat.into());
        }

        let yaml_content = lines[1..yaml_end].join("\n");
        let document_content = if yaml_end + 1 < lines.len() {
            lines[yaml_end + 1..].join("\n")
        }
        else {
            String::new()
        };

        let variables = if yaml_content.trim().is_empty() {
            Value::Object(serde_json::Map::new())
        }
        else {
            oak_yaml::from_str::<serde_json::Value>(&yaml_content)
                .map_err(|e| JekyllError::YamlParseError(e.to_string()))?
        };

        Ok(FrontMatter::new(yaml_content, variables, document_content))
    }

    /// 解析文件中的 Front Matter
    ///
    /// # Arguments
    ///
    /// * `path` - 文件路径
    ///
    /// # Returns
    ///
    /// 返回解析后的 Front Matter 或错误
    pub fn parse_file<P: AsRef<std::path::Path>>(path: P) -> Result<FrontMatter> {
        let content = std::fs::read_to_string(path).map_err(JekyllError::from)?;
        Self::parse(&content)
    }

    /// 将变量转换为 JSON 字符串
    ///
    /// # Arguments
    ///
    /// * `front_matter` - Front Matter 实例
    ///
    /// # Returns
    ///
    /// 返回 JSON 字符串或错误
    pub fn to_json(front_matter: &FrontMatter) -> Result<String> {
        serde_json::to_string_pretty(&front_matter.variables).map_err(|e| JekyllError::YamlParseError(e.to_string()).into())
    }

    /// 将变量转换为 YAML 字符串
    ///
    /// # Arguments
    ///
    /// * `front_matter` - Front Matter 实例
    ///
    /// # Returns
    ///
    /// 返回 YAML 字符串或错误
    pub fn to_yaml(front_matter: &FrontMatter) -> Result<String> {
        oak_yaml::to_string(&front_matter.variables).map_err(|e| JekyllError::YamlParseError(e.to_string()).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_front_matter() {
        let content = r#"---
title: Hello World
layout: post
---

This is the content."#;

        let front_matter = FrontMatterParser::parse(content).unwrap();

        assert_eq!(front_matter.get_str("title"), Some("Hello World"));
        assert_eq!(front_matter.get_str("layout"), Some("post"));
        assert!(front_matter.content().contains("This is the content"));
    }

    #[test]
    fn test_parse_complex_front_matter() {
        let content = r#"---
title: Complex Document
tags:
  - programming
  - rust
author:
  name: John Doe
  email: john@example.com
---

Content here."#;

        let front_matter = FrontMatterParser::parse(content).unwrap();

        assert_eq!(front_matter.get_str("title"), Some("Complex Document"));

        let tags = front_matter.get_array("tags").unwrap();
        assert_eq!(tags.len(), 2);
        assert_eq!(tags[0], "programming");
        assert_eq!(tags[1], "rust");

        let author = front_matter.get_object("author").unwrap();
        assert_eq!(author.get("name").unwrap(), "John Doe");
        assert_eq!(author.get("email").unwrap(), "john@example.com");
    }

    #[test]
    fn test_parse_no_front_matter() {
        let content = "Just plain content without front matter.";

        let front_matter = FrontMatterParser::parse(content).unwrap();

        assert!(front_matter.variables().is_object());
        assert!(front_matter.variables().as_object().unwrap().is_empty());
        assert!(front_matter.content().contains("Just plain content"));
    }

    #[test]
    fn test_parse_empty_front_matter() {
        let content = r#"---
---

Content after empty front matter."#;

        let front_matter = FrontMatterParser::parse(content).unwrap();

        assert!(front_matter.variables().as_object().unwrap().is_empty());
        assert!(front_matter.content().contains("Content after empty front matter"));
    }

    #[test]
    fn test_parse_invalid_front_matter() {
        let content = r#"---
title: Unclosed front matter
This is missing closing ---"#;

        let result = FrontMatterParser::parse(content);
        assert!(result.is_err());
    }
}
