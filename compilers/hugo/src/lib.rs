#![warn(missing_docs)]
#![doc = "Hugo 兼容静态站点生成器"]

/// 编译器模块
pub mod compiler;
/// 插件模块
pub mod plugin;

/// 工具模块
pub mod tools;
/// 类型定义模块
pub mod types;

pub use types::{ast, config, document, errors, ipc, language};

pub use nargo_types::Error as NargoError;
pub use types::{CodeWriter, CompileMode, CompileOptions, Cursor, ErrorKind, NargoValue, Position, Span};

pub use types::{ConfigError, ConfigValidation, HugoConfig};

pub use types::{Result, VutexError};

pub use types::{InvokePluginRequest, InvokePluginResponse, IpcMessage, PluginContext};

pub use compiler::VutexCompiler;
pub use nargo_parser::{DocumentMeta, FrontMatter, FrontMatterParser, MarkdownParser, parse_document, parse_frontmatter};

pub use tools::{
    BuildArgs, CheckArgs, Commands, HugoCli, HugoCommands, InitArgs, NewArgs, VutexCli, cmd, site_generator,
    site_generator::{ConfigLoader, StaticSiteGenerator},
    theme,
    theme::{DefaultTheme, NavItem as ThemeNavItem, PageContext, SidebarGroup, SidebarLink},
};

#[cfg(feature = "dev")]
pub use tools::DevArgs;

#[cfg(feature = "dev")]
pub use tools::ServerArgs;

use nargo_types::Document;
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
pub fn compile_single(source: &str, path: &str) -> Result<String> {
    let mut compiler = VutexCompiler::new();
    compiler.compile_document(source, path).map(|doc| doc.content)
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
