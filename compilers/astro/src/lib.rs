#![warn(missing_docs)]

//! Astro static site generator compiler

pub mod cache;
pub mod compiler;
pub mod config;
pub mod data;
pub mod errors;
pub mod plugin;
pub mod plugin_host;
pub mod router;
pub mod server;
pub mod tools;
pub mod types;
pub mod watcher;

/// Oaks 解析器集成
pub mod parser {
    pub use oak_core::parser::*;
    pub use oak_markdown::*;
}
