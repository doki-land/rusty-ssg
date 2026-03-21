//! 数据源模块
//! 提供从各种数据源创建 GraphQL 节点的功能

use std::{collections::HashMap, path::Path};

use async_graphql_value::{ConstValue, Name};
use indexmap::IndexMap;
use nargo_parser::parse_document;
use nargo_types::Document;

use crate::{
    graphql::{ContentDigest, NodeBuilder},
    types::graphql::{GraphQLResult, Node, NodeId, NodeType},
};

/// Markdown 数据源
pub struct MarkdownDataSource;

impl MarkdownDataSource {
    /// 创建新的 Markdown 数据源
    pub fn new() -> Self {
        MarkdownDataSource
    }

    /// 从 Markdown 内容创建节点
    ///
    /// # Arguments
    ///
    /// * `content` - Markdown 内容
    /// * `path` - 文件路径
    ///
    /// # Errors
    ///
    /// 如果解析失败，返回 `GraphQLError`
    pub fn create_node_from_markdown(&self, content: &str, path: &str) -> GraphQLResult<Node> {
        let doc =
            parse_document(content, path).map_err(|e| crate::types::graphql::GraphQLError::ExecutionError(e.to_string()))?;

        self.create_node_from_document(doc, content)
    }

    /// 从 Document 创建节点
    ///
    /// # Arguments
    ///
    /// * `doc` - 文档对象
    /// * `raw_content` - 原始内容
    pub fn create_node_from_document(&self, doc: Document, raw_content: &str) -> GraphQLResult<Node> {
        let node_id = NodeId::new(doc.meta.path.clone());
        let type_name = NodeType::new("MarkdownRemark".to_string());
        let content_digest = ContentDigest::generate(raw_content);

        let mut builder =
            NodeBuilder::new().id(node_id).type_name(type_name).content_digest(content_digest).content(raw_content.to_string());

        if let Some(title) = &doc.frontmatter.title {
            builder = builder.field("title".to_string(), ConstValue::String(title.clone()));
        }

        if let Some(description) = &doc.frontmatter.description {
            builder = builder.field("description".to_string(), ConstValue::String(description.clone()));
        }

        if let Some(layout) = &doc.frontmatter.layout {
            builder = builder.field("layout".to_string(), ConstValue::String(layout.clone()));
        }

        builder = builder.field("path".to_string(), ConstValue::String(doc.meta.path.clone()));

        if let Some(rendered) = &doc.rendered_content {
            builder = builder.field("html".to_string(), ConstValue::String(rendered.clone()));
        }

        builder = builder.field("rawMarkdownBody".to_string(), ConstValue::String(doc.content.clone()));

        for (key, value) in &doc.frontmatter.custom {
            let const_value = self.nargo_value_to_const_value(value);
            builder = builder.field(key.clone(), const_value);
        }

        builder.build()
    }

    /// 将 NargoValue 转换为 ConstValue
    fn nargo_value_to_const_value(&self, value: &nargo_types::NargoValue) -> ConstValue {
        match value {
            nargo_types::NargoValue::String(s) => ConstValue::String(s.clone()),
            nargo_types::NargoValue::Number(n) => ConstValue::Number(
                async_graphql_value::Number::from_f64(*n).unwrap_or_else(|| async_graphql_value::Number::from(0)),
            ),
            nargo_types::NargoValue::Bool(b) => ConstValue::Boolean(*b),
            nargo_types::NargoValue::Array(arr) => {
                ConstValue::List(arr.iter().map(|v| self.nargo_value_to_const_value(v)).collect())
            }
            nargo_types::NargoValue::Object(obj) => {
                let mut index_map = IndexMap::new();
                for (k, v) in obj {
                    index_map.insert(Name::new(k.clone()), self.nargo_value_to_const_value(v));
                }
                ConstValue::Object(index_map)
            }
            nargo_types::NargoValue::Null => ConstValue::Null,
            _ => ConstValue::Null,
        }
    }
}

impl Default for MarkdownDataSource {
    fn default() -> Self {
        Self::new()
    }
}

