//! 插件系统测试
//! 测试 VitePress 插件系统的功能

use vitepress::{plugin::{PluginMeta, PluginContext, VitePressPlugin, PluginRegistry, PluginType}, types::VitePressError};
use std::collections::HashMap;
use nargo_types::NargoValue;

/// 测试插件
struct TestPlugin {
    meta: PluginMeta,
    setup_called: bool,
    before_compile_called: bool,
    before_parse_called: bool,
    before_render_called: bool,
    after_render_called: bool,
    after_compile_called: bool,
    before_dev_server_called: bool,
    after_dev_server_called: bool,
}

impl TestPlugin {
    fn new() -> Self {
        Self {
            meta: PluginMeta::new(
                "test-plugin".to_string(),
                "1.0.0".to_string(),
                "Test plugin".to_string(),
                PluginType::Other,
            ),
            setup_called: false,
            before_compile_called: false,
            before_parse_called: false,
            before_render_called: false,
            after_render_called: false,
            after_compile_called: false,
            before_dev_server_called: false,
            after_dev_server_called: false,
        }
    }
}

impl VitePressPlugin for TestPlugin {
    fn meta(&self) -> &PluginMeta {
        &self.meta
    }

    fn setup(&mut self, _config: Option<HashMap<String, NargoValue>>) {
        self.setup_called = true;
    }

    fn before_compile(&self) -> Result<(), VitePressError> {
        Ok(())
    }

    fn before_parse(&self, context: PluginContext) -> PluginContext {
        context
    }

    fn before_render(&self, context: PluginContext) -> PluginContext {
        context
    }

    fn after_render(&self, context: PluginContext) -> PluginContext {
        context
    }

    fn after_compile(&self) -> Result<(), VitePressError> {
        Ok(())
    }

    fn before_dev_server(&self, _config: Option<HashMap<String, NargoValue>>) -> Result<(), VitePressError> {
        Ok(())
    }

    fn after_dev_server(&self) -> Result<(), VitePressError> {
        Ok(())
    }
}

#[test]
fn test_plugin_registry() {
    // 创建插件注册表
    let mut registry = PluginRegistry::new();

    // 注册测试插件
    let test_plugin = TestPlugin::new();
    registry.register(test_plugin);

    // 检查插件注册成功
    assert_eq!(registry.plugin_count(), 1);
    assert!(registry.has_plugin("test-plugin"));

    // 初始化所有插件
    registry.setup_all(None);

    // 检查插件是否被初始化
    assert!(registry.has_plugin("test-plugin"));

    println!("插件注册表测试通过！");
}

#[test]
fn test_plugin_hooks() {
    // 创建插件注册表
    let mut registry = PluginRegistry::new();

    // 注册测试插件
    let test_plugin = TestPlugin::new();
    registry.register(test_plugin);

    // 初始化所有插件
    registry.setup_all(None);

    // 测试编译前钩子
    let result = registry.before_compile_all();
    assert!(result.is_ok());

    // 测试解析前钩子
    let context = PluginContext::from_content("# Test".to_string(), "test.md".to_string());
    let updated_context = registry.before_parse_all(context);
    assert_eq!(updated_context.content, "# Test");

    // 测试渲染前钩子
    let context = PluginContext::from_content("# Test".to_string(), "test.md".to_string());
    let updated_context = registry.before_render_all(context);
    assert_eq!(updated_context.content, "# Test");

    // 测试渲染后钩子
    let context = PluginContext::from_content("# Test".to_string(), "test.md".to_string());
    let updated_context = registry.after_render_all(context);
    assert_eq!(updated_context.content, "# Test");

    // 测试编译后钩子
    let result = registry.after_compile_all();
    assert!(result.is_ok());

    // 测试开发服务器启动前钩子
    let result = registry.before_dev_server_all(None);
    assert!(result.is_ok());

    // 测试开发服务器关闭后钩子
    let result = registry.after_dev_server_all();
    assert!(result.is_ok());

    println!("插件钩子测试通过！");
}

#[test]
fn test_plugin_by_type() {
    // 创建插件注册表
    let mut registry = PluginRegistry::new();

    // 注册不同类型的插件
    let test_plugin = TestPlugin::new();
    registry.register(test_plugin);

    // 检查按类型获取插件
    let plugins = registry.plugins_by_type(PluginType::Other);
    assert!(plugins.is_some());
    assert_eq!(plugins.unwrap().len(), 1);

    println!("插件按类型获取测试通过！");
}
