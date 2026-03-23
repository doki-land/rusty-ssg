//! Vue 组件渲染测试

use vuepress::compiler::{HtmlRenderer, VuePressCompiler};
use vuepress::types::VuePressConfig;

#[test]
fn test_vue_component_rendering() {
    // 创建编译器实例
    let config = VuePressConfig::new();
    let mut compiler = VuePressCompiler::with_config(config);

    // 测试包含 Vue 组件的 Markdown 内容
    let markdown = r#"
# Test Page

This is a test page with Vue components.

<MyComponent title="Hello World">
  <p>This is the content of the component.</p>
</MyComponent>

## Another Section

<AnotherComponent />
"#;

    // 编译文档
    let result = compiler.compile_document(markdown, "test.md");
    assert!(result.is_ok());

    let doc = result.unwrap();
    assert!(doc.rendered_content.is_some());

    let rendered_content = doc.rendered_content.unwrap();
    println!("Rendered content: {}", rendered_content);

    // 验证 Vue 组件标签是否被正确保留
    assert!(rendered_content.contains("<MyComponent title=\"Hello World\">"));
    assert!(rendered_content.contains("<p>This is the content of the component.</p>"));
    assert!(rendered_content.contains("</MyComponent>"));
    assert!(rendered_content.contains("<AnotherComponent />"));

    // 验证 Vue 运行时脚本是否被添加
    assert!(rendered_content.contains("https://cdn.jsdelivr.net/npm/vue@3.3.4/dist/vue.global.prod.js"));
    assert!(rendered_content.contains("const { createApp } = Vue"));
    assert!(rendered_content.contains("createApp({}).mount('#app')"));
}

#[test]
fn test_html_renderer_with_vue_components() {
    // 创建 HTML 渲染器实例
    let config = VuePressConfig::new();
    let renderer = HtmlRenderer::new(config);

    // 测试包含 Vue 组件的 Markdown 内容
    let markdown = r#"
# Test Page

<MyComponent>
  <p>Hello</p>
</MyComponent>
"#;

    // 渲染内容
    let result = renderer.render(markdown);
    assert!(result.is_ok());

    let rendered = result.unwrap();
    println!("Rendered HTML: {}", rendered);

    // 验证 Vue 组件标签是否被正确保留
    assert!(rendered.contains("<MyComponent>"));
    assert!(rendered.contains("<p>Hello</p>"));
    assert!(rendered.contains("</MyComponent>"));
}
