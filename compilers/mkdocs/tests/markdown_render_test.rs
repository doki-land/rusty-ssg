//! Markdown 渲染测试

use mkdocs::{compile_batch, compile_single, compiler::HtmlRenderer};
use std::collections::HashMap;

#[test]
fn test_markdown_render_basic() {
    let renderer = HtmlRenderer::new();

    let markdown = r#"
# 标题 1

## 标题 2

这是一个 **粗体** 文本和 *斜体* 文本。
"#;

    let html = renderer.render(markdown);

    assert!(html.contains("<h1>"));
    assert!(html.contains("标题 1"));
    assert!(html.contains("<h2>"));
    assert!(html.contains("标题 2"));
    assert!(html.contains("<strong>") || html.contains("<b>"));
    assert!(html.contains("<em>") || html.contains("<i>"));
}

#[test]
fn test_markdown_render_lists() {
    let renderer = HtmlRenderer::new();

    let markdown = r#"
- 列表项 1
- 列表项 2
- 列表项 3

1. 有序项 1
2. 有序项 2
3. 有序项 3
"#;

    let html = renderer.render(markdown);

    assert!(html.contains("<ul>"));
    assert!(html.contains("<ol>"));
    assert!(html.contains("<li>"));
    assert!(html.contains("列表项 1"));
    assert!(html.contains("有序项 1"));
}

#[test]
fn test_markdown_render_code() {
    let renderer = HtmlRenderer::new();

    let markdown = r#"
```rust
fn main() {
    println!("Hello, world!");
}
```

内联代码: `let x = 5;`
"#;

    let html = renderer.render(markdown);

    assert!(html.contains("<pre>") || html.contains("<code"));
    assert!(html.contains("rust"));
    assert!(html.contains("fn main()"));
    assert!(html.contains("println!"));
}

#[test]
fn test_markdown_render_links() {
    let renderer = HtmlRenderer::new();

    let markdown = r#"
[链接文本](https://example.com)

![图片说明](https://example.com/image.png)
"#;

    let html = renderer.render(markdown);

    assert!(html.contains("<a") || html.contains("href"));
    assert!(html.contains("example.com"));
    assert!(html.contains("<img") || html.contains("src"));
}

#[test]
fn test_markdown_render_quotes() {
    let renderer = HtmlRenderer::new();

    let markdown = r#"> 这是一个引用
> 
> 这是引用的第二行"#;

    let html = renderer.render(markdown);

    assert!(html.contains("<blockquote>"));
}

#[test]
fn test_compile_single() {
    let markdown = "# 测试标题\n\n这是测试内容。";
    let result = compile_single(markdown);
    assert!(result.is_ok());
    let html = result.unwrap();
    assert!(!html.is_empty());
}

#[test]
fn test_compile_batch() {
    let mut documents = HashMap::new();
    documents.insert("index.md".to_string(), "# 首页\n\n欢迎来到首页！".to_string());
    documents.insert("about.md".to_string(), "# 关于\n\n这是关于页面。".to_string());

    let result = compile_batch(&documents);
    assert!(result.success);
    assert_eq!(result.documents.len(), 2);
    assert!(result.compile_time_ms >= 0);
}

#[test]
fn test_html_renderer_with_config() {
    use mkdocs::compiler::HtmlRendererConfig;
    let mut config = HtmlRendererConfig::default();
    config.options.insert("key".to_string(), "value".to_string());

    let renderer = HtmlRenderer::with_config(config);
    assert_eq!(renderer.config().options.get("key"), Some(&"value".to_string()));

    let html = renderer.render("# 测试");
    assert!(!html.is_empty());
}

#[test]
fn test_empty_markdown() {
    let renderer = HtmlRenderer::new();
    let html = renderer.render("");
    assert!(!html.is_empty());
}
