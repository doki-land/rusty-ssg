//! Hugo 配置模块
//! 定义 Hugo 兼容的配置结构，支持从 TOML、YAML 和 JSON 文件加载配置

use std::{
    collections::HashMap,
    error::Error,
    fmt::{self, Display, Formatter},
    path::Path,
};

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
            ConfigError::FileReadError(_) => "hugo.error.config.file_read",
            ConfigError::JsonParseError { .. } => "hugo.error.config.json_parse",
            ConfigError::YamlParseError { .. } => "hugo.error.config.yaml_parse",
            ConfigError::TomlParseError { .. } => "hugo.error.config.toml_parse",
            ConfigError::ValidationError { .. } => "hugo.error.config.validation",
            ConfigError::UnsupportedFormat { .. } => "hugo.error.config.unsupported_format",
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
    fn validate(&self) -> Result<(), ConfigError>;
}

/// Hugo 主配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HugoConfig {
    /// 基础 URL
    pub base_url: Option<String>,
    /// 站点标题
    pub title: Option<String>,
    /// 语言代码
    pub language_code: Option<String>,
    /// 默认语言
    pub default_content_language: Option<String>,
    /// 主题配置
    pub theme: Option<String>,
    /// 主题列表
    pub themes: Option<Vec<String>>,
    /// 发布目录
    pub publish_dir: Option<String>,
    /// 内容目录
    pub content_dir: Option<String>,
    /// 静态文件目录
    pub static_dir: Option<String>,
    /// 布局目录
    pub layouts_dir: Option<String>,
    /// 数据目录
    pub data_dir: Option<String>,
    /// 资源目录
    pub assets_dir: Option<String>,
    /// 缓存目录
    pub cache_dir: Option<String>,
    /// 资源目录（旧名）
    pub resource_dir: Option<String>,
    /// i18n 目录
    pub i18n_dir: Option<String>,
    /// 归档目录
    pub archetypes_dir: Option<String>,
    /// 构建未来日期的内容
    pub build_future: Option<bool>,
    /// 构建过期内容
    pub build_expired: Option<bool>,
    /// 构建草稿内容
    pub build_drafts: Option<bool>,
    /// 是否启用简洁 URL
    pub ugly_urls: Option<bool>,
    /// 是否禁用分类
    pub disable_kinds: Option<Vec<String>>,
    /// 是否禁用 RSS
    pub disable_rss: Option<bool>,
    /// 是否禁用分类列表
    pub disable_taxonomies: Option<bool>,
    /// 是否禁用分类项列表
    pub disable_terms: Option<bool>,
    /// 是否禁用 sitemap
    pub disable_sitemap: Option<bool>,
    /// 是否禁用 robots.txt
    pub disable_robots_t_txt: Option<bool>,
    /// 分类配置
    pub taxonomies: Option<Vec<TaxonomyConfig>>,
    /// 菜单配置
    pub menus: Option<HashMap<String, Vec<MenuItem>>>,
    /// 参数配置
    pub params: Option<HashMap<String, serde_json::Value>>,
    /// 语言配置
    pub languages: Option<HashMap<String, LanguageConfig>>,
    /// 标记配置
    pub markup: Option<MarkupConfig>,
    /// 输出格式配置
    pub outputs: Option<HashMap<String, Vec<String>>>,
    /// 媒体类型配置
    pub media_types: Option<HashMap<String, MediaTypeConfig>>,
    /// 输出格式定义
    pub output_formats: Option<HashMap<String, OutputFormatConfig>>,
    /// 最小化配置
    pub minify: Option<MinifyConfig>,
    /// 相关内容配置
    pub related: Option<RelatedConfig>,
    /// 服务配置
    pub server: Option<ServerConfig>,
    /// 永久链接配置
    pub permalinks: Option<HashMap<String, String>>,
    /// 分页配置
    pub pagination: Option<PaginationConfig>,
    /// Sitemap 配置
    pub sitemap: Option<SitemapConfig>,
    /// RSS 配置
    pub rss: Option<RssConfig>,
    /// 日期配置
    pub date: Option<DateConfig>,
    /// 作者配置
    pub author: Option<AuthorConfig>,
    /// 社交配置
    pub social: Option<HashMap<String, String>>,
    /// 是否启用 Git 信息
    pub enable_git_info: Option<bool>,
    /// 是否启用 Emoji
    pub enable_emoji: Option<bool>,
    /// 是否启用 Robots TXT
    pub enable_robots_t_txt: Option<bool>,
    /// 是否启用缩短的永久链接
    pub enable_missing_translations: Option<bool>,
    /// 是否在多语言模式下回退到默认语言
    pub default_content_language_in_subdir: Option<bool>,
    /// 是否忽略内容文件
    pub ignore_files: Option<Vec<String>>,
    /// 超时时间
    pub timeout: Option<u64>,
}

