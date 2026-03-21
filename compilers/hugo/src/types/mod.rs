//! 类型定义模块
//! 提供 Hugo 编译器所需的所有核心类型、配置和数据结构
pub mod ast;
pub mod config;
pub mod document;
pub mod errors;
pub mod ipc;
pub mod language;
pub mod taxonomies;

pub use nargo_types::{
    CodeWriter, CompileMode, CompileOptions, Cursor, Error as NargoError, ErrorKind, NargoValue, Position, Span,
};

pub use config::{ConfigError, ConfigValidation, HugoConfig};
pub use document::*;
pub use errors::{Result, VutexError};
pub use ipc::{InvokePluginRequest, InvokePluginResponse, IpcMessage, PluginContext};
