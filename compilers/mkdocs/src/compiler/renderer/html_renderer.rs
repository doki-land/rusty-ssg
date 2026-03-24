//! HTML 渲染器
//!
//! 使用 nargo-document 的 Markdown 渲染器将 Markdown 内容转换为 HTML。

use nargo_document::MarkdownRenderer as NargoMarkdownRenderer;
use std::collections::HashMap;

/// HTML 转义函数
fn html_escape(s: &str) -> String {
    s.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;").replace("\"", "&quot;").replace("'", "&#39;")
}

/// HTML 渲染器配置
#[derive(Debug, Clone)]
pub struct HtmlRendererConfig {
    /// 自定义渲染选项
    pub options: HashMap<String, String>,
}

impl Default for HtmlRendererConfig {
    fn default() -> Self {
        Self { options: HashMap::new() }
    }
}

/// HTML 渲染器
pub struct HtmlRenderer {
    /// 渲染器配置
    config: HtmlRendererConfig,
    /// nargo-document 的 Markdown 渲染器
    markdown_renderer: NargoMarkdownRenderer,
}

impl HtmlRenderer {
    /// 创建新的 HTML 渲染器
    pub fn new() -> Self {
        Self::with_config(HtmlRendererConfig::default())
    }

    /// 使用指定配置创建 HTML 渲染器
    ///
    /// # 参数
    /// * `config` - 渲染器配置
    pub fn with_config(config: HtmlRendererConfig) -> Self {
        Self { config, markdown_renderer: NargoMarkdownRenderer::new() }
    }

    /// 获取渲染器配置
    pub fn config(&self) -> &HtmlRendererConfig {
        &self.config
    }

    /// 获取可变的渲染器配置
    pub fn config_mut(&mut self) -> &mut HtmlRendererConfig {
        &mut self.config
    }

    /// 渲染 Markdown 内容为 HTML
    ///
    /// # 参数
    /// * `content` - Markdown 内容字符串
    ///
    /// # 返回值
    /// 渲染后的 HTML 字符串
    pub fn render(&self, content: &str) -> String {
        // 处理空内容
        if content.trim().is_empty() {
            return "<p></p>".to_string();
        }

        // 使用 nargo-document 渲染 Markdown
        match self.markdown_renderer.render(content) {
            Ok(html) => html,
            Err(_) => {
                // 渲染失败时使用简单的后备方法
                self.render_simple_fallback(content)
            }
        }
    }

    /// 简单的后备渲染方法，在 nargo-document 渲染失败时使用
    ///
    /// # 参数
    /// * `content` - Markdown 内容字符串
    ///
    /// # 返回值
    /// 简单转义后的 HTML 字符串
    fn render_simple_fallback(&self, content: &str) -> String {
        let escaped = content
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#39;");
        format!("<p>{}</p>", escaped.replace("\n\n", "</p><p>"))
    }
}

impl Default for HtmlRenderer {
    fn default() -> Self {
        Self::new()
    }
}
