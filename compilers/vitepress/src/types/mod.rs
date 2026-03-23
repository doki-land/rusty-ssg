//! VitePress 类型定义模块
//! 提供 VitePress 文档编译器所需的所有类型定义，包括配置、错误、AST 等

pub mod ast;
pub mod config;
pub mod document;
pub mod errors;
pub mod ipc;
pub mod language;

pub use nargo_types::{
    CodeWriter, CompileMode, CompileOptions, Cursor, Error as NargoError, ErrorKind, NargoValue, Position, Span,
};

pub use config::{
    BuildConfig, ConfigError, ConfigValidation, FooterConfig, LocaleConfig, MarkdownConfig, NavItem, PluginConfig, Sidebar,
    SidebarItem, SocialLink, ThemeConfig, VitePressConfig,
};
pub use errors::{Result, VitePressError};
pub use ipc::{InvokePluginRequest, InvokePluginResponse, IpcMessage, PluginContext};