/// 站点元数据数据源
pub struct SiteMetadataDataSource;

impl SiteMetadataDataSource {
    /// 创建新的站点元数据数据源
    pub fn new() -> Self {
        SiteMetadataDataSource
    }

    /// 创建站点元数据节点
    ///
    /// # Arguments
    ///
    /// * `title` - 站点标题
    /// * `description` - 站点描述
    /// * `site_url` - 站点 URL
    pub fn create_site_metadata_node(
        &self,
        title: Option<String>,
        description: Option<String>,
        site_url: Option<String>,
    ) -> GraphQLResult<Node> {
        let node_id = NodeId::new("Site".to_string());
        let type_name = NodeType::new("Site".to_string());
        let content = format!("{:?}{:?}{:?}", title, description, site_url);
        let content_digest = ContentDigest::generate(&content);

        let mut builder = NodeBuilder::new().id(node_id).type_name(type_name).content_digest(content_digest);

        let mut site_metadata = IndexMap::new();

        if let Some(t) = title {
            site_metadata.insert(Name::new("title"), ConstValue::String(t));
        }

        if let Some(d) = description {
            site_metadata.insert(Name::new("description"), ConstValue::String(d));
        }

        if let Some(u) = site_url {
            site_metadata.insert(Name::new("siteUrl"), ConstValue::String(u));
        }

        builder = builder.field("siteMetadata".to_string(), ConstValue::Object(site_metadata));

        builder.build()
    }
}

impl Default for SiteMetadataDataSource {
    fn default() -> Self {
        Self::new()
    }
}

/// 文件数据源
pub struct FileDataSource;

impl FileDataSource {
    /// 创建新的文件数据源
    pub fn new() -> Self {
        FileDataSource
    }

    /// 创建文件节点
    ///
    /// # Arguments
    ///
    /// * `path` - 文件路径
    /// * `name` - 文件名
    /// * `extension` - 文件扩展名
    /// * `size` - 文件大小
    pub fn create_file_node(&self, path: &str, name: &str, extension: &str, size: u64) -> GraphQLResult<Node> {
        let node_id = NodeId::new(path.to_string());
        let type_name = NodeType::new("File".to_string());
        let content_digest = ContentDigest::generate(path);

        let builder = NodeBuilder::new()
            .id(node_id)
            .type_name(type_name)
            .content_digest(content_digest)
            .field("absolutePath".to_string(), ConstValue::String(path.to_string()))
            .field("base".to_string(), ConstValue::String(format!("{}.{}", name, extension)))
            .field("ext".to_string(), ConstValue::String(format!(".{}", extension)))
            .field("name".to_string(), ConstValue::String(name.to_string()))
            .field("size".to_string(), ConstValue::Number(async_graphql_value::Number::from(size)));

        builder.build()
    }
}

impl Default for FileDataSource {
    fn default() -> Self {
        Self::new()
    }
}

/// 数据层管理器
pub struct DataLayerManager {
    /// Markdown 数据源
    markdown_source: MarkdownDataSource,

    /// 站点元数据数据源
    site_metadata_source: SiteMetadataDataSource,

    /// 文件数据源
    file_source: FileDataSource,
}

impl DataLayerManager {
    /// 创建新的数据层管理器
    pub fn new() -> Self {
        DataLayerManager {
            markdown_source: MarkdownDataSource::new(),
            site_metadata_source: SiteMetadataDataSource::new(),
            file_source: FileDataSource::new(),
        }
    }

    /// 获取 Markdown 数据源
    pub fn markdown_source(&self) -> &MarkdownDataSource {
        &self.markdown_source
    }

    /// 获取站点元数据数据源
    pub fn site_metadata_source(&self) -> &SiteMetadataDataSource {
        &self.site_metadata_source
    }

    /// 获取文件数据源
    pub fn file_source(&self) -> &FileDataSource {
        &self.file_source
    }
}

impl Default for DataLayerManager {
    fn default() -> Self {
        Self::new()
    }
}
