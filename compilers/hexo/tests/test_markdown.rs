//! 简单的 Markdown 解析和渲染测试

use hexo::markdown::{parse_markdown, render_markdown};

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
