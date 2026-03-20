//! 测试 Markdown 处理功能

use hexo::markdown::{parse_markdown, render_markdown, render_markdown_file};
use std::path::Path;

fn main() {
    println!("测试 Markdown 解析和渲染功能...");

    // 测试 1: 解析带有 Front Matter 的 Markdown
    println!("\n测试 1: 解析带有 Front Matter 的 Markdown");
    let content = "---\ntitle: Test Post\ndate: 2026-03-20\ncategories:\n  - Test\ntags:\n  - Rust\n  - Hexo\ncustom_field: value\n---\n\n# Hello World\n\nThis is a test post.";
    
    match parse_markdown(content) {
        Ok((front_matter, markdown_content)) => {
            println!("✓ 解析成功");
            if let Some(fm) = front_matter {
                println!("  标题: {:?}", fm.title);
                println!("  日期: {:?}", fm.date);
                println!("  分类: {:?}", fm.categories);
                println!("  标签: {:?}", fm.tags);
            }
            println!("  内容: {}", markdown_content);
        }
        Err(e) => {
            println!("✗ 解析失败: {:?}", e);
        }
    }

    // 测试 2: 渲染 Markdown
    println!("\n测试 2: 渲染 Markdown");
    let content = "# Hello World\n\nThis is a **test** post with *italic* text.";
    let html = render_markdown(content);
    println!("✓ 渲染成功");
    println!("  HTML: {}", html);

    // 测试 3: 渲染 Markdown 文件
    println!("\n测试 3: 渲染 Markdown 文件");
    let test_file = Path::new("examples/hexo-mvp/source/_posts/guide.md");
    
    match render_markdown_file(test_file) {
        Ok((front_matter, html)) => {
            println!("✓ 渲染文件成功");
            if let Some(fm) = front_matter {
                println!("  标题: {:?}", fm.title);
            }
            println!("  HTML 长度: {} 字符", html.len());
        }
        Err(e) => {
            println!("✗ 渲染文件失败: {:?}", e);
        }
    }

    println!("\n测试完成!");
}