//! 配置模块
//! 提供 Gatsby 配置的加载、解析和验证功能

use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    path::Path,
};

use serde::{Deserialize, Serialize};

/// 配置加载和验证相关的错误类型
#[derive(Debug, Clone)]
pub enum ConfigError {
    /// 文件读取错误
    FileReadError {
        /// 错误原因
        cause: String,
    },

    /// JSON 解析错误
    JsonParseError {
        /// 错误原因
        cause: String,
    },

    /// YAML 解析错误
    YamlParseError {
        /// 错误原因
        cause: String,
    },

    /// TOML 解析错误
    TomlParseError {
        /// 错误原因
        cause: String,
    },

    /// JavaScript 配置错误
    JavaScriptConfigError {
        /// 错误原因
        cause: String,
    },

    /// 配置验证错误
    ValidationError {
        /// 错误原因
        cause: String,
    },

    /// 不支持的配置文件格式
    UnsupportedFormat {
        /// 格式名称
        format: String,
    },

    /// 配置缺失错误
    MissingConfig {
        /// 缺失的配置项
        field: String,
    },
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::FileReadError { cause } => write!(f, "Failed to read config file: {}", cause),
            ConfigError::JsonParseError { cause } => write!(f, "Failed to parse JSON config: {}", cause),
            ConfigError::YamlParseError { cause } => write!(f, "Failed to parse YAML config: {}", cause),
            ConfigError::TomlParseError { cause } => write!(f, "Failed to parse TOML config: {}", cause),
            ConfigError::JavaScriptConfigError { cause } => write!(f, "Failed to parse JavaScript config: {}", cause),
            ConfigError::ValidationError { cause } => write!(f, "Config validation error: {}", cause),
            ConfigError::UnsupportedFormat { format } => write!(f, "Unsupported config file format: {}", format),
            ConfigError::MissingConfig { field } => write!(f, "Missing required config field: {}", field),
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<std::io::Error> for ConfigError {
    fn from(error: std::io::Error) -> Self {
        ConfigError::FileReadError { cause: error.to_string() }
    }
}

impl ConfigError {
    /// 获取错误的 i18n key
    pub fn i18n_key(&self) -> &'static str {
        match self {
            ConfigError::FileReadError { .. } => "gatsby.error.config.file_read",
            ConfigError::JsonParseError { .. } => "gatsby.error.config.json_parse",
            ConfigError::YamlParseError { .. } => "gatsby.error.config.yaml_parse",
            ConfigError::TomlParseError { .. } => "gatsby.error.config.toml_parse",
            ConfigError::JavaScriptConfigError { .. } => "gatsby.error.config.javascript_parse",
            ConfigError::ValidationError { .. } => "gatsby.error.config.validation",
            ConfigError::UnsupportedFormat { .. } => "gatsby.error.config.unsupported_format",
            ConfigError::MissingConfig { .. } => "gatsby.error.config.missing",
        }
    }

    /// 获取错误参数
    pub fn params(&self) -> Vec<(&'static str, String)> {
        match self {
            ConfigError::FileReadError { cause } => vec![("cause", cause.clone())],
            ConfigError::JsonParseError { cause } => vec![("cause", cause.clone())],
            ConfigError::YamlParseError { cause } => vec![("cause", cause.clone())],
            ConfigError::TomlParseError { cause } => vec![("cause", cause.clone())],
            ConfigError::JavaScriptConfigError { cause } => vec![("cause", cause.clone())],
            ConfigError::ValidationError { cause } => vec![("cause", cause.clone())],
            ConfigError::UnsupportedFormat { format } => vec![("format", format.clone())],
            ConfigError::MissingConfig { field } => vec![("field", field.clone())],
        }
    }

    /// 创建 JSON 解析错误
    pub fn json_parse_error(cause: String) -> Self {
        ConfigError::JsonParseError { cause }
    }

    /// 创建 YAML 解析错误
    pub fn yaml_parse_error(cause: String) -> Self {
        ConfigError::YamlParseError { cause }
    }

    /// 创建 TOML 解析错误
    pub fn toml_parse_error(cause: String) -> Self {
        ConfigError::TomlParseError { cause }
    }

