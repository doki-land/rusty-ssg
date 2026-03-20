//! 插件模块

/// 插件错误类型
#[derive(Debug)]
pub enum PluginError {
    /// 插件加载错误
    LoadError(String),
    /// 插件执行错误
    ExecuteError(String),
}

/// 插件 trait
pub trait Plugin {
    /// 插件名称
    fn name(&self) -> &str;

    /// 初始化插件
    fn init(&mut self) -> Result<(), PluginError>;

    /// 执行插件
    fn execute(&self, content: &str) -> Result<String, PluginError>;
}
