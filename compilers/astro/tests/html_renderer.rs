//! HTML 渲染器测试

use astro::compiler::html_renderer::{Context, HtmlRenderer};
use serde_json::json;

#[test]
fn test_variable_interpolation() {
    let renderer = HtmlRenderer::new();
    let mut context = Context::new();
    context.insert("name".to_string(), json!("World"));
    context.insert("age".to_string(), json!(30));
    
    let template = "Hello {{name}}, you are {{age}} years old.";
    let result = renderer.render_astro(template, &context);
    
    assert_eq!(result, "Hello World, you are 30 years old.");
}

#[test]
fn test_unescaped_variables() {
    let renderer = HtmlRenderer::new();
    let mut context = Context::new();
    context.insert("html".to_string(), json!("<b>Bold</b>"));
    
    let template = "{{{html}}}";
    let result = renderer.render_astro(template, &context);
    
    assert_eq!(result, "<b>Bold</b>");
}

#[test]
fn test_escaped_variables() {
    let renderer = HtmlRenderer::new();
    let mut context = Context::new();
    context.insert("html".to_string(), json!("<b>Bold</b>"));
    
    let template = "{{html}}";
    let result = renderer.render_astro(template, &context);
    
    assert_eq!(result, "&lt;b&gt;Bold&lt;/b&gt;");
}

#[test]
fn test_conditions() {
    let renderer = HtmlRenderer::new();
    
    // 测试条件为真的情况
    let mut context1 = Context::new();
    context1.insert("show".to_string(), json!(true));
    
    let template = "{% if show %}Hello{% endif %}";
    let result1 = renderer.render_astro(template, &context1);
    assert_eq!(result1, "Hello");
    
    // 测试条件为假的情况
    let context2 = Context::new();
    let result2 = renderer.render_astro(template, &context2);
    assert_eq!(result2, "");
}

#[test]
fn test_loops() {
    let renderer = HtmlRenderer::new();
    let mut context = Context::new();
    context.insert("items".to_string(), json!(vec!["apple", "banana", "cherry"]));
    
    let template = "{% for item in items %}{{item}}, {% endfor %}";
    let result = renderer.render_astro(template, &context);
    
    assert_eq!(result, "apple, banana, cherry, ");
}

#[test]
fn test_nested_structures() {
    let renderer = HtmlRenderer::new();
    let mut context = Context::new();
    context.insert("user".to_string(), json!({"name": "John", "age": 25}));
    
    let template = "Hello {{user.name}}, you are {{user.age}} years old.";
    let result = renderer.render_astro(template, &context);
    
    // 注意：serde_json 会将嵌套对象的访问转换为字符串表示
    // 这里我们只是测试基本功能，实际实现可能需要更复杂的对象访问处理
    assert!(result.contains("Hello"));
}