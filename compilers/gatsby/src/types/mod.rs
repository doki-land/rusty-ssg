//! Gatsby 类型模块
//! 定义 Gatsby 编译相关的类型定义

/// GraphQL 数据类型
pub mod graphql;

use std::{collections::HashMap, error::Error, fmt};

use nargo_types::Document;
use serde::{Deserialize, Serialize};

/// 编译结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileResult {
    /// 编译后的文档
    pub documents: HashMap<String, Document>,
    /// 编译时间（毫秒）
    pub compile_time_ms: u64,
    /// 是否成功
    pub success: bool,
    /// 错误信息（字符串形式）
    pub errors: Vec<String>,
}

impl CompileResult {
    /// 创建成功的编译结果
    ///
    /// # Arguments
    ///
    /// * `documents` - 编译后的文档映射
    /// * `compile_time_ms` - 编译时间（毫秒）
    pub fn success(documents: HashMap<String, Document>, compile_time_ms: u64) -> Self {
        Self { documents, compile_time_ms, success: true, errors: Vec::new() }
    }

    /// 创建失败的编译结果
    ///
    /// # Arguments
    ///
    /// * `errors` - 错误信息列表
    /// * `compile_time_ms` - 编译时间（毫秒）
    pub fn failure(errors: Vec<String>, compile_time_ms: u64) -> Self {
        Self { documents: HashMap::new(), compile_time_ms, success: false, errors }
    }

    /// 从 GatsbyError 创建失败的编译结果
    ///
    /// # Arguments
    ///
    /// * `errors` - GatsbyError 列表
    /// * `compile_time_ms` - 编译时间（毫秒）
    pub fn from_errors(errors: Vec<GatsbyError>, compile_time_ms: u64) -> Self {
        let error_strings = errors.iter().map(|e| format!("{}", e)).collect();
        Self::failure(error_strings, compile_time_ms)
    }

    /// 序列化为 JSON
    ///
    /// # Errors
    ///
    /// 返回 `serde_json::Error` 如果序列化失败
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }

    /// 序列化为美化的 JSON
    ///
    /// # Errors
    ///
    /// 返回 `serde_json::Error` 如果序列化失败
    pub fn to_json_pretty(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}

/// Gatsby 编译器错误
#[derive(Debug)]
pub enum GatsbyError {
    /// 配置错误
    ConfigError {
        /// 错误消息
        message: String,
    },

    /// 编译错误
    CompileError {
        /// 错误消息
        message: String,
    },

    /// 文件操作错误
    IoError {
        /// 底层 IO 错误
        source: std::io::Error,
    },

    /// 序列化错误
    SerializeError {
        /// 底层序列化错误
        source: serde_json::Error,
    },

    /// 插件错误
    PluginError {
        /// 错误消息
        message: String,
    },

    /// GraphQL 错误
    GraphQLError {
        /// 错误消息
        message: String,
    },

    /// 数据源错误
    DataSourceError {
        /// 错误消息
        message: String,
    },

    /// 渲染错误
    RenderError {
        /// 错误消息
        message: String,
    },
}

impl GatsbyError {
    /// 获取错误的 i18n 键
    pub fn i18n_key(&self) -> &'static str {
        match self {
            GatsbyError::ConfigError { .. } => "gatsby.error.config",
            GatsbyError::CompileError { .. } => "gatsby.error.compile",
            GatsbyError::IoError { .. } => "gatsby.error.io",
            GatsbyError::SerializeError { .. } => "gatsby.error.serialize",
            GatsbyError::PluginError { .. } => "gatsby.error.plugin",
            GatsbyError::GraphQLError { .. } => "gatsby.error.graphql",
            GatsbyError::DataSourceError { .. } => "gatsby.error.data_source",
            GatsbyError::RenderError { .. } => "gatsby.error.render",
        }
    }

    /// 创建配置错误
    pub fn config(message: String) -> Self {
        GatsbyError::ConfigError { message }
    }

    /// 创建编译错误
    pub fn compile(message: String) -> Self {
        GatsbyError::CompileError { message }
    }

    /// 创建插件错误
    pub fn plugin(message: String) -> Self {
        GatsbyError::PluginError { message }
    }

    /// 创建 GraphQL 错误
    pub fn graphql(message: String) -> Self {
        GatsbyError::GraphQLError { message }
    }

    /// 创建数据源错误
    pub fn data_source(message: String) -> Self {
        GatsbyError::DataSourceError { message }
    }

    /// 创建渲染错误
    pub fn render(message: String) -> Self {
        GatsbyError::RenderError { message }
    }
}

