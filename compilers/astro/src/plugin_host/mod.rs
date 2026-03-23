//! 插件宿主模块

use crate::plugin::{Plugin, PluginConfig, PluginManager};
use std::sync::Arc;

/// 插件宿主，负责管理和执行插件
pub struct PluginHost {
    /// 插件管理器
    manager: PluginManager,
    /// 配置名称
    config_name: String,
}

impl PluginHost {
    /// 创建新的插件宿主
    pub fn new(config_name: &str) -> Self {
        Self {
            manager: PluginManager::new(),
            config_name: config_name.to_string(),
        }
    }

    /// 注册插件
    pub fn register_plugin<P: Plugin + 'static>(&mut self, plugin: P) -> Result<(), Box<dyn std::error::Error>> {
        self.manager.register(Arc::new(plugin))?;
        Ok(())
    }

    /// 初始化所有插件
    pub fn init_plugins(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.manager.init_all()?;
        Ok(())
    }

    /// 执行所有插件
    pub fn execute_plugins(&self, content: &str) -> Result<String, Box<dyn std::error::Error>> {
        self.manager.execute_all(content)
    }

    /// 获取插件管理器
    pub fn manager(&self) -> &PluginManager {
        &self.manager
    }

    /// 获取配置名称
    pub fn config_name(&self) -> &str {
        &self.config_name
    }
}
