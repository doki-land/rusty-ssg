//! VuTeX 错误定义

use nargo_types::{Error as NargoError, Span};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// VuTeX 错误类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VutexError {
    /// 解析错误
    ParseError {
        /// 错误信息
        message: String,
        /// 错误发生位置
        span: Span,
    },

    /// 文档未找到
    DocumentNotFound {
        /// 文档路径
        path: String,
    },

    /// 配置错误
    ConfigError {
        /// 错误信息
        message: String,
    },

    /// IO 错误
    IoError {
        /// 错误信息
        message: String,
    },

    /// 外部错误（来自 nargo-types）
    External {
        /// 错误信息
        message: String,
        /// 错误发生位置
        span: Span,
    },
}

impl Display for VutexError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            VutexError::ParseError { message, span } => {
                write!(f, "Parse error at {:?}: {}", span, message)
            }
            VutexError::DocumentNotFound { path } => {
                write!(f, "Document not found: {}", path)
            }
            VutexError::ConfigError { message } => {
                write!(f, "Config error: {}", message)
            }
            VutexError::IoError { message } => {
                write!(f, "IO error: {}", message)
            }
            VutexError::External { message, .. } => {
                write!(f, "{}", message)
            }
        }
    }
}

impl Error for VutexError {}

impl From<NargoError> for VutexError {
    fn from(e: NargoError) -> Self {
        VutexError::External { message: e.to_string(), span: e.span() }
    }
}

impl From<std::io::Error> for VutexError {
    fn from(e: std::io::Error) -> Self {
        VutexError::IoError { message: e.to_string() }
    }
}

impl From<serde_json::Error> for VutexError {
    fn from(e: serde_json::Error) -> Self {
        VutexError::ConfigError { message: e.to_string() }
    }
}

impl From<crate::config::ConfigError> for VutexError {
    fn from(e: crate::config::ConfigError) -> Self {
        VutexError::ConfigError { message: e.to_string() }
    }
}

impl From<walkdir::Error> for VutexError {
    fn from(e: walkdir::Error) -> Self {
        VutexError::IoError { message: e.to_string() }
    }
}

#[cfg(feature = "dev")]
impl From<notify::Error> for VutexError {
    fn from(e: notify::Error) -> Self {
        VutexError::IoError { message: e.to_string() }
    }
}

impl VutexError {
    /// 获取错误的 i18n key
    pub fn i18n_key(&self) -> &'static str {
        match self {
            VutexError::ParseError { .. } => "vuepress.error.parse",
            VutexError::DocumentNotFound { .. } => "vuepress.error.document_not_found",
            VutexError::ConfigError { .. } => "vuepress.error.config",
            VutexError::IoError { .. } => "vuepress.error.io",
            VutexError::External { .. } => "vuepress.error.external",
        }
    }

    /// 创建解析错误
    pub fn parse_error(message: String, span: Span) -> Self {
        VutexError::ParseError { message, span }
    }

    /// 创建文档未找到错误
    pub fn document_not_found(path: String) -> Self {
        VutexError::DocumentNotFound { path }
    }

    /// 创建配置错误
    pub fn config_error(message: String) -> Self {
        VutexError::ConfigError { message }
    }

    /// 创建 IO 错误
    pub fn io_error(message: String) -> Self {
        VutexError::IoError { message }
    }

    /// 获取错误发生的位置
    pub fn span(&self) -> Span {
        match self {
            VutexError::ParseError { span, .. } => *span,
            VutexError::External { span, .. } => *span,
            _ => Span::unknown(),
        }
    }
}

impl From<VutexError> for NargoError {
    fn from(err: VutexError) -> Self {
        match err {
            VutexError::External { message, span } => NargoError::external_error("vutex".to_string(), message, span),
            _ => NargoError::external_error("vutex".to_string(), err.to_string(), err.span()),
        }
    }
}

/// Result 类型别名
pub type Result<T> = std::result::Result<T, VutexError>;
