//! Markdown 渲染器

use crate::markdown::parser::{parse_markdown, render_markdown};
use std::{fs::File, io::Read, path::Path};

use crate::types::{HexoError, Result};

/// 渲染 Markdown 文件
pub fn render_markdown_file(path: &Path) -> Result<(Option<super::parser::FrontMatter>, String)> {
    // 读取文件内容
    let mut file =
        File::open(path).map_err(|e| HexoError::io_error(Some(path.to_string_lossy().to_string()), e.to_string()))?;

    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|e| HexoError::io_error(Some(path.to_string_lossy().to_string()), e.to_string()))?;

    // 解析 Markdown
    let (front_matter, markdown_content) = parse_markdown(&content)?;

    // 渲染为 HTML
    let html = render_markdown(&markdown_content);

    Ok((front_matter, html))
}

/// 批量渲染 Markdown 文件
pub fn render_markdown_files(paths: &[&Path]) -> Result<Vec<(Option<super::parser::FrontMatter>, String)>> {
    paths.iter().map(|path| render_markdown_file(path)).collect()
}
