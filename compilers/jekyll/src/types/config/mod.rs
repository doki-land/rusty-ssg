//! 配置模块
//! 定义 VuTeX 文档系统的配置结构，支持从 TOML 和 JSON 文件加载配置

use nargo_types::NargoValue;
use oak_toml;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

/// 配置加载和验证相关的错误类型
#[derive(Debug)]
pub enum ConfigError {
    /// 文件读取错误
    FileReadError(std::io::Error),

    /// JSON 解析错误
    JsonParseError(serde_json::Error),

    /// TOML 解析错误
    TomlParseError(String),

    /// 配置验证错误
    ValidationError(String),

    /// 不支持的配置文件格式
    UnsupportedFormat(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::FileReadError(error) => write!(f, "Failed to read config file: {}", error),
            ConfigError::JsonParseError(error) => write!(f, "Failed to parse JSON config: {}", error),
            ConfigError::TomlParseError(error) => write!(f, "Failed to parse TOML config: {}", error),
            ConfigError::ValidationError(message) => write!(f, "Config validation error: {}", message),
            ConfigError::UnsupportedFormat(format) => write!(f, "Unsupported config file format: {}", format),
        }
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(error: std::io::Error) -> Self {
        ConfigError::FileReadError(error)
    }
}

impl From<serde_json::Error> for ConfigError {
    fn from(error: serde_json::Error) -> Self {
        ConfigError::JsonParseError(error)
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

/// VuTeX 配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct VutexConfig {
    /// 站点标题
    pub title: Option<String>,
    /// 站点描述
    pub description: Option<String>,
    /// 基础路径
    pub base: Option<String>,
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

impl VutexConfig {
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
            Some(ext) => Err(ConfigError::UnsupportedFormat(ext.to_string())),
            None => Err(ConfigError::UnsupportedFormat("no extension".to_string())),
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
        let config: Self = serde_json::from_str(json_str)?;
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
        let config: Self = oak_toml::from_str(toml_str).map_err(|e| ConfigError::TomlParseError(e.to_string()))?;
        config.validate()?;
        Ok(config)
    }

    /// 从目录中查找并加载配置文件
    ///
    /// 按以下顺序查找配置文件：
    /// 1. vutex.config.toml
    /// 2. vutex.config.json
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

        let toml_path = dir.join("vutex.config.toml");
        if toml_path.exists() {
            return Self::load_from_file(toml_path);
        }

        let json_path = dir.join("vutex.config.json");
        if json_path.exists() {
            return Self::load_from_file(json_path);
        }

        Ok(Self::default())
    }

    /// 将配置序列化为 JSON 字符串
    ///
    /// # Errors
    ///
    /// 返回 `serde_json::Error` 如果序列化失败
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// 将配置序列化为 TOML 字符串
    ///
    /// # Errors
    ///
    /// 返回 `ConfigError::TomlParseError` 如果序列化失败
    pub fn to_toml(&self) -> Result<String, ConfigError> {
        serde_json::to_string(self).map_err(|e| ConfigError::TomlParseError(e.to_string()))
    }
}

impl VutexConfig {
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

    /// 添加语言配置
    pub fn add_locale(mut self, lang: String, config: LocaleConfig) -> Self {
        self.locales.insert(lang, config);
        self
    }
}

impl ConfigValidation for VutexConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        let default_count = self.locales.iter().filter(|(_, cfg)| cfg.default.unwrap_or(false)).count();
        if default_count > 1 {
            return Err(ConfigError::ValidationError(format!(
                "Multiple default locales specified: found {} default locales",
                default_count
            )));
        }

        for (lang_code, locale) in &self.locales {
            if lang_code.is_empty() {
                return Err(ConfigError::ValidationError("Locale code cannot be empty".to_string()));
            }
            locale.validate()?;
        }

        self.theme.validate()?;

        for (i, plugin) in self.plugins.iter().enumerate() {
            if plugin.name.is_empty() {
                return Err(ConfigError::ValidationError(format!("Plugin at index {} has empty name", i)));
            }
        }

        self.markdown.validate()?;
        self.build.validate()?;

        Ok(())
    }
}

/// 语言配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
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

impl LocaleConfig {
    /// 创建新的语言配置
    pub fn new(label: String) -> Self {
        Self { label, description: None, link: None, default: None, nav: None, sidebar: None }
    }

    /// 设置为默认语言
    pub fn with_default(mut self, is_default: bool) -> Self {
        self.default = Some(is_default);
        self
    }

