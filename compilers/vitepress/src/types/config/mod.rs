//! VuTeX 配置模块
//! 定义 VuTeX 兼容 VitePress 的配置结构，支持从 TOML、YAML 和 JSON 文件加载配置

use oak_toml;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    error::Error,
    fmt::{self, Display, Formatter},
    path::Path,
};

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
            ConfigError::FileReadError(_) => "vitepress.error.config.file_read",
            ConfigError::JsonParseError { .. } => "vitepress.error.config.json_parse",
            ConfigError::TomlParseError { .. } => "vitepress.error.config.toml_parse",
            ConfigError::ValidationError { .. } => "vitepress.error.config.validation",
            ConfigError::UnsupportedFormat { .. } => "vitepress.error.config.unsupported_format",
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

/// 头部标签配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeadConfig {
    /// 标签名称
    pub tag_name: String,
    /// 标签属性
    pub attrs: HashMap<String, String>,
    /// 标签内容（可选）
    pub inner_html: Option<String>,
}

/// 可主题化的图片配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ThemeableImage {
    /// 简单的图片路径
    String(String),
    /// 带 alt 属性的图片
    WithAlt {
        /// 图片源
        src: String,
        /// 图片 alt 属性
        alt: Option<String>,
    },
    /// 明暗模式不同的图片
    LightDark {
        /// 浅色模式图片
        light: String,
        /// 深色模式图片
        dark: String,
        /// 图片 alt 属性
        alt: Option<String>,
    },
}

/// 大纲层级配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OutlineLevel {
    /// 单个层级
    Single(u8),
    /// 层级范围
    Range(u8, u8),
    /// 深度配置（h2 到 h6）
    Deep(String),
}

/// 大纲配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Outline {
    /// 要显示在大纲中的标题层级
    pub level: Option<OutlineLevel>,
    /// 大纲标题
    pub label: Option<String>,
}

/// 社交链接图标
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SocialLinkIcon {
    /// 简单的图标名称（来自 simple-icons）
    Name(String),
    /// 自定义 SVG 图标
    Svg {
        /// SVG 内容
        svg: String,
    },
}

/// 社交链接配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SocialLink {
    /// 图标
    pub icon: SocialLinkIcon,
    /// 链接
    pub link: String,
    /// 无障碍标签
    pub aria_label: Option<String>,
}

/// 页脚配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FooterConfig {
    /// 页脚消息
    pub message: Option<String>,
    /// 版权信息
    pub copyright: Option<String>,
}

/// 编辑链接配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EditLink {
    /// 链接模式，使用 :path 作为占位符
    pub pattern: String,
    /// 链接文本
    pub text: Option<String>,
}

/// 最后更新时间格式化选项
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LastUpdatedFormatOptions {
    /// 日期样式
    pub date_style: Option<String>,
    /// 时间样式
    pub time_style: Option<String>,
    /// 强制使用特定语言
    pub force_locale: Option<bool>,
}

/// 最后更新时间配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LastUpdatedOptions {
    /// 显示文本
    pub text: Option<String>,
    /// 格式化选项
    pub format_options: Option<LastUpdatedFormatOptions>,
}

/// Carbon Ads 配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CarbonAdsOptions {
    /// 广告代码
    pub code: String,
    /// 广告位置
    pub placement: String,
}

/// 文档页脚配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocFooter {
    /// 上一页文本，false 表示禁用
    pub prev: Option<DocFooterItem>,
    /// 下一页文本，false 表示禁用
    pub next: Option<DocFooterItem>,
}

/// 文档页脚项
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DocFooterItem {
    /// 文本
    Text(String),
    /// 禁用
    Disabled(bool),
}

/// 外观配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Appearance {
    /// 布尔值，启用/禁用深色模式
    Boolean(bool),
    /// 字符串值
    String(String),
}

/// 导航栏项
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NavItem {
    /// 带链接的导航项
    WithLink(NavItemWithLink),
    /// 带子项的导航项
    WithChildren(NavItemWithChildren),
}

