//! MkDocs 配置模块
//!
//! 包含 MkDocs 配置文件的完整类型定义，用于解析和验证 mkdocs.yml 配置。

use serde::{Deserialize, Serialize};
use oak_yaml;
use serde_json;
use std::collections::HashMap;

/// MkDocs 主配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MkDocsConfig {
    /// 站点名称
    pub site_name: String,
    /// 站点描述
    #[serde(default)]
    pub site_description: Option<String>,
    /// 站点作者
    #[serde(default)]
    pub site_author: Option<String>,
    /// 站点 URL
    #[serde(default)]
    pub site_url: Option<String>,
    /// 仓库 URL
    #[serde(default)]
    pub repo_url: Option<String>,
    /// 仓库名称
    #[serde(default)]
    pub repo_name: Option<String>,
    /// 版权信息
    #[serde(default)]
    pub copyright: Option<String>,
    /// 文档目录
    #[serde(default = "default_docs_dir")]
    pub docs_dir: String,
    /// 站点目录
    #[serde(default = "default_site_dir")]
    pub site_dir: String,
    /// 主题配置
    #[serde(default)]
    pub theme: ThemeConfig,
    /// 导航配置
    #[serde(default)]
    pub nav: Vec<NavItem>,
    /// Markdown 扩展配置
    #[serde(default)]
    pub markdown_extensions: Vec<String>,
    /// 插件配置
    #[serde(default)]
    pub plugins: Vec<PluginConfig>,
    /// 额外配置
    #[serde(default)]
    pub extra: HashMap<String, serde_json::Value>,
    /// 额外的 CSS 文件
    #[serde(default)]
    pub extra_css: Vec<String>,
    /// 额外的 JavaScript 文件
    #[serde(default)]
    pub extra_javascript: Vec<String>,
}

fn default_docs_dir() -> String {
    "docs".to_string()
}

fn default_site_dir() -> String {
    "site".to_string()
}

/// 主题配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThemeConfig {
    /// 主题名称
    #[serde(default = "default_theme_name")]
    pub name: String,
    /// 自定义主题目录
    #[serde(default)]
    pub custom_dir: Option<String>,
    /// 主题语言
    #[serde(default = "default_language")]
    pub language: String,
    /// 主题特性
    #[serde(default)]
    pub features: Vec<String>,
    /// 调色板配置
    #[serde(default)]
    pub palette: Option<PaletteConfig>,
    /// 字体配置
    #[serde(default)]
    pub font: Option<FontConfig>,
    /// 图标配置
    #[serde(default)]
    pub icon: Option<IconConfig>,
    /// 其他主题选项
    #[serde(default)]
    pub options: HashMap<String, serde_json::Value>,
}

fn default_theme_name() -> String {
    "material".to_string()
}

fn default_language() -> String {
    "en".to_string()
}

/// 调色板配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaletteConfig {
    /// 主色调
    #[serde(default)]
    pub primary: Option<String>,
    /// 强调色
    #[serde(default)]
    pub accent: Option<String>,
    /// 深色模式配置
    #[serde(default)]
    pub scheme: Option<String>,
    /// 备用调色板
    #[serde(default)]
    pub alternate: Option<Vec<AlternatePalette>>,
}

/// 备用调色板
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternatePalette {
    /// 调色板方案
    pub scheme: String,
    /// 调色板切换的媒体查询
    pub media: Option<String>,
    /// 调色板切换按钮标签
    pub toggle: Option<ToggleConfig>,
}

/// 调色板切换按钮配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToggleConfig {
    /// 按钮图标
    pub icon: String,
    /// 按钮名称
    pub name: String,
}

/// 字体配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FontConfig {
    /// 正文字体
    #[serde(default)]
    pub text: Option<String>,
    /// 代码字体
    #[serde(default)]
    pub code: Option<String>,
}

/// 图标配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IconConfig {
    /// Logo 图标
    #[serde(default)]
    pub logo: Option<String>,
    /// GitHub 图标
    #[serde(default)]
    pub repo: Option<String>,
    /// 编辑图标
    #[serde(default)]
    pub edit: Option<String>,
}

/// 导航项
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NavItem {
    /// 简单字符串链接
    String(String),
    /// 映射类型导航项
    Map(HashMap<String, NavValue>),
}

/// 导航值
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NavValue {
    /// 字符串路径
    String(String),
    /// 嵌套导航列表
    List(Vec<NavItem>),
}

