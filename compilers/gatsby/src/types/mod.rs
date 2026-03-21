//! Gatsby 类型模块
//! 定义 Gatsby 兼容的配置结构、GraphQL 数据类型，支持从 JSON、YAML 和 TOML 文件加载配置

/// GraphQL 数据类型
pub mod graphql;

use std::{
    collections::HashMap,
    error::Error,
    fmt::{self, Display, Formatter},
    path::Path,
};

use nargo_types::Document;
use serde::{Deserialize, Serialize};

/// 配置加载和验证相关的错误类型
#[derive(Debug, Clone)]
pub enum ConfigError {
    /// 文件读取错误
    FileReadError(String),

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
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::FileReadError(error) => write!(f, "Failed to read config file: {}", error),
            ConfigError::JsonParseError { cause } => write!(f, "Failed to parse JSON config: {}", cause),
            ConfigError::YamlParseError { cause } => write!(f, "Failed to parse YAML config: {}", cause),
            ConfigError::TomlParseError { cause } => write!(f, "Failed to parse TOML config: {}", cause),
            ConfigError::ValidationError { cause } => write!(f, "Config validation error: {}", cause),
            ConfigError::UnsupportedFormat { format } => write!(f, "Unsupported config file format: {}", format),
        }
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(error: std::io::Error) -> Self {
        ConfigError::FileReadError(error.to_string())
    }
}

