//! Markdown 解析器

use oak_yaml;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::{HexoError, Result};

/// Front Matter 结构
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct FrontMatter {
    /// 标题
    pub title: Option<String>,
    /// 日期
    pub date: Option<String>,
    /// 分类
    pub categories: Option<Vec<String>>,
    /// 标签
    pub tags: Option<Vec<String>>,
    /// 其他自定义字段
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 解析 Markdown 文件，提取 Front Matter 和内容
pub fn parse_markdown(content: &str) -> Result<(Option<FrontMatter>, String)> {
    // 检查是否有 Front Matter
    if content.starts_with("---\n") {
        // 找到第二个 --- 分隔符
        if let Some(end_idx) = content.find("\n---\n") {
            let front_matter_str = &content[4..end_idx + 1];
            let content_str = &content[end_idx + 5..];

            // 解析 Front Matter
            let front_matter: FrontMatter =
                oak_yaml::language::from_str(front_matter_str).map_err(|e| HexoError::yaml_error(None, e.to_string()))?;

            Ok((Some(front_matter), content_str.trim_start().to_string()))
        }
        else {
            // 没有找到结束的 ---，视为普通 Markdown
            Ok((None, content.to_string()))
        }
    }
    else {
        // 没有 Front Matter，视为普通 Markdown
        Ok((None, content.to_string()))
    }
}

/// 将 Markdown 内容渲染为 HTML
pub fn render_markdown(content: &str) -> String {
    // 使用 nargo-document 的 MarkdownRenderer
    let renderer = nargo_document::generator::markdown::MarkdownRenderer::default();
    match renderer.render(content) {
        Ok(html) => html.trim_start().to_string(),
        Err(_) => content.to_string(),
    }
}
