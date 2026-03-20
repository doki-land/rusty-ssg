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

/// Astro 编译器错误
#[derive(Debug)]
pub enum AstroError {
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

impl AstroError {
    /// 获取错误的 i18n 键
    pub fn i18n_key(&self) -> &'static str {
        match self {
            AstroError::ConfigError { .. } => "astro.error.config",
            AstroError::CompileError { .. } => "astro.error.compile",
            AstroError::IoError { .. } => "astro.error.io",
            AstroError::SerializeError { .. } => "astro.error.serialize",
        }
    }
}

impl fmt::Display for AstroError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AstroError::ConfigError { message } => write!(f, "Config error: {}", message),
            AstroError::CompileError { message } => write!(f, "Compile error: {}", message),
            AstroError::IoError { source } => write!(f, "IO error: {}", source),
            AstroError::SerializeError { source } => write!(f, "Serialize error: {}", source),
        }
    }
}

impl Error for AstroError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AstroError::IoError { source } => Some(source),
            AstroError::SerializeError { source } => Some(source),
            _ => None,
        }
    }
}

impl From<std::io::Error> for AstroError {
    fn from(source: std::io::Error) -> Self {
        AstroError::IoError { source }
    }
}

impl From<serde_json::Error> for AstroError {
    fn from(source: serde_json::Error) -> Self {
        AstroError::SerializeError { source }
    }
}

/// 结果类型
pub type Result<T> = std::result::Result<T, AstroError>;
