//! HTML 渲染器
//! 
//! 使用 nargo-document 的 Markdown 渲染器将 Markdown 内容转换为 HTML。

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

        // 应用 Markdown 扩展
        let processed_content = self.apply_markdown_extensions(content);
        
        // 直接使用简单的 Markdown 渲染方法，确保标题等基本元素能正确渲染
        let html = self.render_simple(&processed_content);
        
        // 对渲染后的 HTML 进行后处理
        self.post_process_html(html)
    }

    /// 简单的 Markdown 渲染方法
    fn render_simple(&self, content: &str) -> String {
        let mut html = String::new();
        let mut in_code_block = false;
        let mut in_ul = false;
        let mut in_ol = false;

        for line in content.lines() {
            // 对于代码块，我们需要保留原始行内容（包括缩进）
            let trimmed_line = line.trim();

            if trimmed_line.starts_with("```") {
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
                    let lang = trimmed_line.strip_prefix("```").unwrap_or("").trim();
                    // 处理 language- 前缀
                    let lang = lang.strip_prefix("language-").unwrap_or(lang);
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

            let line = trimmed_line;

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
        processed_content = self.process_tables(&processed_content);

        // 处理脚注扩展
        processed_content = self.process_footnotes(&processed_content);

        // 处理代码高亮扩展
        processed_content = self.process_code_highlight(&processed_content);

        processed_content
    }

    /// 处理表格
    fn process_tables(&self, content: &str) -> String {
        use regex::Regex;
        
        // 表格正则表达式：匹配表格行，包括表头、分隔线和数据行
        let table_regex = Regex::new(r"(?m)^\|.*\|\n(?:^\|.*\|\n)*").unwrap();
        
        let mut processed_content = content.to_string();
        
        // 查找并处理所有表格
        for capture in table_regex.captures_iter(content) {
            let table_text = capture.get(0).unwrap().as_str();
            let html_table = self.convert_table_to_html(table_text);
            processed_content = processed_content.replace(table_text, &html_table);
        }
        
        processed_content
    }
    
    /// 将 Markdown 表格转换为 HTML 表格
    fn convert_table_to_html(&self, table_text: &str) -> String {
        let lines: Vec<&str> = table_text.lines().collect();
        if lines.len() < 2 {  // 至少需要表头和分隔线
            return table_text.to_string();
        }
        
        let mut html = String::from("<table>\n");
        
        // 处理表头
        let header_line = lines[0].trim();
        let headers: Vec<&str> = header_line
            .trim_matches('|')
            .split('|')
            .map(|h| h.trim())
            .collect();
        
        html.push_str("  <thead>\n");
        html.push_str("    <tr>\n");
        for header in headers {
            html.push_str(&format!("      <th>{}</th>\n", html_escape(header)));
        }
        html.push_str("    </tr>\n");
        html.push_str("  </thead>\n");
        
        // 处理数据行（跳过分隔线）
        html.push_str("  <tbody>\n");
        for line in &lines[2..] {  // 跳过表头和分隔线
            let line = line.trim();
            if line.is_empty() || !line.starts_with('|') {
                continue;
            }
            
            let cells: Vec<&str> = line
                .trim_matches('|')
                .split('|')
                .map(|c| c.trim())
                .collect();
            
            html.push_str("    <tr>\n");
            for cell in cells {
                html.push_str(&format!("      <td>{}</td>\n", html_escape(cell)));
            }
            html.push_str("    </tr>\n");
        }
        html.push_str("  </tbody>\n");
        html.push_str("</table>\n");
        
        html
    }

    /// 处理脚注
    fn process_footnotes(&self, content: &str) -> String {
        use regex::Regex;
        
        // 脚注定义正则表达式：匹配 [^1]: 脚注内容 这样的脚注定义
        let footnote_def_regex = Regex::new(r"(?m)^\[\^(\d+)\]:\s*(.*)").unwrap();
        
        let mut processed_content = content.to_string();
        let mut footnotes = Vec::new();
        
        // 查找所有脚注定义
        for capture in footnote_def_regex.captures_iter(content) {
            let id = capture.get(1).unwrap().as_str();
            let content = capture.get(2).unwrap().as_str();
            footnotes.push((id, content));
            
            // 从内容中移除脚注定义
            let def_text = capture.get(0).unwrap().as_str();
            processed_content = processed_content.replace(def_text, "");
        }
        
        // 替换所有脚注引用为带有 ID 的链接
        for (id, _) in &footnotes {
            let ref_pattern = format!("[^{}]", id);
            let replacement = format!("<sup id=\"fnref:{}\"><a href=\"#fn:{}\" class=\"footnote-ref\">{}</a></sup>", id, id, id);
            processed_content = processed_content.replace(&ref_pattern, &replacement);
        }
        
        // 如果有脚注，在文档末尾添加脚注列表
        if !footnotes.is_empty() {
            processed_content.push_str("\n<div class=\"footnotes\">\n");
            processed_content.push_str("  <hr>\n");
            processed_content.push_str("  <ol>\n");
            
            for (id, content) in footnotes {
                processed_content.push_str(&format!("    <li id=\"fn:{}\">{}</li>\n", id, html_escape(content)));
            }
            
            processed_content.push_str("  </ol>\n");
            processed_content.push_str("</div>\n");
        }
        
        processed_content
    }

    /// 处理代码高亮
    fn process_code_highlight(&self, content: &str) -> String {
        use regex::Regex;
        
        // 代码块正则表达式：匹配 ```language
        let code_block_regex = Regex::new(r"```(\w+)").unwrap();
        
        let mut processed_content = content.to_string();
        
        // 查找并处理所有代码块
        for capture in code_block_regex.captures_iter(content) {
            let lang = capture.get(1).unwrap().as_str();
            let pattern = format!("```{}", lang);
            let replacement = format!("```language-{}", lang);
            processed_content = processed_content.replace(&pattern, &replacement);
        }
        
        processed_content
    }

    /// 对渲染后的 HTML 进行后处理
    fn post_process_html(&self, html: String) -> String {
        // 这里可以实现 HTML 后处理逻辑
        html
    }


}

impl Default for HtmlRenderer {
    fn default() -> Self {
        Self::new()
    }
}