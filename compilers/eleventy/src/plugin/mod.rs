//! 插件模块

use std::collections::HashMap;

/// 插件错误类型
#[derive(Debug)]
pub enum PluginError {
    /// 插件加载错误
    LoadError(String),
    /// 插件执行错误
    ExecuteError(String),
    /// 插件初始化错误
    InitError(String),
}

/// 插件上下文
pub struct PluginContext {
    /// 全局数据
    pub global_data: serde_json::Value,
    
    /// 配置选项
    pub options: serde_json::Value,
    
    /// 插件注册表
    pub plugins: HashMap<String, Box<dyn Plugin>>,
}

/// 插件 trait
pub trait Plugin: Send + Sync {
    /// 插件名称
    fn name(&self) -> &str;

    /// 初始化插件
    fn init(&mut self, context: &mut PluginContext) -> Result<(), PluginError>;

    /// 执行插件
    fn execute(&self, content: &str, context: &PluginContext) -> Result<String, PluginError>;
}

/// 插件注册表
pub struct PluginRegistry {
    /// 插件映射
    plugins: HashMap<String, Box<dyn Plugin>>,
}

impl PluginRegistry {
    /// 创建新的插件注册表
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }
    
    /// 注册插件
    pub fn register<P: Plugin + 'static>(&mut self, plugin: P) {
        self.plugins.insert(plugin.name().to_string(), Box::new(plugin));
    }
    
    /// 获取插件
    pub fn get(&self, name: &str) -> Option<&Box<dyn Plugin>> {
        self.plugins.get(name)
    }
    
    /// 获取可变插件
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Box<dyn Plugin>> {
        self.plugins.get_mut(name)
    }
    
    /// 初始化所有插件
    pub fn init_all(&mut self, context: &mut PluginContext) -> Result<(), PluginError> {
        for plugin in self.plugins.values_mut() {
            plugin.init(context)?;
        }
        Ok(())
    }
    
    /// 执行所有插件
    pub fn execute_all(&self, content: &str, context: &PluginContext) -> Result<String, PluginError> {
        let mut result = content.to_string();
        
        for plugin in self.plugins.values() {
            result = plugin.execute(&result, context)?;
        }
        
        Ok(result)
    }
}
