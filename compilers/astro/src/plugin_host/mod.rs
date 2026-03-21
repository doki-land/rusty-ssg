//! 插件宿主模块

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde_json::Value;
use crate::plugin::{Plugin, PluginError, PluginConfig, PluginManager};

/// 插件宿主错误类型
#[derive(Debug)]
pub enum PluginHostError {
    /// 宿主启动错误
    StartError(String),
    /// 宿主通信错误
    CommunicationError(String),
    /// 插件加载错误
    PluginLoadError(String),
}

/// 插件宿主
pub struct PluginHost {
    /// 宿主配置
    config: String,
    /// 插件管理器
    plugin_manager: PluginManager,
    /// 插件配置映射
    plugin_configs: HashMap<String, PluginConfig>,
}

impl PluginHost {
    /// 创建新的插件宿主
    pub fn new(config: &str) -> Self {
        Self {
            config: config.to_string(),
            plugin_manager: PluginManager::new(),
            plugin_configs: HashMap::new(),
        }
    }

    /// 加载插件配置
    /// 
    /// # 参数
    /// - `config_path`: 插件配置文件路径
    /// 
    /// # 返回值
    /// - `Result<(), PluginHostError>`: 加载结果
    pub fn load_config(&mut self, config_path: &Path) -> Result<(), PluginHostError> {
        let config_content = fs::read_to_string(config_path)
            .map_err(|e| PluginHostError::StartError(format!("Failed to read config file: {}", e)))?;
        
        let config: Value = serde_json::from_str(&config_content)
            .map_err(|e| PluginHostError::StartError(format!("Failed to parse config file: {}", e)))?;
        
        if let Some(plugins) = config.get("plugins").and_then(|v| v.as_array()) {
            for plugin_config in plugins {
                if let Some(name) = plugin_config.get("name").and_then(|v| v.as_str()) {
                    let options = plugin_config.get("options").unwrap_or(&Value::Object(Default::default())).clone();
                    let config = PluginConfig {
                        name: name.to_string(),
                        options,
                    };
                    self.plugin_configs.insert(name.to_string(), config);
                }
            }
        }
        
        Ok(())
    }

    /// 注册插件
    /// 
    /// # 参数
    /// - `plugin`: 插件实例
    /// 
    /// # 返回值
    /// - `Result<(), PluginError>`: 注册结果
    pub fn register_plugin(&mut self, plugin: impl Plugin + Send + Sync + 'static) -> Result<(), PluginError> {
        use std::sync::Arc;
        let plugin_arc = Arc::new(plugin);
        self.plugin_manager.register(plugin_arc)
    }

    /// 初始化所有插件
    /// 
    /// # 返回值
    /// - `Result<(), PluginError>`: 初始化结果
    pub fn init_plugins(&mut self) -> Result<(), PluginError> {
        self.plugin_manager.init_all()
    }

    /// 执行所有插件
    /// 
    /// # 参数
    /// - `content`: 要处理的内容
    /// 
    /// # 返回值
    /// - `Result<String, PluginError>`: 处理后的内容
    pub fn execute_plugins(&self, content: &str) -> Result<String, PluginError> {
        self.plugin_manager.execute_all(content)
    }

    /// 获取插件管理器
    /// 
    /// # 返回值
    /// - `&PluginManager`: 插件管理器
    pub fn plugin_manager(&self) -> &PluginManager {
        &self.plugin_manager
    }

    /// 获取插件配置
    /// 
    /// # 参数
    /// - `plugin_name`: 插件名称
    /// 
    /// # 返回值
    /// - `Option<&PluginConfig>`: 插件配置
    pub fn plugin_config(&self, plugin_name: &str) -> Option<&PluginConfig> {
        self.plugin_configs.get(plugin_name)
    }
}

