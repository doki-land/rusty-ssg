//! 数据处理模块
//! 提供 Gatsby 数据层的核心功能

use crate::types::{Node, NodeId, NodeType};
use async_graphql_value::ConstValue;
use nargo_types::Document;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

/// 内容摘要生成器
/// 用于生成内容的唯一标识符
pub struct ContentDigest;

impl ContentDigest {
    /// 生成内容的摘要
    ///
    /// # Arguments
    ///
    /// * `content` - 要生成摘要的内容
    ///
    /// # Returns
    ///
    /// 生成的摘要字符串
    pub fn generate(content: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content);
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}

/// Markdown 数据源
/// 负责从 Markdown 文件生成节点
pub struct MarkdownDataSource;

impl MarkdownDataSource {
    /// 创建新的 Markdown 数据源
    pub fn new() -> Self {
        Self
    }

    /// 从 Markdown 内容创建节点
    ///
    /// # Arguments
    ///
    /// * `content` - Markdown 内容
    /// * `path` - 文件路径
    ///
    /// # Returns
    ///
    /// 创建的节点
    pub fn create_node_from_markdown(&self, content: &str, path: &str) -> Result<Node, String> {
        let doc = crate::parse_document(content, path).map_err(|e| e.to_string())?;
        self.create_node_from_document(doc, content)
    }

    /// 从文档对象创建节点
    ///
    /// # Arguments
    ///
    /// * `doc` - 文档对象
    /// * `content` - 原始内容
    ///
    /// # Returns
    ///
    /// 创建的节点
    pub fn create_node_from_document(&self, doc: Document, content: &str) -> Result<Node, String> {
        let path = doc.meta.path.clone();
        let id = NodeId::new(path);
        let content_digest = ContentDigest::generate(content);
        let mut node = Node::new(id, NodeType::new("MarkdownRemark".to_string()), content_digest);

        // 添加 frontmatter 字段
        if let Some(title) = doc.frontmatter.title {
            node.set_field("title".to_string(), ConstValue::String(title));
        }

        if let Some(description) = doc.frontmatter.description {
            node.set_field("description".to_string(), ConstValue::String(description));
        }

        if let Some(layout) = doc.frontmatter.layout {
            node.set_field("layout".to_string(), ConstValue::String(layout));
        }

        // 添加内容字段
        node.set_field("content".to_string(), ConstValue::String(content.to_string()));

        Ok(node)
    }
}

/// 站点元数据数据源
/// 负责生成站点元数据节点
pub struct SiteMetadataDataSource;

impl SiteMetadataDataSource {
    /// 创建新的站点元数据数据源
    pub fn new() -> Self {
        Self
    }

    /// 创建站点元数据节点
    ///
    /// # Arguments
    ///
    /// * `title` - 站点标题
    /// * `description` - 站点描述
    /// * `site_url` - 站点 URL
    ///
    /// # Returns
    ///
    /// 创建的节点
    pub fn create_site_metadata_node(
        &self,
        title: Option<String>,
        description: Option<String>,
        site_url: Option<String>,
    ) -> Result<Node, String> {
        let id = NodeId::new("Site".to_string());
        let content_digest = ContentDigest::generate(&format!("{:?}{:?}{:?}", title, description, site_url));
        let mut node = Node::new(id, NodeType::new("Site".to_string()), content_digest);

        // 创建 siteMetadata 对象
        let mut site_metadata = HashMap::new();

        if let Some(title) = title {
            site_metadata.insert("title".to_string(), title);
        }

        if let Some(description) = description {
            site_metadata.insert("description".to_string(), description);
        }

        if let Some(site_url) = site_url {
            site_metadata.insert("siteUrl".to_string(), site_url);
        }

        // 将 siteMetadata 添加到节点
        node.set_field("siteMetadata".to_string(), ConstValue::String(format!("{:?}", site_metadata)));

        Ok(node)
    }
}

/// 文件数据源
/// 负责从文件生成节点
pub struct FileDataSource;

impl FileDataSource {
    /// 创建新的文件数据源
    pub fn new() -> Self {
        Self
    }

    /// 创建文件节点
    ///
    /// # Arguments
    ///
    /// * `absolute_path` - 文件绝对路径
    /// * `name` - 文件名
    /// * `extension` - 文件扩展名
    /// * `size` - 文件大小
    ///
    /// # Returns
    ///
    /// 创建的节点
    pub fn create_file_node(
        &self,
        absolute_path: &str,
        name: &str,
        extension: &str,
        size: u64,
    ) -> Result<Node, String> {
        let id = NodeId::new(absolute_path.to_string());
        let content_digest = ContentDigest::generate(&format!("{}{}{}{}", absolute_path, name, extension, size));
        let mut node = Node::new(id, NodeType::new("File".to_string()), content_digest);

        node.set_field("name".to_string(), ConstValue::String(name.to_string()));
        node.set_field("extension".to_string(), ConstValue::String(extension.to_string()));
        node.set_field("absolutePath".to_string(), ConstValue::String(absolute_path.to_string()));
        node.set_field("size".to_string(), ConstValue::String(size.to_string()));

        Ok(node)
    }
}

/// 数据层管理器
/// 负责管理所有数据源
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
        Self {
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