    /// 创建 JavaScript 配置错误
    pub fn javascript_config_error(cause: String) -> Self {
        ConfigError::JavaScriptConfigError { cause }
    }

    /// 创建验证错误
    pub fn validation_error(cause: String) -> Self {
        ConfigError::ValidationError { cause }
    }

    /// 创建不支持的格式错误
    pub fn unsupported_format(format: String) -> Self {
        ConfigError::UnsupportedFormat { format }
    }

    /// 创建配置缺失错误
    pub fn missing_config(field: String) -> Self {
        ConfigError::MissingConfig { field }
    }
}

/// 配置验证 trait
pub trait ConfigValidation {
    /// 验证配置的有效性
    ///
    /// # Errors
    ///
    /// 返回 `ConfigError::ValidationError` 如果配置无效
    fn validate(&self) -> Result<(), ConfigError>;
}

/// Gatsby 主配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GatsbyConfig {
    /// 站点元数据
    #[serde(rename = "siteMetadata")]
    pub site_metadata: Option<SiteMetadata>,

    /// 路径前缀
    #[serde(rename = "pathPrefix")]
    pub path_prefix: Option<String>,

    /// 插件配置
    pub plugins: Option<Vec<PluginConfig>>,

    /// 是否启用 polyfill
    pub polyfill: Option<bool>,

    /// 是否启用映射
    pub mapping: Option<HashMap<String, String>>,

    /// 开发服务器配置
    #[serde(rename = "developMiddleware")]
    pub develop_middleware: Option<Vec<String>>,

    /// 代理配置
    pub proxy: Option<Vec<ProxyConfig>>,

    /// 开发服务器配置
    pub develop: Option<DevelopConfig>,

    /// 构建配置
    pub flags: Option<FlagsConfig>,

    /// 图表配置
    #[serde(rename = "graphqlTypegen")]
    pub graphql_typegen: Option<GraphQLTypegenConfig>,

    /// 适配器配置
    pub adapter: Option<AdapterConfig>,

    /// JSX 运行时
    #[serde(rename = "jsxRuntime")]
    pub jsx_runtime: Option<String>,

    /// 部分 hydration 配置
    #[serde(rename = "trailingSlash")]
    pub trailing_slash: Option<TrailingSlash>,

    /// 输出目录
    pub public: Option<String>,

    /// 缓存目录
    pub cache: Option<String>,
}

