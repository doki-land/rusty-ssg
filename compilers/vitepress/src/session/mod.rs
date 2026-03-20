//! 编译会话模块
//! 提供编译会话管理功能

use crate::types::VutexConfig;
use nargo_types::Document;
use std::{collections::HashMap, path::PathBuf};

/// 编译会话
pub struct CompileSession {
    /// 会话配置
    config: VutexConfig,
    /// 编译后的文档
    documents: HashMap<String, Document>,
    /// 源目录
    source_dir: Option<PathBuf>,
    /// 输出目录
    output_dir: Option<PathBuf>,
}

impl CompileSession {
    /// 创建新的编译会话
    pub fn new() -> Self {
        Self { config: VutexConfig::new(), documents: HashMap::new(), source_dir: None, output_dir: None }
    }

    /// 创建带配置的编译会话
    pub fn with_config(config: VutexConfig) -> Self {
        Self { config, documents: HashMap::new(), source_dir: None, output_dir: None }
    }

    /// 设置源目录
    pub fn with_source_dir(mut self, source_dir: PathBuf) -> Self {
        self.source_dir = Some(source_dir);
        self
    }

    /// 设置输出目录
    pub fn with_output_dir(mut self, output_dir: PathBuf) -> Self {
        self.output_dir = Some(output_dir);
        self
    }

    /// 获取会话配置
    pub fn config(&self) -> &VutexConfig {
        &self.config
    }

    /// 获取可变的会话配置
    pub fn config_mut(&mut self) -> &mut VutexConfig {
        &mut self.config
    }

    /// 获取所有编译后的文档
    pub fn documents(&self) -> &HashMap<String, Document> {
        &self.documents
    }

    /// 获取单个文档
    pub fn get_document(&self, path: &str) -> Option<&Document> {
        self.documents.get(path)
    }

    /// 添加文档到会话
    pub fn add_document(&mut self, path: String, document: Document) {
        self.documents.insert(path, document);
    }

    /// 从会话中移除文档
    pub fn remove_document(&mut self, path: &str) -> Option<Document> {
        self.documents.remove(path)
    }

    /// 清空会话中的所有文档
    pub fn clear_documents(&mut self) {
        self.documents.clear();
    }

    /// 获取源目录
    pub fn source_dir(&self) -> Option<&PathBuf> {
        self.source_dir.as_ref()
    }

    /// 获取输出目录
    pub fn output_dir(&self) -> Option<&PathBuf> {
        self.output_dir.as_ref()
    }
}

impl Default for CompileSession {
    fn default() -> Self {
        Self::new()
    }
}
