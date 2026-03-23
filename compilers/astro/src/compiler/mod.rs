//! Astro 编译器模块

pub mod component;
pub mod parser;
pub mod renderer;
pub use component::*;

pub use renderer::HtmlRenderer;
