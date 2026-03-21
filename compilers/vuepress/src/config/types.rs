//! VuePress 配置类型定义

use serde_json;

/// 头部标签配置
#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct HeadConfig {
    /// 标签名称
    pub tag_name: String,
    /// 标签属性
    pub attrs: std::collections::HashMap<String, String>,
    /// 标签内容（可选）
    pub inner_html: Option<String>,
}

/// 站点本地化数据
#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct SiteLocaleData {
    /// 站点语言
    pub lang: String,
    /// 站点标题
    pub title: String,
    /// 站点描述
    pub description: String,
    /// 头部标签配置
    pub head: Vec<HeadConfig>,
}

/// 锚点插件配置
#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct AnchorPluginOptions {
    /// 标题级别
    pub level: Vec<u8>,
    /// 永久链接配置
    pub permalink: Option<String>,
}

/// 资源插件配置
#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct AssetsPluginOptions {
    /// 资源路径处理函数
    pub handle_assets_path: Option<String>,
}

/// 表情插件配置
#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct EmojiPluginOptions {
    /// 表情映射
    pub defs: Option<std::collections::HashMap<String, String>>,
    /// 表情别名
    pub aliases: Option<std::collections::HashMap<String, String>>,
}

/// 前置内容插件配置
#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct FrontmatterPluginOptions {
    /// 前置内容解析函数
    pub parse: Option<String>,
}

/// 标题插件配置
#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct HeadersPluginOptions {
    /// 标题级别
    pub level: Vec<u8>,
}

/// 导入代码插件配置
#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct ImportCodePluginOptions {
    /// 导入路径处理函数
    pub handle_import_path: Option<String>,
}

/// 链接插件配置
#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct LinksPluginOptions {
    /// 内部链接标签
    pub internal_tag: String,
    /// 外部链接属性
    pub external_attrs: std::collections::HashMap<String, String>,
}

/// SFC插件配置
#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct SfcPluginOptions {
    /// 是否启用SFC
    pub enabled: bool,
}

/// 目录插件配置
#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct TocPluginOptions {
    /// 目录级别
    pub level: Vec<u8>,
}

/// 代码块v-pre指令配置
#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct CodeBlockVPreOptions {
    /// 块级代码v-pre
    pub block: bool,
    /// 行内代码v-pre
    pub inline: bool,
}

/// Markdown配置
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct MarkdownOptions {
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
}

impl Default for MarkdownOptions {
    fn default() -> Self {
        Self {
            anchor: Some(AnchorPluginOptions { level: vec![1, 2, 3, 4, 5, 6], permalink: Some("headerLink".to_string()) }),
            assets: None,
            component: None,
            emoji: None,
            frontmatter: None,
            headers: Some(HeadersPluginOptions { level: vec![2, 3] }),
            import_code: None,
            links: Some(LinksPluginOptions {
                internal_tag: "RouteLink".to_string(),
                external_attrs: std::collections::HashMap::from([
                    ("target".to_string(), "_blank".to_string()),
                    ("rel".to_string(), "noopener noreferrer".to_string()),
                ]),
            }),
            sfc: None,
            title: None,
            toc: Some(TocPluginOptions { level: vec![2, 3] }),
            v_pre: Some(CodeBlockVPreOptions { block: true, inline: true }),
        }
    }
}

/// 主题配置
#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Theme {
    /// 主题名称
    pub name: String,
    /// 主题选项
    pub options: Option<serde_json::Value>,
}

/// 打包器配置
#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Bundler {
    /// 打包器名称
    pub name: String,
    /// 打包器选项
    pub options: Option<serde_json::Value>,
}

/// 模板渲染器
#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct TemplateRenderer {
    /// 渲染函数
    pub render: String,
}

/// 预加载配置
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum ShouldPreload {
    /// 启用所有
    All(bool),
    /// 自定义函数
    Custom(String),
}

/// 预取配置
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum ShouldPrefetch {
    /// 启用所有
    All(bool),
    /// 自定义函数
    Custom(String),
}

/// VuePress配置结构体
#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct VuePressConfig {
    /// 基础URL
    pub base: Option<String>,
    /// 站点语言
    pub lang: Option<String>,
    /// 站点标题
    pub title: Option<String>,
    /// 站点描述
    pub description: Option<String>,
    /// 头部标签配置
    pub head: Option<Vec<HeadConfig>>,
    /// 站点本地化配置
    pub locales: Option<std::collections::HashMap<String, SiteLocaleData>>,
    /// 主题配置
    pub theme: Option<Theme>,
    /// 打包器配置
    pub bundler: Option<Bundler>,
    /// 输出目录
    pub dest: Option<String>,
    /// 临时文件目录
    pub temp: Option<String>,
    /// 缓存目录
    pub cache: Option<String>,
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
    /// 开发模板路径
    pub template_dev: Option<String>,
    /// 是否预加载
    pub should_preload: Option<ShouldPreload>,
    /// 是否预取
    pub should_prefetch: Option<ShouldPrefetch>,
    /// 构建模板路径
    pub template_build: Option<String>,
    /// 构建模板渲染器
    pub template_build_renderer: Option<TemplateRenderer>,
    /// Markdown配置
    pub markdown: Option<MarkdownOptions>,
    /// 插件配置
    pub plugins: Option<Vec<serde_json::Value>>,
}

impl VuePressConfig {
    /// 创建默认配置
    pub fn new() -> Self {
        Self {
            base: Some("/".to_string()),
            lang: Some("en-US".to_string()),
            title: Some("".to_string()),
            description: Some("".to_string()),
            head: None,
            locales: None,
            theme: None,
            bundler: None,
            dest: Some(".vuepress/dist".to_string()),
            temp: Some(".vuepress/.temp".to_string()),
            cache: Some(".vuepress/.cache".to_string()),
            public: Some(".vuepress/public".to_string()),
            debug: Some(false),
            page_patterns: Some(vec!["**/*.md".to_string(), ".vuepress".to_string(), "node_modules".to_string()]),
            permalink_pattern: None,
            host: Some("0.0.0.0".to_string()),
            port: Some(8080),
            open: Some(false),
            template_dev: Some("@vuepress/client/templates/dev.html".to_string()),
            should_preload: Some(ShouldPreload::All(true)),
            should_prefetch: Some(ShouldPrefetch::All(true)),
            template_build: Some("@vuepress/client/templates/build.html".to_string()),
            template_build_renderer: None,
            markdown: Some(MarkdownOptions::default()),
            plugins: Some(vec![]),
        }
    }
}
