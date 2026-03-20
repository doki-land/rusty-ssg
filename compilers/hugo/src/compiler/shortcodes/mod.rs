//! Hugo 风格短代码系统
//!
//! 提供短代码解析、渲染和管理功能。

pub mod builtin;
pub mod parser;
pub mod registry;
pub mod types;

pub use parser::{ShortcodeParser, TextFragment};
pub use registry::ShortcodeRegistry;
pub use types::{
    Shortcode, ShortcodeContext, ShortcodeError, ShortcodeHandler, ShortcodeParams, ShortcodeResult, ShortcodeType,
};
