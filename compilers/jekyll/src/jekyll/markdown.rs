#![warn(missing_docs)]

//! Markdown 处理模块
//!
//! 提供 Markdown 文档的转换功能

use crate::errors::Result;
use nargo_document::plugin::{KaTeXPlugin, MermaidPlugin, PluginRegistry, PrismPlugin};
use oak_core::{Builder, parser::session::ParseSession};
use oak_markdown::{
    MarkdownBuilder, MarkdownLanguage,
    ast::{Block, Blockquote, CodeBlock, Heading, Html, List, ListItem, MarkdownRoot, Paragraph, Table, TableCell, TableRow},
};

/// Markdown 转换器
///
/// 负责将 Markdown 文档转换为 HTML
pub struct MarkdownConverter {
    /// 插件注册表
    plugin_registry: PluginRegistry,
    /// Markdown 语言配置
    lang_config: MarkdownLanguage,
}

impl MarkdownConverter {
    /// 创建新的 Markdown 转换器
    pub fn new() -> Self {
        let mut registry = PluginRegistry::new();

        registry.register(KaTeXPlugin::new());
        registry.register(MermaidPlugin::new());
        registry.register(PrismPlugin::new());

        Self {
            plugin_registry: registry,
            lang_config: MarkdownLanguage::default(),
        }
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
        let mut context = nargo_document::plugin::PluginContext::from_content(markdown.to_string(), "".to_string());

        context = self.plugin_registry.before_render_all(context);

        let processed_content = context.content;

        let html = self.render_markdown_with_oak(&processed_content)?;

        context.content = html;
        context = self.plugin_registry.after_render_all(context);

        Ok(context.content)
    }

    /// 使用 oak-markdown 渲染 Markdown
    fn render_markdown_with_oak(&self, markdown: &str) -> Result<String> {
        let builder = MarkdownBuilder::new(&self.lang_config);
        let mut cache = ParseSession::default();
        let build_output = builder.build(markdown, &[], &mut cache);

        let ast = build_output.result.map_err(|e| crate::errors::JekyllError::FrontMatterParseError(e.to_string()))?;

        let html = self.render_ast(&ast);
        Ok(format!(r#"<div class="markdown-body">{}</div>"#, html))
    }

    /// 渲染 Markdown AST
    fn render_ast(&self, ast: &MarkdownRoot) -> String {
        let mut html = String::new();

        for block in &ast.blocks {
            html.push_str(&self.render_block(block));
        }

        html
    }

    /// 渲染块级元素
    fn render_block(&self, block: &Block) -> String {
        match block {
            Block::Heading(heading) => self.render_heading(heading),
            Block::Paragraph(paragraph) => self.render_paragraph(paragraph),
            Block::List(list) => self.render_list(list),
            Block::CodeBlock(code_block) => self.render_code_block(code_block),
            Block::Blockquote(blockquote) => self.render_blockquote(blockquote),
            Block::HorizontalRule(_) => self.render_horizontal_rule(),
            Block::Table(table) => self.render_table(table),
            Block::Html(html) => self.render_html(html),
            Block::AbbreviationDefinition(_) => String::new(),
        }
    }

    /// 渲染水平线
    fn render_horizontal_rule(&self) -> String {
        r#"<hr />"#.to_string()
    }

    /// 渲染表格
    fn render_table(&self, table: &Table) -> String {
        let mut html = String::from(r#"<table>"#);

        html.push_str(r#"<thead><tr>"#);
        for cell in &table.header.cells {
            html.push_str(&format!(r#"<th>{}</th>"#, self.escape_html(&cell.content)));
        }
        html.push_str(r#"</tr></thead>"#);

        html.push_str(r#"<tbody>"#);
        for row in &table.rows {
            html.push_str(r#"<tr>"#);
            for cell in &row.cells {
                html.push_str(&format!(r#"<td>{}</td>"#, self.escape_html(&cell.content)));
            }
            html.push_str(r#"</tr>"#);
        }
        html.push_str(r#"</tbody>"#);

        html.push_str(r#"</table>"#);
        html
    }

    /// 渲染 HTML 块
    fn render_html(&self, html_block: &Html) -> String {
        html_block.content.to_string()
    }

    /// 渲染标题
    fn render_heading(&self, heading: &Heading) -> String {
        let tag = format!(r#"h{}"#, heading.level);
        let content = self.escape_html(&heading.content);
        format!(r#"<{}>{}</{}>"#, tag, content, tag)
    }

    /// 渲染段落
    fn render_paragraph(&self, paragraph: &Paragraph) -> String {
        let content = self.escape_html(&paragraph.content);
        format!(r#"<p>{}</p>"#, content)
    }

    /// 渲染列表
    fn render_list(&self, list: &List) -> String {
        let tag = if list.is_ordered { "ol" } else { "ul" };
        let mut html = format!(r#"<{}>"#, tag);

        for item in &list.items {
            html.push_str(&self.render_list_item(item));
        }

        html.push_str(&format!(r#"</{}>"#, tag));
        html
    }

    /// 渲染列表项
    fn render_list_item(&self, list_item: &ListItem) -> String {
        let mut html = String::from(r#"<li>"#);

        if list_item.is_task {
            let checked = if list_item.is_checked.unwrap_or(false) { "checked" } else { "" };
            html.push_str(&format!(r#"<input type="checkbox" disabled {} /> "#, checked));
        }

        for block in &list_item.content {
            html.push_str(&self.render_block(block));
        }

        html.push_str(r#"</li>"#);
        html
    }

    /// 渲染代码块
    fn render_code_block(&self, code_block: &CodeBlock) -> String {
        let class = if let Some(lang) = &code_block.language {
            format!(r#" class="language-{}""#, self.escape_html(lang))
        }
        else {
            String::new()
        };
        let content = self.escape_html(&code_block.content);
        format!(r#"<pre><code{}>{}</code></pre>"#, class, content)
    }

    /// 渲染引用块
    fn render_blockquote(&self, blockquote: &Blockquote) -> String {
        let mut content = String::new();
        for block in &blockquote.content {
            content.push_str(&self.render_block(block));
        }
        format!(r#"<blockquote>{}</blockquote>"#, content)
    }

    /// 转义 HTML 特殊字符
    fn escape_html(&self, text: &str) -> String {
        let mut result = String::with_capacity(text.len() * 2);
        for c in text.chars() {
            match c {
                '&' => result.push_str("&amp;"),
                '<' => result.push_str("&lt;"),
                '>' => result.push_str("&gt;"),
                '"' => result.push_str("&quot;"),
                '\'' => result.push_str("&#39;"),
                _ => result.push(c),
            }
        }
        result
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

    /// 获取 Markdown 语言配置
    pub fn lang_config(&self) -> &MarkdownLanguage {
        &self.lang_config
    }

    /// 获取可变的 Markdown 语言配置
    pub fn lang_config_mut(&mut self) -> &MarkdownLanguage {
        &mut self.lang_config
    }
}

impl Default for MarkdownConverter {
    fn default() -> Self {
        Self::new()
    }
}