/// 带链接的导航项
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavItemWithLink {
    /// 显示文本
    pub text: String,
    /// 链接
    pub link: String,
    /// 激活匹配模式
    pub active_match: Option<String>,
    /// 链接目标
    pub target: Option<String>,
    /// 链接 rel 属性
    pub rel: Option<String>,
    /// 是否不显示图标
    pub no_icon: Option<bool>,
}

/// 导航项子项
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavItemChildren {
    /// 显示文本
    pub text: Option<String>,
    /// 子项列表
    pub items: Vec<NavItemWithLink>,
}

/// 带子项的导航项
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavItemWithChildren {
    /// 显示文本
    pub text: Option<String>,
    /// 子项列表
    pub items: Vec<NavItem>,
    /// 激活匹配模式
    pub active_match: Option<String>,
}

/// 侧边栏项
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SidebarItem {
    /// 显示文本
    pub text: Option<String>,
    /// 链接
    pub link: Option<String>,
    /// 子项
    pub items: Option<Vec<SidebarItem>>,
    /// 是否折叠，true 表示可折叠且默认折叠，false 表示可折叠但默认展开
    pub collapsed: Option<bool>,
    /// 子项的基础路径
    pub base: Option<String>,
    /// 自定义上一页/下一页页脚文本
    pub doc_footer_text: Option<String>,
    /// 链接 rel 属性
    pub rel: Option<String>,
    /// 链接目标
    pub target: Option<String>,
}

/// 侧边栏配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Sidebar {
    /// 侧边栏项列表
    List(Vec<SidebarItem>),
    /// 多路径侧边栏配置
    Multi(HashMap<String, SidebarMultiItem>),
}

/// 多路径侧边栏项
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SidebarMultiItem {
    /// 侧边栏项列表
    List(Vec<SidebarItem>),
    /// 带基础路径的侧边栏配置
    WithBase {
        /// 侧边栏项列表
        items: Vec<SidebarItem>,
        /// 基础路径
        base: String,
    },
}

/// 默认主题配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemeConfig {
    /// 是否启用国际化路由
    pub i18n_routing: Option<bool>,
    /// 导航栏 Logo
    pub logo: Option<ThemeableImage>,
    /// 站点标题，false 表示禁用
    pub site_title: Option<SiteTitle>,
    /// 导航栏配置
    pub nav: Option<Vec<NavItem>>,
    /// 侧边栏配置
    pub sidebar: Option<Sidebar>,
    /// 侧边栏位置配置
    pub aside: Option<Aside>,
    /// 大纲配置
    pub outline: Option<OutlineConfig>,
    /// 社交链接
    pub social_links: Option<Vec<SocialLink>>,
    /// 页脚配置
    pub footer: Option<FooterConfig>,
    /// 编辑链接配置
    pub edit_link: Option<EditLink>,
    /// 最后更新时间配置
    pub last_updated: Option<LastUpdatedOptions>,
    /// Algolia 搜索配置
    pub algolia: Option<serde_json::Value>,
    /// Carbon Ads 配置
    pub carbon_ads: Option<CarbonAdsOptions>,
    /// 文档页脚配置
    pub doc_footer: Option<DocFooter>,
    /// 深色模式切换标签（移动端）
    pub dark_mode_switch_label: Option<String>,
    /// 浅色模式切换标题
    pub light_mode_switch_title: Option<String>,
    /// 深色模式切换标题
    pub dark_mode_switch_title: Option<String>,
    /// 侧边栏菜单标签（移动端）
    pub sidebar_menu_label: Option<String>,
    /// 返回顶部标签（移动端）
    pub return_to_top_label: Option<String>,
    /// 语言菜单标签
    pub lang_menu_label: Option<String>,
    /// 跳转到内容标签
    pub skip_to_content_label: Option<String>,
    /// 是否显示外部链接图标
    pub external_link_icon: Option<bool>,
    /// 自定义配置
    pub custom: Option<HashMap<String, String>>,
}

/// 站点标题配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SiteTitle {
    /// 标题文本
    String(String),
    /// 禁用标题
    Disabled(bool),
}

