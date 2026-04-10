#![warn(missing_docs)]

//! Front Matter 解析器
//!
//! 解析 Jekyll 文件中的 YAML Front Matter

use crate::errors::{JekyllError, Result};
use serde_json::Value;
use oak_yaml;

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

        if trimmed.starts_with("---") {
            // YAML Front Matter
            Self::parse_yaml_front_matter(trimmed, content)
        } else if trimmed.starts_with("{") {
            // JSON Front Matter
            Self::parse_json_front_matter(trimmed, content)
        } else if trimmed.starts_with("+++") {
            // TOML Front Matter
            Self::parse_toml_front_matter(trimmed, content)
        } else {
            // No Front Matter
            Ok(FrontMatter::new(String::new(), Value::Object(serde_json::Map::new()), content.to_string()))
        }
    }

    /// 解析 YAML Front Matter
    fn parse_yaml_front_matter(trimmed: &str, original: &str) -> Result<FrontMatter> {
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
        let document_content = if yaml_end + 1 < lines.len() { lines[yaml_end + 1..].join("\n") } else { String::new() };

        let variables = if yaml_content.trim().is_empty() {
            Value::Object(serde_json::Map::new())
        } else {
            let value: Value = oak_yaml::from_str(&yaml_content).map_err(|e| JekyllError::YamlParseError(e.to_string()))?;
            value
        };

        Ok(FrontMatter::new(yaml_content, variables, document_content))
    }

    /// 解析 JSON Front Matter
    fn parse_json_front_matter(trimmed: &str, original: &str) -> Result<FrontMatter> {
        // 找到 JSON 的结束位置
        let mut brace_count = 0;
        let mut found_start = false;
        let mut json_end = 0;

        for (i, c) in trimmed.chars().enumerate() {
            if c == '{' {
                found_start = true;
                brace_count += 1;
            } else if c == '}' {
                brace_count -= 1;
                if found_start && brace_count == 0 {
                    json_end = i + 1;
                    break;
                }
            }
        }

        if json_end == 0 {
            return Err(JekyllError::InvalidFrontMatterFormat.into());
        }

        let json_content = trimmed[0..json_end].to_string();
        let document_content = if json_end < trimmed.len() {
            trimmed[json_end..].trim_start().to_string()
        } else {
            String::new()
        };

        let variables = if json_content.trim().is_empty() {
            Value::Object(serde_json::Map::new())
        } else {
            let value: Value = serde_json::from_str(&json_content).map_err(|e| JekyllError::YamlParseError(e.to_string()))?;
            value
        };

        Ok(FrontMatter::new(json_content, variables, document_content))
    }

    /// 解析 TOML Front Matter
    fn parse_toml_front_matter(trimmed: &str, original: &str) -> Result<FrontMatter> {
        let lines: Vec<&str> = trimmed.lines().collect();

        if lines.len() < 2 {
            return Err(JekyllError::InvalidFrontMatterFormat.into());
        }

        let mut toml_end = 1;
        let mut found_end = false;

        for (i, line) in lines.iter().enumerate().skip(1) {
            if line.trim() == "+++" {
                toml_end = i;
                found_end = true;
                break;
            }
        }

        if !found_end {
            return Err(JekyllError::InvalidFrontMatterFormat.into());
        }

        let toml_content = lines[1..toml_end].join("\n");
        let document_content = if toml_end + 1 < lines.len() { lines[toml_end + 1..].join("\n") } else { String::new() };

        let variables = if toml_content.trim().is_empty() {
            Value::Object(serde_json::Map::new())
        } else {
            let value: Value = oak_yaml::from_str(&toml_content).map_err(|e| JekyllError::YamlParseError(e.to_string()))?;
            value
        };

        Ok(FrontMatter::new(toml_content, variables, document_content))
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
        // 简单的实现，实际需要根据 oak_yaml 的 API 进行调整
        Ok(front_matter.raw_yaml().to_string())
    }
}
