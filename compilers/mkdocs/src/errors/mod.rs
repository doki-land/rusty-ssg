//! 错误处理模块
//! 
//! 定义错误类型和错误处理机制

use std::fmt;

/// MkDocs 错误类型
#[derive(Debug)]
pub enum MkDocsError {
    /// 配置解析错误
    ConfigParseError { message: String },
    /// 配置验证错误
    ConfigValidationError { message: String },
    /// 路径错误
    PathError { message: String },
    /// IO 错误
    IoError(std::io::Error),
    /// JSON 解析错误
    JsonError(serde_json::Error),
    /// YAML 解析错误
    YamlError(oak_yaml::language::Error),
    /// 渲染错误
    RenderError { message: String },
    /// 插件错误
    PluginError { message: String },
    /// 其他错误
    Other { message: String },
}

impl fmt::Display for MkDocsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MkDocsError::ConfigParseError { message } => write!(f, "Config parse error: {}", message),
            MkDocsError::ConfigValidationError { message } => write!(f, "Config validation error: {}", message),
            MkDocsError::PathError { message } => write!(f, "Path error: {}", message),
            MkDocsError::IoError(err) => write!(f, "IO error: {}", err),
            MkDocsError::JsonError(err) => write!(f, "JSON error: {}", err),
            MkDocsError::YamlError(err) => write!(f, "YAML error: {}", err),
            MkDocsError::RenderError { message } => write!(f, "Render error: {}", message),
            MkDocsError::PluginError { message } => write!(f, "Plugin error: {}", message),
            MkDocsError::Other { message } => write!(f, "Error: {}", message),
        }
    }
}

impl std::error::Error for MkDocsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MkDocsError::IoError(err) => Some(err),
            MkDocsError::JsonError(err) => Some(err),
            MkDocsError::YamlError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for MkDocsError {
    fn from(err: std::io::Error) -> Self {
        MkDocsError::IoError(err)
    }
}

impl From<serde_json::Error> for MkDocsError {
    fn from(err: serde_json::Error) -> Self {
        MkDocsError::JsonError(err)
    }
}

impl From<oak_yaml::language::Error> for MkDocsError {
    fn from(err: oak_yaml::language::Error) -> Self {
        MkDocsError::YamlError(err)
    }
}

/// 结果类型
pub type Result<T> = std::result::Result<T, MkDocsError>;