/// 插件配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PluginConfig {
    /// 简单插件名称
    String(String),
    /// 带配置的插件
    Map(HashMap<String, PluginOptions>),
}

/// 插件选项
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PluginOptions {
    /// 插件是否启用
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// 其他插件选项
    #[serde(default)]
    pub options: HashMap<String, serde_json::Value>,
}

fn default_true() -> bool {
    true
}

impl MkDocsConfig {
    /// 创建默认配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 从 YAML 字符串解析配置
    pub fn from_yaml(content: &str) -> Result<Self, crate::types::errors::MkDocsError> {
        oak_yaml::language::from_str(content).map_err(|e| crate::types::errors::MkDocsError::ConfigParseError { message: e.to_string() })
    }

    /// 从文件加载配置
    ///
    /// # Arguments
    ///
    /// * `path` - 配置文件的路径
    ///
    /// # Errors
    ///
    /// 返回错误如果文件读取、解析或验证失败
    pub fn load_from_file(path: &std::path::PathBuf) -> Result<Self, crate::types::errors::MkDocsError> {
        let content = std::fs::read_to_string(path)?;
        let config = Self::from_yaml(&content)?;
        config.validate()?;
        Ok(config)
    }

    /// 从目录查找并加载配置
    ///
    /// 按以下顺序查找配置文件：
    /// 1. mkdocs.yml
    /// 2. mkdocs.yaml
    ///
    /// # Arguments
    ///
    /// * `dir` - 要搜索的目录路径
    ///
    /// # Errors
    ///
    /// 返回错误如果配置文件读取、解析或验证失败
    pub fn load_from_dir(dir: &std::path::PathBuf) -> Result<Self, crate::types::errors::MkDocsError> {
        let yml_path = dir.join("mkdocs.yml");
        if yml_path.exists() {
            return Self::load_from_file(&yml_path);
        }

        let yaml_path = dir.join("mkdocs.yaml");
        if yaml_path.exists() {
            return Self::load_from_file(&yaml_path);
        }

        Ok(MkDocsConfig::default())
    }

    /// 验证配置是否有效
    pub fn validate(&self) -> Result<(), crate::types::errors::MkDocsError> {
        if self.site_name.is_empty() {
            return Err(crate::types::errors::MkDocsError::ConfigValidationError {
                message: "site_name cannot be empty".to_string(),
            });
        }
        Ok(())
    }

    /// 获取站点名称
    pub fn site_name(&self) -> &str {
        &self.site_name
    }

    /// 获取文档目录
    pub fn docs_dir(&self) -> &str {
        &self.docs_dir
    }

    /// 获取站点目录
    pub fn site_dir(&self) -> &str {
        &self.site_dir
    }

    /// 获取主题配置
    pub fn theme(&self) -> &ThemeConfig {
        &self.theme
    }

    /// 获取导航配置
    pub fn nav(&self) -> &Vec<NavItem> {
        &self.nav
    }

    /// 获取 Markdown 扩展配置
    pub fn markdown_extensions(&self) -> &Vec<String> {
        &self.markdown_extensions
    }

    /// 获取插件配置
    pub fn plugins(&self) -> &Vec<PluginConfig> {
        &self.plugins
    }

    /// 获取额外配置
    pub fn extra(&self) -> &HashMap<String, serde_json::Value> {
        &self.extra
    }

    /// 获取额外的 CSS 文件
    pub fn extra_css(&self) -> &Vec<String> {
        &self.extra_css
    }

    /// 获取额外的 JavaScript 文件
    pub fn extra_javascript(&self) -> &Vec<String> {
        &self.extra_javascript
    }
}

impl ThemeConfig {
    /// 创建默认主题配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 获取主题名称
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 获取主题语言
    pub fn language(&self) -> &str {
        &self.language
    }

    /// 获取主题特性
    pub fn features(&self) -> &Vec<String> {
        &self.features
    }

    /// 获取其他主题选项
    pub fn options(&self) -> &HashMap<String, serde_json::Value> {
        &self.options
    }
}

impl PluginOptions {
    /// 创建默认插件选项
    pub fn new() -> Self {
        Self::default()
    }

    /// 获取插件是否启用
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    /// 获取其他插件选项
    pub fn options(&self) -> &HashMap<String, serde_json::Value> {
        &self.options
    }
}