impl HugoConfig {
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
        let config: Self = oak_toml::language::from_str(toml_str).map_err(|e| ConfigError::toml_parse_error(e.to_string()))?;
        config.validate()?;
        Ok(config)
    }

    /// 从目录中查找并加载配置文件
    ///
    /// 按以下顺序查找配置文件：
    /// 1. hugo.toml
    /// 2. hugo.yaml
    /// 3. hugo.yml
    /// 4. hugo.json
    /// 5. config.toml
    /// 6. config.yaml
    /// 7. config.yml
    /// 8. config.json
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

        let filenames =
            ["hugo.toml", "hugo.yaml", "hugo.yml", "hugo.json", "config.toml", "config.yaml", "config.yml", "config.json"];

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
        oak_json::to_string(self).map_err(|e| ConfigError::json_parse_error(e.to_string()))
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
}

impl HugoConfig {
    /// 创建新的 Hugo 配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置基础 URL
    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = Some(base_url);
        self
    }

    /// 设置站点标题
    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    /// 设置主题
    pub fn with_theme(mut self, theme: String) -> Self {
        self.theme = Some(theme);
        self
    }
}

impl ConfigValidation for HugoConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        if let Some(base_url) = &self.base_url {
            if base_url.is_empty() {
                return Err(ConfigError::validation_error("Base URL cannot be empty".to_string()));
            }
        }

        if let Some(title) = &self.title {
            if title.is_empty() {
                return Err(ConfigError::validation_error("Title cannot be empty".to_string()));
            }
        }

        if let Some(taxonomies) = &self.taxonomies {
            for (i, taxonomy) in taxonomies.iter().enumerate() {
                taxonomy.validate().map_err(|e| ConfigError::validation_error(format!("Taxonomy at index {}: {}", i, e)))?;
            }
        }

        if let Some(menus) = &self.menus {
            for (menu_name, items) in menus {
                for (i, item) in items.iter().enumerate() {
                    item.validate().map_err(|e| {
                        ConfigError::validation_error(format!("Menu item '{}' at index {}: {}", menu_name, i, e))
                    })?;
                }
            }
        }

        if let Some(languages) = &self.languages {
            for (lang_code, lang) in languages {
                lang.validate().map_err(|e| ConfigError::validation_error(format!("Language '{}': {}", lang_code, e)))?;
            }
        }

        if let Some(markup) = &self.markup {
            markup.validate()?;
        }

        if let Some(related) = &self.related {
            related.validate()?;
        }

        if let Some(server) = &self.server {
            server.validate()?;
        }

        if let Some(pagination) = &self.pagination {
            pagination.validate()?;
        }

        if let Some(sitemap) = &self.sitemap {
            sitemap.validate()?;
        }

        if let Some(rss) = &self.rss {
            rss.validate()?;
        }

        if let Some(date) = &self.date {
            date.validate()?;
        }

        if let Some(author) = &self.author {
            author.validate()?;
        }

        Ok(())
    }
}

/// 分类配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TaxonomyConfig {
    /// 分类名称（复数形式）
    pub name: String,
    /// 分类单数形式
    pub singular: Option<String>,
    /// 是否禁用此分类
    pub disable: Option<bool>,
}

impl TaxonomyConfig {
    /// 创建新的分类配置
    pub fn new(name: String) -> Self {
        Self { name, singular: None, disable: None }
    }
}

