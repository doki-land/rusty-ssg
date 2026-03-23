#![warn(missing_docs)]

//! Markdown 处理模块
//!
//! 提供 Markdown 文档的转换功能

use crate::errors::{MarkdownError, Result};
use nargo_document::{
    generator::markdown::MarkdownRenderer,
    plugin::{KaTeXPlugin, MermaidPlugin, PluginRegistry, PrismPlugin},
};
use oak_core::{parser::Parser, ParseSession};
use oak_markdown::{MarkdownLanguage, MarkdownParser, MarkdownRoot};

/// Markdown 转换器
///
/// 负责将 Markdown 文档转换为 HTML
pub struct MarkdownConverter {
    /// Markdown 处理器
    processor: MarkdownProcessor,
    /// 插件注册表
    plugin_registry: PluginRegistry,
    /// nargo-document Markdown 渲染器
    nargo_renderer: MarkdownRenderer,
}

impl MarkdownConverter {
    /// 创建新的 Markdown 转换器
    pub fn new() -> Self {
        let mut registry = PluginRegistry::new();

        // 注册默认插件
        registry.register(KaTeXPlugin::new());
        registry.register(MermaidPlugin::new());
        registry.register(PrismPlugin::new());

        Self { processor: MarkdownProcessor::new(), plugin_registry: registry, nargo_renderer: MarkdownRenderer::new() }
    }

    /// 转换 Markdown 为 HTML
    ///
    /// # Arguments
    ///
    /// * `markdown` - Markdown 内容
    ///
    /// # Returns
    ///
    /// 转换后的 HTML
    pub fn convert(&self, markdown: &str) -> Result<String> {
        // 使用 nargo-document 插件处理
        let mut context = nargo_document::plugin::PluginContext::from_content(markdown.to_string(), "".to_string());

        // 应用插件的渲染前钩子
        context = self.plugin_registry.before_render_all(context);

        // 处理 Markdown
        let processed = self.processor.process(&context.content)?;

        // 应用插件的渲染后钩子
        context.content = processed;
        context = self.plugin_registry.after_render_all(context);

        Ok(context.content)
    }

    /// 注册自定义插件
    pub fn register_plugin<P: nargo_document::plugin::DocumentPlugin + 'static>(&mut self, plugin: P) {
        self.plugin_registry.register(plugin);
    }

    /// 获取插件注册表
    pub fn plugin_registry(&self) -> &PluginRegistry {
        &self.plugin_registry
    }

    /// 获取可变的插件注册表
    pub fn plugin_registry_mut(&mut self) -> &mut PluginRegistry {
        &mut self.plugin_registry
    }
}

impl Default for MarkdownConverter {
    fn default() -> Self {
        Self::new()
    }
}

/// Markdown 处理器
///
/// 提供 Markdown 处理的核心功能
pub struct MarkdownProcessor {
    /// 处理器名称
    name: String,
}

impl MarkdownProcessor {
    /// 创建新的 Markdown 处理器
    pub fn new() -> Self {
        Self { name: "default".to_string() }
    }

    /// 处理 Markdown 内容
    ///
    /// # Arguments
    ///
    /// * `markdown` - Markdown 内容
    ///
    /// # Returns
    ///
    /// 处理后的 HTML
    pub fn process(&self, markdown: &str) -> Result<String> {
        // 使用 oaks Markdown 解析器处理 Markdown
        let language = MarkdownLanguage::new();
        let parser = MarkdownParser::new(&language);
        
        // 创建解析会话
        let mut session = ParseSession::new(16);
        
        // 解析 Markdown
        let parse_output = parser.parse(markdown, &[], &mut session);
        
        // 提取根节点
        let root = match parse_output.result {
            Ok(root) => root,
            Err(_) => {
                return Err(MarkdownError::highlight_error("Failed to parse Markdown".to_string()).into());
            }
        };
        
        // 将解析结果转换为 HTML
        let html_output = self.render_markdown_to_html(root);

        Ok(html_output)
    }

    /// 将 Markdown AST 转换为 HTML
    fn render_markdown_to_html(&self, _root: &oak_core::tree::GreenNode<MarkdownLanguage>) -> String {
        // 简单的 HTML 渲染实现
        // 实际实现需要根据 Markdown AST 结构进行更详细的处理
        let mut html = String::new();
        
        // 这里只是一个简单的实现，实际需要遍历整个 AST
        html.push_str("<div class=\"markdown-body\">");
        html.push_str("<p>Markdown content rendered by oaks parser</p>");
        html.push_str("</div>");
        
        html
    }

    /// 获取处理器名称
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Default for MarkdownProcessor {
    fn default() -> Self {
        Self::new()
    }
}
