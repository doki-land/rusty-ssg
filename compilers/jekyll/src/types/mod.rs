pub mod ast;
pub mod config;
pub mod document;
pub mod errors;
pub mod ipc;
pub mod language;

pub use nargo_types::{
    CodeWriter, CompileMode, CompileOptions, Cursor, Document, Error as NargoError, ErrorKind, NargoValue, Position, Span,
};

pub use config::{
    BuildConfig, ConfigError, ConfigValidation, FooterConfig, LocaleConfig, MarkdownConfig, NavItem, PluginConfig, SidebarItem,
    SocialLink, ThemeConfig, VutexConfig,
};
pub use errors::{Result, VutexError};
pub use ipc::{InvokePluginRequest, InvokePluginResponse, IpcMessage, PluginContext};

use std::collections::HashMap;

/// 编译结果
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
    /// * `document_paths` - 文档路径列表
    /// * `compile_time_ms` - 编译时间（毫秒）
    pub fn success(document_paths: Vec<String>, compile_time_ms: u64) -> Self {
        let mut documents = HashMap::new();
        for path in document_paths {
            // This is a placeholder - in real implementation, we would have the actual documents
        }
        Self { documents, compile_time_ms, success: true, errors: Vec::new() }
    }

    /// 从错误创建编译结果
    ///
    /// # Arguments
    ///
    /// * `errors` - 错误列表
    /// * `compile_time_ms` - 编译时间（毫秒）
    pub fn from_errors(errors: Vec<impl std::fmt::Display>, compile_time_ms: u64) -> Self {
        let error_strings = errors.iter().map(|e| format!("{}", e)).collect();
        Self { documents: HashMap::new(), compile_time_ms, success: false, errors: error_strings }
    }
}