impl ConfigValidation for TaxonomyConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        if self.name.is_empty() {
            return Err(ConfigError::validation_error("Taxonomy name cannot be empty".to_string()));
        }
        Ok(())
    }
}

/// 菜单项配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MenuItem {
    /// 菜单项标识符
    pub identifier: Option<String>,
    /// 菜单项名称
    pub name: Option<String>,
    /// 菜单项 URL
    pub url: Option<String>,
    /// 菜单项标题
    pub title: Option<String>,
    /// 菜单项权重（用于排序）
    pub weight: Option<i32>,
    /// 父菜单项标识符
    pub parent: Option<String>,
    /// 菜单项前置文本
    pub pre: Option<String>,
    /// 菜单项后置文本
    pub post: Option<String>,
    /// 子菜单项
    pub children: Option<Vec<MenuItem>>,
    /// 菜单项参数
    pub params: Option<HashMap<String, String>>,
}

impl MenuItem {
    /// 创建新的菜单项
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置菜单项名称
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// 设置菜单项 URL
    pub fn with_url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }
}

impl ConfigValidation for MenuItem {
    fn validate(&self) -> Result<(), ConfigError> {
        if self.name.is_none() && self.url.is_none() {
            return Err(ConfigError::validation_error("Menu item must have either name or url".to_string()));
        }

        if let Some(children) = &self.children {
            for (i, child) in children.iter().enumerate() {
                child
                    .validate()
                    .map_err(|e| ConfigError::validation_error(format!("Child menu item at index {}: {}", i, e)))?;
            }
        }

        Ok(())
    }
}

/// 语言配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct LanguageConfig {
    /// 语言名称
    pub language_name: Option<String>,
    /// 语言标题
    pub title: Option<String>,
    /// 语言权重
    pub weight: Option<i32>,
    /// 是否为默认语言
    pub default: Option<bool>,
    /// 是否禁用此语言
    pub disabled: Option<bool>,
    /// 语言方向
    pub language_direction: Option<String>,
    /// 日期格式
    pub date_format: Option<String>,
    /// 语言特定的菜单
    pub menus: Option<HashMap<String, Vec<MenuItem>>>,
    /// 语言特定的参数
    pub params: Option<HashMap<String, String>>,
}

impl LanguageConfig {
    /// 创建新的语言配置
    pub fn new() -> Self {
        Self::default()
    }
}

impl ConfigValidation for LanguageConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        if let Some(menus) = &self.menus {
            for (menu_name, items) in menus {
                for (i, item) in items.iter().enumerate() {
                    item.validate().map_err(|e| {
                        ConfigError::validation_error(format!("Language menu item '{}' at index {}: {}", menu_name, i, e))
                    })?;
                }
            }
        }
        Ok(())
    }
}

/// 标记配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MarkupConfig {
    /// Goldmark 配置
    pub goldmark: Option<GoldmarkConfig>,
    /// Asciidoctor 配置
    pub asciidoctor: Option<AsciidoctorConfig>,
    /// 表格化配置
    pub table_of_contents: Option<TableOfContentsConfig>,
    /// 高亮配置
    pub highlight: Option<HighlightConfig>,
}

impl ConfigValidation for MarkupConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        if let Some(goldmark) = &self.goldmark {
            goldmark.validate()?;
        }
        if let Some(highlight) = &self.highlight {
            highlight.validate()?;
        }
        Ok(())
    }
}

/// Goldmark 配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GoldmarkConfig {
    /// 渲染器配置
    pub renderer: Option<GoldmarkRendererConfig>,
    /// 扩展配置
    pub extensions: Option<GoldmarkExtensionsConfig>,
    /// 解析器配置
    pub parser: Option<GoldmarkParserConfig>,
}

impl ConfigValidation for GoldmarkConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        Ok(())
    }
}

/// Goldmark 渲染器配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GoldmarkRendererConfig {
    /// 是否启用硬换行
    pub hard_wraps: Option<bool>,
    /// 是否启用不安全内容
    #[serde(rename = "unsafe")]
    pub unsafe_content: Option<bool>,
}

