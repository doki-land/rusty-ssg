//! MkDocs 编译会话模块
//!
//! 提供 MkDocs 编译会话管理功能，包括配置管理、文档管理、源目录和输出目录设置等。

use crate::types::MkDocsConfig;
use nargo_types::Document;
use std::{collections::HashMap, path::PathBuf};

/// MkDocs 编译会话
///
/// 管理整个 MkDocs 编译过程中的状态，包括配置、文档、源目录和输出目录等。
pub struct CompileSession {
    /// MkDocs 配置
    config: MkDocsConfig,
    /// 编译后的文档集合
    documents: HashMap<String, Document>,
    /// 源目录路径
    source_dir: Option<PathBuf>,
    /// 输出目录路径
    output_dir: Option<PathBuf>,
}

impl CompileSession {
    /// 创建新的 MkDocs 编译会话
    ///
    /// 使用默认配置初始化会话。
    pub fn new() -> Self {
        Self {
            config: MkDocsConfig::default(),
            documents: HashMap::new(),
            source_dir: None,
            output_dir: None,
        }
    }

    /// 使用指定配置创建新的 MkDocs 编译会话
    ///
    /// # 参数
    ///
    /// * `config` - MkDocs 配置对象
    pub fn with_config(config: MkDocsConfig) -> Self {
        Self {
            config,
            documents: HashMap::new(),
            source_dir: None,
            output_dir: None,
        }
    }

    /// 设置源目录
    ///
    /// # 参数
    ///
    /// * `source_dir` - 源目录路径
    ///
    /// # 返回
    ///
    /// 返回链式调用的会话对象
    pub fn with_source_dir(mut self, source_dir: PathBuf) -> Self {
        self.source_dir = Some(source_dir);
        self
    }

    /// 设置输出目录
    ///
    /// # 参数
    ///
    /// * `output_dir` - 输出目录路径
    ///
    /// # 返回
    ///
    /// 返回链式调用的会话对象
    pub fn with_output_dir(mut self, output_dir: PathBuf) -> Self {
        self.output_dir = Some(output_dir);
        self
    }

    /// 获取 MkDocs 配置的不可变引用
    ///
    /// # 返回
    ///
    /// 返回配置对象的不可变引用
    pub fn config(&self) -> &MkDocsConfig {
        &self.config
    }

    /// 获取 MkDocs 配置的可变引用
    ///
    /// # 返回
    ///
    /// 返回配置对象的可变引用
    pub fn config_mut(&mut self) -> &mut MkDocsConfig {
        &mut self.config
    }

    /// 获取所有编译后的文档集合
    ///
    /// # 返回
    ///
    /// 返回文档集合的不可变引用
    pub fn documents(&self) -> &HashMap<String, Document> {
        &self.documents
    }

    /// 根据路径获取单个文档
    ///
    /// # 参数
    ///
    /// * `path` - 文档路径
    ///
    /// # 返回
    ///
    /// 如果找到文档，返回文档的不可变引用；否则返回 None
    pub fn get_document(&self, path: &str) -> Option<&Document> {
        self.documents.get(path)
    }

    /// 添加文档到会话
    ///
    /// # 参数
    ///
    /// * `path` - 文档路径
    /// * `document` - 文档对象
    pub fn add_document(&mut self, path: String, document: Document) {
        self.documents.insert(path, document);
    }

    /// 从会话中移除文档
    ///
    /// # 参数
    ///
    /// * `path` - 文档路径
    ///
    /// # 返回
    ///
    /// 如果找到文档，返回被移除的文档；否则返回 None
    pub fn remove_document(&mut self, path: &str) -> Option<Document> {
        self.documents.remove(path)
    }

    /// 清空会话中的所有文档
    pub fn clear_documents(&mut self) {
        self.documents.clear();
    }

    /// 获取源目录
    ///
    /// # 返回
    ///
    /// 如果设置了源目录，返回源目录的不可变引用；否则返回 None
    pub fn source_dir(&self) -> Option<&PathBuf> {
        self.source_dir.as_ref()
    }

    /// 获取输出目录
    ///
    /// # 返回
    ///
    /// 如果设置了输出目录，返回输出目录的不可变引用；否则返回 None
    pub fn output_dir(&self) -> Option<&PathBuf> {
        self.output_dir.as_ref()
    }
}

impl Default for CompileSession {
    fn default() -> Self {
        Self::new()
    }
}
