//! 配置文件解析模块

use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde::{Deserialize, Serialize};

/// 配置错误类型
#[derive(Debug)]
pub enum ConfigError {
    /// 文件读取错误
    FileReadError(std::io::Error),
    /// 配置解析错误
    ParseError(String),
    /// 配置验证错误
    ValidationError(String),
}

/// 配置选项
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    /// 输入目录
    #[serde(default = "default_input_dir")]
    pub input_dir: String,
    
    /// 输出目录
    #[serde(default = "default_output_dir")]
    pub output_dir: String,
    
    /// 模板目录
    #[serde(default = "default_template_dir")]
    pub template_dir: String,
    
    /// 数据目录
    #[serde(default = "default_data_dir")]
    pub data_dir: String,
    
    /// 插件配置
    #[serde(default = "default_plugins")]
    pub plugins: Vec<PluginConfig>,
    
    /// 全局数据
    #[serde(default = "default_global_data")]
    pub global_data: serde_json::Value,
    
    /// 过滤器配置
    #[serde(default = "default_filters")]
    pub filters: serde_json::Value,
    
    /// 集合配置
    #[serde(default = "default_collections")]
    pub collections: serde_json::Value,
}

/// 插件配置
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PluginConfig {
    /// 插件名称
    pub name: String,
    
    /// 插件选项
    pub options: serde_json::Value,
}

/// 默认输入目录
fn default_input_dir() -> String {
    ".".to_string()
}

/// 默认输出目录
fn default_output_dir() -> String {
    "_site".to_string()
}

/// 默认模板目录
fn default_template_dir() -> String {
    "_includes".to_string()
}

/// 默认数据目录
fn default_data_dir() -> String {
    "_data".to_string()
}

/// 默认插件列表
fn default_plugins() -> Vec<PluginConfig> {
    Vec::new()
}

/// 默认全局数据
fn default_global_data() -> serde_json::Value {
    serde_json::Value::Object(serde_json::Map::new())
}

/// 默认过滤器
fn default_filters() -> serde_json::Value {
    serde_json::Value::Object(serde_json::Map::new())
}

/// 默认集合
fn default_collections() -> serde_json::Value {
    serde_json::Value::Object(serde_json::Map::new())
}

impl Config {
    /// 从文件加载配置
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let path = path.as_ref();
        let extension = path.extension().unwrap_or_default().to_string_lossy().to_lowercase();
        
        let mut file = File::open(path).map_err(ConfigError::FileReadError)?;
        let mut content = String::new();
        file.read_to_string(&mut content).map_err(ConfigError::FileReadError)?;
        
        match extension.as_str() {
            "json" => Self::from_json(&content),
            "js" => Self::from_js(&content),
            _ => Self::from_string(&content),
        }
    }
    
    /// 从字符串加载配置
    pub fn from_string(content: &str) -> Result<Self, ConfigError> {
        // 尝试解析为 JSON
        if let Ok(config) = Self::from_json(content) {
            return Ok(config);
        }
        
        // 尝试解析为 JavaScript
        if let Ok(config) = Self::from_js(content) {
            return Ok(config);
        }
        
        // 返回默认配置
        Ok(Self::default())
    }
    
    /// 从 JSON 字符串加载配置
    pub fn from_json(content: &str) -> Result<Self, ConfigError> {
        let mut config: Self = serde_json::from_str(content).map_err(|e| ConfigError::ParseError(e.to_string()))?;
        // 验证配置
        config.validate()?;
        Ok(config)
    }
    
    /// 从 JavaScript 字符串加载配置
    pub fn from_js(content: &str) -> Result<Self, ConfigError> {
        // 这里需要实现 JavaScript 配置文件的解析
        // 暂时返回默认配置
        Ok(Self::default())
    }
    
    /// 获取默认配置
    pub fn default() -> Self {
        Self {
            input_dir: default_input_dir(),
            output_dir: default_output_dir(),
            template_dir: default_template_dir(),
            data_dir: default_data_dir(),
            plugins: default_plugins(),
            global_data: default_global_data(),
            filters: default_filters(),
            collections: default_collections(),
        }
    }
    
    /// 验证配置
    pub fn validate(&self) -> Result<(), ConfigError> {
        // 验证配置的有效性
        Ok(())
    }
}