/// Goldmark 扩展配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GoldmarkExtensionsConfig {
    /// 是否启用表格
    pub table: Option<bool>,
    /// 是否启用任务列表
    pub task_list: Option<bool>,
    /// 是否启用删除线
    pub strikethrough: Option<bool>,
    /// 是否启用脚注
    pub footnote: Option<bool>,
    /// 是否启用定义列表
    pub definition_list: Option<bool>,
    /// 是否启用自动链接
    pub linkify: Option<bool>,
    /// 是否启用转义
    pub typographer: Option<bool>,
}

/// Goldmark 解析器配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GoldmarkParserConfig {
    /// 是否启用自动标题 ID
    pub auto_heading_id: Option<bool>,
    /// 是否启用属性
    pub attribute: Option<bool>,
}

/// Asciidoctor 配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AsciidoctorConfig {
    /// 是否启用背页
    pub backend: Option<String>,
    /// 属性列表
    pub attributes: Option<HashMap<String, String>>,
    /// 是否启用扩展
    pub extensions: Option<Vec<String>>,
}

/// 目录配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TableOfContentsConfig {
    /// 开始层级
    pub start_level: Option<u32>,
    /// 结束层级
    pub end_level: Option<u32>,
    /// 是否启用有序列表
    pub ordered: Option<bool>,
}

/// 高亮配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HighlightConfig {
    /// 代码高亮风格
    pub style: Option<String>,
    /// 是否启用行号
    pub line_no: Option<bool>,
    /// 高亮的表格类
    pub tab_width: Option<u32>,
    /// 是否无类
    pub no_classes: Option<bool>,
    /// HL_LINES 选项
    pub hl_lines: Option<String>,
    /// HL_STYLE 选项
    pub hl_style: Option<String>,
    /// 默认语言
    pub default_language: Option<String>,
    /// 猜测语法
    pub guess_syntax: Option<bool>,
}

impl ConfigValidation for HighlightConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        Ok(())
    }
}

/// 媒体类型配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MediaTypeConfig {
    /// MIME 类型
    pub mime_type: Option<String>,
    /// 后缀列表
    pub suffixes: Option<Vec<String>>,
    /// 文件扩展名
    pub file_extension: Option<String>,
}

/// 输出格式配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct OutputFormatConfig {
    /// 基础名称
    pub base_name: Option<String>,
    /// 是否为首页
    pub is_plain_text: Option<bool>,
    /// 是否为首页
    pub is_html: Option<bool>,
    /// 媒体类型
    pub media_type: Option<String>,
    /// 协议
    pub protocol: Option<String>,
    /// 链接关系
    pub rel: Option<String>,
    /// 允许无扩展名 URL
    pub no_uids: Option<bool>,
    /// 路径上没有扩展名
    pub path: Option<String>,
    /// 永久链接
    pub permalinkable: Option<bool>,
    /// 主扩展名
    pub extension: Option<String>,
}

/// 最小化配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MinifyConfig {
    /// 是否启用禁用
    pub disable: Option<bool>,
    /// TDE 启用
    pub tdew: Option<bool>,
    /// CSS 配置
    pub css: Option<MinifyCssConfig>,
    /// HTML 配置
    pub html: Option<MinifyHtmlConfig>,
    /// JS 配置
    pub js: Option<MinifyJsConfig>,
    /// JSON 配置
    pub json: Option<MinifyJsonConfig>,
    /// SVG 配置
    pub svg: Option<MinifySvgConfig>,
    /// XML 配置
    pub xml: Option<MinifyXmlConfig>,
}

/// CSS 最小化配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MinifyCssConfig {
    /// 是否启用
    pub enable: Option<bool>,
}

/// HTML 最小化配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MinifyHtmlConfig {
    /// 是否启用
    pub enable: Option<bool>,
    /// 是否移除注释
    pub keep_comments: Option<bool>,
    /// 是否保留默认属性
    pub keep_default_attr_vals: Option<bool>,
    /// 是否保持结束标签
    pub keep_end_tags: Option<bool>,
    /// 是否保留空白
    pub keep_whitespace: Option<bool>,
    /// 引号字符
    pub quote_char: Option<String>,
}

/// JS 最小化配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MinifyJsConfig {
    /// 是否启用
    pub enable: Option<bool>,
    /// 版本
    pub keep_annotations: Option<bool>,
}

