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
pub mod plugin;
pub mod tools;
pub mod types;
pub mod watcher;

/// 导出错误类型
pub use errors::*;

/// 导出类型模块
pub use types::*;

/// 导出编译器模块
pub use compiler::*;

/// 导出插件模块
pub use plugin::*;

/// 导出站点生成器相关功能
pub use tools::site_generator::*;

/// 导出工具模块
pub use tools::*;
