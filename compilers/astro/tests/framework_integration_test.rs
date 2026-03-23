//! 框架集成测试

use astro::compiler::{
    component::{Component, ComponentRegistry, Framework},
    framework_parser::{FrameworkParser, FrameworkParserManager, ReactParser, SvelteParser, VueParser},
    renderer::html_renderer::{Context, HtmlRenderer},
};
use std::path::Path;

#[test]
fn test_framework_enum() {
    // 测试框架枚举的创建
    let react = Framework::React;
    let vue = Framework::Vue;
    let svelte = Framework::Svelte;
    let solid = Framework::Solid;
    let preact = Framework::Preact;
    let lit = Framework::Lit;
    let other = Framework::Other("CustomFramework".to_string());

    // 测试框架枚举的比较
    assert_eq!(react, Framework::React);
    assert_eq!(vue, Framework::Vue);
    assert_eq!(svelte, Framework::Svelte);
    assert_eq!(solid, Framework::Solid);
    assert_eq!(preact, Framework::Preact);
    assert_eq!(lit, Framework::Lit);
    assert_eq!(other, Framework::Other("CustomFramework".to_string()));
}

#[test]
fn test_component_creation() {
    // 测试创建不同框架的组件
    let react_component = Component::new_react("TestReact", "<div>React Component</div>");
    let vue_component = Component::new_vue("TestVue", "<div>Vue Component</div>");
    let svelte_component = Component::new_svelte("TestSvelte", "<div>Svelte Component</div>");
    let solid_component = Component::new_solid("TestSolid", "<div>Solid Component</div>");
    let preact_component = Component::new_preact("TestPreact", "<div>Preact Component</div>");
    let lit_component = Component::new_lit("TestLit", "<div>Lit Component</div>");
    let other_component = Component::new_other("TestOther", "<div>Other Component</div>", "CustomFramework");

    // 测试组件属性
    assert_eq!(react_component.name(), "TestReact");
    assert_eq!(*react_component.framework(), Framework::React);

    assert_eq!(vue_component.name(), "TestVue");
    assert_eq!(*vue_component.framework(), Framework::Vue);

    assert_eq!(svelte_component.name(), "TestSvelte");
    assert_eq!(*svelte_component.framework(), Framework::Svelte);

    assert_eq!(solid_component.name(), "TestSolid");
    assert_eq!(*solid_component.framework(), Framework::Solid);

    assert_eq!(preact_component.name(), "TestPreact");
    assert_eq!(*preact_component.framework(), Framework::Preact);

    assert_eq!(lit_component.name(), "TestLit");
    assert_eq!(*lit_component.framework(), Framework::Lit);

    assert_eq!(other_component.name(), "TestOther");
    assert_eq!(*other_component.framework(), Framework::Other("CustomFramework".to_string()));
}

#[test]
fn test_component_rendering() {
    // 创建不同框架的组件
    let react_component = Component::new_react("TestReact", "<div>React Component</div>");
    let vue_component = Component::new_vue("TestVue", "<div>Vue Component</div>");
    let svelte_component = Component::new_svelte("TestSvelte", "<div>Svelte Component</div>");

    // 创建上下文
    let mut context = Context::new();
    context.insert("message".to_string(), serde_json::Value::String("Hello".to_string()));

    // 测试组件渲染
    let react_rendered = react_component.render(&context);
    assert!(react_rendered.contains("data-react-component=\"TestReact\""));

    let vue_rendered = vue_component.render(&context);
    assert!(vue_rendered.contains("data-vue-component=\"TestVue\""));

    let svelte_rendered = svelte_component.render(&context);
    assert!(svelte_rendered.contains("data-svelte-component=\"TestSvelte\""));
}

#[test]
fn test_framework_parser() {
    // 测试框架解析器
    let react_parser = ReactParser;
    let vue_parser = VueParser;
    let svelte_parser = SvelteParser;

    // 测试文件类型检测
    let react_path = Path::new("TestComponent.jsx");
    let vue_path = Path::new("TestComponent.vue");
    let svelte_path = Path::new("TestComponent.svelte");
    let other_path = Path::new("TestComponent.txt");

    assert!(react_parser.is_framework_component(react_path));
    assert!(!react_parser.is_framework_component(vue_path));
    assert!(!react_parser.is_framework_component(svelte_path));

    assert!(!vue_parser.is_framework_component(react_path));
    assert!(vue_parser.is_framework_component(vue_path));
    assert!(!vue_parser.is_framework_component(svelte_path));

    assert!(!svelte_parser.is_framework_component(react_path));
    assert!(!svelte_parser.is_framework_component(vue_path));
    assert!(svelte_parser.is_framework_component(svelte_path));

    // 测试框架类型获取
    assert_eq!(react_parser.framework(), Framework::React);
    assert_eq!(vue_parser.framework(), Framework::Vue);
    assert_eq!(svelte_parser.framework(), Framework::Svelte);
}

#[test]
fn test_framework_parser_manager() {
    // 测试框架解析器管理器
    let manager = FrameworkParserManager::new();

    // 测试文件类型检测
    let react_path = Path::new("TestComponent.jsx");
    let vue_path = Path::new("TestComponent.vue");
    let svelte_path = Path::new("TestComponent.svelte");
    let other_path = Path::new("TestComponent.txt");

    assert_eq!(manager.get_framework(react_path), Some(Framework::React));
    assert_eq!(manager.get_framework(vue_path), Some(Framework::Vue));
    assert_eq!(manager.get_framework(svelte_path), Some(Framework::Svelte));
    assert_eq!(manager.get_framework(other_path), None);
}

#[test]
fn test_component_registry() {
    // 测试组件注册表
    let mut registry = ComponentRegistry::new();

    // 创建测试组件
    let react_component = Component::new_react("TestReact", "<div>React Component</div>");
    let vue_component = Component::new_vue("TestVue", "<div>Vue Component</div>");

    // 注册组件
    registry.register(react_component);
    registry.register(vue_component);

    // 测试组件获取
    assert!(registry.exists("TestReact"));
    assert!(registry.exists("TestVue"));
    assert!(!registry.exists("TestSvelte"));

    let react_component = registry.get("TestReact").unwrap();
    assert_eq!(react_component.name(), "TestReact");
    assert_eq!(*react_component.framework(), Framework::React);

    let vue_component = registry.get("TestVue").unwrap();
    assert_eq!(vue_component.name(), "TestVue");
    assert_eq!(*vue_component.framework(), Framework::Vue);
}

#[test]
fn test_html_renderer_with_framework_components() {
    // 创建 HTML 渲染器
    let mut renderer = HtmlRenderer::new();

    // 创建组件注册表
    let mut registry = ComponentRegistry::new();

    // 创建测试组件
    let react_component = Component::new_react("TestReact", "<div>React Component</div>");
    let vue_component = Component::new_vue("TestVue", "<div>Vue Component</div>");

    // 注册组件
    registry.register(react_component);
    registry.register(vue_component);

    // 设置组件注册表
    renderer.set_component_registry(registry);

    // 创建上下文
    let mut context = Context::new();
    context.insert("message".to_string(), serde_json::Value::String("Hello".to_string()));

    // 测试渲染包含框架组件的模板
    let template = r#"
    <div>
        <h1>{{ message }}</h1>
        <TestReact />
        <TestVue />
    </div>
    "#;

    let rendered = renderer.render_astro(template, &context);
    assert!(rendered.contains("Hello"));
    assert!(rendered.contains("data-react-component=\"TestReact\""));
    assert!(rendered.contains("data-vue-component=\"TestVue\""));
}