impl GatsbyConfig {
    /// 从文件加载配置，根据文件扩展名自动选择解析器
    ///
    /// # Arguments
    ///
    /// * `path` - 配置文件的路径
    ///
    /// # Errors
    ///
    /// 返回 `ConfigError` 如果文件读取或解析失败
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)?;

        match path.extension().and_then(|ext| ext.to_str()) {
            Some("js") | Some("mjs") | Some("cjs") => Self::load_from_javascript(&content, path),
            Some("json") => Self::load_from_json_str(&content),
            Some("yaml") | Some("yml") => Self::load_from_yaml_str(&content),
            Some("toml") => Self::load_from_toml_str(&content),
            Some(ext) => Err(ConfigError::unsupported_format(ext.to_string())),
            None => Err(ConfigError::unsupported_format("no extension".to_string())),
        }
    }

    /// 从 JavaScript 配置文件加载
    ///
    /// # Arguments
    ///
    /// * `content` - JavaScript 文件内容
    /// * `path` - 配置文件路径
    ///
    /// # Errors
    ///
    /// 返回 `ConfigError::JavaScriptConfigError` 如果解析失败
    pub fn load_from_javascript(content: &str, path: &Path) -> Result<Self, ConfigError> {
        let config = Self::parse_javascript_config(content)?;
        config.validate()?;
        Ok(config)
    }

    /// 解析 JavaScript 配置
    fn parse_javascript_config(content: &str) -> Result<Self, ConfigError> {
        let module_exports_pattern = regex::Regex::new(r"module\.exports\s*=\s*")
            .map_err(|e| ConfigError::javascript_config_error(format!("Failed to compile regex: {}", e)))?;

        let content = module_exports_pattern.replace(content, "");

        let content = content.trim();
        let content = content.trim_end_matches(';').trim();

        if content.starts_with('{') && content.ends_with('}') {
            Self::load_from_json_str(content)
        }
        else {
            Err(ConfigError::javascript_config_error(
                "JavaScript config must export an object. Dynamic JavaScript is not supported in static mode.".to_string(),
            ))
        }
    }

    /// 从 JSON 字符串加载配置
    ///
    /// # Arguments
    ///
    /// * `json_str` - JSON 格式的配置字符串
    ///
    /// # Errors
    ///
    /// 返回 `ConfigError::JsonParseError` 如果 JSON 解析失败
    pub fn load_from_json_str(json_str: &str) -> Result<Self, ConfigError> {
        let config: Self = oak_json::from_str(json_str).map_err(|e| ConfigError::json_parse_error(e.to_string()))?;
        config.validate()?;
        Ok(config)
    }

    /// 从 YAML 字符串加载配置
    ///
    /// # Arguments
    ///
    /// * `yaml_str` - YAML 格式的配置字符串
    ///
    /// # Errors
    ///
    /// 返回 `ConfigError::YamlParseError` 如果 YAML 解析失败
    pub fn load_from_yaml_str(yaml_str: &str) -> Result<Self, ConfigError> {
        let config: Self = oak_yaml::language::from_str(yaml_str).map_err(|e| ConfigError::yaml_parse_error(e.to_string()))?;
        config.validate()?;
        Ok(config)
    }

    /// 从 TOML 字符串加载配置
    ///
    /// # Arguments
    ///
    /// * `toml_str` - TOML 格式的配置字符串
    ///
    /// # Errors
    ///
    /// 返回 `ConfigError::TomlParseError` 如果 TOML 解析失败
    pub fn load_from_toml_str(toml_str: &str) -> Result<Self, ConfigError> {
        let config: Self = oak_toml::from_str(toml_str).map_err(|e| ConfigError::toml_parse_error(e.to_string()))?;
        config.validate()?;
        Ok(config)
    }

    /// 从目录中查找并加载配置文件
    ///
    /// 按以下顺序查找配置文件：
    /// 1. gatsby-config.js
    /// 2. gatsby-config.mjs
    /// 3. gatsby-config.cjs
    /// 4. gatsby-config.json
    /// 5. gatsby-config.yaml
    /// 6. gatsby-config.yml
    /// 7. gatsby-config.toml
    ///
    /// # Arguments
    ///
    /// * `dir` - 要搜索的目录路径
    ///
    /// # Errors
    ///
    /// 返回 `ConfigError` 如果配置文件读取或解析失败
    pub fn load_from_dir<P: AsRef<Path>>(dir: P) -> Result<Self, ConfigError> {
        let dir = dir.as_ref();

        let filenames = [
            "gatsby-config.js",
            "gatsby-config.mjs",
            "gatsby-config.cjs",
            "gatsby-config.json",
            "gatsby-config.yaml",
            "gatsby-config.yml",
            "gatsby-config.toml",
        ];

        for filename in filenames {
            let path = dir.join(filename);
            if path.exists() {
                return Self::load_from_file(path);
            }
        }

        Ok(Self::default())
    }

    /// 将配置序列化为 JSON 字符串
    ///
    /// # Errors
    ///
    /// 返回 `ConfigError::JsonParseError` 如果序列化失败
    pub fn to_json(&self) -> Result<String, ConfigError> {
        serde_json::to_string(self).map_err(|e| ConfigError::json_parse_error(e.to_string()))
    }

    /// 将配置序列化为美化的 JSON 字符串
    ///
    /// # Errors
    ///
    /// 返回 `ConfigError::JsonParseError` 如果序列化失败
    pub fn to_json_pretty(&self) -> Result<String, ConfigError> {
        serde_json::to_string_pretty(self).map_err(|e| ConfigError::json_parse_error(e.to_string()))
    }

    /// 将配置序列化为 YAML 字符串
    ///
    /// # Errors
    ///
    /// 返回 `ConfigError::YamlParseError` 如果序列化失败
    pub fn to_yaml(&self) -> Result<String, ConfigError> {
        oak_yaml::language::to_string(self).map_err(|e| ConfigError::yaml_parse_error(e.to_string()))
    }

    /// 将配置序列化为 TOML 字符串
    ///
    /// # Errors
    ///
    /// 返回 `ConfigError::TomlParseError` 如果序列化失败
    pub fn to_toml(&self) -> Result<String, ConfigError> {
        oak_toml::language::to_string(self).map_err(|e| ConfigError::toml_parse_error(e.to_string()))
    }

    /// 创建新的 Gatsby 配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置站点元数据
    pub fn with_site_metadata(mut self, site_metadata: SiteMetadata) -> Self {
        self.site_metadata = Some(site_metadata);
        self
    }

    /// 设置路径前缀
    pub fn with_path_prefix(mut self, path_prefix: String) -> Self {
        self.path_prefix = Some(path_prefix);
        self
    }

    /// 添加插件
    pub fn with_plugin(mut self, plugin: PluginConfig) -> Self {
        let mut plugins = self.plugins.unwrap_or_default();
        plugins.push(plugin);
        self.plugins = Some(plugins);
        self
    }

    /// 获取站点标题
    pub fn site_title(&self) -> &str {
        self.site_metadata.as_ref().and_then(|m| m.title.as_deref()).unwrap_or("Gatsby Site")
    }

    /// 获取站点描述
    pub fn site_description(&self) -> Option<&str> {
        self.site_metadata.as_ref().and_then(|m| m.description.as_deref())
    }

    /// 获取站点 URL
    pub fn site_url(&self) -> Option<&str> {
        self.site_metadata.as_ref().and_then(|m| m.site_url.as_deref())
    }

    /// 获取输出目录
    pub fn output_dir(&self) -> &str {
        self.public.as_deref().unwrap_or("public")
    }

    /// 获取缓存目录
    pub fn cache_dir(&self) -> &str {
        self.cache.as_deref().unwrap_or(".cache")
    }

    /// 获取所有插件
    pub fn get_plugins(&self) -> Vec<&PluginConfig> {
        self.plugins.as_ref().map(|p| p.iter().collect()).unwrap_or_default()
    }

    /// 检查是否有指定名称的插件
    pub fn has_plugin(&self, name: &str) -> bool {
        self.plugins.as_ref().map(|p| p.iter().any(|plugin| plugin.name() == name)).unwrap_or(false)
    }

    /// 获取指定插件的配置
    pub fn get_plugin_options(&self, name: &str) -> Option<&HashMap<String, serde_json::Value>> {
        self.plugins
            .as_ref()
            .and_then(|plugins| plugins.iter().find_map(|plugin| if plugin.name() == name { plugin.options() } else { None }))
    }
}

