//! 插件 API

use serde_json::Value;

use crate::types::Result;

/// 插件 API
pub struct PluginApi {
    /// 站点配置
    pub config: Value,
    /// 插件配置
    pub plugin_config: Value,
}

impl PluginApi {
    /// 创建插件 API
    pub fn new(config: Value, plugin_config: Value) -> Self {
        Self { config, plugin_config }
    }

    /// 获取站点配置
    pub fn get_config(&self, key: &str) -> Option<&Value> {
        self.config.get(key)
    }

    /// 获取插件配置
    pub fn get_plugin_config(&self, key: &str) -> Option<&Value> {
        self.plugin_config.get(key)
    }

    /// 设置站点配置
    pub fn set_config(&mut self, key: &str, value: Value) {
        if let Value::Object(map) = &mut self.config {
            map.insert(key.to_string(), value);
        }
    }

    /// 设置插件配置
    pub fn set_plugin_config(&mut self, key: &str, value: Value) {
        if let Value::Object(map) = &mut self.plugin_config {
            map.insert(key.to_string(), value);
        }
    }

    /// 注册辅助函数
    pub fn register_helper(&self, name: &str, helper: HelperFunction) {
        // 这里需要实现辅助函数的注册
    }

    /// 注册过滤器
    pub fn register_filter(&self, name: &str, filter: FilterFunction) {
        // 这里需要实现过滤器的注册
    }

    /// 注册标签
    pub fn register_tag(&self, name: &str, tag: TagFunction) {
        // 这里需要实现标签的注册
    }

    /// 注册生成器
    pub fn register_generator(&self, name: &str, generator: GeneratorFunction) {
        // 这里需要实现生成器的注册
    }

    /// 注册部署器
    pub fn register_deployer(&self, name: &str, deployer: DeployerFunction) {
        // 这里需要实现部署器的注册
    }

    /// 注册命令
    pub fn register_command(&self, name: &str, command: CommandFunction) {
        // 这里需要实现命令的注册
    }
}

/// 辅助函数类型
pub type HelperFunction = Box<dyn Fn(&Value) -> Value + Send + Sync>;

/// 过滤器函数类型
pub type FilterFunction = Box<dyn Fn(&Value) -> Value + Send + Sync>;

/// 标签函数类型
pub type TagFunction = Box<dyn Fn(&str) -> Value + Send + Sync>;

/// 生成器函数类型
pub type GeneratorFunction = Box<dyn Fn() -> Vec<Value> + Send + Sync>;

/// 部署器函数类型
pub type DeployerFunction = Box<dyn Fn(&Value) -> Result<()> + Send + Sync>;

/// 命令函数类型
pub type CommandFunction = Box<dyn Fn(&[&str]) -> Result<()> + Send + Sync>;
