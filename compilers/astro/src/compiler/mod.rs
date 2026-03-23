//! Astro 编译器模块

pub mod component;
pub mod dependency;
pub mod optimization;
pub mod parser;
pub mod renderer;
pub use component::*;

pub use dependency::DependencyAnalyzer;
pub use optimization::Optimizer;
pub use renderer::HtmlRenderer;
