//! HTML 渲染器模块
//! 提供将 Markdown 文本渲染为 HTML 的功能

use oak_core::{Builder, ParseSession};
use oak_markdown::{
    MarkdownBuilder, MarkdownLanguage,
    ast::{Block, Blockquote, CodeBlock, Heading, Html, Inline, List, ListItem, MarkdownRoot, Paragraph, Table, TableCell},
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
    /// Markdown 语言配置
    lang_config: MarkdownLanguage,
}

impl HtmlRenderer {
    /// 创建新的 HTML 渲染器
    pub fn new() -> Self {
        Self { config: HtmlRendererConfig::default(), lang_config: MarkdownLanguage::default() }
    }

    /// 创建带配置的 HTML 渲染器
    ///
    /// # Arguments
    ///
    /// * `config` - 渲染器配置
    pub fn with_config(config: HtmlRendererConfig) -> Self {
        let mut lang_config = MarkdownLanguage::default();
        lang_config.allow_tables = config.enable_tables;
        lang_config.allow_footnotes = config.enable_footnotes;
        lang_config.allow_strikethrough = config.enable_strikethrough;
        lang_config.allow_task_lists = config.enable_tasklists;

        Self { config, lang_config }
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
        let builder = MarkdownBuilder::new(&self.lang_config);
        let mut cache = ParseSession::default();
        let build_output = builder.build(markdown, &[], &mut cache);

        match build_output.result {
            Ok(ast_root) => self.render_ast(&ast_root),
            Err(_) => self.escape_html(markdown),
        }
    }

    /// 渲染 Markdown AST 为 HTML
    ///
    /// # Arguments
    ///
    /// * `root` - Markdown AST 根节点
    ///
    /// # Returns
    ///
    /// 渲染后的 HTML 字符串
    fn render_ast(&self, root: &MarkdownRoot) -> String {
        let mut html = String::new();

        for block in &root.blocks {
            html.push_str(&self.render_block(block));
        }

        html
    }

    /// 渲染块级元素
    ///
    /// # Arguments
    ///
    /// * `block` - 块级元素
    ///
    /// # Returns
    ///
    /// 渲染后的 HTML 字符串
    fn render_block(&self, block: &Block) -> String {
        match block {
            Block::Heading(heading) => self.render_heading(heading),
            Block::Paragraph(paragraph) => self.render_paragraph(paragraph),
            Block::CodeBlock(code_block) => self.render_code_block(code_block),
            Block::List(list) => self.render_list(list),
            Block::Blockquote(blockquote) => self.render_blockquote(blockquote),
            Block::HorizontalRule(_) => "<hr />\n".to_string(),
            Block::Table(table) => self.render_table(table),
            Block::Html(html) => self.render_html(html),
            Block::AbbreviationDefinition(_) => String::new(),
        }
    }

    /// 渲染标题
    ///
    /// # Arguments
    ///
    /// * `heading` - 标题元素
    ///
    /// # Returns
    ///
    /// 渲染后的 HTML 字符串
    fn render_heading(&self, heading: &Heading) -> String {
        let tag = format!("h{}", heading.level);
        let escaped_content = self.escape_html(&heading.content);
        format!("<{}>{}</{}>\n", tag, escaped_content, tag)
    }

    /// 渲染段落
    ///
    /// # Arguments
    ///
    /// * `paragraph` - 段落元素
    ///
    /// # Returns
    ///
    /// 渲染后的 HTML 字符串
    fn render_paragraph(&self, paragraph: &Paragraph) -> String {
        let escaped_content = self.escape_html(&paragraph.content);
        format!("<p>{}</p>\n", escaped_content)
    }

    /// 渲染代码块
    ///
    /// # Arguments
    ///
    /// * `code_block` - 代码块元素
    ///
    /// # Returns
    ///
    /// 渲染后的 HTML 字符串
    fn render_code_block(&self, code_block: &CodeBlock) -> String {
        let class = if let Some(lang) = &code_block.language {
            format!(" class=\"language-{}\"", self.escape_html(lang))
        }
        else {
            String::new()
        };
        let escaped_content = self.escape_html(&code_block.content);
        format!("<pre><code{}>{}</code></pre>\n", class, escaped_content)
    }

