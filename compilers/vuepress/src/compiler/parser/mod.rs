//! 解析器模块
//! 提供 Markdown 和 Vue 组件文件的解析功能

use crate::types::Result;
use lazy_static::lazy_static;
use nargo_parser::{FrontMatterParser, parse_document};
use nargo_types::Document;
use regex::Regex;

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
    // 首先解析前置元数据
    let (frontmatter, content_start) = FrontMatterParser::parse(source)?;

    // 提取内容部分
    let content = if content_start < source.len() { source[content_start..].to_string() } else { String::new() };

    // 创建文档对象
    let mut doc = Document::new().with_path(path.to_string()).with_frontmatter(frontmatter).with_content(content);

    // 提取标题
    if doc.frontmatter.title.is_none() {
        doc.meta.title = extract_first_heading(&doc.content);
    }
    else {
        doc.meta.title = doc.frontmatter.title.clone();
    }

    Ok(doc)
}

/// 从 Markdown 内容中提取第一个标题
fn extract_first_heading(content: &str) -> Option<String> {
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with('#') {
            let heading = line.trim_start_matches('#').trim();
            if !heading.is_empty() {
                return Some(heading.to_string());
            }
        }
    }
    None
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
    // 解析 Vue 组件结构
    // 提取 template、script 和 style 部分
    lazy_static! {
        static ref TEMPLATE_REGEX: Regex = Regex::new(r#"<template[^>]*>([\s\S]*?)</template>"#).unwrap();
        static ref SCRIPT_REGEX: Regex = Regex::new(r#"<script[^>]*>([\s\S]*?)</script>"#).unwrap();
        static ref STYLE_REGEX: Regex = Regex::new(r#"<style[^>]*>([\s\S]*?)</style>"#).unwrap();
        static ref SCRIPT_SETUP_REGEX: Regex = Regex::new(r#"<script[^>]*setup[^>]*>([\s\S]*?)</script>"#).unwrap();
    }

    // 提取模板内容
    let template_content = TEMPLATE_REGEX.captures(source).and_then(|cap| cap.get(1)).map(|m| m.as_str()).unwrap_or("");

    // 提取脚本内容（优先处理 setup 脚本）
    let script_content = if let Some(cap) = SCRIPT_SETUP_REGEX.captures(source) {
        cap.get(1).map(|m| m.as_str()).unwrap_or("")
    }
    else {
        SCRIPT_REGEX.captures(source).and_then(|cap| cap.get(1)).map(|m| m.as_str()).unwrap_or("")
    };

    // 提取样式内容
    let style_content = STYLE_REGEX.captures(source).and_then(|cap| cap.get(1)).map(|m| m.as_str()).unwrap_or("");

    // 构建组合内容
    let combined_content = format!(
        "# Vue Component: {}

## Template
```html
{}
```

## Script
```javascript
{}
```

## Style
```css
{}
```",
        path, template_content, script_content, style_content
    );

    // 使用 Markdown 解析器解析组合内容
    parse_markdown(&combined_content, path)
}

/// 解析 HTML 文件
///
/// # Arguments
///
/// * `source` - HTML 文件内容
/// * `path` - 文件路径
///
/// # Returns
///
/// 解析后的文档
pub fn parse_html_file(source: &str, path: &str) -> Result<Document> {
    // 提取 HTML 标题
    lazy_static! {
        static ref TITLE_REGEX: Regex = Regex::new(r#"<title>([\s\S]*?)</title>"#).unwrap();
    }

    let title = TITLE_REGEX.captures(source).and_then(|cap| cap.get(1)).map(|m| m.as_str().trim().to_string());

    // 构建内容
    let html_content = format!(
        "# HTML File: {}

{}

```html
{}
```",
        path,
        title.as_deref().unwrap_or(""),
        source
    );

    let mut doc = parse_markdown(&html_content, path)?;
    if title.is_some() {
        doc.meta.title = title;
    }

    Ok(doc)
}

/// 解析 JavaScript/TypeScript 文件
///
/// # Arguments
///
/// * `source` - JavaScript/TypeScript 文件内容
/// * `path` - 文件路径
///
/// # Returns
///
/// 解析后的文档
pub fn parse_script_file(source: &str, path: &str) -> Result<Document> {
    // 提取脚本信息
    let extension = path.split('.').last().unwrap_or("js");

    // 构建内容
    let script_content = format!(
        "# {} File: {}

```{}
{}
```",
        if extension == "ts" { "TypeScript" } else { "JavaScript" },
        path,
        extension,
        source
    );

    parse_markdown(&script_content, path)
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
    match path.to_lowercase().as_str() {
        // Markdown 文件
        path if path.ends_with(".md") || path.ends_with(".markdown") || path.ends_with(".mdx") => parse_markdown(source, path),
        // Vue 组件文件
        path if path.ends_with(".vue") => parse_vue_component(source, path),
        // HTML 文件
        path if path.ends_with(".html") || path.ends_with(".htm") => parse_html_file(source, path),
        // JavaScript/TypeScript 文件
        path if path.ends_with(".js") || path.ends_with(".ts") || path.ends_with(".jsx") || path.ends_with(".tsx") => {
            parse_script_file(source, path)
        }
        // 其他文件类型
        _ => {
            // 尝试作为 Markdown 解析
            parse_markdown(source, path)
        }
    }
}
