//! 列表渲染测试

use vitepress::compiler::HtmlRenderer;

#[test]
fn test_list_render() {
    // 创建 HTML 渲染器
    let renderer = HtmlRenderer::new();

    // 测试简单的无序列表
    let markdown = r#"- 列表项 1
- 列表项 2
- 列表项 3"#;

    // 渲染 Markdown
    let html = renderer.render(markdown);

    // 打印渲染结果以便调试
    println!("渲染结果:");
    println!("{}", html);

    // 验证渲染结果
    assert!(html.contains("<ul>"));
    assert!(html.contains("列表项 1"));
    assert!(html.contains("列表项 2"));
    assert!(html.contains("列表项 3"));

    println!("列表渲染测试通过！");
}
