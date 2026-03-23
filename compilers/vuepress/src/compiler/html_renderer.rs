//! HTML 渲染器模块
//! 负责将解析后的内容渲染为 HTML

use crate::{Result, types::VuePressConfig};
use nargo_parser::MarkdownParser;

/// HTML 渲染器配置
pub struct HtmlRendererConfig {
    /// 是否启用压缩
    pub minify: bool,
    /// 是否启用代码高亮
    pub highlight_code: bool,
    /// 是否启用数学公式渲染
    pub math: bool,
}

impl Default for HtmlRendererConfig {
    fn default() -> Self {
        Self {
            minify: false,
            highlight_code: true,
            math: true,
        }
    }
}

/// HTML 渲染器
pub struct HtmlRenderer {
    /// 渲染器配置
    config: VuePressConfig,
    /// HTML 渲染器配置
    html_config: HtmlRendererConfig,
}

impl HtmlRenderer {
    /// 创建新的 HTML 渲染器实例
    pub fn new(config: VuePressConfig) -> Self {
        Self { config, html_config: HtmlRendererConfig::default() }
    }

    /// 创建带自定义配置的 HTML 渲染器实例
    pub fn with_config(config: VuePressConfig, html_config: HtmlRendererConfig) -> Self {
        Self { config, html_config }
    }

    /// 渲染内容为 HTML
    pub fn render(&self, content: &str) -> Result<String> {
        // 处理 Vue 组件标签
        let content_with_vue = self.process_vue_components(content);
        
        // 使用 MarkdownParser 将 Markdown 转换为 HTML
        let mut html = MarkdownParser::to_html(&content_with_vue)?;
        
        // 应用代码高亮
        if self.html_config.highlight_code {
            html = self.highlight_code(&html)?;
        }
        
        // 应用数学公式渲染
        if self.html_config.math {
            html = self.render_math(&html)?;
        }
        
        // 压缩 HTML
        if self.html_config.minify {
            html = self.minify_html(&html)?;
        }
        
        Ok(html)
    }

    /// 处理 Vue 组件标签
    fn process_vue_components(&self, content: &str) -> String {
        let mut result = content.to_string();
        
        // 识别并处理 Vue 组件标签
        // 1. 处理单文件组件 (SFC) 格式
        lazy_static::lazy_static! {
            static ref VUE_SFC_REGEX: regex::Regex = regex::Regex::new(r#"```vue\n([\s\S]*?)```"#).unwrap();
        }
        
        result = VUE_SFC_REGEX.replace_all(&result, |caps: &regex::Captures| {
            let component_content = caps.get(1).unwrap().as_str();
            // 保留完整的 Vue SFC 内容
            format!(r#"<template>{}</template>"#, component_content)
        }).to_string();
        
        // 2. 处理内联 Vue 组件标签
        lazy_static::lazy_static! {
            static ref VUE_COMPONENT_REGEX: regex::Regex = regex::Regex::new(r#"<([A-Z][a-zA-Z0-9-]+)([^>]*?)>([\s\S]*?)</\1>"#).unwrap();
        }
        
        result = VUE_COMPONENT_REGEX.replace_all(&result, |caps: &regex::Captures| {
            let component_name = caps.get(1).unwrap().as_str();
            let attributes = caps.get(2).unwrap().as_str();
            let content = caps.get(3).unwrap().as_str();
            
            // 保留 Vue 组件标签，不进行 Markdown 解析
            format!(r#"<{} {}>{}</{}>"#, component_name, attributes, content, component_name)
        }).to_string();
        
        // 3. 处理自闭合 Vue 组件标签
        lazy_static::lazy_static! {
            static ref VUE_SELF_CLOSING_REGEX: regex::Regex = regex::Regex::new(r#"<([A-Z][a-zA-Z0-9-]+)([^>]*?)/>"#).unwrap();
        }
        
        result = VUE_SELF_CLOSING_REGEX.replace_all(&result, |caps: &regex::Captures| {
            let component_name = caps.get(1).unwrap().as_str();
            let attributes = caps.get(2).unwrap().as_str();
            
            // 保留自闭合 Vue 组件标签
            format!(r#"<{} {} />"#, component_name, attributes)
        }).to_string();
        
        result
    }

    /// HTML 转义函数
    fn escape_html(&self, text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#39;")
    }

    /// 高亮代码块
    fn highlight_code(&self, html: &str) -> Result<String> {
        // 简单的代码高亮实现
        // 实际项目中可能需要使用更复杂的高亮库
        let mut result = html.to_string();
        
        // 处理代码块
        lazy_static::lazy_static! {
            static ref CODE_BLOCK_REGEX: regex::Regex = regex::Regex::new(r#"```([a-z]+)\n([\s\S]*?)```"#).unwrap();
        }
        
        result = CODE_BLOCK_REGEX.replace_all(&result, |caps: &regex::Captures| {
            let language = caps.get(1).unwrap().as_str();
            let code = caps.get(2).unwrap().as_str();
            format!(
                r#"<pre><code class="language-{}">{}</code></pre>"#,
                language,
                self.escape_html(code)
            )
        }).to_string();
        
        Ok(result)
    }

    /// 渲染数学公式
    fn render_math(&self, html: &str) -> Result<String> {
        // 简单的数学公式渲染实现
        // 实际项目中可能需要使用 KaTeX 或 MathJax
        let mut result = html.to_string();
        
        // 处理行内数学公式
        lazy_static::lazy_static! {
            static ref INLINE_MATH_REGEX: regex::Regex = regex::Regex::new(r#"\$(.*?)\$"#).unwrap();
        }
        
        result = INLINE_MATH_REGEX.replace_all(&result, |caps: &regex::Captures| {
            let math = caps.get(1).unwrap().as_str();
            format!(r#"<span class="math inline">{}</span>"#, math)
        }).to_string();
        
        // 处理块级数学公式
        lazy_static::lazy_static! {
            static ref BLOCK_MATH_REGEX: regex::Regex = regex::RegexBuilder::new(r#"\$\$(.*?)\$\$"#)
                .dot_matches_new_line(true)
                .build()
                .unwrap();
        }
        
        result = BLOCK_MATH_REGEX.replace_all(&result, |caps: &regex::Captures| {
            let math = caps.get(1).unwrap().as_str();
            format!(r#"<div class="math block">{}</div>"#, math)
        }).to_string();
        
        Ok(result)
    }

    /// 压缩 HTML
    fn minify_html(&self, html: &str) -> Result<String> {
        // 简单的 HTML 压缩实现
        let result = html
            .replace(&['\n', '\r', '\t'][..], " ")
            .replace(r#" >"#, ">")
            .replace(r#"< "#, "<")
            .replace(r#"  "#, " ");
        
        Ok(result)
    }
}
