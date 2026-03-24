//! HTML 渲染器
//! 
//! 使用 nargo-document 的 Markdown 渲染器将 Markdown 内容转换为 HTML。

use nargo_document::MarkdownRenderer as NargoMarkdownRenderer;
use std::collections::HashMap;

/// HTML 转义函数
fn html_escape(s: &str) -> String {
    s.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;").replace("\"", "&quot;").replace("'", "&#39;")
}

/// HTML 渲染器配置
#[derive(Debug, Clone)]
pub struct HtmlRendererConfig {
    /// 自定义渲染选项
    pub options: HashMap<String, String>,
    /// Markdown 扩展
    pub markdown_extensions: Vec<String>,
}

impl Default for HtmlRendererConfig {
    fn default() -> Self {
        Self {
            options: HashMap::new(),
            markdown_extensions: Vec::new()
        }
    }
}

/// HTML 渲染器
pub struct HtmlRenderer {
    /// 渲染器配置
    config: HtmlRendererConfig,
    /// nargo-document 的 Markdown 渲染器
    markdown_renderer: NargoMarkdownRenderer,
    /// Markdown 扩展
    markdown_extensions: Vec<String>,
}

impl HtmlRenderer {
    /// 创建新的 HTML 渲染器
    pub fn new() -> Self {
        Self::with_config(HtmlRendererConfig::default())
    }

    /// 使用指定配置创建 HTML 渲染器
    ///
    /// # 参数
    /// * `config` - 渲染器配置
    pub fn with_config(config: HtmlRendererConfig) -> Self {
        Self {
            config: config.clone(),
            markdown_renderer: NargoMarkdownRenderer::new(),
            markdown_extensions: config.markdown_extensions,
        }
    }

    /// 设置 Markdown 扩展
    pub fn set_markdown_extensions(&mut self, extensions: Vec<String>) {
        self.markdown_extensions = extensions;
    }

    /// 获取 Markdown 扩展
    pub fn markdown_extensions(&self) -> &Vec<String> {
        &self.markdown_extensions
    }

    /// 获取渲染器配置
    pub fn config(&self) -> &HtmlRendererConfig {
        &self.config
    }

    /// 获取可变的渲染器配置
    pub fn config_mut(&mut self) -> &mut HtmlRendererConfig {
        &mut self.config
    }

    /// 渲染 Markdown 内容为 HTML
    ///
    /// # 参数
    /// * `content` - Markdown 内容字符串
    ///
    /// # 返回值
    /// 渲染后的 HTML 字符串
    pub fn render(&self, content: &str) -> String {
        // 处理空内容
        if content.trim().is_empty() {
            return "<p></p>".to_string();
        }

        // 直接使用简单的 Markdown 渲染方法，确保标题等基本元素能正确渲染
        self.render_simple(content)
    }

