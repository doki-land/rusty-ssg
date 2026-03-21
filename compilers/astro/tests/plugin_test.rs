//! 插件系统测试

use astro::{
    compiler::HtmlRenderer,
    plugin::{ExamplePlugin, PluginConfig, PluginManager},
    plugin_host::PluginHost,
};
use std::path::Path;

#[test]
fn test_plugin_manager() {
    // 创建插件管理器
    let mut manager = PluginManager::new();

    // 创建插件配置
    let config = PluginConfig {
        name: "example-plugin".to_string(),
        options: serde_json::json!({
            "key": "value",
            "number": 42
        }),
    };

    // 创建插件实例
    let plugin = ExamplePlugin::new(config);

    // 注册插件
    manager.register(std::sync::Arc::new(plugin)).unwrap();

    // 初始化所有插件
    manager.init_all().unwrap();

    // 执行插件
    let content = "Hello, World!";
    let result = manager.execute_all(content).unwrap();

    println!("Original content: {}", content);
    println!("Processed content: {}", result);

    // 验证插件执行结果
    assert!(result.contains("<!-- Example Plugin Start -->"));
    assert!(result.contains("Hello, World!"));
    assert!(result.contains("<!-- Example Plugin End -->"));
}

#[test]
fn test_plugin_host() {
    // 创建插件宿主
    let mut host = PluginHost::new("test-config");

    // 创建插件配置
    let config = PluginConfig {
        name: "example-plugin".to_string(),
        options: serde_json::json!({
            "key": "value"
        }),
    };

    // 注册插件
    let plugin = ExamplePlugin::new(config);
    host.register_plugin(plugin).unwrap();

    // 初始化插件
    host.init_plugins().unwrap();

    // 执行插件
    let content = "Hello, Plugin Host!";
    let result = host.execute_plugins(content).unwrap();

    println!("Original content: {}", content);
    println!("Processed content: {}", result);

    // 验证插件执行结果
    assert!(result.contains("<!-- Example Plugin Start -->"));
    assert!(result.contains("Hello, Plugin Host!"));
    assert!(result.contains("<!-- Example Plugin End -->"));
}

#[test]
fn test_html_renderer_with_plugins() {
    // 创建插件宿主
    let mut host = PluginHost::new("test-config");

    // 创建插件配置
    let config = PluginConfig {
        name: "example-plugin".to_string(),
        options: serde_json::json!({
            "key": "value"
        }),
    };

    // 注册插件
    let plugin = ExamplePlugin::new(config);
    host.register_plugin(plugin).unwrap();

    // 初始化插件
    host.init_plugins().unwrap();

    // 创建 HTML 渲染器
    let mut renderer = HtmlRenderer::new();
    renderer.set_plugin_host(host);

    // 渲染 Markdown 内容
    let markdown = "# Hello, Markdown!\n\nThis is a test.";
    let result = renderer.render(markdown);

    println!("Markdown content: {}", markdown);
    println!("Rendered content: {}", result);

    // 验证渲染结果
    assert!(result.contains("<!-- Example Plugin Start -->"));
    assert!(result.contains("<h1>Hello, Markdown!</h1>"));
    assert!(result.contains("<p>This is a test.</p>"));
    assert!(result.contains("<!-- Example Plugin End -->"));
}