    /// 渲染列表
    ///
    /// # Arguments
    ///
    /// * `list` - 列表元素
    ///
    /// # Returns
    ///
    /// 渲染后的 HTML 字符串
    fn render_list(&self, list: &List) -> String {
        let tag = if list.is_ordered { "ol" } else { "ul" };
        let mut html = format!("<{}>\n", tag);

        for item in &list.items {
            html.push_str(&self.render_list_item(item));
        }

        html.push_str(&format!("</{}>\n", tag));
        html
    }

    /// 渲染列表项
    ///
    /// # Arguments
    ///
    /// * `list_item` - 列表项元素
    ///
    /// # Returns
    ///
    /// 渲染后的 HTML 字符串
    fn render_list_item(&self, list_item: &ListItem) -> String {
        let mut html = String::from("<li>");

        if list_item.is_task {
            let checked = if list_item.is_checked.unwrap_or(false) { "checked" } else { "" };
            html.push_str(&format!("<input type=\"checkbox\" disabled {} /> ", checked));
        }

        for block in &list_item.content {
            html.push_str(&self.render_block(block));
        }

        html.push_str("</li>\n");
        html
    }

    /// 渲染引用块
    ///
    /// # Arguments
    ///
    /// * `blockquote` - 引用块元素
    ///
    /// # Returns
    ///
    /// 渲染后的 HTML 字符串
    fn render_blockquote(&self, blockquote: &Blockquote) -> String {
        let mut html = String::from("<blockquote>\n");

        for block in &blockquote.content {
            html.push_str(&self.render_block(block));
        }

        html.push_str("</blockquote>\n");
        html
    }

    /// 渲染表格
    ///
    /// # Arguments
    ///
    /// * `table` - 表格元素
    ///
    /// # Returns
    ///
    /// 渲染后的 HTML 字符串
    fn render_table(&self, table: &Table) -> String {
        let mut html = String::from("<table>\n");

        html.push_str("<thead>\n<tr>\n");
        for cell in &table.header.cells {
            html.push_str(&self.render_table_cell(cell, "th"));
        }
        html.push_str("</tr>\n</thead>\n");

        html.push_str("<tbody>\n");
        for row in &table.rows {
            html.push_str("<tr>\n");
            for cell in &row.cells {
                html.push_str(&self.render_table_cell(cell, "td"));
            }
            html.push_str("</tr>\n");
        }
        html.push_str("</tbody>\n");

        html.push_str("</table>\n");
        html
    }

    /// 渲染表格单元格
    ///
    /// # Arguments
    ///
    /// * `cell` - 表格单元格元素
    /// * `tag` - 单元格标签 (th 或 td)
    ///
    /// # Returns
    ///
    /// 渲染后的 HTML 字符串
    fn render_table_cell(&self, cell: &TableCell, tag: &str) -> String {
        let escaped_content = self.escape_html(&cell.content);
        format!("<{}>{}</{}>\n", tag, escaped_content, tag)
    }

    /// 渲染 HTML 块
    ///
    /// # Arguments
    ///
    /// * `html` - HTML 块元素
    ///
    /// # Returns
    ///
    /// 渲染后的 HTML 字符串
    fn render_html(&self, html: &Html) -> String {
        format!("{}\n", html.content)
    }

    /// 渲染内联元素
    ///
    /// # Arguments
    ///
    /// * `inline` - 内联元素
    ///
    /// # Returns
    ///
    /// 渲染后的 HTML 字符串
    fn render_inline(&self, inline: &Inline) -> String {
        match inline {
            Inline::Text(text) => self.escape_html(text),
            Inline::Bold(text) => format!("<strong>{}</strong>", self.escape_html(text)),
            Inline::Italic(text) => format!("<em>{}</em>", self.escape_html(text)),
            Inline::Code(text) => format!("<code>{}</code>", self.escape_html(text)),
            Inline::Link { text, url, title } => {
                let title_attr =
                    if let Some(t) = title { format!(" title=\"{}\"", self.escape_html(t)) } else { String::new() };
                format!("<a href=\"{}\"{}>{}</a>", self.escape_html(url), title_attr, self.escape_html(text))
            }
            Inline::Image { alt, url, title } => {
                let title_attr =
                    if let Some(t) = title { format!(" title=\"{}\"", self.escape_html(t)) } else { String::new() };
                format!("<img src=\"{}\" alt=\"{}\"{} />", self.escape_html(url), self.escape_html(alt), title_attr)
            }
            Inline::Abbreviation { key, .. } => key.clone(),
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