impl ConfigValidation for GatsbyConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        if let Some(path_prefix) = &self.path_prefix {
            if path_prefix.is_empty() {
                return Err(ConfigError::validation_error("Path prefix cannot be empty".to_string()));
            }
            if !path_prefix.starts_with('/') {
                return Err(ConfigError::validation_error("Path prefix must start with a forward slash".to_string()));
            }
        }

        if let Some(plugins) = &self.plugins {
            for (i, plugin) in plugins.iter().enumerate() {
                plugin.validate().map_err(|e| ConfigError::validation_error(format!("Plugin at index {}: {}", i, e)))?;
            }
        }

        if let Some(develop) = &self.develop {
            develop.validate()?;
        }

        if let Some(flags) = &self.flags {
            flags.validate()?;
        }

        if let Some(site_metadata) = &self.site_metadata {
            site_metadata.validate()?;
        }

        Ok(())
    }
}

/// 站点元数据
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SiteMetadata {
    /// 站点标题
    pub title: Option<String>,

    /// 站点描述
    pub description: Option<String>,

    /// 站点作者
    pub author: Option<String>,

    /// 站点 URL
    #[serde(rename = "siteUrl")]
    pub site_url: Option<String>,

    /// 社交媒体链接
    pub social: Option<Vec<SocialLink>>,

    /// 站点语言
    pub lang: Option<String>,

    /// 自定义元数据
    #[serde(flatten)]
    pub custom: HashMap<String, serde_json::Value>,
}