    /// 简单的 Markdown 渲染方法
    fn render_simple(&self, content: &str) -> String {
        let mut html = String::new();
        let mut in_code_block = false;
        let mut in_ul = false;
        let mut in_ol = false;

        for line in content.lines() {
            let line = line.trim();

            if line.starts_with("```") {
                // 关闭之前的列表
                if in_ul {
                    html.push_str("</ul>\n");
                    in_ul = false;
                }
                if in_ol {
                    html.push_str("</ol>\n");
                    in_ol = false;
                }

                in_code_block = !in_code_block;
                if in_code_block {
                    let lang = line.strip_prefix("```").unwrap_or("").trim();
                    if !lang.is_empty() {
                        html.push_str(&format!("<pre><code class=\"language-{}\">\n", lang));
                    } else {
                        html.push_str("<pre><code>\n");
                    }
                }
                else {
                    html.push_str("</code></pre>\n");
                }
                continue;
            }

            if in_code_block {
                html.push_str(&html_escape(line));
                html.push('\n');
                continue;
            }

            if line.is_empty() {
                // 关闭之前的列表
                if in_ul {
                    html.push_str("</ul>\n");
                    in_ul = false;
                }
                if in_ol {
                    html.push_str("</ol>\n");
                    in_ol = false;
                }
                html.push_str("<p></p>\n");
                continue;
            }

            if line.starts_with("# ") {
                // 关闭之前的列表
                if in_ul {
                    html.push_str("</ul>\n");
                    in_ul = false;
                }
                if in_ol {
                    html.push_str("</ol>\n");
                    in_ol = false;
                }
                html.push_str(&format!("<h1>{}</h1>\n", &line[2..]));
            }
            else if line.starts_with("## ") {
                // 关闭之前的列表
                if in_ul {
                    html.push_str("</ul>\n");
                    in_ul = false;
                }
                if in_ol {
                    html.push_str("</ol>\n");
                    in_ol = false;
                }
                html.push_str(&format!("<h2>{}</h2>\n", &line[3..]));
            }
            else if line.starts_with("### ") {
                // 关闭之前的列表
                if in_ul {
                    html.push_str("</ul>\n");
                    in_ul = false;
                }
                if in_ol {
                    html.push_str("</ol>\n");
                    in_ol = false;
                }
                html.push_str(&format!("<h3>{}</h3>\n", &line[4..]));
            }
            else if line.starts_with("- ") || line.starts_with("* ") {
                if !in_ul {
                    html.push_str("<ul>\n");
                    in_ul = true;
                }
                // 关闭有序列表
                if in_ol {
                    html.push_str("</ol>\n");
                    in_ol = false;
                }
                html.push_str(&format!("<li>{}</li>\n", &line[2..]));
            }
            else if line.starts_with(|c: char| c.is_digit(10)) && line.contains(". ") {
                if !in_ol {
                    html.push_str("<ol>\n");
                    in_ol = true;
                }
                // 关闭无序列表
                if in_ul {
                    html.push_str("</ul>\n");
                    in_ul = false;
                }
                // 提取列表项内容
                let content = line.split(". ").nth(1).unwrap_or(line);
                html.push_str(&format!("<li>{}</li>\n", content));
            }
            else if line.starts_with("> ") {
                // 关闭之前的列表
                if in_ul {
                    html.push_str("</ul>\n");
                    in_ul = false;
                }
                if in_ol {
                    html.push_str("</ol>\n");
                    in_ol = false;
                }
                html.push_str(&format!("<blockquote>{}</blockquote>\n", &line[2..]));
            }
            else {
                // 关闭之前的列表
                if in_ul {
                    html.push_str("</ul>\n");
                    in_ul = false;
                }
                if in_ol {
                    html.push_str("</ol>\n");
                    in_ol = false;
                }
                // 处理粗体和斜体
                let mut processed_line = line.to_string();
                
                // 处理链接和图片
                use regex::Regex;
                // 处理图片
                let img_regex = Regex::new(r"!\[([^\]]+)\]\(([^\)]+)\)").unwrap();
                processed_line = img_regex.replace_all(&processed_line, "<img src=\"$2\" alt=\"$1\" />").to_string();
                // 处理链接
                let link_regex = Regex::new(r"\[([^\]]+)\]\(([^\)]+)\)").unwrap();
                processed_line = link_regex.replace_all(&processed_line, "<a href=\"$2\">$1</a>").to_string();
                
                // 处理粗体
                let mut parts = processed_line.split("**");
                let mut processed_parts = Vec::new();
                let mut in_bold = false;
                for part in parts {
                    if in_bold {
                        processed_parts.push(format!("<strong>{}</strong>", part));
                    } else {
                        processed_parts.push(part.to_string());
                    }
                    in_bold = !in_bold;
                }
                processed_line = processed_parts.join("");
                
                // 处理斜体
                parts = processed_line.split("*");
                processed_parts = Vec::new();
                let mut in_italic = false;
                for part in parts {
                    if in_italic {
                        processed_parts.push(format!("<em>{}</em>", part));
                    } else {
                        processed_parts.push(part.to_string());
                    }
                    in_italic = !in_italic;
                }
                processed_line = processed_parts.join("");
                
                html.push_str(&format!("<p>{}</p>\n", processed_line));
            }
        }

        // 关闭最后的列表
        if in_ul {
            html.push_str("</ul>\n");
        }
        if in_ol {
            html.push_str("</ol>\n");
        }

        html
    }

    /// 应用 Markdown 扩展
    fn apply_markdown_extensions(&self, content: &str) -> String {
        let mut processed_content = content.to_string();

        // 处理表格扩展
        if self.markdown_extensions.contains(&"tables".to_string()) {
            processed_content = self.process_tables(&processed_content);
        }

        // 处理脚注扩展
        if self.markdown_extensions.contains(&"footnotes".to_string()) {
            processed_content = self.process_footnotes(&processed_content);
        }

        // 处理代码高亮扩展
        if self.markdown_extensions.contains(&"codehilite".to_string()) || 
           self.markdown_extensions.contains(&"prism".to_string()) {
            processed_content = self.process_code_highlight(&processed_content);
        }

        processed_content
    }

    /// 处理表格
    fn process_tables(&self, content: &str) -> String {
        // 这里可以实现表格处理逻辑
        content.to_string()
    }

    /// 处理脚注
    fn process_footnotes(&self, content: &str) -> String {
        // 这里可以实现脚注处理逻辑
        content.to_string()
    }

    /// 处理代码高亮
    fn process_code_highlight(&self, content: &str) -> String {
        // 这里可以实现代码高亮处理逻辑
        content.to_string()
    }

    /// 对渲染后的 HTML 进行后处理
    fn post_process_html(&self, html: String) -> String {
        // 这里可以实现 HTML 后处理逻辑
        html
    }

    /// 简单的后备渲染方法，在 nargo-document 渲染失败时使用
    ///
    /// # 参数
    /// * `content` - Markdown 内容字符串
    ///
    /// # 返回值
    /// 简单转义后的 HTML 字符串
    fn render_simple_fallback(&self, content: &str) -> String {
        let escaped = content
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#39;");
        format!("<p>{}</p>", escaped.replace("\n\n", "</p><p>"))
    }
}

impl Default for HtmlRenderer {
    fn default() -> Self {
        Self::new()
    }
}