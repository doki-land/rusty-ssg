//! Jekyll 文档解析器模块
//!
//! 负责解析 Markdown 文档和 Front Matter

use crate::{jekyll::FrontMatterParser, types::Result};

/// 解析结果
pub struct ParsedDocument {
    /// 标题
    pub title: Option<String>,
    /// Front Matter 数据
    pub front_matter: serde_json::Value,
    /// 内容
    pub content: String,
    /// 布局
    pub layout: Option<String>,
}

/// Jekyll 文档解析器
pub struct JekyllParser {}

impl JekyllParser {
    /// 创建新的解析器
    pub fn new() -> Self {
        Self {}
    }

    /// 解析文档
    ///
    /// # Arguments
    ///
    /// * `source` - 文档内容
    /// * `path` - 文档路径
    ///
    /// # Returns
    ///
    /// 解析后的文档
    pub fn parse(&self, source: &str, path: &str) -> Result<ParsedDocument> {
        let front_matter = crate::FrontMatterParser::parse(source)?;

        let title = front_matter.get_str("title").map(|s| s.to_string());
        let layout = front_matter.get_str("layout").map(|s| s.to_string());

        Ok(ParsedDocument {
            title,
            front_matter: front_matter.variables().clone(),
            content: front_matter.content().to_string(),
            layout,
        })
    }
}

impl Default for JekyllParser {
    fn default() -> Self {
        Self::new()
    }
}