/// JSON 最小化配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MinifyJsonConfig {
    /// 是否启用
    pub enable: Option<bool>,
}

/// SVG 最小化配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MinifySvgConfig {
    /// 是否启用
    pub enable: Option<bool>,
}

/// XML 最小化配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MinifyXmlConfig {
    /// 是否启用
    pub enable: Option<bool>,
    /// 是否保留注释
    pub keep_whitespace: Option<bool>,
}

/// 相关内容配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RelatedConfig {
    /// 阈值
    pub threshold: Option<f64>,
    /// 是否启用新索引
    pub include_newer: Option<bool>,
    /// 类别配置
    pub categories: Option<HashMap<String, RelatedCategoryConfig>>,
}

impl ConfigValidation for RelatedConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        Ok(())
    }
}

/// 相关内容类别配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RelatedCategoryConfig {
    /// 权重
    pub weight: Option<f64>,
    /// 阈值
    pub threshold: Option<f64>,
    /// 是否区分大小写
    pub case_sensitive: Option<bool>,
}

/// 服务配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ServerConfig {
    /// HTTP 端口
    pub port: Option<u16>,
    /// 网络接口
    pub interface: Option<String>,
    /// 是否启用实时重载
    pub disable_live_reload: Option<bool>,
    /// 是否启用 HTTP 2
    pub http2: Option<bool>,
    /// 是否启用导航到更改页面
    pub navigate_to_changed: Option<bool>,
    /// 包含的头部
    pub includedirs: Option<Vec<String>>,
    /// 静态头
    pub staticheaders: Option<Vec<String>>,
    /// 源映射
    pub sourcemap: Option<bool>,
}

impl ConfigValidation for ServerConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        if let Some(port) = self.port {
            if port == 0 {
                return Err(ConfigError::validation_error(format!("Invalid port number: {}", port)));
            }
        }
        Ok(())
    }
}

/// 分页配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PaginationConfig {
    /// 每页显示的项目数
    pub pager_size: Option<u32>,
    /// 路径
    pub path: Option<String>,
}

impl ConfigValidation for PaginationConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        Ok(())
    }
}

/// Sitemap 配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SitemapConfig {
    /// 文件名
    pub filename: Option<String>,
    /// 优先级
    pub priority: Option<f64>,
    /// 变更频率
    pub changefreq: Option<String>,
}

impl ConfigValidation for SitemapConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        Ok(())
    }
}

/// RSS 配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RssConfig {
    /// 项目数量限制
    pub limit: Option<u32>,
}

impl ConfigValidation for RssConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        Ok(())
    }
}

/// 日期配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DateConfig {
    /// 日期格式
    pub format: Option<String>,
    /// 时间戳格式
    pub timestamp_format: Option<String>,
}

impl ConfigValidation for DateConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        Ok(())
    }
}

/// 作者配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AuthorConfig {
    /// 作者名称
    pub name: Option<String>,
    /// 作者邮箱
    pub email: Option<String>,
    /// 作者主页
    pub homepage: Option<String>,
}

impl ConfigValidation for AuthorConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        Ok(())
    }
}

// 旧的 Vutex 配置（保留向后兼容）
// pub use self::vutex::*;

// mod vutex {
//     use super::*;
//     // use nargo_types::NargoValue;

//     /// Vutex 配置（旧版，保留向后兼容）
//     #[derive(Debug, Clone, PartialEq, Default)]
//     #[deprecated(note = "Use HugoConfig instead")]
//     pub struct VutexConfig {
//         /// 站点标题
//         pub title: Option<String>,
//         /// 站点描述
//         pub description: Option<String>,
//         /// 基础路径
//         pub base: Option<String>,
//         /// 语言配置
//         pub locales: HashMap<String, LocaleConfig>,
//         /// 主题配置
//         pub theme: ThemeConfig,
//         /// 插件配置
//         pub plugins: Vec<PluginConfig>,
//         /// Markdown 配置
//         pub markdown: MarkdownConfig,
//         /// 构建配置
//         pub build: BuildConfig,
//     }

