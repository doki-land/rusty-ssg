#![warn(missing_docs)]

//! Astro static site generator compiler

pub mod compiler;
pub mod plugin;
pub mod tools;
pub mod types;

/// Oaks 解析器集成
pub mod parser {
    pub use oak_core::parser::*;
    pub use oak_markdown::*;
}
