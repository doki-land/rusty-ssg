//! Astro 编译器模块

pub mod html_renderer;
pub mod component;
pub mod framework_parser;
#[cfg(test)]
mod component_test;
#[cfg(test)]
mod framework_integration_test;
pub use html_renderer::HtmlRenderer;
pub use component::*;
pub use framework_parser::*;
