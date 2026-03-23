//! VitePress 错误定义

use nargo_types::{Error as NargoError, Span};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// VitePress 错误类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VitePressError {
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

impl Display for VitePressError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            VitePressError::ParseError { message, span } => {
                write!(f, "Parse error at {:?}: {}", span, message)
            }
            VitePressError::DocumentNotFound { path } => {
                write!(f, "Document not found: {}", path)
            }
            VitePressError::ConfigError { message } => {
                write!(f, "Config error: {}", message)
            }
            VitePressError::IoError { message } => {
                write!(f, "IO error: {}", message)
            }
            VitePressError::External { message, .. } => {
                write!(f, "{}", message)
            }
        }
    }
}

impl Error for VitePressError {}

impl From<NargoError> for VitePressError {
    fn from(e: NargoError) -> Self {
        VitePressError::External { message: e.to_string(), span: e.span() }
    }
}

impl From<std::io::Error> for VitePressError {
    fn from(e: std::io::Error) -> Self {
        VitePressError::IoError { message: e.to_string() }
    }
}

impl From<serde_json::Error> for VitePressError {
    fn from(e: serde_json::Error) -> Self {
        VitePressError::ConfigError { message: e.to_string() }
    }
}

impl From<crate::config::ConfigError> for VitePressError {
    fn from(e: crate::config::ConfigError) -> Self {
        VitePressError::ConfigError { message: e.to_string() }
    }
}

impl From<walkdir::Error> for VitePressError {
    fn from(e: walkdir::Error) -> Self {
        VitePressError::IoError { message: e.to_string() }
    }
}

#[cfg(feature = "dev")]
impl From<notify::Error> for VitePressError {
    fn from(e: notify::Error) -> Self {
        VitePressError::IoError { message: e.to_string() }
    }
}

impl VitePressError {
    /// 获取错误的 i18n key
    pub fn i18n_key(&self) -> &'static str {
        match self {
            VitePressError::ParseError { .. } => "vitepress.error.parse",
            VitePressError::DocumentNotFound { .. } => "vitepress.error.document_not_found",
            VitePressError::ConfigError { .. } => "vitepress.error.config",
            VitePressError::IoError { .. } => "vitepress.error.io",
            VitePressError::External { .. } => "vitepress.error.external",
        }
    }

    /// 创建解析错误
    pub fn parse_error(message: String, span: Span) -> Self {
        VitePressError::ParseError { message, span }
    }

    /// 创建文档未找到错误
    pub fn document_not_found(path: String) -> Self {
        VitePressError::DocumentNotFound { path }
    }

    /// 创建配置错误
    pub fn config_error(message: String) -> Self {
        VitePressError::ConfigError { message }
    }

    /// 创建 IO 错误
    pub fn io_error(message: String) -> Self {
        VitePressError::IoError { message }
    }

    /// 获取错误发生的位置
    pub fn span(&self) -> Span {
        match self {
            VitePressError::ParseError { span, .. } => *span,
            VitePressError::External { span, .. } => *span,
            _ => Span::unknown(),
        }
    }
}

impl From<VitePressError> for NargoError {
    fn from(err: VitePressError) -> Self {
        match err {
            VitePressError::External { message, span } => NargoError::external_error("vitepress".to_string(), message, span),
            _ => NargoError::external_error("vitepress".to_string(), err.to_string(), err.span()),
        }
    }
}

/// Result 类型别名
pub type Result<T> = std::result::Result<T, VitePressError>;
