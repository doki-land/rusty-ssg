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
        /// 错误路径
        path: Option<String>,
    },

    /// 文档未找到
    DocumentNotFound {
        /// 文档路径
        path: String,
        /// 建议
        suggestion: Option<String>,
    },

    /// 配置错误
    ConfigError {
        /// 错误信息
        message: String,
        /// 错误路径
        path: Option<String>,
        /// 建议
        suggestion: Option<String>,
    },

    /// IO 错误
    IoError {
        /// 错误信息
        message: String,
        /// 错误路径
        path: Option<String>,
    },

    /// 插件错误
    PluginError {
        /// 错误信息
        message: String,
        /// 插件名称
        plugin: String,
        /// 错误路径
        path: Option<String>,
    },

    /// 构建错误
    BuildError {
        /// 错误信息
        message: String,
        /// 错误路径
        path: Option<String>,
    },

    /// 外部错误（来自 nargo-types）
    External {
        /// 错误信息
        message: String,
        /// 错误发生位置
        span: Span,
        /// 错误路径
        path: Option<String>,
    },
}

impl Display for VitePressError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            VitePressError::ParseError { message, span, path } => {
                if let Some(path) = path {
                    write!(f, "Parse error in {} at {:?}: {}", path, span, message)
                } else {
                    write!(f, "Parse error at {:?}: {}", span, message)
                }
            }
            VitePressError::DocumentNotFound { path, suggestion } => {
                if let Some(suggestion) = suggestion {
                    write!(f, "Document not found: {}. {}", path, suggestion)
                } else {
                    write!(f, "Document not found: {}", path)
                }
            }
            VitePressError::ConfigError { message, path, suggestion } => {
                let mut msg = if let Some(path) = path {
                    format!("Config error in {}: {}", path, message)
                } else {
                    format!("Config error: {}", message)
                };
                if let Some(suggestion) = suggestion {
                    msg.push_str(&format!(" {}", suggestion));
                }
                write!(f, "{}", msg)
            }
            VitePressError::IoError { message, path } => {
                if let Some(path) = path {
                    write!(f, "IO error in {}: {}", path, message)
                } else {
                    write!(f, "IO error: {}", message)
                }
            }
            VitePressError::PluginError { message, plugin, path } => {
                if let Some(path) = path {
                    write!(f, "Plugin error in {} ({plugin}): {}", path, message)
                } else {
                    write!(f, "Plugin error ({plugin}): {}", message)
                }
            }
            VitePressError::BuildError { message, path } => {
                if let Some(path) = path {
                    write!(f, "Build error in {}: {}", path, message)
                } else {
                    write!(f, "Build error: {}", message)
                }
            }
            VitePressError::External { message, span, path } => {
                if let Some(path) = path {
                    write!(f, "External error in {} at {:?}: {}", path, span, message)
                } else {
                    write!(f, "External error at {:?}: {}", span, message)
                }
            }
        }
    }
}

impl Error for VitePressError {}

impl From<NargoError> for VitePressError {
    fn from(e: NargoError) -> Self {
        VitePressError::External { message: e.to_string(), span: e.span(), path: None }
    }
}

impl From<std::io::Error> for VitePressError {
    fn from(e: std::io::Error) -> Self {
        VitePressError::IoError { message: e.to_string(), path: None }
    }
}

impl From<serde_json::Error> for VitePressError {
    fn from(e: serde_json::Error) -> Self {
        VitePressError::ConfigError { message: e.to_string(), path: None, suggestion: None }
    }
}

impl From<oak_toml::Error> for VitePressError {
    fn from(e: oak_toml::Error) -> Self {
        VitePressError::ConfigError { message: e.to_string(), path: None, suggestion: None }
    }
}

impl From<oak_yaml::Error> for VitePressError {
    fn from(e: oak_yaml::Error) -> Self {
        VitePressError::ConfigError { message: e.to_string(), path: None, suggestion: None }
    }
}

impl From<crate::config::ConfigError> for VitePressError {
    fn from(e: crate::config::ConfigError) -> Self {
        VitePressError::ConfigError { message: e.to_string(), path: None, suggestion: None }
    }
}

impl From<walkdir::Error> for VitePressError {
    fn from(e: walkdir::Error) -> Self {
        VitePressError::IoError { message: e.to_string(), path: None }
    }
}

#[cfg(feature = "dev")]
impl From<notify::Error> for VitePressError {
    fn from(e: notify::Error) -> Self {
        VitePressError::IoError { message: e.to_string(), path: None }
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
            VitePressError::PluginError { .. } => "vitepress.error.plugin",
            VitePressError::BuildError { .. } => "vitepress.error.build",
            VitePressError::External { .. } => "vitepress.error.external",
        }
    }

    /// 创建解析错误
    pub fn parse_error(message: String, span: Span, path: Option<String>) -> Self {
        VitePressError::ParseError { message, span, path }
    }

    /// 创建文档未找到错误
    pub fn document_not_found(path: String, suggestion: Option<String>) -> Self {
        VitePressError::DocumentNotFound { path, suggestion }
    }

    /// 创建配置错误
    pub fn config_error(message: String, path: Option<String>, suggestion: Option<String>) -> Self {
        VitePressError::ConfigError { message, path, suggestion }
    }

    /// 创建 IO 错误
    pub fn io_error(message: String, path: Option<String>) -> Self {
        VitePressError::IoError { message, path }
    }

    /// 创建插件错误
    pub fn plugin_error(message: String, plugin: String, path: Option<String>) -> Self {
        VitePressError::PluginError { message, plugin, path }
    }

    /// 创建构建错误
    pub fn build_error(message: String, path: Option<String>) -> Self {
        VitePressError::BuildError { message, path }
    }

    /// 获取错误发生的位置
    pub fn span(&self) -> Span {
        match self {
            VitePressError::ParseError { span, .. } => *span,
            VitePressError::External { span, .. } => *span,
            _ => Span::unknown(),
        }
    }

    /// 获取错误路径
    pub fn path(&self) -> Option<&String> {
        match self {
            VitePressError::ParseError { path, .. } => path.as_ref(),
            VitePressError::ConfigError { path, .. } => path.as_ref(),
            VitePressError::IoError { path, .. } => path.as_ref(),
            VitePressError::PluginError { path, .. } => path.as_ref(),
            VitePressError::BuildError { path, .. } => path.as_ref(),
            VitePressError::External { path, .. } => path.as_ref(),
            _ => None,
        }
    }
}

impl From<VitePressError> for NargoError {
    fn from(err: VitePressError) -> Self {
        match err {
            VitePressError::External { message, span, .. } => NargoError::external_error("vitepress".to_string(), message, span),
            _ => NargoError::external_error("vitepress".to_string(), err.to_string(), err.span()),
        }
    }
}

/// Result 类型别名
pub type Result<T> = std::result::Result<T, VitePressError>;
