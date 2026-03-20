//! Markdown 渲染测试

use vitepress::compiler::HtmlRenderer;

#[test]
fn test_markdown_render() {
    // 创建 HTML 渲染器
    let renderer = HtmlRenderer::new();

    // 测试基本的 Markdown 语法
    let markdown = r#"
# 标题 1

## 标题 2

这是一个 **粗体** 文本和 *斜体* 文本。

- 列表项 1
- 列表项 2

```rust
fn main() {
    println!("Hello, world!");
}
```

> 这是一个引用

| 表头 1 | 表头 2 |
|--------|--------|
| 单元格 1 | 单元格 2 |
| 单元格 3 | 单元格 4 |
"#;

    // 渲染 Markdown
    let html = renderer.render(markdown);

    // 验证渲染结果
    assert!(html.contains("<h1>标题 1</h1>"));
    assert!(html.contains("<h2>标题 2</h2>"));
    assert!(html.contains("<strong>粗体</strong>"));
    assert!(html.contains("<em>斜体</em>"));
    assert!(html.contains("<ul>"));
    assert!(html.contains("<li>列表项 1</li>"));
    assert!(html.contains("<li>列表项 2</li>"));
    assert!(html.contains("<pre><code class=\"language-rust\">"));
    assert!(html.contains("<blockquote>"));
    assert!(html.contains("<table>"));
    assert!(html.contains("<th>表头 1</th>"));
    assert!(html.contains("<td>单元格 1</td>"));

    println!("Markdown 渲染测试通过！");
    println!("渲染结果:");
    println!("{}", html);
}
