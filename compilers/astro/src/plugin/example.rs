//! 示例插件

use crate::plugin::{Plugin, PluginConfig, PluginError};

/// 示例插件
///
/// 用于演示插件系统的使用方式
pub struct ExamplePlugin {
    /// 插件配置
    config: PluginConfig,
}

impl ExamplePlugin {
    /// 创建新的示例插件
    pub fn new(config: PluginConfig) -> Self {
        Self { config }
    }
}

impl Plugin for ExamplePlugin {
    /// 插件名称
    fn name(&self) -> &str {
        "example-plugin"
    }

    /// 初始化插件
    fn init(&self) -> Result<(), PluginError> {
        println!("Example plugin initialized with config: {:?}", self.config);
        Ok(())
    }

    /// 执行插件
    fn execute(&self, content: &str) -> Result<String, PluginError> {
        // 简单的处理：在内容前后添加标记
        let result = format!("<!-- Example Plugin Start -->\n{}\n<!-- Example Plugin End -->", content);
        Ok(result)
    }

    /// 获取插件配置
    fn config(&self) -> Option<&PluginConfig> {
        Some(&self.config)
    }
}
