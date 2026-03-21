#![warn(missing_docs)]
#![doc = "VuTeX 文档编译器 - 将 Markdown 文档编译为元信息供 JS 运行时使用"]

pub mod compiler;
pub mod config;
pub mod plugin;
pub mod plugin_host;
pub mod session;
pub mod tools;
pub mod types;

pub use types::{ast, config as types_config, document, errors, ipc, language};

pub use nargo_types::Error as NargoError;
pub use types::{CodeWriter, CompileMode, CompileOptions, Cursor, ErrorKind, NargoValue, Position, Span};

pub use types_config::{
    BuildConfig, ConfigError, ConfigValidation, FooterConfig, LocaleConfig, MarkdownConfig, NavItem as ConfigNavItem, PluginConfig, SidebarItem,
    SocialLink, ThemeConfig, VutexConfig,
};

pub use types::{Result, VutexError};

pub use plugin::{PluginContext, PluginMeta, PluginRegistry, VutexPlugin};
pub use plugin::katex::KaTeXPlugin;
pub use types::{InvokePluginRequest, InvokePluginResponse, IpcMessage};

pub use compiler::VutexCompiler;
pub use nargo_types::Document;
pub use plugin_host::{PluginHost, PluginHostError};
pub use session::CompileSession;
pub use tools::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub fn success(documents: HashMap<String, Document>, compile_time_ms: u64) -> Self {
        Self { documents, compile_time_ms, success: true, errors: Vec::new() }
    }

    /// 创建失败的编译结果
    pub fn failure(errors: Vec<String>, compile_time_ms: u64) -> Self {
        Self { documents: HashMap::new(), compile_time_ms, success: false, errors }
    }

    /// 从 VutexError 创建失败的编译结果
    pub fn from_errors(errors: Vec<VutexError>, compile_time_ms: u64) -> Self {
        let error_strings = errors.iter().map(|e| format!("{}", e)).collect();
        Self::failure(error_strings, compile_time_ms)
    }

    /// 序列化为 JSON
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }

    /// 序列化为美化的 JSON
    pub fn to_json_pretty(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}

/// 便捷的编译函数
///
/// # Arguments
///
/// * `source` - Markdown 源文件内容
/// * `path` - 文档路径
///
/// # Returns
///
/// 编译后的文档
pub fn compile_single(source: &str, path: &str) -> Result<Document> {
    let mut compiler = VutexCompiler::new();
    compiler.compile_document(source, path)
}

/// 编译多个文档
///
/// # Arguments
///
/// * `documents` - 文档映射（路径 -> 内容）
///
/// # Returns
///
/// 编译结果
pub fn compile_batch(documents: &HashMap<String, String>) -> CompileResult {
    let mut compiler = VutexCompiler::new();
    compiler.compile_batch(documents)
}
