#![warn(missing_docs)]

//! Gatsby static site generator compiler

/// 编译器模块
pub mod compiler;
/// 配置模块
pub mod config;
/// 数据处理模块
pub mod data;
/// 错误处理模块
pub mod errors;
/// 插件模块
pub mod plugin;
/// 工具模块
pub mod tools;
/// 类型定义模块
pub mod types;
/// 文件监听器模块
pub mod watcher;

pub use config::{
    AdapterConfig, ConfigError, ConfigValidation, DevelopConfig, FlagsConfig, GatsbyConfig, GraphQLTypegenConfig, PluginConfig,
    ProxyConfig, SiteMetadata, SocialLink, TrailingSlash,
};

pub use tools::{
    BuildArgs, CheckArgs, ConfigLoader, DefaultTheme, DevServer, GatsbyCli, GatsbyCommands, InitArgs, LocaleInfo, NavItem,
    NewArgs, PageContext, SidebarGroup, SidebarLink, StaticSiteGenerator, TemplateEngineType, UnifiedTemplateManager,
};

pub use watcher::{FileChangeEvent, FileWatcher};

#[cfg(feature = "dev")]
pub use tools::DevelopArgs;

pub use compiler::{GatsbyCompiler, HtmlRenderer, HtmlRendererConfig, MarkdownParser, Parser, ParserConfig};
pub use nargo_parser::{DocumentMeta, FrontMatter, FrontMatterParser, parse_frontmatter};
pub use nargo_types::Document;

pub use plugin::{Page, Plugin, PluginContext, PluginError, PluginMeta, PluginRegistry};

pub use data::{ContentDigest, DataLayerManager, FileDataSource, MarkdownDataSource, SiteMetadataDataSource};
pub use types::{
    CompileResult,
    graphql::{
        Directive, FieldSelection, GraphQLArgument, GraphQLError, GraphQLField, GraphQLFieldType, GraphQLObjectType,
        GraphQLRequest, GraphQLResponse, GraphQLResult, GraphQLSchema, Internal, Node, NodeActions, NodeId, NodeStore,
        NodeType, Selection, SelectionSet,
    },
    graphql_service::GraphQLService,
};

use std::collections::HashMap;

/// 便捷的编译函数
///
/// # Arguments
///
/// * `source` - Markdown 源文件内容
/// * `path` - 文档路径
///
/// # Returns
///
/// 编译后的文档
pub fn compile_single(source: &str, path: &str) -> types::Result<Document> {
    let mut compiler = GatsbyCompiler::new();
    compiler.compile_document(source, path)
}

/// 编译多个文档
///
/// # Arguments
///
/// * `documents` - 文档映射（路径 -> 内容）
///
/// # Returns
///
/// 编译结果
pub fn compile_batch(documents: &HashMap<String, String>) -> CompileResult {
    let mut compiler = GatsbyCompiler::new();
    compiler.compile_batch(documents)
}
