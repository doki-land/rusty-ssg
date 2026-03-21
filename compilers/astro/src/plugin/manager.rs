//! 插件管理器

use super::{Plugin, PluginError};
use std::{collections::HashMap, sync::Arc};

/// 插件管理器
///
/// 负责插件的注册、加载和执行
#[derive(Default)]
pub struct PluginManager {
    /// 已注册的插件
    plugins: HashMap<String, Arc<dyn Plugin + Send + Sync>>,
}

impl PluginManager {
    /// 创建新的插件管理器
    pub fn new() -> Self {
        Self::default()
    }

    /// 注册插件
    ///
    /// # 参数
    /// - `plugin`: 插件实例
    ///
    /// # 返回值
    /// - `Result<(), PluginError>`: 注册结果
    pub fn register(&mut self, plugin: Arc<dyn Plugin + Send + Sync>) -> Result<(), PluginError> {
        let name = plugin.name().to_string();
        if self.plugins.contains_key(&name) {
            return Err(PluginError::LoadError(format!("Plugin with name '{}' already registered", name)));
        }
        self.plugins.insert(name, plugin);
        Ok(())
    }

    /// 加载插件
    ///
    /// # 参数
    /// - `name`: 插件名称
    ///
    /// # 返回值
    /// - `Option<&Arc<dyn Plugin + Send + Sync>>`: 加载的插件
    pub fn get(&self, name: &str) -> Option<&Arc<dyn Plugin + Send + Sync>> {
        self.plugins.get(name)
    }

    /// 获取所有插件
    ///
    /// # 返回值
    /// - `&HashMap<String, Arc<dyn Plugin + Send + Sync>>`: 所有已注册的插件
    pub fn all(&self) -> &HashMap<String, Arc<dyn Plugin + Send + Sync>> {
        &self.plugins
    }

    /// 初始化所有插件
    ///
    /// # 返回值
    /// - `Result<(), PluginError>`: 初始化结果
    pub fn init_all(&self) -> Result<(), PluginError> {
        for plugin in self.plugins.values() {
            plugin.init()?;
        }
        Ok(())
    }

    /// 执行所有插件
    ///
    /// # 参数
    /// - `content`: 要处理的内容
    ///
    /// # 返回值
    /// - `Result<String, PluginError>`: 处理后的内容
    pub fn execute_all(&self, content: &str) -> Result<String, PluginError> {
        let mut result = content.to_string();
        for plugin in self.plugins.values() {
            result = plugin.execute(&result)?;
        }
        Ok(result)
    }
}
