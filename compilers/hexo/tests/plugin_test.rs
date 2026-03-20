//! 插件系统测试

use hexo::plugin::{HookContext, HookManager, HookType, PluginManager};
use std::path::Path;

#[test]
fn test_plugin_manager() {
    let mut plugin_manager = PluginManager::new();

    // 添加插件搜索路径
    let test_plugin_path = Path::new("../../examples/hexo-mvp/plugins");
    plugin_manager.add_search_path(test_plugin_path);

    // 加载插件
    let result = plugin_manager.load_plugins();
    assert!(result.is_ok());

    // 获取已加载的插件
    let plugins = plugin_manager.get_plugins();
    assert!(!plugins.is_empty());
}

#[tokio::test]
async fn test_hook_manager() {
    let hook_manager = HookManager::new();

    // 注册钩子
    let callback = std::sync::Arc::new(|context: &mut HookContext| {
        context.add_data("test", serde_json::Value::String("test value".to_string()));
        Ok(())
    });

    hook_manager.register_hook(HookType::Init, callback.clone()).await;

    // 触发钩子
    let mut context = HookContext::new();
    let result = hook_manager.trigger_hook(HookType::Init, &mut context).await;
    assert!(result.is_ok());

    // 验证钩子执行结果
    assert_eq!(context.get_data("test").unwrap(), &serde_json::Value::String("test value".to_string()));

    // 获取钩子数量
    let count = hook_manager.get_hook_count(&HookType::Init).await;
    assert_eq!(count, 1);

    // 移除钩子
    hook_manager.remove_hook(HookType::Init, callback).await;

    // 验证钩子已移除
    let count = hook_manager.get_hook_count(&HookType::Init).await;
    assert_eq!(count, 0);
}
