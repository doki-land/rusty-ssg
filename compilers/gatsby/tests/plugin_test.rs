use gatsby::plugin::{Plugin, PluginMeta, PluginContext, PluginRegistry, Node, Page};
use gatsby::config::PluginConfig;
use nargo_types::NargoValue;
use std::collections::HashMap;

// 测试插件实现
struct TestPlugin {
    meta: PluginMeta,
    called_hooks: Vec<String>,
}

impl Plugin for TestPlugin {
    fn meta(&self) -> &PluginMeta {
        &self.meta
    }

    fn on_pre_bootstrap(&mut self, _context: &mut PluginContext) -> gatsby::types::Result<()> {
        self.called_hooks.push("on_pre_bootstrap".to_string());
        Ok(())
    }

    fn on_bootstrap(&mut self, _context: &mut PluginContext) -> gatsby::types::Result<()> {
        self.called_hooks.push("on_bootstrap".to_string());
        Ok(())
    }

    fn on_post_bootstrap(&mut self, _context: &mut PluginContext) -> gatsby::types::Result<()> {
        self.called_hooks.push("on_post_bootstrap".to_string());
        Ok(())
    }

    fn on_pre_build(&mut self, _context: &mut PluginContext) -> gatsby::types::Result<()> {
        self.called_hooks.push("on_pre_build".to_string());
        Ok(())
    }

    fn on_post_build(&mut self, _context: &mut PluginContext) -> gatsby::types::Result<()> {
        self.called_hooks.push("on_post_build".to_string());
        Ok(())
    }
}

#[test]
fn test_plugin_registration() {
    let mut registry = PluginRegistry::new();
    let plugin = TestPlugin {
        meta: PluginMeta::new("test-plugin".to_string(), "1.0.0".to_string(), "Test plugin".to_string()),
        called_hooks: Vec::new(),
    };

    registry.register(plugin);
    assert_eq!(registry.plugin_count(), 1);
    assert_eq!(registry.plugin_metas().len(), 1);
    assert_eq!(registry.plugin_metas()[0].name, "test-plugin");
}

#[test]
fn test_plugin_hooks() {
    let mut registry = PluginRegistry::new();
    let mut plugin = TestPlugin {
        meta: PluginMeta::new("test-plugin".to_string(), "1.0.0".to_string(), "Test plugin".to_string()),
        called_hooks: Vec::new(),
    };

    let mut context = PluginContext::new();

    // 调用各个生命周期钩子
    plugin.on_pre_bootstrap(&mut context).unwrap();
    plugin.on_bootstrap(&mut context).unwrap();
    plugin.on_post_bootstrap(&mut context).unwrap();
    plugin.on_pre_build(&mut context).unwrap();
    plugin.on_post_build(&mut context).unwrap();

    // 验证所有钩子都被调用
    assert_eq!(plugin.called_hooks, [
        "on_pre_bootstrap",
        "on_bootstrap",
        "on_post_bootstrap",
        "on_pre_build",
        "on_post_build"
    ]);
}

#[test]
fn test_plugin_context() {
    let mut context = PluginContext::new();

    // 测试添加和获取节点
    let node = Node::new("test-node".to_string(), "TestType".to_string());
    context.add_node(node.clone());
    assert!(context.get_node("test-node").is_some());

    // 测试添加页面
    let page = Page::new("/test".to_string(), "test-component".to_string());
    context.add_page(page);
    assert_eq!(context.pages.len(), 1);
    assert_eq!(context.pages[0].path, "/test");
}

#[test]
fn test_plugin_registry_hooks() {
    let mut registry = PluginRegistry::new();
    let plugin = TestPlugin {
        meta: PluginMeta::new("test-plugin".to_string(), "1.0.0".to_string(), "Test plugin".to_string()),
        called_hooks: Vec::new(),
    };

    registry.register(plugin);
    let mut context = PluginContext::new();

    // 调用注册表的钩子方法
    registry.on_pre_bootstrap_all(&mut context).unwrap();
    registry.on_bootstrap_all(&mut context).unwrap();
    registry.on_post_bootstrap_all(&mut context).unwrap();

    // 验证插件被正确调用
    // 注意：由于插件被包装在Box中，我们无法直接访问其内部状态
    // 这里只测试调用不会失败
}

#[test]
fn test_plugin_load_from_config() {
    let mut registry = PluginRegistry::new();
    let plugins_config = vec![
        PluginConfig::simple("gatsby-plugin-sharp".to_string()),
        PluginConfig::simple("gatsby-transformer-remark".to_string()),
    ];

    // 测试从配置加载插件
    let result = registry.load_from_config(&plugins_config);
    assert!(result.is_ok());
    assert_eq!(registry.plugin_count(), 2);
}