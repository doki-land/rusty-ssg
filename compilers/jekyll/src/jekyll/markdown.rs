#![warn(missing_docs)]

//! Markdown 处理模块
//! 
//! 提供 Markdown 文档的转换功能

use crate::errors::{Result};
use nargo_document::{
    plugin::{KaTeXPlugin, MermaidPlugin, PluginRegistry, PrismPlugin},
};
use oak_core::{Builder, parser::session::ParseSession, source::SourceText};
use oak_markdown::{MarkdownBuilder, MarkdownLanguage};

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

        // 注册默认插件
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
        // 使用 nargo-document 插件处理
        let mut context = nargo_document::plugin::PluginContext::from_content(markdown.to_string(), "".to_string());

        // 应用插件的渲染前钩子
        context = self.plugin_registry.before_render_all(context);

        // 使用处理后的内容进行 Markdown 到 HTML 转换
        let processed_content = context.content;
        
        // 简单的 Markdown 到 HTML 转换
        // 注意：这是一个临时实现，稍后会使用 oak-markdown 的完整 AST 渲染
        let mut html = String::new();
        html.push_str(r#"<div class="markdown-body">"#);
        
        let lines: Vec<&str> = processed_content.lines().collect();
        let mut i = 0;
        
        while i < lines.len() {
            let line = lines[i].trim();
            
            // 处理标题
            if line.starts_with('#') {
                let level = line.chars().take_while(|&c| c == '#').count();
                if level <= 6 {
                    let text = line[level..].trim();
                    html.push_str(&format!(r#"<h{}>{}</h{}>"#, level, text, level));
                    i += 1;
                    continue;
                }
            }
            
            // 处理代码块
            if line.starts_with("```") {
                let lang = line[3..].trim();
                html.push_str(&format!(r#"<pre><code class="language-{}">"#, lang));
                i += 1;
                
                while i < lines.len() && !lines[i].starts_with("```") {
                    html.push_str(lines[i]);
                    html.push('\n');
                    i += 1;
                }
                
                if i < lines.len() {
                    i += 1;
                }
                
                html.push_str(r#"</code></pre>"#);
                continue;
            }
            
            // 处理段落
            if !line.is_empty() {
                let mut paragraph = String::new();
                paragraph.push_str(r#"<p>"#);
                
                // 处理粗体和斜体
                let mut processed_line = line.to_string();
                
                // 处理粗体 **text**
                processed_line = regex::Regex::new(r"\*\*(.*?)\*\*")
                    .unwrap()
                    .replace_all(&processed_line, r#"<strong>$1</strong>"#)
                    .to_string();
                
                // 处理斜体 *text*
                processed_line = regex::Regex::new(r"\*(.*?)\*")
                    .unwrap()
                    .replace_all(&processed_line, r#"<em>$1</em>"#)
                    .to_string();
                
                paragraph.push_str(&processed_line);
                paragraph.push_str(r#"</p>"#);
                html.push_str(&paragraph);
            }
            
            i += 1;
        }
        
        html.push_str(r#"</div>"#);

        // 应用插件的渲染后钩子
        context.content = html;
        context = self.plugin_registry.after_render_all(context);

        Ok(context.content)
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
    fn render_block(&self, block: &oak_markdown::ast::Block) -> String {
        match block {
            oak_markdown::ast::Block::Heading(heading) => self.render_heading(heading),
            oak_markdown::ast::Block::Paragraph(paragraph) => self.render_paragraph(paragraph),
            oak_markdown::ast::Block::List(list) => self.render_list(list),
            oak_markdown::ast::Block::CodeBlock(code_block) => self.render_code_block(code_block),
            oak_markdown::ast::Block::Blockquote(blockquote) => self.render_blockquote(blockquote),
            _ => String::new(),
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
    fn render_heading(&self, heading: &oak_markdown::ast::Heading) -> String {
        let tag = format!(r#"h{}"#, heading.level);
        let content = self.escape_html(&heading.content);
        format!(r#"<{}>{}</{}>"#, tag, content, tag)
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
    fn render_paragraph(&self, paragraph: &oak_markdown::ast::Paragraph) -> String {
        let content = self.escape_html(&paragraph.content);
        format!(r#"<p>{}</p>"#, content)
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
    fn render_list(&self, list: &oak_markdown::ast::List) -> String {
        let tag = if list.is_ordered { "ol" } else { "ul" };
        let mut html = format!(r#"<{}>"#, tag);

        for item in &list.items {
            html.push_str(&self.render_list_item(item));
        }

        html.push_str(&format!(r#"</{}>"#, tag));
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
    fn render_list_item(&self, list_item: &oak_markdown::ast::ListItem) -> String {
        let mut html = String::from(r#"<li>"#);

        if list_item.is_task {
            let checked = if list_item.is_checked.unwrap_or(false) { "checked" } else { "" };
            html.push_str(&format!(r#"<input type="checkbox" disabled {} /> "#, checked));
        }

        // 渲染列表项的内容
        for block in &list_item.content {
            html.push_str(&self.render_block(block));
        }

        html.push_str(r#"</li>"#);
        html
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
    fn render_code_block(&self, code_block: &oak_markdown::ast::CodeBlock) -> String {
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
    ///
    /// # Arguments
    ///
    /// * `blockquote` - 引用块元素
    ///
    /// # Returns
    ///
    /// 渲染后的 HTML 字符串
    fn render_blockquote(&self, blockquote: &oak_markdown::ast::Blockquote) -> String {
        let mut content = String::new();
        for block in &blockquote.content {
            content.push_str(&self.render_block(block));
        }
        format!(r#"<blockquote>{}</blockquote>"#, content)
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
    pub fn lang_config_mut(&mut self) -> &mut MarkdownLanguage {
        &mut self.lang_config
    }
}

impl Default for MarkdownConverter {
    fn default() -> Self {
        Self::new()
    }
}
