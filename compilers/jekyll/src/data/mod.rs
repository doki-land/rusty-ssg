#![warn(missing_docs)]

//! 数据文件处理模块
//! 
//! 提供 Jekyll 数据文件的加载和处理功能

use std::path::{Path, PathBuf};

use serde_json::{self, Value};
use csv;
use oak_yaml;

use crate::errors::{DataError, Result};

/// 数据文件处理器
pub struct DataProcessor {
    /// 数据文件目录
    data_dir: PathBuf,
    /// 加载的数据
    data: serde_json::Map<String, Value>,
}

impl DataProcessor {
    /// 创建新的数据文件处理器
    ///
    /// # Arguments
    ///
    /// * `data_dir` - 数据文件目录
    ///
    /// # Returns
    ///
    /// 返回数据文件处理器实例
    pub fn new(data_dir: PathBuf) -> Self {
        Self { data_dir, data: serde_json::Map::new() }
    }

    /// 加载所有数据文件
    ///
    /// # Returns
    ///
    /// 返回加载的数据或错误
    pub fn load_data(&mut self) -> Result<serde_json::Map<String, Value>> {
        if !self.data_dir.exists() {
            return Ok(self.data.clone());
        }

        self.data.clear();
        self.load_data_recursive(&self.data_dir, "")?;

        Ok(self.data.clone())
    }

    /// 递归加载数据文件
    fn load_data_recursive(&mut self, dir: &Path, prefix: &str) -> Result<()>
    {
        for entry in std::fs::read_dir(dir).map_err(DataError::from)? {
            let entry = entry.map_err(DataError::from)?;
            let path = entry.path();

            if path.is_dir() {
                let dir_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
                let new_prefix = if prefix.is_empty() {
                    dir_name.to_string()
                } else {
                    format!("{}.{}", prefix, dir_name)
                };
                self.load_data_recursive(&path, &new_prefix)?;
            } else if path.is_file() {
                self.load_data_file(&path, prefix)?;
            }
        }

        Ok(())
    }

    /// 加载单个数据文件
    fn load_data_file(&mut self, path: &Path, prefix: &str) -> Result<()>
    {
        let file_name = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        let key = if prefix.is_empty() {
            file_name.to_string()
        } else {
            format!("{}.{}", prefix, file_name)
        };

        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        let content = std::fs::read_to_string(path).map_err(|e| DataError::ReadError(e.to_string()))?;

        let value = match extension {
            "yaml" | "yml" => {
                // 使用 oak_yaml 解析 YAML 文件
                let yaml_root = oak_yaml::parse(&content).map_err(|e| DataError::YamlParseError(e))?;
                Self::yaml_to_json(&yaml_root)
            },
            "json" => {
                // 解析 JSON 文件
                serde_json::from_str(&content).map_err(|e| DataError::JsonParseError(e.to_string()))?
            },
            "csv" => {
                // 解析 CSV 文件
                Self::parse_csv(&content)?
            },
            _ => {
                return Err(DataError::UnsupportedFormat(extension.to_string()));
            }
        };

        self.data.insert(key, value);

        Ok(())
    }

    /// 解析 CSV 文件
    fn parse_csv(content: &str) -> Result<Value> {
        let mut reader = csv::Reader::from_reader(content.as_bytes());
        let headers = reader.headers().map_err(|e| DataError::CsvParseError(e.to_string()))?;

        let mut records = Vec::new();

        for result in reader.records() {
            let record = result.map_err(|e| DataError::CsvParseError(e.to_string()))?;
            let mut record_map = serde_json::Map::new();

            for (i, header) in headers.iter().enumerate() {
                if let Some(value) = record.get(i) {
                    record_map.insert(header.to_string(), Value::String(value.to_string()));
                }
            }

            records.push(Value::Object(record_map));
        }

        Ok(Value::Array(records))
    }

    /// 将 YAML AST 转换为 JSON Value
    fn yaml_to_json(yaml_root: &oak_yaml::ast::YamlRoot) -> Value {
        // 简单的转换实现
        // 实际实现需要根据 YAML AST 结构进行更详细的处理
        Value::Object(serde_json::Map::new())
    }

    /// 获取加载的数据
    pub fn data(&self) -> &serde_json::Map<String, Value> {
        &self.data
    }

    /// 获取指定键的数据
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}
