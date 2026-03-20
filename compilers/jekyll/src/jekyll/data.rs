/// 数据文件处理模块
///
/// 该模块提供 Jekyll 数据文件（YAML、JSON、CSV）的加载、解析和管理功能，
/// 支持从 `_data` 目录加载数据文件，并在模板中通过 `site.data` 访问。
use crate::errors::DataError;
use oak_yaml;
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// 数据文件格式枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataFormat {
    /// YAML 格式（.yml 或 .yaml）
    Yaml,
    /// JSON 格式（.json）
    Json,
    /// CSV 格式（.csv）
    Csv,
}

impl DataFormat {
    /// 从文件扩展名获取数据格式
    ///
    /// # Arguments
    ///
    /// * `ext` - 文件扩展名
    ///
    /// # Returns
    ///
    /// 返回对应的 `DataFormat`，如果扩展名不被支持则返回 `None`
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "yml" | "yaml" => Some(DataFormat::Yaml),
            "json" => Some(DataFormat::Json),
            "csv" => Some(DataFormat::Csv),
            _ => None,
        }
    }

    /// 获取格式的名称
    ///
    /// # Returns
    ///
    /// 返回格式名称的字符串切片
    pub fn name(&self) -> &'static str {
        match self {
            DataFormat::Yaml => "YAML",
            DataFormat::Json => "JSON",
            DataFormat::Csv => "CSV",
        }
    }
}

/// 单个数据文件
#[derive(Debug, Clone)]
pub struct DataFile {
    /// 数据文件路径
    path: PathBuf,
    /// 数据文件格式
    format: DataFormat,
    /// 解析后的内容
    content: Value,
}

impl DataFile {
    /// 创建新的数据文件
    ///
    /// # Arguments
    ///
    /// * `path` - 数据文件路径
    /// * `format` - 数据文件格式
    /// * `content` - 解析后的内容
    pub fn new(path: PathBuf, format: DataFormat, content: Value) -> Self {
        Self { path, format, content }
    }

    /// 获取数据文件路径
    ///
    /// # Returns
    ///
    /// 返回数据文件路径的引用
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// 获取数据文件格式
    ///
    /// # Returns
    ///
    /// 返回数据文件格式
    pub fn format(&self) -> DataFormat {
        self.format
    }

    /// 获取解析后的内容
    ///
    /// # Returns
    ///
    /// 返回解析后的内容的引用
    pub fn content(&self) -> &Value {
        &self.content
    }

    /// 获取解析后的内容的可变引用
    ///
    /// # Returns
    ///
    /// 返回解析后的内容的可变引用
    pub fn content_mut(&mut self) -> &mut Value {
        &mut self.content
    }
}

/// 数据管理器，负责加载和管理所有数据文件
#[derive(Debug, Clone)]
pub struct DataManager {
    /// 已加载的数据文件，键为文件名（不含扩展名），值为解析后的数据
    data: HashMap<String, Value>,
}

