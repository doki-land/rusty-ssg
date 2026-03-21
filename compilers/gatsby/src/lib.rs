#![warn(missing_docs)]

//! Gatsby static site generator compiler

/// 编译器模块
pub mod compiler;
/// 数据源模块
pub mod data_source;
/// GraphQL 执行引擎模块
pub mod graphql;
/// 插件模块
pub mod plugin;
/// 插件主机模块
pub mod plugin_host;
/// 编译会话模块
pub mod session;
/// 工具模块
pub mod tools;
/// 类型定义模块
pub mod types;

pub use types::{
    AdapterConfig, CompileResult, ConfigError, ConfigValidation, DevelopConfig, FlagsConfig, GatsbyConfig, GatsbyError,
    GraphQLTypegenConfig, PluginConfig, ProxyConfig, Result, SiteMetadata, SocialLink, TrailingSlash,
};

pub use tools::{
    BuildArgs, CheckArgs, ConfigLoader, DefaultTheme, GatsbyCli, GatsbyCommands, InitArgs, LocaleInfo, NavItem,
    NewArgs, PageContext, SidebarGroup, SidebarLink, StaticSiteGenerator, TemplateEngineType, UnifiedTemplateManager,
};

#[cfg(feature = "dev")]
pub use tools::DevelopArgs;

pub use compiler::{GatsbyCompiler, HtmlRenderer, HtmlRendererConfig};
pub use nargo_parser::{DocumentMeta, FrontMatter, FrontMatterParser, MarkdownParser, parse_document, parse_frontmatter};
pub use nargo_types::Document;
pub use session::CompileSession;

pub use plugin::{Page, Plugin, PluginContext, PluginError, PluginMeta, PluginRegistry};

pub use plugin_host::{PluginHost, PluginHostError};

pub use graphql::{ContentDigest, GraphQLExecutor, NodeBuilder, SchemaBuilder};

pub use types::graphql::{
    Directive, FieldSelection, GraphQLArgument, GraphQLError, GraphQLField, GraphQLFieldType, GraphQLObjectType,
    GraphQLRequest, GraphQLResponse, GraphQLResult, GraphQLSchema, Internal, Node, NodeActions, NodeId, NodeStore, NodeType,
    Selection, SelectionSet,
};

pub use data_source::{DataLayerManager, FileDataSource, MarkdownDataSource, SiteMetadataDataSource};

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
pub fn compile_single(source: &str, path: &str) -> Result<Document> {
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
