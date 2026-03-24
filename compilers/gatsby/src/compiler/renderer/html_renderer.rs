//! HTML 渲染器模块
//! 提供 Markdown 到 HTML 的渲染功能

use oak_core::{Builder, ParseSession};
use oak_markdown::{
    MarkdownBuilder, MarkdownLanguage,
    ast::{Block, Blockquote, CodeBlock, Heading, Html, Inline, List, ListItem, MarkdownRoot, Paragraph, Table, TableCell},
};
use std::collections::HashMap;

/// HTML 渲染器配置
#[derive(Debug, Clone)]
pub struct HtmlRendererConfig {
    /// 是否启用 GFM (GitHub Flavored Markdown)
    pub gfm: bool,
    /// 是否启用表格支持
    pub tables: bool,
    /// 是否启用任务列表支持
    pub tasklists: bool,
    /// 是否启用删除线支持
    pub strikethrough: bool,
    /// 是否启用自动链接
    pub autolink: bool,
}

impl Default for HtmlRendererConfig {
    fn default() -> Self {
        Self { gfm: true, tables: true, tasklists: true, strikethrough: true, autolink: true }
    }
}

impl HtmlRendererConfig {
    /// 创建默认的 HTML 渲染器配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置是否启用 GFM
    ///
    /// # Arguments
    ///
    /// * `gfm` - 是否启用 GFM
    pub fn with_gfm(mut self, gfm: bool) -> Self {
        self.gfm = gfm;
        self
    }

    /// 设置是否启用表格支持
    ///
    /// # Arguments
    ///
    /// * `tables` - 是否启用表格支持
    pub fn with_tables(mut self, tables: bool) -> Self {
        self.tables = tables;
        self
    }

    /// 设置是否启用任务列表支持
    ///
    /// # Arguments
    ///
    /// * `tasklists` - 是否启用任务列表支持
    pub fn with_tasklists(mut self, tasklists: bool) -> Self {
        self.tasklists = tasklists;
        self
    }

    /// 设置是否启用删除线支持
    ///
    /// # Arguments
    ///
    /// * `strikethrough` - 是否启用删除线支持
    pub fn with_strikethrough(mut self, strikethrough: bool) -> Self {
        self.strikethrough = strikethrough;
        self
    }

    /// 设置是否启用自动链接
    ///
    /// # Arguments
    ///
    /// * `autolink` - 是否启用自动链接
    pub fn with_autolink(mut self, autolink: bool) -> Self {
        self.autolink = autolink;
        self
    }
}

/// HTML 渲染器
///
/// 负责将 Markdown 内容渲染为 HTML
#[derive(Clone)]
pub struct HtmlRenderer {
    /// 渲染配置
    config: HtmlRendererConfig,
    /// Markdown 语言配置
    lang_config: MarkdownLanguage,
    /// 自定义渲染选项
    options: HashMap<String, String>,
}

impl HtmlRenderer {
    /// 创建新的 HTML 渲染器
    pub fn new() -> Self {
        Self::with_config(HtmlRendererConfig::default())
    }

    /// 创建带配置的 HTML 渲染器
    ///
    /// # Arguments
    ///
    /// * `config` - 渲染器配置
    pub fn with_config(config: HtmlRendererConfig) -> Self {
        let mut lang_config = MarkdownLanguage::default();
        lang_config.allow_tables = config.tables;
        lang_config.allow_task_lists = config.tasklists;
        lang_config.allow_strikethrough = config.strikethrough;

        Self { config, lang_config, options: HashMap::new() }
    }

    /// 获取渲染器配置
    pub fn config(&self) -> &HtmlRendererConfig {
        &self.config
    }

    /// 获取可变的渲染器配置
    pub fn config_mut(&mut self) -> &mut HtmlRendererConfig {
        &mut self.config
    }

    /// 设置自定义选项
    ///
    /// # Arguments
    ///
    /// * `key` - 选项键
    /// * `value` - 选项值
    pub fn set_option(&mut self, key: String, value: String) {
        self.options.insert(key, value);
    }

    /// 获取自定义选项
    ///
    /// # Arguments
    ///
    /// * `key` - 选项键
    pub fn get_option(&self, key: &str) -> Option<&String> {
        self.options.get(key)
    }

    /// 渲染 Markdown 内容为 HTML
    ///
    /// # Arguments
    ///
    /// * `content` - Markdown 内容
    ///
    /// # Returns
    ///
    /// 渲染后的 HTML 字符串
    pub fn render(&self, content: &str) -> String {
        let builder = MarkdownBuilder::new(&self.lang_config);
        let mut cache = ParseSession::default();
        let build_output = builder.build(content, &[], &mut cache);

        match build_output.result {
            Ok(ast_root) => self.render_ast(&ast_root),
            Err(_) => {
                let escaped_text = self.escape_html(content);
                format!("<pre>{}</pre>", escaped_text)
            }
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
        // 移除开头的标题标记
        let mut content = heading.content.trim().to_string();
        // 移除开头的 # 标记和空格
        if content.starts_with('#') {
            // 计算标题级别对应的 # 数量
            let expected_hashes = "#".repeat(heading.level as usize);
            if content.starts_with(&expected_hashes) {
                // 移除 # 标记和后面的空格
                content = content[expected_hashes.len()..].trim().to_string();
            }
        }
        let escaped_content = self.escape_html(&content);
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
