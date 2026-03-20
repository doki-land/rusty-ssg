//! 独立的 Markdown 解析和渲染测试

use oak_core::{Builder, ParseSession};
use oak_markdown::{MarkdownBuilder, MarkdownLanguage};
use oak_yaml;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
pub fn parse_markdown(content: &str) -> Result<(Option<FrontMatter>, String), Box<dyn std::error::Error>> {
    // 检查是否有 Front Matter
    if content.starts_with("---\n") {
        // 找到第二个 --- 分隔符
        if let Some(end_idx) = content.find("\n---\n") {
            let front_matter_str = &content[4..end_idx + 1];
            let content_str = &content[end_idx + 5..];

            // 解析 Front Matter
            let front_matter: FrontMatter = oak_yaml::language::from_str(front_matter_str)?;

            Ok((Some(front_matter), content_str.to_string()))
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
    // 配置 Markdown 解析选项
    let mut lang_config = MarkdownLanguage::default();
    lang_config.allow_tables = true;
    lang_config.allow_footnotes = true;
    lang_config.allow_strikethrough = true;
    lang_config.allow_task_lists = true;

    // 创建 Markdown 构建器
    let builder = MarkdownBuilder::new(&lang_config);
    let mut cache = ParseSession::default();
    let build_output = builder.build(content, &[], &mut cache);

    match build_output.result {
        Ok(ast_root) => render_ast(&ast_root),
        Err(_) => content.to_string(),
    }
}

/// 渲染 Markdown AST 为 HTML
fn render_ast(root: &oak_markdown::ast::MarkdownRoot) -> String {
    let mut html = String::new();

    for block in &root.blocks {
        html.push_str(&render_block(block));
    }

    html
}

/// 渲染块级元素
fn render_block(block: &oak_markdown::ast::Block) -> String {
    use oak_markdown::ast::Block;

    match block {
        Block::Heading(heading) => render_heading(heading),
        Block::Paragraph(paragraph) => render_paragraph(paragraph),
        Block::CodeBlock(code_block) => render_code_block(code_block),
        Block::List(list) => render_list(list),
        Block::Blockquote(blockquote) => render_blockquote(blockquote),
        Block::HorizontalRule(_) => "<hr /\n".to_string(),
        Block::Table(table) => render_table(table),
        Block::Html(html) => render_html(html),
        Block::AbbreviationDefinition(_) => String::new(),
    }
}

/// 渲染标题
fn render_heading(heading: &oak_markdown::ast::Heading) -> String {
    let tag = format!("h{}", heading.level);
    let escaped_content = escape_html(&heading.content);
    format!("<{}>{}</{}>\n", tag, escaped_content, tag)
}

/// 渲染段落
fn render_paragraph(paragraph: &oak_markdown::ast::Paragraph) -> String {
    let escaped_content = escape_html(&paragraph.content);
    format!("<p>{}</p>\n", escaped_content)
}

/// 渲染代码块
fn render_code_block(code_block: &oak_markdown::ast::CodeBlock) -> String {
    let class = if let Some(lang) = &code_block.language {
        format!(" class=\"language-{}\"", escape_html(lang))
    }
    else {
        String::new()
    };
    let escaped_content = escape_html(&code_block.content);
    format!("<pre><code{}>{}</code></pre>\n", class, escaped_content)
}

/// 渲染列表
fn render_list(list: &oak_markdown::ast::List) -> String {
    let tag = if list.is_ordered { "ol" } else { "ul" };
    let mut html = format!("<{}>\n", tag);

    for item in &list.items {
        html.push_str(&render_list_item(item));
    }

    html.push_str(&format!("</{}>\n", tag));
    html
}

/// 渲染列表项
fn render_list_item(list_item: &oak_markdown::ast::ListItem) -> String {
    let mut html = String::from("<li>");

    if list_item.is_task {
        let checked = if list_item.is_checked.unwrap_or(false) { "checked" } else { "" };
        html.push_str(&format!("<input type=\"checkbox\" disabled {} /> ", checked));
    }

    for block in &list_item.content {
        html.push_str(&render_block(block));
    }

    html.push_str("</li>\n");
    html
}

/// 渲染引用块
fn render_blockquote(blockquote: &oak_markdown::ast::Blockquote) -> String {
    let mut html = String::from("<blockquote>\n");

    for block in &blockquote.content {
        html.push_str(&render_block(block));
    }

    html.push_str("</blockquote>\n");
    html
}

/// 渲染表格
fn render_table(table: &oak_markdown::ast::Table) -> String {
    let mut html = String::from("<table>\n");

    html.push_str("<thead>\n<tr>\n");
    for cell in &table.header.cells {
        html.push_str(&render_table_cell(cell, "th"));
    }
    html.push_str("</tr>\n</thead>\n");

    html.push_str("<tbody>\n");
    for row in &table.rows {
        html.push_str("<tr>\n");
        for cell in &row.cells {
            html.push_str(&render_table_cell(cell, "td"));
        }
        html.push_str("</tr>\n");
    }
    html.push_str("</tbody>\n");

    html.push_str("</table>\n");
    html
}

/// 渲染表格单元格
fn render_table_cell(cell: &oak_markdown::ast::TableCell, tag: &str) -> String {
    let escaped_content = escape_html(&cell.content);
    format!("<{}>{}</{}>\n", tag, escaped_content, tag)
}

/// 渲染 HTML 块
fn render_html(html: &oak_markdown::ast::Html) -> String {
    format!("{}\n", html.content)
}

/// 转义 HTML 特殊字符
fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('\"', "&quot;").replace('\'', "&#39;")
}

fn main() {
    println!("Testing Markdown parsing and rendering...");

    // 测试带有 Front Matter 的 Markdown
    let content_with_front_matter = "---\ntitle: Test Post\ndate: 2026-03-20\ncategories:\n  - Test\ntags:\n  - Rust\n  - Hexo\ncustom_field: value\n---\n\n# Hello World\n\nThis is a test post.";

    println!("\n1. Testing Markdown with Front Matter:");
    match parse_markdown(content_with_front_matter) {
        Ok((front_matter, markdown_content)) => {
            println!("   ✓ Front Matter parsed successfully");
            if let Some(fm) = front_matter {
                println!("   Title: {:?}", fm.title);
                println!("   Date: {:?}", fm.date);
                println!("   Categories: {:?}", fm.categories);
                println!("   Tags: {:?}", fm.tags);
            }
            println!("   Markdown content: {}", markdown_content);

            let html = render_markdown(&markdown_content);
            println!("   ✓ Markdown rendered to HTML successfully");
            println!("   HTML output: {}", html);
        }
        Err(e) => println!("   ✗ Error: {:?}", e),
    }

    // 测试没有 Front Matter 的 Markdown
    let content_without_front_matter = "# Hello World\n\nThis is a test post without Front Matter.";

    println!("\n2. Testing Markdown without Front Matter:");
    match parse_markdown(content_without_front_matter) {
        Ok((front_matter, markdown_content)) => {
            println!("   ✓ Markdown parsed successfully");
            println!("   Front Matter present: {:?}", front_matter.is_some());
            println!("   Markdown content: {}", markdown_content);

            let html = render_markdown(&markdown_content);
            println!("   ✓ Markdown rendered to HTML successfully");
            println!("   HTML output: {}", html);
        }
        Err(e) => println!("   ✗ Error: {:?}", e),
    }

    println!("\nAll tests completed!");
}