/// 侧边栏位置配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Aside {
    /// 布尔值，启用/禁用
    Boolean(bool),
    /// 左侧
    Left(String),
}

/// 大纲配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OutlineConfig {
    /// 禁用
    Disabled(bool),
    /// 层级
    Level(OutlineLevel),
    /// 完整配置
    Config(Outline),
}

/// 锚点插件配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AnchorPluginOptions {
    /// 标题级别
    pub level: Option<Vec<u8>>,
    /// 永久链接配置
    pub permalink: Option<String>,
}

/// 资源插件配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AssetsPluginOptions {
    /// 资源路径处理函数
    pub handle_assets_path: Option<String>,
}

/// 表情插件配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EmojiPluginOptions {
    /// 表情映射
    pub defs: Option<HashMap<String, String>>,
    /// 表情别名
    pub aliases: Option<HashMap<String, String>>,
}

/// 前置内容插件配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FrontmatterPluginOptions {
    /// 前置内容解析函数
    pub parse: Option<String>,
}

/// 标题插件配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HeadersPluginOptions {
    /// 标题级别
    pub level: Option<Vec<u8>>,
}

/// 导入代码插件配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ImportCodePluginOptions {
    /// 导入路径处理函数
    pub handle_import_path: Option<String>,
}

/// 链接插件配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct LinksPluginOptions {
    /// 内部链接标签
    pub internal_tag: Option<String>,
    /// 外部链接属性
    pub external_attrs: Option<HashMap<String, String>>,
}

/// SFC插件配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SfcPluginOptions {
    /// 是否启用SFC
    pub enabled: Option<bool>,
}

/// 目录插件配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TocPluginOptions {
    /// 目录级别
    pub level: Option<Vec<u8>>,
}

/// 代码块v-pre指令配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CodeBlockVPreOptions {
    /// 块级代码v-pre
    pub block: Option<bool>,
    /// 行内代码v-pre
    pub inline: Option<bool>,
}

/// Markdown配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MarkdownConfig {
    /// 锚点插件配置
    pub anchor: Option<AnchorPluginOptions>,
    /// 资源插件配置
    pub assets: Option<AssetsPluginOptions>,
    /// 组件插件配置
    pub component: Option<bool>,
    /// 表情插件配置
    pub emoji: Option<EmojiPluginOptions>,
    /// 前置内容插件配置
    pub frontmatter: Option<FrontmatterPluginOptions>,
    /// 标题插件配置
    pub headers: Option<HeadersPluginOptions>,
    /// 导入代码插件配置
    pub import_code: Option<ImportCodePluginOptions>,
    /// 链接插件配置
    pub links: Option<LinksPluginOptions>,
    /// SFC插件配置
    pub sfc: Option<SfcPluginOptions>,
    /// 标题插件配置
    pub title: Option<bool>,
    /// 目录插件配置
    pub toc: Option<TocPluginOptions>,
    /// 代码块v-pre指令
    pub v_pre: Option<CodeBlockVPreOptions>,
    /// 是否启用行号
    pub line_numbers: Option<bool>,
    /// 代码主题
    pub code_theme: Option<String>,
    /// 自定义配置
    pub custom: Option<HashMap<String, String>>,
}

/// 构建配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BuildConfig {
    /// 源目录
    pub src_dir: Option<String>,
    /// 源文件排除模式
    pub src_exclude: Option<Vec<String>>,
    /// 输出目录
    pub out_dir: Option<String>,
    /// 资源目录
    pub assets_dir: Option<String>,
    /// 缓存目录
    pub cache_dir: Option<String>,
    /// 是否忽略死链接
    pub ignore_dead_links: Option<IgnoreDeadLinks>,
    /// 是否提取元数据到单独的块
    pub meta_chunk: Option<bool>,
    /// 是否启用 MPA 模式
    pub mpa: Option<bool>,
    /// 是否启用清理
    pub clean: Option<bool>,
    /// 是否启用压缩
    pub minify: Option<bool>,
}

