//! GraphQL 服务模块
//! 提供 Gatsby 兼容的 GraphQL 服务实现

use super::{GraphQLRequest, GraphQLResponse, GraphQLSchema, Node, NodeId, NodeStore, NodeType};
use async_graphql_value::ConstValue;
use std::collections::{HashMap, HashSet};

/// GraphQL 服务
/// 负责处理 GraphQL 查询和生成 schema
pub struct GraphQLService {
    /// 节点存储
    node_store: NodeStore,
    /// GraphQL Schema
    schema: GraphQLSchema,
}

impl GraphQLService {
    /// 创建新的 GraphQL 服务
    pub fn new() -> Self {
        let schema = Self::generate_default_schema();
        Self { node_store: NodeStore::new(), schema }
    }

    /// 生成默认的 GraphQL Schema
    fn generate_default_schema() -> GraphQLSchema {
        use super::{GraphQLField, GraphQLFieldType, GraphQLObjectType};

        // 创建 File 类型
        let file_type = GraphQLObjectType::new("File")
            .with_description("A file in the Gatsby site")
            .with_field(GraphQLField::new("id", GraphQLFieldType::ID)
                .with_description("The ID of the file"))
            .with_field(GraphQLField::new("name", GraphQLFieldType::String)
                .with_description("The name of the file"))
            .with_field(GraphQLField::new("relativePath", GraphQLFieldType::String)
                .with_description("The relative path to the file"))
            .with_field(GraphQLField::new("absolutePath", GraphQLFieldType::String)
                .with_description("The absolute path to the file"))
            .with_field(GraphQLField::new("extension", GraphQLFieldType::String)
                .with_description("The file extension"))
            .with_field(GraphQLField::new("size", GraphQLFieldType::Int)
                .with_description("The file size in bytes"))
            .with_field(GraphQLField::new("modifiedTime", GraphQLFieldType::DateTime)
                .with_description("The last modified time"));

        // 创建 MarkdownRemark 类型
        let markdown_remark_type = GraphQLObjectType::new("MarkdownRemark")
            .with_description("A Markdown file processed by Remark")
            .with_field(GraphQLField::new("id", GraphQLFieldType::ID)
                .with_description("The ID of the Markdown file"))
            .with_field(GraphQLField::new("frontmatter", GraphQLFieldType::Custom("Frontmatter".to_string()))
                .with_description("The frontmatter of the Markdown file"))
            .with_field(GraphQLField::new("html", GraphQLFieldType::String)
                .with_description("The HTML rendered from the Markdown"))
            .with_field(GraphQLField::new("excerpt", GraphQLFieldType::String)
                .with_description("A short excerpt of the Markdown content"))
            .with_field(GraphQLField::new("file", GraphQLFieldType::Custom("File".to_string()))
                .with_description("The file node associated with this Markdown"));

        // 创建 Frontmatter 类型
        let frontmatter_type = GraphQLObjectType::new("Frontmatter")
            .with_description("The frontmatter of a Markdown file")
            .with_field(GraphQLField::new("title", GraphQLFieldType::String)
                .with_description("The title of the Markdown file"))
            .with_field(GraphQLField::new("description", GraphQLFieldType::String)
                .with_description("The description of the Markdown file"))
            .with_field(GraphQLField::new("date", GraphQLFieldType::Date)
                .with_description("The date of the Markdown file"))
            .with_field(GraphQLField::new("author", GraphQLFieldType::String)
                .with_description("The author of the Markdown file"))
            .with_field(GraphQLField::new("tags", GraphQLFieldType::List(Box::new(GraphQLFieldType::String)))
                .with_description("The tags of the Markdown file"))
            .with_field(GraphQLField::new("categories", GraphQLFieldType::List(Box::new(GraphQLFieldType::String)))
                .with_description("The categories of the Markdown file"));

        // 创建 Site 类型
        let site_type = GraphQLObjectType::new("Site")
            .with_description("The site configuration")
            .with_field(GraphQLField::new("siteMetadata", GraphQLFieldType::Custom("SiteMetadata".to_string()))
                .with_description("The site metadata"));

        // 创建 SiteMetadata 类型
        let site_metadata_type = GraphQLObjectType::new("SiteMetadata")
            .with_description("The site metadata")
            .with_field(GraphQLField::new("title", GraphQLFieldType::String)
                .with_description("The site title"))
            .with_field(GraphQLField::new("description", GraphQLFieldType::String)
                .with_description("The site description"))
            .with_field(GraphQLField::new("author", GraphQLFieldType::String)
                .with_description("The site author"))
            .with_field(GraphQLField::new("siteUrl", GraphQLFieldType::String)
                .with_description("The site URL"))
            .with_field(GraphQLField::new("social", GraphQLFieldType::Custom("Social".to_string()))
                .with_description("Social media links"));

        // 创建 Social 类型
        let social_type = GraphQLObjectType::new("Social")
            .with_description("Social media links")
            .with_field(GraphQLField::new("twitter", GraphQLFieldType::String)
                .with_description("Twitter handle"))
            .with_field(GraphQLField::new("github", GraphQLFieldType::String)
                .with_description("GitHub username"))
            .with_field(GraphQLField::new("linkedin", GraphQLFieldType::String)
                .with_description("LinkedIn profile"));

        // 创建 Query 类型
        let query_type = GraphQLObjectType::new("Query")
            .with_description("The root query type")
            .with_field(GraphQLField::new("site", GraphQLFieldType::Custom("Site".to_string()))
                .with_description("The site configuration"))
            .with_field(GraphQLField::new("allMarkdownRemark", GraphQLFieldType::List(Box::new(GraphQLFieldType::Custom("MarkdownRemark".to_string())))
                .with_description("All MarkdownRemark nodes"))
            .with_field(GraphQLField::new("markdownRemark", GraphQLFieldType::Custom("MarkdownRemark".to_string()))
                .with_description("A single MarkdownRemark node")
                .with_argument(super::GraphQLArgument::new("id", GraphQLFieldType::ID)
                    .with_description("The ID