//! HTML 渲染器模块
//! 提供将 Markdown 文本渲染为 HTML 的功能

use nargo_document::generator::markdown::{
    MarkdownRenderer as NargoMarkdownRenderer, MarkdownRendererConfig as NargoMarkdownRendererConfig,
};

/// HTML 渲染器配置
#[derive(Debug, Clone)]
pub struct HtmlRendererConfig {
    /// 是否启用表格支持
    pub enable_tables: bool,
    /// 是否启用脚注支持
    pub enable_footnotes: bool,
    /// 是否启用删除线支持
    pub enable_strikethrough: bool,
    /// 是否启用任务列表支持
    pub enable_tasklists: bool,
    /// 是否启用智能标点
    pub enable_smart_punctuation: bool,
}

impl Default for HtmlRendererConfig {
    fn default() -> Self {
        Self {
            enable_tables: true,
            enable_footnotes: true,
            enable_strikethrough: true,
            enable_tasklists: true,
            enable_smart_punctuation: true,
        }
    }
}

/// HTML 渲染器
#[derive(Debug, Clone)]
pub struct HtmlRenderer {
    /// 渲染器配置
    config: HtmlRendererConfig,
    /// 内部使用的 nargo-document Markdown 渲染器
    markdown_renderer: NargoMarkdownRenderer,
}

impl HtmlRenderer {
    /// 创建新的 HTML 渲染器
    pub fn new() -> Self {
        let config = HtmlRendererConfig::default();
        let nargo_config = NargoMarkdownRendererConfig {
            enable_tables: config.enable_tables,
            enable_footnotes: config.enable_footnotes,
            enable_strikethrough: config.enable_strikethrough,
            enable_tasklists: config.enable_tasklists,
            enable_smart_punctuation: config.enable_smart_punctuation,
        };
        let markdown_renderer = NargoMarkdownRenderer::with_config(nargo_config);

        Self { config, markdown_renderer }
    }

    /// 创建带配置的 HTML 渲染器
    ///
    /// # Arguments
    ///
    /// * `config` - 渲染器配置
    pub fn with_config(config: HtmlRendererConfig) -> Self {
        let nargo_config = NargoMarkdownRendererConfig {
            enable_tables: config.enable_tables,
            enable_footnotes: config.enable_footnotes,
            enable_strikethrough: config.enable_strikethrough,
            enable_tasklists: config.enable_tasklists,
            enable_smart_punctuation: config.enable_smart_punctuation,
        };
        let markdown_renderer = NargoMarkdownRenderer::with_config(nargo_config);

        Self { config, markdown_renderer }
    }

    /// 获取渲染器配置
    pub fn config(&self) -> &HtmlRendererConfig {
        &self.config
    }

    /// 获取可变的渲染器配置
    pub fn config_mut(&mut self) -> &mut HtmlRendererConfig {
        &mut self.config
    }

    /// 将 Markdown 文本渲染为 HTML
    ///
    /// # Arguments
    ///
    /// * `markdown` - Markdown 文本内容
    ///
    /// # Returns
    ///
    /// 渲染后的 HTML 字符串
    pub fn render(&self, markdown: &str) -> String {
        match self.markdown_renderer.render(markdown) {
            Ok(html) => html,
            Err(_) => {
                // 当渲染失败时，返回转义后的原始文本
                self.escape_html(markdown)
            }
        }
    }

    /// 转义 HTML 特殊字符
    ///
    /// # Arguments
    ///
    /// * `text` - 原始文本
    ///
    /// # Returns
    ///
    /// 转义后的文本
    fn escape_html(&self, text: &str) -> String {
        text.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;").replace('\'', "&#39;")
    }
}

impl Default for HtmlRenderer {
    fn default() -> Self {
        Self::new()
    }
}
