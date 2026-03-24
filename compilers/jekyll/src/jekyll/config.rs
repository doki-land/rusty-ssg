#![warn(missing_docs)]

//! Jekyll 配置模块
//!
//! 提供 Jekyll 配置文件（_config.yml）的加载和处理功能

use std::path::{Path, PathBuf};

use serde_json::{self, Value};
use oak_yaml;

use crate::errors::{JekyllError, Result};

/// Jekyll 配置
#[derive(Debug, Clone, Default)]
pub struct JekyllConfig {
    /// 站点标题
    pub title: Option<String>,
    /// 站点描述
    pub description: Option<String>,
    /// 作者
    pub author: Option<String>,
    /// 站点 URL
    pub url: Option<String>,
    /// 基础路径
    pub baseurl: Option<String>,
    /// 永久链接格式
    pub permalink: Option<String>,
    /// Markdown 处理器
    pub markdown: Option<String>,
    /// 排除的文件/目录
    pub exclude: Option<Vec<String>>,
    /// 包含的文件/目录
    pub include: Option<Vec<String>>,
    /// 数据文件
    pub data: Option<serde_json::Map<String, Value>>,
    /// 集合配置
    pub collections: Option<serde_json::Map<String, Value>>,
    /// 插件配置
    pub plugins: Option<Vec<String>>,
    /// 主题配置
    pub theme: Option<String>,
    /// 构建配置
    pub build: Option<serde_json::Map<String, Value>>,
    /// 开发配置
    pub development: Option<serde_json::Map<String, Value>>,
    /// 生产配置
    pub production: Option<serde_json::Map<String, Value>>,
    /// 时区
    pub timezone: Option<String>,
    /// 语言
    pub lang: Option<String>,
    /// 编码
    pub encoding: Option<String>,
    /// 自定义配置
    pub custom: serde_json::Map<String, Value>,
}

impl JekyllConfig {
    /// 创建新的配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置站点标题
    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    /// 设置站点描述
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// 设置作者
    pub fn with_author(mut self, author: String) -> Self {
        self.author = Some(author);
        self
    }

