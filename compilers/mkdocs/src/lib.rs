#![warn(missing_docs)]
#![doc = "MkDocs 文档编译器 - 将 Markdown 文档编译为 HTML 网站"]

pub mod compiler;
pub mod plugin;
pub mod tools;
pub mod types;
pub mod watcher;

pub use types::{config, errors};

pub use nargo_types::{
    CodeWriter, CompileMode, CompileOptions, Cursor, Error as NargoError, ErrorKind, NargoValue, Position, Span,
};

pub use types::{
    AlternatePalette, ExtraJavaScript, ExtraJavaScriptConfig, FontConfig, IconConfig, LinksValidationConfig, MarkdownExtension,
    MkDocsConfig, NavItem, NavValue, NavValidationConfig, PaletteConfig, PluginConfig, PluginOptions, ThemeConfig, ToggleConfig,
    ValidationConfig, ValidationLevel,
};

pub use types::{MkDocsError, Result};

pub use compiler::{HtmlRenderer, HtmlRendererConfig, MkDocsCompiler};

pub use tools::{
    BuildArgs, BuildCommand, CheckArgs, CheckCommand, InitArgs, InitCommand, MkDocsCli, MkDocsCommands, NewArgs, NewCommand,
    ServeArgs, ServeCommand, VersionCommand,
};

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

    /// 从 MkDocsError 创建失败的编译结果
    pub fn from_errors(errors: Vec<MkDocsError>, compile_time_ms: u64) -> Self {
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

/// 便捷的单个文档编译函数
///
/// # Arguments
///
/// * `source` - Markdown 源文件内容
/// * `path` - 文档路径
///
/// # Returns
///
/// 编译后的 HTML 内容
pub fn compile_single(source: &str) -> Result<String> {
    let renderer = HtmlRenderer::new();
    Ok(renderer.render(source))
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
    let start_time = std::time::Instant::now();
    let renderer = HtmlRenderer::new();
    let mut result_documents = HashMap::new();
    let mut errors = Vec::new();

    for (path, content) in documents {
        use nargo_types::{DocumentMeta, FrontMatter};
        let html = renderer.render(content);
        let doc = Document {
            meta: DocumentMeta { path: path.clone(), title: None, lang: None, last_updated: None, extra: HashMap::new() },
            content: html,
            frontmatter: FrontMatter::new(),
            rendered_content: None,
            span: Default::default(),
        };
        result_documents.insert(path.clone(), doc);
    }

    let duration = start_time.elapsed();
    if errors.is_empty() {
        CompileResult::success(result_documents, duration.as_millis() as u64)
    }
    else {
        CompileResult::failure(errors, duration.as_millis() as u64)
    }
}
