//! 类型定义模块

use std::{error::Error, fmt};

/// 编译结果
pub struct CompileResult {
    /// 成功状态
    pub success: bool,
    /// 编译时间（毫秒）
    pub compile_time_ms: u64,
}

impl CompileResult {
    /// 创建成功的编译结果
    pub fn success(compile_time_ms: u64) -> Self {
        Self { success: true, compile_time_ms }
    }

    /// 创建失败的编译结果
    pub fn failure(compile_time_ms: u64) -> Self {
        Self { success: false, compile_time_ms }
    }
}

/// MkDocs 编译器错误
#[derive(Debug)]
pub enum MkDocsError {
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
}

impl MkDocsError {
    /// 获取错误的 i18n 键
    pub fn i18n_key(&self) -> &'static str {
        match self {
            MkDocsError::ConfigError { .. } => "mkdocs.error.config",
            MkDocsError::CompileError { .. } => "mkdocs.error.compile",
            MkDocsError::IoError { .. } => "mkdocs.error.io",
            MkDocsError::SerializeError { .. } => "mkdocs.error.serialize",
        }
    }
}

impl fmt::Display for MkDocsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MkDocsError::ConfigError { message } => write!(f, "Config error: {}", message),
            MkDocsError::CompileError { message } => write!(f, "Compile error: {}", message),
            MkDocsError::IoError { source } => write!(f, "IO error: {}", source),
            MkDocsError::SerializeError { source } => write!(f, "Serialize error: {}", source),
        }
    }
}

impl Error for MkDocsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MkDocsError::IoError { source } => Some(source),
            MkDocsError::SerializeError { source } => Some(source),
            _ => None,
        }
    }
}

impl From<std::io::Error> for MkDocsError {
    fn from(source: std::io::Error) -> Self {
        MkDocsError::IoError { source }
    }
}

impl From<serde_json::Error> for MkDocsError {
    fn from(source: serde_json::Error) -> Self {
        MkDocsError::SerializeError { source }
    }
}

/// 结果类型
pub type Result<T> = std::result::Result<T, MkDocsError>;
