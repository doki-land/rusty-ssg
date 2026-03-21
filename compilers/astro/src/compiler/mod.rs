//! Astro 编译器模块

pub mod component;
pub mod parser;
pub mod renderer;
#[cfg(test)]
mod component_test;
#[cfg(test)]
mod framework_integration_test;
pub use component::*;
pub use renderer::HtmlRenderer;
