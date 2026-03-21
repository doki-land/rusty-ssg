#![warn(missing_docs)]

//! Eleventy static site generator compiler
//!
//! 一个 100% 兼容 Eleventy 的 Rust 静态站点生成器实现

pub mod build;
pub mod compiler;
pub mod config;
pub mod data;
pub mod plugin;
pub mod plugin_host;
pub mod server;
pub mod session;
pub mod tools;
pub mod types;