//     /// 语言配置（旧版）
//     #[derive(Debug, Clone, PartialEq, Default)]
//     #[deprecated(note = "Use LanguageConfig instead")]
//     pub struct LocaleConfig {
//         /// 语言标签
//         pub label: String,
//         /// 语言描述
//         pub description: Option<String>,
//         /// 语言链接
//         pub link: Option<String>,
//         /// 是否为默认语言
//         pub default: Option<bool>,
//         /// 导航栏配置（语言特定）
//         pub nav: Option<Vec<NavItem>>,
//         /// 侧边栏配置（语言特定）
//         pub sidebar: Option<HashMap<String, Vec<SidebarItem>>>,
//     }

//     /// 主题配置（旧版）
//     #[derive(Debug, Clone, PartialEq, Default)]
//     #[deprecated(note = "Use HugoConfig's theme field instead")]
//     pub struct ThemeConfig {
//         /// 导航栏配置
//         pub nav: Vec<NavItem>,
//         /// 侧边栏配置
//         pub sidebar: HashMap<String, Vec<SidebarItem>>,
//         /// 社交链接
//         pub social_links: Vec<SocialLink>,
//         /// 页脚配置
//         pub footer: Option<FooterConfig>,
//         /// 自定义配置
//         pub custom: HashMap<String, String>, // HashMap<String, NargoValue>,
//     }

//     /// 导航栏项（旧版）
//     #[derive(Debug, Clone, PartialEq)]
//     #[deprecated(note = "Use MenuItem instead")]
//     pub struct NavItem {
//         /// 显示文本
//         pub text: String,
//         /// 链接
//         pub link: Option<String>,
//         /// 子项
//         pub items: Option<Vec<NavItem>>,
//     }

//     /// 侧边栏项（旧版）
//     #[derive(Debug, Clone, PartialEq)]
//     #[deprecated(note = "Use MenuItem instead")]
//     pub struct SidebarItem {
//         /// 显示文本
//         pub text: String,
//         /// 链接
//         pub link: Option<String>,
//         /// 子项
//         pub items: Option<Vec<SidebarItem>>,
//         /// 是否折叠
//         pub collapsed: Option<bool>,
//     }

//     /// 社交链接（旧版）
//     #[derive(Debug, Clone, PartialEq)]
//     #[deprecated(note = "Use HugoConfig's params instead")]
//     pub struct SocialLink {
//         /// 平台名称
//         pub platform: String,
//         /// 链接
//         pub link: String,
//     }

//     /// 页脚配置（旧版）
//     #[derive(Debug, Clone, PartialEq, Default)]
//     #[deprecated(note = "Use HugoConfig's params instead")]
//     pub struct FooterConfig {
//         /// 版权信息
//         pub copyright: Option<String>,
//         /// 页脚消息
//         pub message: Option<String>,
//     }

//     /// 插件配置（旧版）
//     #[derive(Debug, Clone, PartialEq)]
//     #[deprecated(note = "Use HugoConfig's params instead")]
//     pub struct PluginConfig {
//         /// 插件名称
//         pub name: String,
//         /// 插件配置
//         pub options: HashMap<String, String>, // HashMap<String, NargoValue>,
//     }

//     /// Markdown 配置（旧版）
//     #[derive(Debug, Clone, PartialEq, Default)]
//     #[deprecated(note = "Use MarkupConfig instead")]
//     pub struct MarkdownConfig {
//         /// 是否启用行号
//         pub line_numbers: bool,
//         /// 代码主题
//         pub code_theme: Option<String>,
//         /// 自定义配置
//         pub custom: HashMap<String, String>, // HashMap<String, NargoValue>,
//     }

//     /// 构建配置（旧版）
//     #[derive(Debug, Clone, PartialEq, Default)]
//     #[deprecated(note = "Use HugoConfig's build_* fields instead")]
//     pub struct BuildConfig {
//         /// 输出目录
//         pub out_dir: Option<String>,
//         /// 源目录
//         pub src_dir: Option<String>,
//         /// 是否启用清理
//         pub clean: bool,
//         /// 是否启用压缩
//         pub minify: bool,
//     }
// }
