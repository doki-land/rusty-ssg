//! 插件宿主模块

/// 插件宿主错误类型
#[derive(Debug)]
pub enum PluginHostError {
    /// 宿主启动错误
    StartError(String),
    /// 宿主通信错误
    CommunicationError(String),
}

/// 插件宿主
pub struct PluginHost {
    /// 宿主配置
    config: String,
}

impl PluginHost {
    /// 创建新的插件宿主
    pub fn new(config: &str) -> Self {
        Self { config: config.to_string() }
    }
}
