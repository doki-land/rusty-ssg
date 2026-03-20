#![warn(missing_docs)]

//! Jekyll 博客框架核心编译器
//!
//! 该库提供完整的 Jekyll 博客框架功能，包括：
//! - 目录结构识别和处理
//! - 帖子（Posts）处理
//! - 集合（Collections）支持
//! - Liquid 模板引擎集成
//! - Markdown 处理

pub mod compiler;
pub mod errors;
pub mod jekyll;
pub mod plugin_host;
pub mod tools;
pub mod types;

/// 导出错误类型
pub use errors::{CollectionError, DataError, JekyllError, LiquidError, MarkdownError, PostError};

/// 导出核心功能
pub use jekyll::*;

/// 导出工具模块
pub use tools::*;

/// 导出类型模块
pub use types::*;

/// 导出编译器模块
pub use compiler::*;

/// 导出插件宿主模块
pub use plugin_host::*;
