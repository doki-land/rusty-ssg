//! Markdown 解析和渲染模块

use crate::types::{FrontMatter, HexoError, Result};
use nargo_document::generator::markdown::MarkdownRenderer;
use nargo_types::{Document, DocumentMeta, FrontMatter as NargoFrontMatter};
use std::fs::read_to_string;
use std::path::Path;

/// 解析 Markdown 文档，提取前置元数据和内容
///
/// # Arguments
///
/// * `content` - Markdown 文档内容
///
/// # Returns
///
/// 包含前置元数据和 Markdown 内容的元组
pub fn parse_markdown(content: &str) -> Result<(Option<FrontMatter>, String)> {
    let mut lines = content.lines();
    let mut front_matter_lines = Vec::new();
    let mut in_front_matter = false;
    let mut markdown_content = Vec::new();

    // 解析前置元数据
    for line in lines {
        if line == "---" {
            if !in_front_matter {
                in_front_matter = true;
            } else {
                in_front_matter = false;
                continue;
            }
        } else if in_front_matter {
            front_matter_lines.push(line);
        } else {
            markdown_content.push(line);
        }
    }

    // 解析前置元数据
    let front_matter = if !front_matter_lines.is_empty() {
        let front_matter_str = front_matter_lines.join("\n");
        match serde_yaml::from_str::<serde_yaml::Value>(&front_matter_str) {
            Ok(value) => {
                let mut front_matter = FrontMatter {
                    title: value.get("title").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    date: value.get("date").and_then(|v| v.as_str()).map(|s| s.to_string()),
                    updated: value.get("updated").and_then(|v| v.as_str()).map(|s| s.to_string()),
                    author: value.get("author").and_then(|v| v.as_str()).map(|s| s.to_string()),
                    categories: value.get("categories").and_then(|v| v.as_sequence()).map(|seq| {
                        seq.iter().filter_map(|item| item.as_str().map(|s| s.to_string())).collect()
                    }),
                    tags: value.get("tags").and_then(|v| v.as_sequence()).map(|seq| {
                        seq.iter().filter_map(|item| item.as_str().map(|s| s.to_string())).collect()
                    }),
                    permalink: value.get("permalink").and_then(|v| v.as_str()).map(|s| s.to_string()),
                    published: value.get("published").and_then(|v| v.as_bool()),
                };
                Some(front_matter)
            }
            Err(_) => None,
        }
    } else {
        None
    };

    Ok((front_matter, markdown_content.join("\n")))
}

/// 渲染 Markdown 内容为 HTML
///
/// # Arguments
///
/// * `content` - Markdown 内容
///
/// # Returns
///
/// 渲染后的 HTML 内容
pub fn render_markdown(content: &str) -> String {
    let renderer = MarkdownRenderer::new();
    match renderer.render(content) {
        Ok(html) => html,
        Err(_) => content.to_string(),
    }
}

/// 从文件中读取并渲染 Markdown 内容
///
/// # Arguments
///
/// * `path` - Markdown 文件路径
///
/// # Returns
///
/// 包含前置元数据和渲染后的 HTML 内容的元组
pub fn render_markdown_file(path: &Path) -> Result<(Option<FrontMatter>, String)> {
    let content = read_to_string(path).map_err(|e| HexoError::io_error(Some(path.to_string_lossy().to_string()), e.to_string()))?;
    let (front_matter, markdown_content) = parse_markdown(&content)?;
    let html = render_markdown(&markdown_content);
    Ok((front_matter, html))
}