impl ConfigError {
    /// 获取错误的 i18n key
    pub fn i18n_key(&self) -> &'static str {
        match self {
            ConfigError::FileReadError(_) => "gatsby.error.config.file_read",
            ConfigError::JsonParseError { .. } => "gatsby.error.config.json_parse",
            ConfigError::YamlParseError { .. } => "gatsby.error.config.yaml_parse",
            ConfigError::TomlParseError { .. } => "gatsby.error.config.toml_parse",
            ConfigError::ValidationError { .. } => "gatsby.error.config.validation",
            ConfigError::UnsupportedFormat { .. } => "gatsby.error.config.unsupported_format",
        }
    }

    /// 获取错误参数
    pub fn params(&self) -> Vec<(&'static str, String)> {
        match self {
            ConfigError::FileReadError(error) => {
                vec![("error", error.to_string())]
            }
            ConfigError::JsonParseError { cause } => {
                vec![("cause", cause.clone())]
            }
            ConfigError::YamlParseError { cause } => {
                vec![("cause", cause.clone())]
            }
            ConfigError::TomlParseError { cause } => {
                vec![("cause", cause.clone())]
            }
            ConfigError::ValidationError { cause } => {
                vec![("cause", cause.clone())]
            }
            ConfigError::UnsupportedFormat { format } => {
                vec![("format", format.clone())]
            }
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

    /// 创建验证错误
    pub fn validation_error(cause: String) -> Self {
        ConfigError::ValidationError { cause }
    }

    /// 创建不支持的格式错误
    pub fn unsupported_format(format: String) -> Self {
        ConfigError::UnsupportedFormat { format }
    }
}

/// 配置验证 trait
pub trait ConfigValidation {
    /// 验证配置的有效性
    ///
    /// # Errors
    ///
    /// 返回 `ConfigError::ValidationError` 如果配置无效
    fn validate(&self) -> std::result::Result<(), ConfigError>;
}

/// Gatsby 主配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GatsbyConfig {
    /// 站点元数据
    pub site_metadata: Option<SiteMetadata>,

    /// 路径前缀
    pub path_prefix: Option<String>,

    /// 插件配置
    pub plugins: Option<Vec<PluginConfig>>,

    /// 是否启用 polyfill
    pub polyfill: Option<bool>,

    /// 是否启用映射
    pub mapping: Option<HashMap<String, String>>,

    /// 开发服务器配置
    pub develop_middleware: Option<Vec<String>>,

    /// 代理配置
    pub proxy: Option<Vec<ProxyConfig>>,

    /// 开发服务器配置
    pub develop: Option<DevelopConfig>,

    /// 构建配置
    pub flags: Option<FlagsConfig>,

    /// 图表配置
    pub graphql_typegen: Option<GraphQLTypegenConfig>,

    /// 适配器配置
    pub adapter: Option<AdapterConfig>,

    /// JSX 运行时
    pub jsx_runtime: Option<String>,

    /// 部分 hydration 配置
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
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> std::result::Result<Self, ConfigError> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)?;

        match path.extension().and_then(|ext| ext.to_str()) {
            Some("json") => Self::load_from_json_str(&content),
            Some("yaml") | Some("yml") => Self::load_from_yaml_str(&content),
            Some("toml") => Self::load_from_toml_str(&content),
            Some(ext) => Err(ConfigError::unsupported_format(ext.to_string())),
            None => Err(ConfigError::unsupported_format("no extension".to_string())),
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
    pub fn load_from_json_str(json_str: &str) -> std::result::Result<Self, ConfigError> {
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
    pub fn load_from_yaml_str(yaml_str: &str) -> std::result::Result<Self, ConfigError> {
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
    pub fn load_from_toml_str(toml_str: &str) -> std::result::Result<Self, ConfigError> {
        let config: Self = oak_toml::from_str(toml_str).map_err(|e| ConfigError::toml_parse_error(e.to_string()))?;
        config.validate()?;
        Ok(config)
    }

    /// 从目录中查找并加载配置文件
    ///
    /// 按以下顺序查找配置文件：
    /// 1. gatsby-config.js
    /// 2. gatsby-config.json
    /// 3. gatsby-config.yaml
    /// 4. gatsby-config.yml
    /// 5. gatsby-config.toml
    ///
    /// # Arguments
    ///
    /// * `dir` - 要搜索的目录路径
    ///
    /// # Errors
    ///
    /// 返回 `ConfigError` 如果配置文件读取或解析失败
    pub fn load_from_dir<P: AsRef<Path>>(dir: P) -> std::result::Result<Self, ConfigError> {
        let dir = dir.as_ref();

        let filenames =
            ["gatsby-config.js", "gatsby-config.json", "gatsby-config.yaml", "gatsby-config.yml", "gatsby-config.toml"];

        for filename in filenames {
            let path = dir.join(filename);
            if path.exists() {
                if filename.ends_with(".js") {
                    return Ok(Self::default());
                }
                else {
                    return Self::load_from_file(path);
                }
            }
        }

        Ok(Self::default())
    }

    /// 将配置序列化为 JSON 字符串
    ///
    /// # Errors
    ///
    /// 返回 `ConfigError::JsonParseError` 如果序列化失败
    pub fn to_json(&self) -> std::result::Result<String, ConfigError> {
        oak_json::to_string(self).map_err(|e| ConfigError::json_parse_error(e.to_string()))
    }

    /// 将配置序列化为 YAML 字符串
    ///
    /// # Errors
    ///
    /// 返回 `ConfigError::YamlParseError` 如果序列化失败
    pub fn to_yaml(&self) -> std::result::Result<String, ConfigError> {
        oak_yaml::language::to_string(self).map_err(|e| ConfigError::yaml_parse_error(e.to_string()))
    }

    /// 将配置序列化为 TOML 字符串
    ///
    /// # Errors
    ///
    /// 返回 `ConfigError::TomlParseError` 如果序列化失败
    pub fn to_toml(&self) -> std::result::Result<String, ConfigError> {
        toml::to_string(self).map_err(|e| ConfigError::toml_parse_error(e.to_string()))
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
}

impl ConfigValidation for GatsbyConfig {
    fn validate(&self) -> std::result::Result<(), ConfigError> {
        if let Some(path_prefix) = &self.path_prefix {
            if path_prefix.is_empty() {
                return Err(ConfigError::validation_error("Path prefix cannot be empty".to_string()));
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
    pub site_url: Option<String>,

    /// 社交媒体链接
    pub social: Option<Vec<SocialLink>>,

    /// 自定义元数据
    pub custom: Option<HashMap<String, serde_json::Value>>,
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
}

/// 社交链接
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SocialLink {
    /// 平台名称
    pub name: String,

    /// 链接 URL
    pub url: String,
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

impl ConfigValidation for PluginConfig {
    fn validate(&self) -> std::result::Result<(), ConfigError> {
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
}

impl ConfigValidation for DevelopConfig {
    fn validate(&self) -> std::result::Result<(), ConfigError> {
        if let Some(port) = self.port {
            if port == 0 || port > 65535 {
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
    pub preserve_file_downloads: Option<bool>,

    /// 是否快速刷新
    pub fast_refresh: Option<bool>,

    /// 是否启用详细日志
    pub verbose: Option<bool>,

    /// 是否启用类型生成
    pub typegen: Option<bool>,
}

impl ConfigValidation for FlagsConfig {
    fn validate(&self) -> std::result::Result<(), ConfigError> {
        Ok(())
    }
}

/// GraphQL 类型生成配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GraphQLTypegenConfig {
    /// 是否启用类型生成
    pub generate_on_build: Option<bool>,
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

/// 编译结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileResult {
    /// 编译后的文档
    pub documents: HashMap<String, Document>,
    /// 编译时间（毫秒）
    pub compile_time_ms: u64,
    /// 是否成功
    pub success: bool,
    /// 错误信息（字符串形式）
    pub errors: Vec<String>,
}

impl CompileResult {
    /// 创建成功的编译结果
    ///
    /// # Arguments
    ///
    /// * `documents` - 编译后的文档映射
    /// * `compile_time_ms` - 编译时间（毫秒）
    pub fn success(documents: HashMap<String, Document>, compile_time_ms: u64) -> Self {
        Self { documents, compile_time_ms, success: true, errors: Vec::new() }
    }

    /// 创建失败的编译结果
    ///
    /// # Arguments
    ///
    /// * `errors` - 错误信息列表
    /// * `compile_time_ms` - 编译时间（毫秒）
    pub fn failure(errors: Vec<String>, compile_time_ms: u64) -> Self {
        Self { documents: HashMap::new(), compile_time_ms, success: false, errors }
    }

    /// 从 GatsbyError 创建失败的编译结果
    ///
    /// # Arguments
    ///
    /// * `errors` - GatsbyError 列表
    /// * `compile_time_ms` - 编译时间（毫秒）
    pub fn from_errors(errors: Vec<GatsbyError>, compile_time_ms: u64) -> Self {
        let error_strings = errors.iter().map(|e| format!("{}", e)).collect();
        Self::failure(error_strings, compile_time_ms)
    }

    /// 序列化为 JSON
    ///
    /// # Errors
    ///
    /// 返回 `serde_json::Error` 如果序列化失败
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }

    /// 序列化为美化的 JSON
    ///
    /// # Errors
    ///
    /// 返回 `serde_json::Error` 如果序列化失败
    pub fn to_json_pretty(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}

/// Gatsby 编译器错误
#[derive(Debug)]
pub enum GatsbyError {
    /// 配置错误
    ConfigError {
        /// 错误消息
        message: String,
    },

    /// 编译错误
    CompileError {
        /// 错误消息
        message: String,
    },

    /// 文件操作错误
    IoError {
        /// 底层 IO 错误
        source: std::io::Error,
    },

    /// 序列化错误
    SerializeError {
        /// 底层序列化错误
        source: serde_json::Error,
    },
}

impl GatsbyError {
    /// 获取错误的 i18n 键
    pub fn i18n_key(&self) -> &'static str {
        match self {
            GatsbyError::ConfigError { .. } => "gatsby.error.config",
            GatsbyError::CompileError { .. } => "gatsby.error.compile",
            GatsbyError::IoError { .. } => "gatsby.error.io",
            GatsbyError::SerializeError { .. } => "gatsby.error.serialize",
        }
    }
}

impl fmt::Display for GatsbyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GatsbyError::ConfigError { message } => write!(f, "Config error: {}", message),
            GatsbyError::CompileError { message } => write!(f, "Compile error: {}", message),
            GatsbyError::IoError { source } => write!(f, "IO error: {}", source),
            GatsbyError::SerializeError { source } => write!(f, "Serialize error: {}", source),
        }
    }
}

impl Error for GatsbyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            GatsbyError::IoError { source } => Some(source),
            GatsbyError::SerializeError { source } => Some(source),
            _ => None,
        }
    }
}

impl From<std::io::Error> for GatsbyError {
    fn from(source: std::io::Error) -> Self {
        GatsbyError::IoError { source }
    }
}

impl From<serde_json::Error> for GatsbyError {
    fn from(source: serde_json::Error) -> Self {
        GatsbyError::SerializeError { source }
    }
}

impl From<ConfigError> for GatsbyError {
    fn from(error: ConfigError) -> Self {
        GatsbyError::ConfigError { message: error.to_string() }
    }
}

impl From<nargo_types::Error> for GatsbyError {
    fn from(error: nargo_types::Error) -> Self {
        GatsbyError::CompileError { message: error.to_string() }
    }
}

/// 结果类型
pub type Result<T> = std::result::Result<T, GatsbyError>;
