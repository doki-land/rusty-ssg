//! 插件管理器

use super::{Plugin, PluginConfig, PluginContext, PluginError, PluginFactory, PluginLifecycleEvent};
use crate::config::AstroConfig;
use std::{collections::HashMap, path::Path, sync::Arc};

/// 插件管理器
///
/// 负责插件的注册、加载和执行
#[derive(Default)]
pub struct PluginManager {
    /// 已注册的插件
    plugins: HashMap<String, Arc<dyn Plugin + Send + Sync>>,
    /// 插件上下文
    context: PluginContext,
    /// 插件工厂
    factories: HashMap<String, Box<dyn PluginFactory>>,
}

impl PluginManager {
    /// 创建新的插件管理器
    pub fn new() -> Self {
        Self::default()
    }

    /// 从配置文件加载插件
    ///
    /// # 参数
    /// - `config`: 项目配置
    /// - `project_path`: 项目路径
    ///
    /// # 返回值
    /// - `Result<(), PluginError>`: 加载结果
    pub fn load_from_config(&mut self, config: &AstroConfig, project_path: &Path) -> Result<(), PluginError> {
        // 从 integrations 配置中加载插件
        for integration in &config.integrations {
            if let Some(name) = integration.get("name").and_then(|v| v.as_str()) {
                let options = integration.get("options").unwrap_or(&serde_json::Value::Null).clone();
                let plugin_config = PluginConfig { name: name.to_string(), options };
                self.load_plugin(&plugin_config, project_path)?;
            }
        }
        Ok(())
    }

    /// 加载插件
    ///
    /// # 参数
    /// - `config`: 插件配置
    /// - `project_path`: 项目路径
    ///
    /// # 返回值
    /// - `Result<(), PluginError>`: 加载结果
    pub fn load_plugin(&mut self, config: &PluginConfig, project_path: &Path) -> Result<(), PluginError> {
        // 尝试从本地加载插件
        let plugin = self.create_plugin(config)?;
        let plugin_arc = Arc::new(plugin);
        self.register(plugin_arc)
    }

    /// 创建插件实例
    ///
    /// # 参数
    /// - `config`: 插件配置
    ///
    /// # 返回值
    /// - `Result<Box<dyn Plugin>, PluginError>`: 插件实例
    fn create_plugin(&self, config: &PluginConfig) -> Result<Box<dyn Plugin>, PluginError> {
        // 这里简化处理，实际应该从插件目录或npm包加载
        // 现在返回默认插件实现
        Ok(Box::new(super::DefaultPlugin::new(&config.name, Some(config.clone()))))
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

    /// 注册插件工厂
    ///
    /// # 参数
    /// - `factory`: 插件工厂
    ///
    /// # 返回值
    /// - `Result<(), PluginError>`: 注册结果
    pub fn register_factory(&mut self, factory: Box<dyn PluginFactory>) -> Result<(), PluginError> {
        let name = factory.name().to_string();
        if self.factories.contains_key(&name) {
            return Err(PluginError::LoadError(format!("Plugin factory with name '{}' already registered", name)));
        }
        self.factories.insert(name, factory);
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
            plugin.init(&self.context)?;
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
            result = plugin.execute(&result, &self.context)?;
        }
        Ok(result)
    }

    /// 触发生命周期事件
    ///
    /// # 参数
    /// - `event`: 生命周期事件
    ///
    /// # 返回值
    /// - `Result<(), PluginError>`: 执行结果
    pub fn trigger_event(&mut self, event: &PluginLifecycleEvent) -> Result<(), PluginError> {
        for plugin in self.plugins.values() {
            plugin.on_event(event, &mut self.context)?;
        }
        Ok(())
    }

    /// 设置插件上下文
    ///
    /// # 参数
    /// - `context`: 插件上下文
    pub fn set_context(&mut self, context: PluginContext) {
        self.context = context;
    }

    /// 获取插件上下文
    ///
    /// # 返回值
    /// - `&PluginContext`: 插件上下文
    pub fn context(&self) -> &PluginContext {
        &self.context
    }

    /// 获取可变插件上下文
    ///
    /// # 返回值
    /// - `&mut PluginContext`: 可变插件上下文
    pub fn context_mut(&mut self) -> &mut PluginContext {
        &mut self.context
    }

    /// 清理插件
    pub fn cleanup(&mut self) {
        self.plugins.clear();
        self.factories.clear();
    }
}
