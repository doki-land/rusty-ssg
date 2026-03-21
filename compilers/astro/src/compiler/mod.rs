//! Astro 编译器模块

pub mod component;
#[cfg(test)]
mod component_test;
#[cfg(test)]
mod framework_integration_test;
pub mod framework_parser;
pub mod html_renderer;
pub use component::*;
pub use framework_parser::*;
pub use html_renderer::HtmlRenderer;