impl DataManager {
    /// 创建一个空的 DataManager
    ///
    /// # Returns
    ///
    /// 返回新创建的 DataManager 实例
    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }

    /// 从 _data 目录加载所有数据文件
    ///
    /// # Arguments
    ///
    /// * `data_dir` - _data 目录的路径
    ///
    /// # Errors
    ///
    /// 返回 `DataError` 如果目录无法访问、文件读取失败或解析失败
    pub fn load_from_dir<P: AsRef<Path>>(data_dir: P) -> Result<Self, DataError> {
        let mut manager = Self::new();
        manager.load_dir(data_dir.as_ref(), None)?;
        Ok(manager)
    }

    /// 递归加载目录中的数据文件
    ///
    /// # Arguments
    ///
    /// * `dir` - 要加载的目录路径
    /// * `prefix` - 当前命名空间前缀（用于子目录）
    ///
    /// # Errors
    ///
    /// 返回 `DataError` 如果目录无法访问、文件读取失败或解析失败
    fn load_dir(&mut self, dir: &Path, prefix: Option<&str>) -> Result<(), DataError> {
        if !dir.is_dir() {
            return Ok(());
        }

        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries {
                let entry = entry?;
                let path = entry.path();

                if path.is_dir() {
                    let sub_dir_name = path.file_name()
                        .and_then(|n| n.to_str())
                        .ok_or_else(|| DataError::ReadError("Invalid directory name".to_string()))?;
                    let new_prefix = match prefix {
                        Some(p) => format!("{}.{}", p, sub_dir_name),
                        None => sub_dir_name.to_string(),
                    };
                    self.load_dir(&path, Some(&new_prefix))?;
                } else if path.is_file() {
                    self.load_file(&path, prefix)?;
                }
            }
        }

        Ok(())
    }

    /// 加载单个数据文件
    ///
    /// # Arguments
    ///
    /// * `path` - 数据文件路径
    /// * `prefix` - 命名空间前缀
    ///
    /// # Errors
    ///
    /// 返回 `DataError` 如果文件读取失败或解析失败
    fn load_file(&mut self, path: &Path, prefix: Option<&str>) -> Result<(), DataError> {
        let ext = path.extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| DataError::UnsupportedFormat("No file extension".to_string()))?;

        let format = DataFormat::from_extension(ext)
            .ok_or_else(|| DataError::UnsupportedFormat(ext.to_string()))?;

        let content = Self::parse_file(path, format)?;

        let file_stem = path.file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| DataError::ReadError("Invalid file name".to_string()))?;

        let key = match prefix {
            Some(p) => format!("{}.{}", p, file_stem),
            None => file_stem.to_string(),
        };

        self.data.insert(key, content);

        Ok(())
    }

    /// 解析数据文件内容
    ///
    /// # Arguments
    ///
    /// * `path` - 数据文件路径
    /// * `format` - 数据文件格式
    ///
    /// # Errors
    ///
    /// 返回 `DataError` 如果文件读取失败或解析失败
    fn parse_file(path: &Path, format: DataFormat) -> Result<Value, DataError> {
        let content = std::fs::read_to_string(path)?;

        match format {
            DataFormat::Yaml => Self::parse_yaml(&content),
            DataFormat::Json => Self::parse_json(&content),
            DataFormat::Csv => Self::parse_csv(&content),
        }
    }

    /// 解析 YAML 内容
    ///
    /// # Arguments
    ///
    /// * `content` - YAML 格式的字符串
    ///
    /// # Errors
    ///
    /// 返回 `DataError::YamlParseError` 如果解析失败
    fn parse_yaml(content: &str) -> Result<Value, DataError> {
        oak_yaml::from_str(content)
            .map_err(|e| DataError::YamlParseError(e.to_string()))
    }

    /// 解析 JSON 内容
    ///
    /// # Arguments
    ///
    /// * `content` - JSON 格式的字符串
    ///
    /// # Errors
    ///
    /// 返回 `DataError::JsonParseError` 如果解析失败
    fn parse_json(content: &str) -> Result<Value, DataError> {
        serde_json::from_str(content)
            .map_err(|e| DataError::JsonParseError(e.to_string()))
    }

    /// 解析 CSV 内容
    ///
    /// # Arguments
    ///
    /// * `content` - CSV 格式的字符串
    ///
    /// # Errors
    ///
    /// 返回 `DataError::CsvParseError` 如果解析失败
    fn parse_csv(content: &str) -> Result<Value, DataError> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(content.as_bytes());

        let headers = reader.headers()
            .map_err(|e| DataError::CsvParseError(e.to_string()))?
            .clone();

        let mut records = Vec::new();

        for result in reader.records() {
            let record = result.map_err(|e| DataError::CsvParseError(e.to_string()))?;
            let mut obj = serde_json::Map::new();

            for (i, header) in headers.iter().enumerate() {
                if let Some(field) = record.get(i) {
                    obj.insert(header.to_string(), Value::String(field.to_string()));
                }
            }

            records.push(Value::Object(obj));
        }

        Ok(Value::Array(records))
    }

    /// 获取所有数据
    ///
    /// # Returns
    ///
    /// 返回所有数据的 HashMap 引用
    pub fn data(&self) -> &HashMap<String, Value> {
        &self.data
    }

    /// 获取指定键的数据
    ///
    /// # Arguments
    ///
    /// * `key` - 数据键
    ///
    /// # Returns
    ///
    /// 返回对应的数据值（如果存在）
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }

    /// 检查是否包含指定键的数据
    ///
    /// # Arguments
    ///
    /// * `key` - 数据键
    ///
    /// # Returns
    ///
    /// 如果存在返回 true，否则返回 false
    pub fn has(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    /// 获取数据的数量
    ///
    /// # Returns
    ///
    /// 返回数据的数量
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// 检查是否为空
    ///
    /// # Returns
    ///
    /// 如果为空返回 true，否则返回 false
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// 将所有数据转换为 JSON 对象
    ///
    /// # Returns
    ///
    /// 返回包含所有数据的 JSON 对象
    pub fn to_json_object(&self) -> Value {
        let mut obj = serde_json::Map::new();
        for (key, value) in &self.data {
            let parts: Vec<&str> = key.split('.').collect();
            Self::insert_nested(&mut obj, &parts, value.clone());
        }
        Value::Object(obj)
    }

    /// 插入嵌套值到 JSON 对象中
    ///
    /// # Arguments
    ///
    /// * `obj` - 目标 JSON 对象
    /// * `parts` - 键的部分列表
    /// * `value` - 要插入的值
    fn insert_nested(obj: &mut serde_json::Map<String, Value>, parts: &[&str], value: Value) {
        if parts.is_empty() {
            return;
        }

        let first = parts[0];
        let rest = &parts[1..];

        if rest.is_empty() {
            obj.insert(first.to_string(), value);
        } else {
            let entry = obj.entry(first.to_string()).or_insert_with(|| Value::Object(serde_json::Map::new()));
            if let Value::Object(ref mut inner_obj) = entry {
                Self::insert_nested(inner_obj, rest, value);
            }
        }
    }
}

impl Default for DataManager {
    fn default() -> Self {
        Self::new()
    }
}
