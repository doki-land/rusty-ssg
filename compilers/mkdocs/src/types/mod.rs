//! MkDocs 编译器类型模块
//!
//! 包含 MkDocs 编译器所需的所有类型定义，包括配置类型、错误类型等。

/// 配置模块
pub mod config;
/// 错误模块
pub mod errors;

// 重新导出常用类型
pub use config::{
    AlternatePalette, FontConfig, IconConfig, MkDocsConfig, NavItem, NavValue, PaletteConfig, PluginConfig, PluginOptions,
    ThemeConfig, ToggleConfig,
};
pub use errors::{MkDocsError, Result};
