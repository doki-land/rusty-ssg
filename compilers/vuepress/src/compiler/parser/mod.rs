//! 解析器模块
//! 提供 Markdown 和 Vue 组件文件的解析功能

use crate::types::Result;
use nargo_parser::parse_document;
use nargo_types::Document;

/// 解析 Markdown 文件
///
/// # Arguments
///
/// * `source` - Markdown 文件内容
/// * `path` - 文件路径
///
/// # Returns
///
/// 解析后的文档
pub fn parse_markdown(source: &str, path: &str) -> Result<Document> {
    Ok(parse_document(source, path)?)
}

/// 解析 Vue 组件文件
///
/// # Arguments
///
/// * `source` - Vue 组件文件内容
/// * `path` - 文件路径
///
/// # Returns
///
/// 解析后的文档
pub fn parse_vue_component(source: &str, path: &str) -> Result<Document> {
    // 暂时使用 Markdown 解析器处理 Vue 组件
    // 实际实现需要解析 Vue 组件的结构
    Ok(parse_document(source, path)?)
}

/// 解析内容文件
///
/// # Arguments
///
/// * `source` - 文件内容
/// * `path` - 文件路径
///
/// # Returns
///
/// 解析后的文档
pub fn parse_content_file(source: &str, path: &str) -> Result<Document> {
    if path.ends_with(".md") || path.ends_with(".markdown") {
        parse_markdown(source, path)
    } else if path.ends_with(".vue") {
        parse_vue_component(source, path)
    } else {
        // 对于其他文件类型，尝试使用 Markdown 解析器
        Ok(parse_document(source, path)?)
    }
}