//! MkDocs 编译器错误模块
//!
//! 包含 MkDocs 编译器运行时可能出现的所有错误类型。

use std::{error::Error, fmt};

/// MkDocs 编译器错误枚举
#[derive(Debug)]
pub enum MkDocsError {
    /// 配置解析错误
    ConfigParseError {
        /// 错误消息
        message: String,
    },
    /// 配置验证错误
    ConfigValidationError {
        /// 错误消息
        message: String,
    },
    /// 文件系统 IO 错误
    IoError {
        /// 底层 IO 错误
        source: std::io::Error,
    },
    /// 编译错误
    CompileError {
        /// 错误消息
        message: String,
    },
    /// Markdown 渲染错误
    MarkdownRenderError {
        /// 错误消息
        message: String,
    },
    /// 模板渲染错误
    TemplateRenderError {
        /// 错误消息
        message: String,
    },
    /// 插件错误
    PluginError {
        /// 插件名称
        plugin_name: String,
        /// 错误消息
        message: String,
    },
    /// JSON 序列化/反序列化错误
    JsonError {
        /// 底层 JSON 错误
        source: serde_json::Error,
    },
    /// 路径操作错误
    PathError {
        /// 错误消息
        message: String,
    },
}

impl MkDocsError {
    /// 获取错误的 i18n 键
    pub fn i18n_key(&self) -> &'static str {
        match self {
            MkDocsError::ConfigParseError { .. } => "mkdocs.error.config_parse",
            MkDocsError::ConfigValidationError { .. } => "mkdocs.error.config_validation",
            MkDocsError::IoError { .. } => "mkdocs.error.io",
            MkDocsError::CompileError { .. } => "mkdocs.error.compile",
            MkDocsError::MarkdownRenderError { .. } => "mkdocs.error.markdown_render",
            MkDocsError::TemplateRenderError { .. } => "mkdocs.error.template_render",
            MkDocsError::PluginError { .. } => "mkdocs.error.plugin",
            MkDocsError::JsonError { .. } => "mkdocs.error.json",
            MkDocsError::PathError { .. } => "mkdocs.error.path",
        }
    }
}

impl fmt::Display for MkDocsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MkDocsError::ConfigParseError { message } => {
                write!(f, "Failed to parse configuration: {}", message)
            }
            MkDocsError::ConfigValidationError { message } => {
                write!(f, "Configuration validation error: {}", message)
            }
            MkDocsError::IoError { source } => {
                write!(f, "IO error: {}", source)
            }
            MkDocsError::CompileError { message } => {
                write!(f, "Compile error: {}", message)
            }
            MkDocsError::MarkdownRenderError { message } => {
                write!(f, "Markdown render error: {}", message)
            }
            MkDocsError::TemplateRenderError { message } => {
                write!(f, "Template render error: {}", message)
            }
            MkDocsError::PluginError { plugin_name, message } => {
                write!(f, "Plugin '{}' error: {}", plugin_name, message)
            }
            MkDocsError::JsonError { source } => {
                write!(f, "JSON error: {}", source)
            }
            MkDocsError::PathError { message } => {
                write!(f, "Path error: {}", message)
            }
        }
    }
}

impl Error for MkDocsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MkDocsError::IoError { source } => Some(source),
            MkDocsError::JsonError { source } => Some(source),
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
        MkDocsError::JsonError { source }
    }
}

impl From<walkdir::Error> for MkDocsError {
    fn from(source: walkdir::Error) -> Self {
        use std::io::ErrorKind;
        let io_error = std::io::Error::new(ErrorKind::Other, source);
        MkDocsError::IoError { source: io_error }
    }
}

impl From<notify::Error> for MkDocsError {
    fn from(source: notify::Error) -> Self {
        use std::io::ErrorKind;
        let io_error = std::io::Error::new(ErrorKind::Other, source);
        MkDocsError::IoError { source: io_error }
    }
}

/// MkDocs 编译器结果类型
pub type Result<T> = std::result::Result<T, MkDocsError>;
