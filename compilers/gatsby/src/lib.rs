#![warn(missing_docs)]

//! Gatsby static site generator compiler

/// 编译器模块
pub mod compiler;
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
/// GraphQL 执行引擎模块
pub mod graphql;
/// 数据源模块
pub mod data_source;

pub use types::{
    ConfigError, ConfigValidation, GatsbyConfig, SiteMetadata, SocialLink, PluginConfig,
    ProxyConfig, DevelopConfig, FlagsConfig, GraphQLTypegenConfig, AdapterConfig,
    TrailingSlash, CompileResult, GatsbyError, Result,
};

pub use tools::{
    GatsbyCli, GatsbyCommands, NewArgs, BuildArgs, DevelopArgs, InitArgs, CheckArgs,
    ConfigLoader, StaticSiteGenerator, UnifiedTemplateManager,
    DefaultTheme, LocaleInfo, NavItem, PageContext, SidebarGroup, SidebarLink,
    TemplateEngineType,
};

pub use compiler::GatsbyCompiler;
pub use compiler::{HtmlRenderer, HtmlRendererConfig};
pub use nargo_parser::{DocumentMeta, FrontMatter, FrontMatterParser, MarkdownParser, parse_document, parse_frontmatter};
pub use nargo_types::Document;
pub use session::CompileSession;

pub use plugin::{
    Plugin, PluginError, PluginMeta, PluginRegistry, PluginContext,
    Page,
};

pub use plugin_host::{PluginHost, PluginHostError};

pub use graphql::{
    GraphQLExecutor, SchemaBuilder, NodeBuilder, ContentDigest,
};

pub use types::graphql::{
    GraphQLError, GraphQLResult, NodeId, NodeType, Node, Internal, NodeActions,
    GraphQLFieldType, GraphQLField, GraphQLArgument, GraphQLObjectType, GraphQLSchema,
    SelectionSet, Selection, FieldSelection, Directive, GraphQLRequest, GraphQLResponse,
    NodeStore,
};

pub use data_source::{
    MarkdownDataSource, SiteMetadataDataSource, FileDataSource, DataLayerManager,
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
