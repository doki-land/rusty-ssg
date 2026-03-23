#![warn(missing_docs)]

//! Markdown 处理模块
//! 
//! 提供 Markdown 文档的转换功能

use crate::errors::{MarkdownError, Result};
use pulldown_cmark::{html, Parser};

/// Markdown 转换器
/// 
/// 负责将 Markdown 文档转换为 HTML
pub struct MarkdownConverter {
    /// Markdown 处理器
    processor: MarkdownProcessor,
}

impl MarkdownConverter {
    /// 创建新的 Markdown 转换器
    pub fn new() -> Self {
        Self {
            processor: MarkdownProcessor::new(),
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
        self.processor.process(markdown)
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
        Self {
            name: "default".to_string(),
        }
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
        // 使用 pulldown-cmark 处理 Markdown
        let parser = Parser::new(markdown);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        
        Ok(html_output)
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
