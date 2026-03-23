//! VuTeX 配置模块
//! 定义 VuTeX 兼容的配置结构，支持从 TOML、YAML 和 JSON 文件加载配置

use oak_toml;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    error::Error,
    fmt::{self, Display, Formatter},
    path::Path,
};

/// 配置加载和验证相关的错误类型
#[derive(Debug)]
pub enum ConfigError {
    /// 文件读取错误
    FileReadError {
        /// 底层 IO 错误
        source: std::io::Error,
    },

    /// JSON 解析错误
    JsonParseError {
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
            ConfigError::FileReadError { source } => write!(f, "Failed to read config file: {}", source),
            ConfigError::JsonParseError { cause } => write!(f, "Failed to parse JSON config: {}", cause),
            ConfigError::TomlParseError { cause } => write!(f, "Failed to parse TOML config: {}", cause),
            ConfigError::ValidationError { cause } => write!(f, "Config validation error: {}", cause),
            ConfigError::UnsupportedFormat { format } => write!(f, "Unsupported config file format: {}", format),
        }
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ConfigError::FileReadError { source } => Some(source),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(source: std::io::Error) -> Self {
        ConfigError::FileReadError { source }
    }
}

impl ConfigError {
    /// 获取错误的 i18n key
    pub fn i18n_key(&self) -> &'static str {
        match self {
            ConfigError::FileReadError { .. } => "vuepress.error.config.file_read",
            ConfigError::JsonParseError { .. } => "vuepress.error.config.json_parse",
            ConfigError::TomlParseError { .. } => "vuepress.error.config.toml_parse",
            ConfigError::ValidationError { .. } => "vuepress.error.config.validation",
            ConfigError::UnsupportedFormat { .. } => "vuepress.error.config.unsupported_format",
        }
    }

    /// 获取错误参数
    pub fn params(&self) -> Vec<(&'static str, String)> {
        match self {
            ConfigError::FileReadError { source } => {
                vec![("error", source.to_string())]
            }
            ConfigError::JsonParseError { cause } => {
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
    fn validate(&self) -> Result<(), ConfigError>;
}

/// VuePress 主配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct VuePressConfig {
    /// 基础路径
    pub base: Option<String>,
    /// 站点标题
    pub title: Option<String>,
    /// 站点描述
    pub description: Option<String>,
    /// 语言配置
    pub locales: HashMap<String, LocaleConfig>,
    /// 主题配置
    pub theme: ThemeConfig,
    /// 插件配置
    pub plugins: Vec<PluginConfig>,
    /// Markdown 配置
    pub markdown: MarkdownConfig,
    /// 构建配置
    pub build: BuildConfig,
}

impl VuePressConfig {
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
            Some("json") => Self::load_from_json_str(&content),
            Some("toml") => Self::load_from_toml_str(&content),
            Some(ext) => Err(ConfigError::UnsupportedFormat { format: ext.to_string() }),
            None => Err(ConfigError::UnsupportedFormat { format: "no extension".to_string() }),
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
        #[cfg(feature = "serde")]
        {
            let config: Self =
                serde_json::from_str(json_str).map_err(|e| ConfigError::JsonParseError { cause: e.to_string() })?;
            config.validate()?;
            Ok(config)
        }
        #[cfg(not(feature = "serde"))]
        {
            Err(ConfigError::JsonParseError { cause: "serde feature not enabled".to_string() })
        }
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
        #[cfg(feature = "serde")]
        {
            let config: Self =
                oak_toml::from_str(toml_str).map_err(|e| ConfigError::TomlParseError { cause: e.to_string() })?;
            config.validate()?;
            Ok(config)
        }
        #[cfg(not(feature = "serde"))]
        {
            Err(ConfigError::TomlParseError { cause: "serde feature not enabled".to_string() })
        }
    }

    /// 从目录中查找并加载配置文件
    ///
    /// 按以下顺序查找配置文件：
    /// 1. vutex.toml
    /// 2. vutex.json
    /// 3. config.toml
    /// 4. config.json
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

        let filenames = ["vutex.toml", "vutex.json", "config.toml", "config.json"];

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
        #[cfg(feature = "serde")]
        {
            serde_json::to_string(self).map_err(|e| ConfigError::JsonParseError { cause: e.to_string() })
        }
        #[cfg(not(feature = "serde"))]
        {
            Err(ConfigError::JsonParseError { cause: "serde feature not enabled".to_string() })
        }
    }

    /// 将配置序列化为 TOML 字符串
    ///
    /// # Errors
    ///
    /// 返回 `ConfigError::TomlParseError` 如果序列化失败
    pub fn to_toml(&self) -> Result<String, ConfigError> {
        #[cfg(feature = "serde")]
        {
            oak_toml::to_string(self).map_err(|e| ConfigError::TomlParseError { cause: e.to_string() })
        }
        #[cfg(not(feature = "serde"))]
        {
            Err(ConfigError::TomlParseError { cause: "serde feature not enabled".to_string() })
        }
    }
}