impl SiteMetadata {
    /// 创建新的站点元数据
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

    /// 设置站点作者
    pub fn with_author(mut self, author: String) -> Self {
        self.author = Some(author);
        self
    }

    /// 设置站点 URL
    pub fn with_site_url(mut self, site_url: String) -> Self {
        self.site_url = Some(site_url);
        self
    }

    /// 设置站点语言
    pub fn with_lang(mut self, lang: String) -> Self {
        self.lang = Some(lang);
        self
    }

    /// 添加自定义元数据
    pub fn with_custom(mut self, key: String, value: serde_json::Value) -> Self {
        self.custom.insert(key, value);
        self
    }
}

impl ConfigValidation for SiteMetadata {
    fn validate(&self) -> Result<(), ConfigError> {
        if let Some(site_url) = &self.site_url {
            if site_url.is_empty() {
                return Err(ConfigError::validation_error("Site URL cannot be empty if provided".to_string()));
            }
        }
        Ok(())
    }
}

/// 社交链接
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SocialLink {
    /// 平台名称
    pub name: String,

    /// 链接 URL
    pub url: String,
}

impl SocialLink {
    /// 创建新的社交链接
    pub fn new(name: String, url: String) -> Self {
        Self { name, url }
    }
}

/// 插件配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PluginConfig {
    /// 简单插件名称
    Name(String),

    /// 带选项的插件
    WithOptions {
        /// 插件名称
        resolve: String,

        /// 插件选项
        options: Option<HashMap<String, serde_json::Value>>,
    },
}

impl PluginConfig {
    /// 获取插件名称
    pub fn name(&self) -> &str {
        match self {
            PluginConfig::Name(name) => name,
            PluginConfig::WithOptions { resolve, .. } => resolve,
        }
    }

    /// 获取插件选项
    pub fn options(&self) -> Option<&HashMap<String, serde_json::Value>> {
        match self {
            PluginConfig::Name(_) => None,
            PluginConfig::WithOptions { options, .. } => options.as_ref(),
        }
    }

    /// 创建简单插件配置
    pub fn simple(name: String) -> Self {
        PluginConfig::Name(name)
    }

    /// 创建带选项的插件配置
    pub fn with_options(resolve: String, options: HashMap<String, serde_json::Value>) -> Self {
        PluginConfig::WithOptions { resolve, options: Some(options) }
    }
}

impl ConfigValidation for PluginConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        match self {
            PluginConfig::Name(name) => {
                if name.is_empty() {
                    return Err(ConfigError::validation_error("Plugin name cannot be empty".to_string()));
                }
                Ok(())
            }
            PluginConfig::WithOptions { resolve, .. } => {
                if resolve.is_empty() {
                    return Err(ConfigError::validation_error("Plugin resolve name cannot be empty".to_string()));
                }
                Ok(())
            }
        }
    }
}

/// 代理配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ProxyConfig {
    /// 代理前缀
    pub prefix: String,

    /// 代理目标 URL
    pub url: String,
}

impl ProxyConfig {
    /// 创建新的代理配置
    pub fn new(prefix: String, url: String) -> Self {
        Self { prefix, url }
    }
}

/// 开发服务器配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DevelopConfig {
    /// 开发服务器主机
    pub host: Option<String>,

    /// 开发服务器端口
    pub port: Option<u16>,

    /// 是否启用 HTTPS
    pub https: Option<bool>,

    /// 是否启用自动打开浏览器
    pub open: Option<bool>,

    /// 是否启用热重载
    #[serde(rename = "hotLoader")]
    pub hot_loader: Option<bool>,

    /// 是否启用快速刷新
    #[serde(rename = "fastRefresh")]
    pub fast_refresh: Option<bool>,
}

impl DevelopConfig {
    /// 创建新的开发服务器配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置主机
    pub fn with_host(mut self, host: String) -> Self {
        self.host = Some(host);
        self
    }

