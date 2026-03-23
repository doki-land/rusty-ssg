//! 插件测试
//! 测试 VitePress 插件系统的功能

use vitepress::plugin::{prism::PrismPlugin, PluginContext, PluginRegistry, VitePressPlugin};
use nargo_types::NargoValue;
use std::collections::HashMap;

#[test]
fn test_prism_plugin() {
    // 创建 Prism 插件
    let mut prism_plugin = PrismPlugin::new();

    // 测试插件元数据
    let meta = prism_plugin.meta();
    assert_eq!(meta.name, "prism");
    assert_eq!(meta.version, "0.1.0");

    // 测试插件设置
    let mut config = HashMap::new();
    config.insert("line_numbers".to_string(), NargoValue::Bool(true));
    config.insert("theme".to_string(), NargoValue::String("prism-tomorrow".to_string()));
    prism_plugin.setup(Some(config));

    // 测试渲染后钩子
    let context = PluginContext::from_content(
        r#"<pre><code class="language-rust">fn main() {}</code></pre>"#.to_string(),
        "test.md".to_string(),
    );

    let result = prism_plugin.after_render(context);
    assert!(result.content.contains("prism.min.js"));
    assert!(result.content.contains("prism-tomorrow.min.css"));
    assert!(result.content.contains("prism-line-numbers.min.js"));

    println!("Prism 插件测试通过！");
}

#[test]
fn test_plugin_registry() {
    // 创建插件注册表
    let mut registry = PluginRegistry::new();

    // 注册 Prism 插件
    let prism_plugin = PrismPlugin::new();
    registry.register(prism_plugin);

    // 检查插件数量
    assert_eq!(registry.plugin_count(), 1);

    // 测试插件元数据
    let metas = registry.plugin_metas();
    assert_eq!(metas.len(), 1);
    assert_eq!(metas[0].name, "prism");

    println!("插件注册表测试通过！");
}
