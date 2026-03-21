//! 组件系统测试

use crate::compiler::{Component, ComponentRegistry, HtmlRenderer};
use crate::compiler::html_renderer::Context;

#[test]
fn test_component_creation() {
    // 创建一个简单的组件
    let mut component = Component::new_astro("TestComponent", "Hello, {{{name}}}!");
    component.set_script("console.log('TestComponent loaded');");
    component.set_style("h1 { color: blue; }");
    
    assert_eq!(component.name(), "TestComponent");
}

#[test]
fn test_component_registry() {
    // 创建组件注册表
    let mut registry = ComponentRegistry::new();
    
    // 创建并注册组件
    let component = Component::new_astro("TestComponent", "Hello, {{{name}}}!");
    registry.register(component);
    
    // 检查组件是否存在
    assert!(registry.exists("TestComponent"));
    assert!(!registry.exists("NonExistentComponent"));
    
    // 获取组件
    let retrieved_component = registry.get("TestComponent").unwrap();
    assert_eq!(retrieved_component.name(), "TestComponent");
}

#[test]
fn test_component_rendering() {
    // 创建组件
    let component = Component::new_astro("TestComponent", "<h1>Hello, {{{name}}}!</h1>");
    
    // 创建 props
    let mut props = Context::new();
    props.insert("name".to_string(), serde_json::Value::String("World".to_string()));
    
    // 渲染组件
    let rendered = component.render(&props);
    assert_eq!(rendered, "<h1>Hello, World!</h1>");
}

#[test]
fn test_html_renderer_with_components() {
    // 创建组件
    let component = Component::new_astro("TestComponent", "<h1>Hello, {{{name}}}!</h1>");
    
    // 创建组件注册表
    let mut registry = ComponentRegistry::new();
    registry.register(component);
    
    // 创建 HTML 渲染器
    let mut renderer = HtmlRenderer::new();
    renderer.set_component_registry(registry);
    
    // 创建模板
    let template = "<div><TestComponent name=\"World\" /></div>";
    
    // 创建上下文
    let context = Context::new();
    
    // 渲染模板
    let rendered = renderer.render_astro(template, &context);
    assert_eq!(rendered, "<div><h1>Hello, World!</h1></div>");
}

#[test]
fn test_component_with_expressions() {
    // 创建组件
    let component = Component::new_astro("TestComponent", "<h1>Hello, {{{name}}}!</h1><p>Age: {{age}}</p>");
    
    // 创建组件注册表
    let mut registry = ComponentRegistry::new();
    registry.register(component);
    
    // 创建 HTML 渲染器
    let mut renderer = HtmlRenderer::new();
    renderer.set_component_registry(registry);
    
    // 测试组件直接渲染
    let mut props = Context::new();
    props.insert("name".to_string(), serde_json::Value::String("World".to_string()));
    props.insert("age".to_string(), serde_json::Value::String("25".to_string()));
    
    let component = renderer.component_registry().get("TestComponent").unwrap();
    let direct_rendered = component.render(&props);
    println!("Direct rendered: {}", direct_rendered);
    
    // 创建模板
    let template = "<div><TestComponent name=\"World\" age=\"25\" /></div>";
    
    // 创建上下文
    let context = Context::new();
    
    // 渲染模板
    let rendered = renderer.render_astro(template, &context);
    println!("Template rendered: {}", rendered);
    assert_eq!(rendered, "<div><h1>Hello, World!</h1><p>Age: 25</p></div>");
}