/// 忽略死链接配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IgnoreDeadLinks {
    /// 布尔值
    Boolean(bool),
    /// 仅忽略 localhost 链接
    LocalhostLinks(String),
    /// 自定义规则列表
    List(Vec<IgnoreDeadLinkRule>),
}

/// 忽略死链接规则
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IgnoreDeadLinkRule {
    /// 精确 URL
    String(String),
    /// 正则表达式
    Regex(String),
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
    /// 站点语言
    pub lang: Option<String>,
    /// 站点标题
    pub title: Option<String>,
    /// 头部标签配置
    pub head: Option<Vec<HeadConfig>>,
    /// 导航栏配置（语言特定）
    pub nav: Option<Vec<NavItem>>,
    /// 侧边栏配置（语言特定）
    pub sidebar: Option<Sidebar>,
}

/// 插件配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PluginConfig {
    /// 插件名称
    pub name: String,
    /// 插件配置
    pub options: Option<HashMap<String, String>>,
}

/// VuTeX 主配置，完全兼容 VitePress
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct VutexConfig {
    /// 基础路径
    pub base: Option<String>,
    /// 站点语言
    pub lang: Option<String>,
    /// 站点标题
    pub title: Option<String>,
    /// 标题模板
    pub title_template: Option<TitleTemplate>,
    /// 站点描述
    pub description: Option<String>,
    /// 头部标签配置
    pub head: Option<Vec<HeadConfig>>,
    /// 语言配置
    pub locales: Option<HashMap<String, LocaleConfig>>,
    /// 外观配置
    pub appearance: Option<Appearance>,
    /// 是否启用最后更新时间
    pub last_updated: Option<bool>,
    /// 主题配置
    pub theme_config: Option<ThemeConfig>,
    /// 插件配置
    pub plugins: Option<Vec<PluginConfig>>,
    /// Markdown 配置
    pub markdown: Option<MarkdownConfig>,
    /// 构建配置
    pub build: Option<BuildConfig>,
    /// 源目录（构建配置的简写）
    pub src_dir: Option<String>,
    /// 源文件排除模式（构建配置的简写）
    pub src_exclude: Option<Vec<String>>,
    /// 输出目录（构建配置的简写）
    pub out_dir: Option<String>,
    /// 资源目录（构建配置的简写）
    pub assets_dir: Option<String>,
    /// 缓存目录（构建配置的简写）
    pub cache_dir: Option<String>,
    /// 临时文件目录
    pub temp: Option<String>,
    /// 公共文件目录
    pub public: Option<String>,
    /// 调试模式
    pub debug: Option<bool>,
    /// 页面文件模式
    pub page_patterns: Option<Vec<String>>,
    /// 永久链接模式
    pub permalink_pattern: Option<String>,
    /// 开发服务器主机
    pub host: Option<String>,
    /// 开发服务器端口
    pub port: Option<u16>,
    /// 是否自动打开浏览器
    pub open: Option<bool>,
    /// 是否使用简洁 URL
    pub clean_urls: Option<bool>,
    /// 路由重写规则
    pub rewrites: Option<HashMap<String, String>>,
    /// Vite 配置
    pub vite: Option<serde_json::Value>,
    /// Vue 配置
    pub vue: Option<serde_json::Value>,
}

/// 标题模板配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TitleTemplate {
    /// 模板字符串
    String(String),
    /// 禁用标题后缀
    Disabled(bool),
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            i18n_routing: None,
            logo: None,
            site_title: None,
            nav: None,
            sidebar: None,
            aside: None,
            outline: None,
            social_links: None,
            footer: None,
            edit_link: None,
            last_updated: None,
            algolia: None,
            carbon_ads: None,
            doc_footer: None,
            dark_mode_switch_label: None,
            light_mode_switch_title: None,
            dark_mode_switch_title: None,
            sidebar_menu_label: None,
            return_to_top_label: None,
            lang_menu_label: None,
            skip_to_content_label: None,
            external_link_icon: None,
            custom: None,
        }
    }
}

