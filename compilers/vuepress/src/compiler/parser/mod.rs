//! 解析器模块
//! 提供 Markdown 和 Vue 组件文件的解析功能

use crate::types::Result;
use nargo_parser::parse_document;
use nargo_types::Document;
use regex::Regex;
use lazy_static::lazy_static;

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
    // 解析 Vue 组件结构
    // 提取 template、script 和 style 部分
    lazy_static! {
        static ref TEMPLATE_REGEX: Regex = Regex::new(r#"<template>([\s\S]*?)</template>"#).unwrap();
        static ref SCRIPT_REGEX: Regex = Regex::new(r#"<script[^>]*>([\s\S]*?)</script>"#).unwrap();
        static ref STYLE_REGEX: Regex = Regex::new(r#"<style[^>]*>([\s\S]*?)</style>"#).unwrap();
    }

    // 提取模板内容
    let template_content = TEMPLATE_REGEX.captures(source)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str())
        .unwrap_or("");

    // 提取脚本内容
    let script_content = SCRIPT_REGEX.captures(source)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str())
        .unwrap_or("");

    // 提取样式内容
    let style_content = STYLE_REGEX.captures(source)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str())
        .unwrap_or("");

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
        path,
        template_content,
        script_content,
        style_content
    );

    // 使用 Markdown 解析器解析组合内容
    Ok(parse_document(&combined_content, path)?)
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
    } else if path.ends_with(".html") || path.ends_with(".htm") {
        // 解析 HTML 文件
        let html_content = format!(
            "# HTML File: {}

```html
{}
```",
            path,
            source
        );
        Ok(parse_document(&html_content, path)?)
    } else if path.ends_with(".js") || path.ends_with(".ts") {
        // 解析 JavaScript/TypeScript 文件
        let js_content = format!(
            "# JavaScript File: {}

```javascript
{}
```",
            path,
            source
        );
        Ok(parse_document(&js_content, path)?)
    } else {
        // 对于其他文件类型，尝试使用 Markdown 解析器
        Ok(parse_document(source, path)?)
    }
}