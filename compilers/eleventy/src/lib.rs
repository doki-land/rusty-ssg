#![warn(missing_docs)]

#![doc = include_str!("../readme.md")]

pub mod compiler;
pub mod config;
pub mod data;
pub mod plugin;
pub mod tools;
pub mod types;

/// Oaks 解析器集成
pub mod parser {
    pub use oak_core::parser::{ParseSession, Parser};
    pub use oak_yaml::{YamlLanguage, YamlLexer, YamlParser};
    pub use oak_toml::{TomlLanguage, TomlLexer, TomlParser};
    pub use oak_markdown::{MarkdownLanguage, MarkdownLexer, MarkdownParser};
}