impl Default for MarkdownConfig {
    fn default() -> Self {
        Self {
            anchor: Some(AnchorPluginOptions {
                level: Some(vec![1, 2, 3, 4, 5, 6]),
                permalink: Some("headerLink".to_string()),
            }),
            assets: None,
            component: None,
            emoji: None,
            frontmatter: None,
            headers: Some(HeadersPluginOptions {
                level: Some(vec![2, 3]),
            }),
            import_code: None,
            links: Some(LinksPluginOptions {
                internal_tag: Some("RouteLink".to_string()),
                external_attrs: Some(HashMap::from([
                    ("target".to_string(), "_blank".to_string()),
                    ("rel".to_string(), "noopener noreferrer".to_string()),
                ])),
            }),
            sfc: None,
            title: None,
            toc: Some(TocPluginOptions {
                level: Some(vec![2, 3]),
            }),
            v_pre: Some(CodeBlockVPreOptions {
                block: Some(true),
                inline: Some(true),
            }),
            line_numbers: Some(false),
            code_theme: None,
            custom: None,
        }
    }
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            src_dir: Some(".".to_string()),
            src_exclude: None,
            out_dir: Some("./.vitepress/dist".to_string()),
            assets_dir: Some("assets".to_string()),
            cache_dir: Some("./.vitepress/cache".to_string()),
            ignore_dead_links: Some(IgnoreDeadLinks::Boolean(false)),
            meta_chunk: Some(false),
            mpa: Some(false),
            clean: Some(true),
            minify: Some(true),
        }
    }
}

impl VutexConfig {
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

    /// 设置站点语言
    pub fn with_lang(mut self, lang: String) -> Self {
        self.lang = Some(lang);
        self
    }

    /// 设置站点描述
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

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
        #[cfg(feature = "serde")]
        {
            let config: Self = serde_json::from_str(json_str).map_err(|e| ConfigError::json_parse_error(e.to_string()))?;
            config.validate()?;
            Ok(config)
        }
        #[cfg(not(feature = "serde"))]
        {
            Err(ConfigError::json_parse_error("serde feature not enabled".to_string()))
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
            let config: Self = oak_toml::from_str(toml_str).map_err(|e| ConfigError::toml_parse_error(e.to_string()))?;
            config.validate()?;
            Ok(config)
        }
        #[cfg(not(feature = "serde"))]
        {
            Err(ConfigError::toml_parse_error("serde feature not enabled".to_string()))
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
            serde_json::to_string(self).map_err(|e| ConfigError::json_parse_error(e.to_string()))
        }
        #[cfg(not(feature = "serde"))]
        {
            Err(ConfigError::json_parse_error("serde feature not enabled".to_string()))
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
            oak_toml::to_string(self).map_err(|e| ConfigError::toml_parse_error(e.to_string()))
        }
        #[cfg(not(feature = "serde"))]
        {
            Err(ConfigError::toml_parse_error("serde feature not enabled".to_string()))
        }
    }
}

impl ConfigValidation for VutexConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        if let Some(base) = &self.base {
            if base.is_empty() {
                return Err(ConfigError::validation_error("Base URL cannot be empty".to_string()));
            }
            if !base.starts_with('/') {
                return Err(ConfigError::validation_error("Base URL must start with a slash".to_string()));
            }
        }

        if let Some(title) = &self.title {
            if title.is_empty() {
                return Err(ConfigError::validation_error("Title cannot be empty".to_string()));
            }
        }

        if let Some(lang) = &self.lang {
            if lang.is_empty() {
                return Err(ConfigError::validation_error("Lang cannot be empty".to_string()));
            }
        }

        if let Some(port) = &self.port {
            if *port == 0 || *port > 65535 {
                return Err(ConfigError::validation_error("Port must be between 1 and 65535".to_string()));
            }
        }

        Ok(())
    }
}

impl ConfigValidation for ThemeConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        Ok(())
    }
}

impl ConfigValidation for MarkdownConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        Ok(())
    }
}

impl ConfigValidation for BuildConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        Ok(())
    }
}