    /// 设置导航栏配置
    pub fn with_nav(mut self, nav: Vec<NavItem>) -> Self {
        self.nav = Some(nav);
        self
    }

    /// 设置侧边栏配置
    pub fn with_sidebar(mut self, sidebar: HashMap<String, Vec<SidebarItem>>) -> Self {
        self.sidebar = Some(sidebar);
        self
    }
}

impl ConfigValidation for LocaleConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        if self.label.is_empty() {
            return Err(ConfigError::ValidationError("Locale label cannot be empty".to_string()));
        }

        if let Some(nav) = &self.nav {
            for (i, item) in nav.iter().enumerate() {
                item.validate().map_err(|e| ConfigError::ValidationError(format!("Nav item at index {}: {}", i, e)))?;
            }
        }

        if let Some(sidebar) = &self.sidebar {
            for (group_key, items) in sidebar {
                for (i, item) in items.iter().enumerate() {
                    item.validate().map_err(|e| {
                        ConfigError::ValidationError(format!("Sidebar item in group '{}' at index {}: {}", group_key, i, e))
                    })?;
                }
            }
        }

        Ok(())
    }
}

/// 主题配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
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
    pub custom: HashMap<String, NargoValue>,
}

impl ThemeConfig {
    /// 创建新的主题配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 添加导航栏项
    pub fn add_nav_item(mut self, item: NavItem) -> Self {
        self.nav.push(item);
        self
    }
}

impl ConfigValidation for ThemeConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        for (i, item) in self.nav.iter().enumerate() {
            item.validate().map_err(|e| ConfigError::ValidationError(format!("Theme nav item at index {}: {}", i, e)))?;
        }

        for (group_key, items) in &self.sidebar {
            for (i, item) in items.iter().enumerate() {
                item.validate().map_err(|e| {
                    ConfigError::ValidationError(format!("Theme sidebar item in group '{}' at index {}: {}", group_key, i, e))
                })?;
            }
        }

        for (i, link) in self.social_links.iter().enumerate() {
            if link.platform.is_empty() {
                return Err(ConfigError::ValidationError(format!("Social link at index {} has empty platform name", i)));
            }
            if link.link.is_empty() {
                return Err(ConfigError::ValidationError(format!("Social link at index {} has empty URL", i)));
            }
        }

        Ok(())
    }
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

impl NavItem {
    /// 创建新的导航栏项
    pub fn new(text: String) -> Self {
        Self { text, link: None, items: None }
    }

    /// 设置链接
    pub fn with_link(mut self, link: String) -> Self {
        self.link = Some(link);
        self
    }

    /// 添加子项
    pub fn add_item(mut self, item: NavItem) -> Self {
        if self.items.is_none() {
            self.items = Some(Vec::new());
        }
        if let Some(items) = &mut self.items {
            items.push(item);
        }
        self
    }
}

impl ConfigValidation for NavItem {
    fn validate(&self) -> Result<(), ConfigError> {
        if self.text.is_empty() {
            return Err(ConfigError::ValidationError("Nav item text cannot be empty".to_string()));
        }

        if let Some(items) = &self.items {
            for (i, item) in items.iter().enumerate() {
                item.validate().map_err(|e| ConfigError::ValidationError(format!("Sub-item at index {}: {}", i, e)))?;
            }
        }

        Ok(())
    }
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

impl SidebarItem {
    /// 创建新的侧边栏项
    pub fn new(text: String) -> Self {
        Self { text, link: None, items: None, collapsed: None }
    }

    /// 设置链接
    pub fn with_link(mut self, link: String) -> Self {
        self.link = Some(link);
        self
    }
}

impl ConfigValidation for SidebarItem {
    fn validate(&self) -> Result<(), ConfigError> {
        if self.text.is_empty() {
            return Err(ConfigError::ValidationError("Sidebar item text cannot be empty".to_string()));
        }

        if let Some(items) = &self.items {
            for (i, item) in items.iter().enumerate() {
                item.validate().map_err(|e| ConfigError::ValidationError(format!("Sub-item at index {}: {}", i, e)))?;
            }
        }

        Ok(())
    }
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
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
    pub options: HashMap<String, NargoValue>,
}

/// Markdown 配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MarkdownConfig {
    /// 是否启用行号
    pub line_numbers: bool,
    /// 代码主题
    pub code_theme: Option<String>,
    /// 自定义配置
    pub custom: HashMap<String, NargoValue>,
}

impl ConfigValidation for MarkdownConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        Ok(())
    }
}

/// 构建配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
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

impl ConfigValidation for BuildConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        Ok(())
    }
}
