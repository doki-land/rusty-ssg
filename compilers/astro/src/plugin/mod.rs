//! 插件模块

pub mod example;
pub mod manager;
pub use example::ExamplePlugin;
pub use manager::PluginManager;

/// 插件错误类型
#[derive(Debug)]
pub enum PluginError {
    /// 插件加载错误
    LoadError(String),
    /// 插件执行错误
    ExecuteError(String),
    /// 插件生命周期错误
    LifecycleError(String),
}

impl std::fmt::Display for PluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginError::LoadError(msg) => write!(f, "LoadError: {}", msg),
            PluginError::ExecuteError(msg) => write!(f, "ExecuteError: {}", msg),
            PluginError::LifecycleError(msg) => write!(f, "LifecycleError: {}", msg),
        }
    }
}

/// 插件配置
#[derive(Debug, Default, Clone)]
pub struct PluginConfig {
    /// 插件名称
    pub name: String,
    /// 插件选项
    pub options: serde_json::Value,
}

/// 插件上下文
#[derive(Debug, Default)]
pub struct PluginContext {
    /// 项目配置
    pub config: serde_json::Value,
    /// 构建信息
    pub build_info: serde_json::Value,
    /// 插件间共享数据
    pub shared_data: serde_json::Value,
}

/// 插件生命周期事件
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PluginLifecycleEvent {
    /// 插件初始化
    Init,
    /// 构建开始
    BuildStart,
    /// 构建结束
    BuildEnd,
    /// 页面渲染前
    PageRenderStart,
    /// 页面渲染后
    PageRenderEnd,
    /// 服务器启动
    ServerStart,
    /// 服务器停止
    ServerStop,
}

/// 插件 trait
pub trait Plugin: Send + Sync {
    /// 插件名称
    fn name(&self) -> &str;

    /// 初始化插件
    fn init(&self, context: &PluginContext) -> Result<(), PluginError>;

    /// 执行插件
    fn execute(&self, content: &str, context: &PluginContext) -> Result<String, PluginError>;

    /// 处理生命周期事件
    fn on_event(&self, event: &PluginLifecycleEvent, context: &mut PluginContext) -> Result<(), PluginError>;

    /// 获取插件配置
    fn config(&self) -> Option<&PluginConfig> {
        None
    }

    /// 获取插件版本
    fn version(&self) -> &str {
        "1.0.0"
    }

    /// 检查插件是否支持特定功能
    fn supports(&self, feature: &str) -> bool {
        false
    }
}

/// 插件工厂 trait
pub trait PluginFactory: Send + Sync {
    /// 创建插件实例
    fn create(&self, config: &PluginConfig) -> Result<Box<dyn Plugin>, PluginError>;

    /// 获取插件名称
    fn name(&self) -> &str;

    /// 获取插件版本
    fn version(&self) -> &str;
}

/// 默认插件实现
#[derive(Debug)]
pub struct DefaultPlugin {
    name: String,
    config: Option<PluginConfig>,
}

impl DefaultPlugin {
    /// 创建新的默认插件
    pub fn new(name: &str, config: Option<PluginConfig>) -> Self {
        Self { name: name.to_string(), config }
    }
}

impl Plugin for DefaultPlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn init(&self, _context: &PluginContext) -> Result<(), PluginError> {
        Ok(())
    }

    fn execute(&self, content: &str, _context: &PluginContext) -> Result<String, PluginError> {
        Ok(content.to_string())
    }

    fn on_event(&self, _event: &PluginLifecycleEvent, _context: &mut PluginContext) -> Result<(), PluginError> {
        Ok(())
    }

    fn config(&self) -> Option<&PluginConfig> {
        self.config.as_ref()
    }
}

/// 插件注册宏
#[macro_export]
macro_rules! register_plugin {
    ($name:expr, $factory:ty) => {
        #[no_mangle]
        pub extern "C" fn plugin_factory() -> *mut dyn PluginFactory {
            Box::into_raw(Box::new(<$factory as Default>::default()))
        }
    };
}