    /// 设置端口
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// 获取主机地址
    pub fn get_host(&self) -> &str {
        self.host.as_deref().unwrap_or("localhost")
    }

    /// 获取端口
    pub fn get_port(&self) -> u16 {
        self.port.unwrap_or(8000)
    }
}

impl ConfigValidation for DevelopConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        if let Some(port) = self.port {
            if port == 0 {
                return Err(ConfigError::validation_error(format!("Invalid port number: {}", port)));
            }
        }
        Ok(())
    }
}

/// 构建标志配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FlagsConfig {
    /// 是否保留数据查询
    #[serde(rename = "preserveFileDownloads")]
    pub preserve_file_downloads: Option<bool>,

    /// 是否快速刷新
    #[serde(rename = "fastRefresh")]
    pub fast_refresh: Option<bool>,

    /// 是否启用详细日志
    pub verbose: Option<bool>,

    /// 是否启用类型生成
    pub typegen: Option<bool>,

    /// 是否启用并行处理
    pub parallel: Option<bool>,

    /// 是否启用增量构建
    #[serde(rename = "incrementalBuild")]
    pub incremental_build: Option<bool>,
}

impl FlagsConfig {
    /// 创建新的构建标志配置
    pub fn new() -> Self {
        Self::default()
    }
}

impl ConfigValidation for FlagsConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        Ok(())
    }
}

/// GraphQL 类型生成配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GraphQLTypegenConfig {
    /// 是否启用类型生成
    #[serde(rename = "generateOnBuild")]
    pub generate_on_build: Option<bool>,

    /// 类型生成输出路径
    #[serde(rename = "outputPath")]
    pub output_path: Option<String>,
}

/// 适配器配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AdapterConfig {
    /// 简单适配器名称
    Name(String),

    /// 带选项的适配器
    WithOptions {
        /// 适配器名称
        resolve: String,

        /// 适配器选项
        options: Option<HashMap<String, serde_json::Value>>,
    },
}

impl AdapterConfig {
    /// 获取适配器名称
    pub fn name(&self) -> &str {
        match self {
            AdapterConfig::Name(name) => name,
            AdapterConfig::WithOptions { resolve, .. } => resolve,
        }
    }
}

/// 尾随斜杠配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TrailingSlash {
    /// 总是添加尾随斜杠
    Always,

    /// 从不添加尾随斜杠
    Never,

    /// 忽略尾随斜杠
    Ignore,
}

impl Default for TrailingSlash {
    fn default() -> Self {
        TrailingSlash::Always
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_json_config() {
        let json = r#"{
            "siteMetadata": {
                "title": "Test Site",
                "description": "A test site"
            },
            "plugins": ["gatsby-plugin-test"]
        }"#;

        let config = GatsbyConfig::load_from_json_str(json).unwrap();
        assert_eq!(config.site_title(), "Test Site");
        assert_eq!(config.site_description(), Some("A test site"));
    }

    #[test]
    fn test_load_toml_config() {
        let toml = r#"
[siteMetadata]
title = "Test Site"
description = "A test site"

[[plugins]]
resolve = "gatsby-plugin-test"
"#;

        let config = GatsbyConfig::load_from_toml_str(toml).unwrap();
        assert_eq!(config.site_title(), "Test Site");
    }

    #[test]
    fn test_plugin_config() {
        let plugin = PluginConfig::simple("gatsby-plugin-test".to_string());
        assert_eq!(plugin.name(), "gatsby-plugin-test");
        assert!(plugin.options().is_none());

        let mut options = HashMap::new();
        options.insert("key".to_string(), serde_json::json!("value"));
        let plugin_with_options = PluginConfig::with_options("gatsby-plugin-test".to_string(), options);
        assert_eq!(plugin_with_options.name(), "gatsby-plugin-test");
        assert!(plugin_with_options.options().is_some());
    }

    #[test]
    fn test_develop_config_defaults() {
        let config = DevelopConfig::new();
        assert_eq!(config.get_host(), "localhost");
        assert_eq!(config.get_port(), 8000);
    }
}