    /// 设置站点 URL
    pub fn with_url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }

    /// 设置基础路径
    pub fn with_baseurl(mut self, baseurl: String) -> Self {
        self.baseurl = Some(baseurl);
        self
    }

    /// 设置永久链接格式
    pub fn with_permalink(mut self, permalink: String) -> Self {
        self.permalink = Some(permalink);
        self
    }

    /// 从 YAML 字符串加载配置
    ///
    /// # Arguments
    ///
    /// * `yaml` - YAML 格式的配置字符串
    ///
    /// # Returns
    ///
    /// 返回配置或错误
    pub fn from_yaml_str(yaml: &str) -> Result<Self> {
        let value: Value = oak_yaml::from_str(yaml).map_err(|e| JekyllError::YamlParseError(e.to_string()))?;
        Self::from_json_value(value)
    }

    /// 从 JSON 值加载配置
    fn from_json_value(value: Value) -> Result<Self> {
        let mut config = Self::new();

        if let Value::Object(map) = value {
            for (key, val) in map {
                match key.as_str() {
                    "title" => {
                        if let Value::String(s) = val {
                            config.title = Some(s);
                        }
                    }
                    "description" => {
                        if let Value::String(s) = val {
                            config.description = Some(s);
                        }
                    }
                    "author" => {
                        if let Value::String(s) = val {
                            config.author = Some(s);
                        }
                    }
                    "url" => {
                        if let Value::String(s) = val {
                            config.url = Some(s);
                        }
                    }
                    "baseurl" => {
                        if let Value::String(s) = val {
                            config.baseurl = Some(s);
                        }
                    }
                    "permalink" => {
                        if let Value::String(s) = val {
                            config.permalink = Some(s);
                        }
                    }
                    "markdown" => {
                        if let Value::String(s) = val {
                            config.markdown = Some(s);
                        }
                    }
                    "exclude" => {
                        if let Value::Array(arr) = val {
                            config.exclude = Some(
                                arr.iter()
                                    .filter_map(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
                                    .collect(),
                            );
                        }
                    }
                    "include" => {
                        if let Value::Array(arr) = val {
                            config.include = Some(
                                arr.iter()
                                    .filter_map(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
                                    .collect(),
                            );
                        }
                    }
                    "collections" => {
                        if let Value::Object(obj) = val {
                            config.collections = Some(obj);
                        }
                    }
                    "plugins" => {
                        if let Value::Array(arr) = val {
                            config.plugins = Some(
                                arr.iter()
                                    .filter_map(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
                                    .collect(),
                            );
                        }
                    }
                    "theme" => {
                        if let Value::String(s) = val {
                            config.theme = Some(s);
                        }
                    }
                    "build" => {
                        if let Value::Object(obj) = val {
                            config.build = Some(obj);
                        }
                    }
                    "development" => {
                        if let Value::Object(obj) = val {
                            config.development = Some(obj);
                        }
                    }
                    "production" => {
                        if let Value::Object(obj) = val {
                            config.production = Some(obj);
                        }
                    }
                    "timezone" => {
                        if let Value::String(s) = val {
                            config.timezone = Some(s);
                        }
                    }
                    "lang" => {
                        if let Value::String(s) = val {
                            config.lang = Some(s);
                        }
                    }
                    "encoding" => {
                        if let Value::String(s) = val {
                            config.encoding = Some(s);
                        }
                    }
                    _ => {
                        config.custom.insert(key, val);
                    }
                }
            }
        }

        Ok(config)
    }

    /// 从文件加载配置
    ///
    /// # Arguments
    ///
    /// * `path` - 配置文件路径
    ///
    /// # Returns
    ///
    /// 返回配置或错误
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path).map_err(|e| JekyllError::ConfigReadError(e.to_string()))?;
        Self::from_yaml_str(&content)
    }

    /// 合并两个配置
    ///
    /// # Arguments
    ///
    /// * `other` - 要合并的配置
    ///
    /// # Returns
    ///
    /// 返回合并后的配置
    pub fn merge(&self, other: &Self) -> Self {
        let mut result = self.clone();

        if other.title.is_some() {
            result.title = other.title.clone();
        }
        if other.description.is_some() {
            result.description = other.description.clone();
        }
        if other.author.is_some() {
            result.author = other.author.clone();
        }
        if other.url.is_some() {
            result.url = other.url.clone();
        }
        if other.baseurl.is_some() {
            result.baseurl = other.baseurl.clone();
        }
        if other.permalink.is_some() {
            result.permalink = other.permalink.clone();
        }
        if other.markdown.is_some() {
            result.markdown = other.markdown.clone();
        }
        if let Some(exclude) = &other.exclude {
            result.exclude = Some(match &result.exclude {
                Some(orig) => {
                    let mut merged = orig.clone();
                    merged.extend(exclude.clone());
                    merged
                }
                None => exclude.clone(),
            });
        }
        if let Some(include) = &other.include {
            result.include = Some(match &result.include {
                Some(orig) => {
                    let mut merged = orig.clone();
                    merged.extend(include.clone());
                    merged
                }
                None => include.clone(),
            });
        }
        if other.data.is_some() {
            result.data = other.data.clone();
        }
        if other.collections.is_some() {
            result.collections = other.collections.clone();
        }
        if other.plugins.is_some() {
            result.plugins = other.plugins.clone();
        }
        if other.theme.is_some() {
            result.theme = other.theme.clone();
        }
        if other.build.is_some() {
            result.build = other.build.clone();
        }
        if other.development.is_some() {
            result.development = other.development.clone();
        }
        if other.production.is_some() {
            result.production = other.production.clone();
        }
        if other.timezone.is_some() {
            result.timezone = other.timezone.clone();
        }
        if other.lang.is_some() {
            result.lang = other.lang.clone();
        }
        if other.encoding.is_some() {
            result.encoding = other.encoding.clone();
        }

        for (key, value) in &other.custom {
            result.custom.insert(key.clone(), value.clone());
        }

        result
    }

    /// 获取自定义字段
    ///
    /// # Arguments
    ///
    /// * `key` - 字段键
    ///
    /// # Returns
    ///
    /// 返回字段值，如果不存在返回 None
    pub fn get_custom(&self, key: &str) -> Option<&Value> {
        self.custom.get(key)
    }

    /// 转换为 JSON 字符串
    pub fn to_json(&self) -> Result<String> {
        let mut map = serde_json::Map::new();

        if let Some(title) = &self.title {
            map.insert("title".to_string(), Value::String(title.clone()));
        }
        if let Some(description) = &self.description {
            map.insert("description".to_string(), Value::String(description.clone()));
        }
        if let Some(author) = &self.author {
            map.insert("author".to_string(), Value::String(author.clone()));
        }
        if let Some(url) = &self.url {
            map.insert("url".to_string(), Value::String(url.clone()));
        }
        if let Some(baseurl) = &self.baseurl {
            map.insert("baseurl".to_string(), Value::String(baseurl.clone()));
        }
        if let Some(permalink) = &self.permalink {
            map.insert("permalink".to_string(), Value::String(permalink.clone()));
        }
        if let Some(markdown) = &self.markdown {
            map.insert("markdown".to_string(), Value::String(markdown.clone()));
        }
        if let Some(exclude) = &self.exclude {
            map.insert("exclude".to_string(), Value::Array(exclude.iter().map(|s| Value::String(s.clone())).collect()));
        }
        if let Some(include) = &self.include {
            map.insert("include".to_string(), Value::Array(include.iter().map(|s| Value::String(s.clone())).collect()));
        }
        if let Some(data) = &self.data {
            map.insert("data".to_string(), Value::Object(data.clone()));
        }
        if let Some(collections) = &self.collections {
            map.insert("collections".to_string(), Value::Object(collections.clone()));
        }
        if let Some(plugins) = &self.plugins {
            map.insert("plugins".to_string(), Value::Array(plugins.iter().map(|s| Value::String(s.clone())).collect()));
        }

        for (key, value) in &self.custom {
            map.insert(key.clone(), value.clone());
        }

        serde_json::to_string_pretty(&Value::Object(map)).map_err(|e| JekyllError::YamlParseError(e.to_string()).into())
    }
}

/// Jekyll 配置加载器
pub struct JekyllConfigLoader;

impl JekyllConfigLoader {
    /// 从目录加载配置
    ///
    /// 会自动查找 _config.yml 和 _config.local.yml
    ///
    /// # Arguments
    ///
    /// * `dir` - 项目目录
    ///
    /// # Returns
    ///
    /// 返回合并后的配置或错误
    pub fn load_from_dir<P: AsRef<Path>>(dir: P) -> Result<JekyllConfig> {
        let dir = dir.as_ref();
        let config_path = dir.join("_config.yml");
        let local_config_path = dir.join("_config.local.yml");

        let mut config = if config_path.exists() { JekyllConfig::from_file(&config_path)? } else { JekyllConfig::new() };

        if local_config_path.exists() {
            let local_config = JekyllConfig::from_file(&local_config_path)?;
            config = config.merge(&local_config);
        }

        Ok(config)
    }

    /// 从文件加载配置
    ///
    /// # Arguments
    ///
    /// * `path` - 配置文件路径
    ///
    /// # Returns
    ///
    /// 返回配置或错误
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<JekyllConfig> {
        JekyllConfig::from_file(path)
    }
}
