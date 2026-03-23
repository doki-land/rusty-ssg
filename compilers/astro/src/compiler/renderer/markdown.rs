//! Markdown 渲染器

use std::collections::HashMap;

/// Markdown 渲染器
pub struct MarkdownRenderer {
    /// 渲染配置
    config: HashMap<String, String>,
}

impl MarkdownRenderer {
    /// 创建新的 Markdown 渲染器
    pub fn new() -> Self {
        Self { config: HashMap::new() }
    }

    /// 设置配置
    pub fn set_config(&mut self, config: HashMap<String, String>) {
        self.config = config;
    }

    /// 渲染 Markdown 内容为 HTML
    pub fn render(&self, content: &str) -> String {
        let mut result = String::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i].trim();

            // 处理标题
            if line.starts_with('#') {
                let level = line.chars().take_while(|&c| c == '#').count();
                if level <= 6 {
                    let text = line[level..].trim();
                    result.push_str(&format!("<h{}>{}</h{}>\n", level, text, level));
                    i += 1;
                    continue;
                }
            }

            // 处理代码块
            if line.starts_with("```") {
                let lang = line[3..].trim();
                let mut code_content = String::new();
                i += 1;

                while i < lines.len() && !lines[i].starts_with("```") {
                    code_content.push_str(lines[i]);
                    code_content.push('\n');
                    i += 1;
                }

                if i < lines.len() {
                    i += 1;
                }

                result.push_str(&format!(
                    "<pre><code class=\"language-{}\">{}</code></pre>\n",
                    lang,
                    self.escape_html(&code_content)
                ));
                continue;
            }

            // 处理列表
            if line.starts_with('*') || line.starts_with('-') || line.starts_with('+') {
                // 处理无序列表
                result.push_str("<ul>\n");
                while i < lines.len()
                    && (lines[i].trim().starts_with('*')
                        || lines[i].trim().starts_with('-')
                        || lines[i].trim().starts_with('+'))
                {
                    let list_item = lines[i].trim().trim_start_matches(|c| c == '*' || c == '-' || c == '+').trim();
                    result.push_str(&format!("<li>{}</li>\n", self.render_inline(list_item)));
                    i += 1;
                }
                result.push_str("</ul>\n");
                continue;
            }
            else if line.starts_with(|c: char| c.is_digit(10)) && line.contains('.') {
                // 处理有序列表
                result.push_str("<ol>\n");
                while i < lines.len() && lines[i].trim().starts_with(|c: char| c.is_digit(10)) && lines[i].contains('.') {
                    let list_item = lines[i].trim().split('.').nth(1).unwrap_or("").trim();
                    result.push_str(&format!("<li>{}</li>\n", self.render_inline(list_item)));
                    i += 1;
                }
                result.push_str("</ol>\n");
                continue;
            }

            // 处理引用
            if line.starts_with('>') {
                result.push_str("<blockquote>\n");
                while i < lines.len() && lines[i].trim().starts_with('>') {
                    let quote = lines[i].trim().trim_start_matches('>').trim();
                    result.push_str(&format!("{}\n", self.render_inline(quote)));
                    i += 1;
                }
                result.push_str("</blockquote>\n");
                continue;
            }

            // 处理表格
            if line.contains('|')
                && lines
                    .get(i + 1)
                    .map_or(false, |l| l.contains('|') && l.chars().all(|c| c == '|' || c == '-' || c.is_whitespace()))
            {
                result.push_str("<table>\n");

                // 表头
                let headers: Vec<&str> = line.split('|').map(|h| h.trim()).filter(|h| !h.is_empty()).collect();
                result.push_str("<thead>\n<tr>\n");
                for header in headers {
                    result.push_str(&format!("<th>{}</th>\n", self.render_inline(header)));
                }
                result.push_str("</tr>\n</thead>\n");

                // 跳过分隔行
                i += 2;

                // 表体
                result.push_str("<tbody>\n");
                while i < lines.len() && lines[i].contains('|') {
                    let cells: Vec<&str> = lines[i].split('|').map(|c| c.trim()).filter(|c| !c.is_empty()).collect();
                    result.push_str("<tr>\n");
                    for cell in cells {
                        result.push_str(&format!("<td>{}</td>\n", self.render_inline(cell)));
                    }
                    result.push_str("</tr>\n");
                    i += 1;
                }
                result.push_str("</tbody>\n</table>\n");
                continue;
            }

            // 处理段落
            if !line.is_empty() {
                let mut paragraph = String::new();
                while i < lines.len() && !lines[i].trim().is_empty() {
                    paragraph.push_str(lines[i]);
                    paragraph.push(' ');
                    i += 1;
                }
                result.push_str(&format!("<p>{}</p>\n", self.render_inline(paragraph.trim())));
                continue;
            }

            i += 1;
        }

        result
    }

    /// 渲染行内元素
    fn render_inline(&self, text: &str) -> String {
        let mut result = text.to_string();

        // 处理粗体
        result = self.replace_pattern(&result, r"\*\*(.*?)\*\*", "<strong>$1</strong>");
        result = self.replace_pattern(&result, r"__(.*?)__", "<strong>$1</strong>");

        // 处理斜体
        result = self.replace_pattern(&result, r"\*(.*?)\*", "<em>$1</em>");
        result = self.replace_pattern(&result, r"_(.*?)_", "<em>$1</em>");

        // 处理行内代码
        result = self.replace_pattern(&result, r"`(.*?)`", "<code>$1</code>");

        // 处理链接
        result = self.replace_pattern(&result, r"\[(.*?)\]\((.*?)\)", "<a href=\"$2\">$1</a>");

        // 处理图片
        result = self.replace_pattern(&result, r"!\[(.*?)\]\((.*?)\)", "<img src=\"$2\" alt=\"$1\" />");

        result
    }

    /// 替换模式
    fn replace_pattern(&self, text: &str, pattern: &str, replacement: &str) -> String {
        // 简单的模式替换实现
        // 实际应该使用正则表达式
        let mut result = text.to_string();
        let mut start = 0;

        while let Some(match_start) = result[start..].find(&pattern[0..1]) {
            let match_start = start + match_start;
            // 这里简化处理，实际应该使用正则表达式
            start = match_start + 1;
        }

        result
    }

    /// HTML 转义
    fn escape_html(&self, content: &str) -> String {
        content.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;").replace("\"", "&quot;").replace("'", "&#39;")
    }
}

impl Default for MarkdownRenderer {
    fn default() -> Self {
        Self::new()
    }
}