impl VuePressConfig {
    /// 创建新的 VuTeX 配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置基础路径
    pub fn with_base(mut self, base: String) -> Self {
        self.base = Some(base);
        self
    }

    /// 设置站点标题
    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }
}

impl ConfigValidation for VuePressConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        if let Some(base) = &self.base {
            if base.is_empty() {
                return Err(ConfigError::ValidationError { cause: "Base URL cannot be empty".to_string() });
            }
        }

        if let Some(title) = &self.title {
            if title.is_empty() {
                return Err(ConfigError::ValidationError { cause: "Title cannot be empty".to_string() });
            }
        }

        Ok(())
    }
}

/// 语言配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct LocaleConfig {
    /// 语言标签
    pub label: String,
    /// 语言描述
    pub description: Option<String>,
    /// 语言链接
    pub link: Option<String>,
    /// 是否为默认语言
    pub default: Option<bool>,
    /// 导航栏配置（语言特定）
    pub nav: Option<Vec<NavItem>>,
    /// 侧边栏配置（语言特定）
    pub sidebar: Option<HashMap<String, Vec<SidebarItem>>>,
}

/// 主题配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ThemeConfig {
    /// 导航栏配置
    pub nav: Vec<NavItem>,
    /// 侧边栏配置
    pub sidebar: HashMap<String, Vec<SidebarItem>>,
    /// 社交链接
    pub social_links: Vec<SocialLink>,
    /// 页脚配置
    pub footer: Option<FooterConfig>,
    /// 自定义配置
    pub custom: HashMap<String, String>,
}

/// 导航栏项
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavItem {
    /// 显示文本
    pub text: String,
    /// 链接
    pub link: Option<String>,
    /// 子项
    pub items: Option<Vec<NavItem>>,
}

/// 侧边栏项
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SidebarItem {
    /// 显示文本
    pub text: String,
    /// 链接
    pub link: Option<String>,
    /// 子项
    pub items: Option<Vec<SidebarItem>>,
    /// 是否折叠
    pub collapsed: Option<bool>,
}

/// 社交链接
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SocialLink {
    /// 平台名称
    pub platform: String,
    /// 链接
    pub link: String,
}

/// 页脚配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FooterConfig {
    /// 版权信息
    pub copyright: Option<String>,
    /// 页脚消息
    pub message: Option<String>,
}

/// 插件配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PluginConfig {
    /// 插件名称
    pub name: String,
    /// 插件配置
    pub options: HashMap<String, String>,
}

/// Markdown 配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MarkdownConfig {
    /// 是否启用行号
    pub line_numbers: bool,
    /// 代码主题
    pub code_theme: Option<String>,
    /// 自定义配置
    pub custom: HashMap<String, String>,
}

/// 构建配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BuildConfig {
    /// 输出目录
    pub out_dir: Option<String>,
    /// 源目录
    pub src_dir: Option<String>,
    /// 是否启用清理
    pub clean: bool,
    /// 是否启用压缩
    pub minify: bool,
}