impl fmt::Display for GatsbyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GatsbyError::ConfigError { message } => write!(f, "Config error: {}", message),
            GatsbyError::CompileError { message } => write!(f, "Compile error: {}", message),
            GatsbyError::IoError { source } => write!(f, "IO error: {}", source),
            GatsbyError::SerializeError { source } => write!(f, "Serialize error: {}", source),
            GatsbyError::PluginError { message } => write!(f, "Plugin error: {}", message),
            GatsbyError::GraphQLError { message } => write!(f, "GraphQL error: {}", message),
            GatsbyError::DataSourceError { message } => write!(f, "Data source error: {}", message),
            GatsbyError::RenderError { message } => write!(f, "Render error: {}", message),
        }
    }
}

impl Error for GatsbyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            GatsbyError::IoError { source } => Some(source),
            GatsbyError::SerializeError { source } => Some(source),
            _ => None,
        }
    }
}

impl From<std::io::Error> for GatsbyError {
    fn from(source: std::io::Error) -> Self {
        GatsbyError::IoError { source }
    }
}

impl From<serde_json::Error> for GatsbyError {
    fn from(source: serde_json::Error) -> Self {
        GatsbyError::SerializeError { source }
    }
}

impl From<crate::config::ConfigError> for GatsbyError {
    fn from(error: crate::config::ConfigError) -> Self {
        GatsbyError::ConfigError { message: error.to_string() }
    }
}

impl From<nargo_types::Error> for GatsbyError {
    fn from(error: nargo_types::Error) -> Self {
        GatsbyError::CompileError { message: error.to_string() }
    }
}

/// 结果类型
pub type Result<T> = std::result::Result<T, GatsbyError>;

/// 页面数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageData {
    /// 页面路径
    pub path: String,
    /// 页面组件
    pub component: String,
    /// 页面上下文
    pub context: HashMap<String, serde_json::Value>,
    /// 页面查询
    pub query: Option<String>,
    /// 页面匹配
    pub match_path: Option<String>,
}

impl PageData {
    /// 创建新的页面数据
    pub fn new(path: String, component: String) -> Self {
        Self { path, component, context: HashMap::new(), query: None, match_path: None }
    }

    /// 设置页面上下文
    pub fn with_context(mut self, context: HashMap<String, serde_json::Value>) -> Self {
        self.context = context;
        self
    }

    /// 设置页面查询
    pub fn with_query(mut self, query: String) -> Self {
        self.query = Some(query);
        self
    }

    /// 设置页面匹配路径
    pub fn with_match_path(mut self, match_path: String) -> Self {
        self.match_path = Some(match_path);
        self
    }
}

/// 构建状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BuildState {
    /// 初始化中
    Initializing,
    /// 加载配置
    LoadingConfig,
    /// 运行插件
    RunningPlugins,
    /// 提取查询
    ExtractingQueries,
    /// 执行查询
    ExecutingQueries,
    /// 生成页面
    GeneratingPages,
    /// 写入文件
    WritingFiles,
    /// 完成
    Completed,
    /// 失败
    Failed,
}

/// 构建统计
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BuildStats {
    /// 处理的文件数量
    pub files_processed: usize,
    /// 生成的页面数量
    pub pages_generated: usize,
    /// 执行的查询数量
    pub queries_executed: usize,
    /// 构建时间（毫秒）
    pub build_time_ms: u64,
    /// 错误数量
    pub error_count: usize,
    /// 警告数量
    pub warning_count: usize,
}

impl BuildStats {
    /// 创建新的构建统计
    pub fn new() -> Self {
        Self::default()
    }
}
