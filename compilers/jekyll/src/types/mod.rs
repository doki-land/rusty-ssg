pub mod ast;
pub mod config;
pub mod document;
pub mod errors;
pub mod ipc;
pub mod language;

use std::collections::HashMap;

/// 通用结果类型
pub use crate::errors::Result;

/// 编译结果
pub struct CompileResult {
    /// 编译后的文档
    pub documents: HashMap<String, String>,
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

/// 构建参数
#[derive(Debug, Clone)]
pub struct BuildArgs {
    /// 源目录
    pub source: Option<std::path::PathBuf>,
    /// 输出目录
    pub output: Option<std::path::PathBuf>,
    /// 清理选项
    pub clean: bool,
    /// 增量构建
    pub incremental: bool,
    /// 配置选项
    pub config_options: Vec<String>,
    /// 性能分析
    pub profile: bool,
}

/// 开发服务器参数
#[derive(Debug, Clone)]
pub struct DevArgs {
    /// 源目录
    pub source: Option<std::path::PathBuf>,
    /// 输出目录
    pub output: Option<std::path::PathBuf>,
    /// 端口
    pub port: u16,
    /// 主机
    pub host: String,
    /// 实时重载
    pub livereload: bool,
}

/// 初始化参数
#[derive(Debug, Clone)]
pub struct InitArgs {
    /// 目录路径
    pub path: Option<std::path::PathBuf>,
    /// 模板名称
    pub template: Option<String>,
    /// 强制覆盖
    pub force: bool,
}

/// 检查参数
#[derive(Debug, Clone)]
pub struct CheckArgs {
    /// 源目录
    pub source: Option<std::path::PathBuf>,
    /// 详细输出
    pub verbose: bool,
}
