#![warn(missing_docs)]

//! Jekyll 集合管理模块
//!
//! 提供 Jekyll 集合（Collections）的加载和管理功能

use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::errors::{CollectionError, Result};

use super::{FrontMatterParser, JekyllConfig, JekyllStructure};

/// 集合配置
#[derive(Debug, Clone)]
pub struct CollectionConfig {
    /// 集合名称
    pub name: String,
    /// 是否输出为独立页面
    pub output: bool,
    /// 永久链接格式
    pub permalink: Option<String>,
    /// 元数据
    pub metadata: Value,
}

impl CollectionConfig {
    /// 创建新的集合配置
    ///
    /// # Arguments
    ///
    /// * `name` - 集合名称
    ///
    /// # Returns
    ///
    /// 返回配置实例
    pub fn new(name: String) -> Self {
        Self { name, output: false, permalink: None, metadata: Value::Null }
    }

    /// 设置是否输出为独立页面
    pub fn with_output(mut self, output: bool) -> Self {
        self.output = output;
        self
    }

    /// 设置永久链接格式
    pub fn with_permalink(mut self, permalink: &str) -> Self {
        self.permalink = Some(permalink.to_string());
        self
    }

    /// 设置元数据
    pub fn with_metadata(mut self, metadata: Value) -> Self {
        self.metadata = metadata;
        self
    }
}

/// 集合项目
#[derive(Debug, Clone)]
pub struct CollectionItem {
    /// 项目名称
    pub name: String,
    /// 项目 slug
    pub slug: String,
    /// 所属集合
    pub collection: String,
    /// 内容
    pub content: String,
    /// Front Matter 变量
    pub front_matter: Value,
    /// 永久链接
    pub permalink: String,
    /// 原始文件路径
    pub path: String,
}

impl CollectionItem {
    /// 从文件创建集合项目
    ///
    /// # Arguments
    ///
    /// * `path` - 文件路径
    /// * `collection` - 集合名称
    /// * `config` - 集合配置
    ///
    /// # Returns
    ///
    /// 返回集合项目或错误
    pub fn from_file<P: AsRef<Path>>(path: P, collection: &str, config: &CollectionConfig) -> Result<Self> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path).map_err(CollectionError::from)?;

        let front_matter = FrontMatterParser::parse(&content).map_err(CollectionError::from)?;

        let name = path.file_stem().and_then(|s| s.to_str()).map(|s| s.to_string()).unwrap_or_else(|| "unnamed".to_string());

        let slug = super::Post::slugify(&name);

        let permalink = if let Some(format) = &config.permalink {
            format.replace(":name", &slug)
        }
        else {
            format!("/{}/{}/", collection, slug)
        };

        Ok(Self {
            name,
            slug,
            collection: collection.to_string(),
            content: front_matter.content().to_string(),
            front_matter: front_matter.variables().clone(),
            permalink,
            path: path.display().to_string(),
        })
    }
}

/// Jekyll 集合
#[derive(Debug)]
pub struct Collection {
    /// 集合名称
    name: String,
    /// 集合配置
    config: CollectionConfig,
    /// 集合目录
    directory: PathBuf,
    /// 集合项目
    items: Vec<CollectionItem>,
}

impl Collection {
    /// 创建新的集合
    ///
    /// # Arguments
    ///
    /// * `name` - 集合名称
    /// * `config` - 集合配置
    /// * `directory` - 集合目录路径
    ///
    /// # Returns
    ///
    /// 返回集合实例
    pub fn new(name: String, config: CollectionConfig, directory: PathBuf) -> Self {
        Self { name, config, directory, items: Vec::new() }
    }

    /// 加载集合中的所有项目
    ///
    /// # Returns
    ///
    /// 返回加载的项目数量或错误
    pub fn load_items(&mut self) -> Result<usize> {
        let mut count = 0;

        if !self.directory.exists() {
            return Ok(0);
        }

        for entry in
            walkdir::WalkDir::new(&self.directory).into_iter().filter_map(|e| e.ok()).filter(|e| e.file_type().is_file())
        {
            let path = entry.path();

            if let Some(ext) = path.extension() {
                if ext == "md" || ext == "markdown" || ext == "html" || ext == "liquid" {
                    match CollectionItem::from_file(path, &self.name, &self.config) {
                        Ok(item) => {
                            self.items.push(item);
                            count += 1;
                        }
                        Err(e) => {
                            eprintln!("Warning: Failed to load collection item {}: {}", path.display(), e);
                        }
                    }
                }
            }
        }

        Ok(count)
    }

    /// 获取集合名称
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 获取集合配置
    pub fn config(&self) -> &CollectionConfig {
        &self.config
    }

    /// 获取集合目录
    pub fn directory(&self) -> &Path {
        &self.directory
    }

    /// 获取所有项目
    pub fn items(&self) -> &[CollectionItem] {
        &self.items
    }

    /// 获取项目数量
    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// 检查是否需要输出为独立页面
    pub fn should_output(&self) -> bool {
        self.config.output
    }
}

/// 集合管理器
#[derive(Debug)]
pub struct CollectionManager {
    /// Jekyll 目录结构
    structure: JekyllStructure,
    /// Jekyll 配置
    config: JekyllConfig,
    /// 集合列表
    collections: std::collections::HashMap<String, Collection>,
}

impl CollectionManager {
    /// 创建新的集合管理器
    ///
    /// # Arguments
    ///
    /// * `structure` - Jekyll 目录结构
    /// * `config` - Jekyll 配置
    ///
    /// # Returns
    ///
    /// 返回集合管理器实例
    pub fn new(structure: JekyllStructure, config: JekyllConfig) -> Self {
        Self { structure, config, collections: std::collections::HashMap::new() }
    }

    /// 加载所有集合
    ///
    /// # Returns
    ///
    /// 返回加载的集合数量或错误
    pub fn load_collections(&mut self) -> Result<usize> {
        let mut count = 0;

        for collection_name in self.structure.collections() {
            let collection_dir = self.structure.collection_dir(collection_name);

            let config = self.parse_collection_config(collection_name)?;

            let mut collection = Collection::new(collection_name.to_string(), config, collection_dir);

            count += collection.load_items()?;

            self.collections.insert(collection_name.to_string(), collection);
        }

        if let Some(collections_config) = &self.config.collections {
            for (name, value) in collections_config {
                if !self.collections.contains_key(name) {
                    let collection_dir = self.structure.collection_dir(name);

                    let mut config = CollectionConfig::new(name.clone());

                    if let Value::Object(obj) = value {
                        if let Some(output) = obj.get("output") {
                            if let Value::Bool(b) = output {
                                config = config.with_output(*b);
                            }
                        }
                        if let Some(permalink) = obj.get("permalink") {
                            if let Value::String(s) = permalink {
                                config = config.with_permalink(s);
                            }
                        }
                    }

                    let mut collection = Collection::new(name.clone(), config, collection_dir.clone());

                    if collection_dir.exists() {
                        count += collection.load_items()?;
                    }

                    self.collections.insert(name.clone(), collection);
                }
            }
        }

        Ok(count)
    }

    /// 解析集合配置
    fn parse_collection_config(&self, name: &str) -> Result<CollectionConfig> {
        let mut config = CollectionConfig::new(name.to_string());

        if let Some(collections) = &self.config.collections {
            if let Some(collection_config) = collections.get(name) {
                if let Value::Object(obj) = collection_config {
                    if let Some(output) = obj.get("output") {
                        if let Value::Bool(b) = output {
                            config = config.with_output(*b);
                        }
                    }
                    if let Some(permalink) = obj.get("permalink") {
                        if let Value::String(s) = permalink {
                            config = config.with_permalink(s);
                        }
                    }
                }
            }
        }

        Ok(config)
    }

    /// 检查是否存在指定集合
    ///
    /// # Arguments
    ///
    /// * `name` - 集合名称
    ///
    /// # Returns
    ///
    /// 如果存在返回 true
    pub fn has_collection(&self, name: &str) -> bool {
        self.collections.contains_key(name)
    }

    /// 获取指定集合
    ///
    /// # Arguments
    ///
    /// * `name` - 集合名称
    ///
    /// # Returns
    ///
    /// 返回集合引用，如果不存在返回 None
    pub fn get_collection(&self, name: &str) -> Option<&Collection> {
        self.collections.get(name)
    }

    /// 获取所有集合
    pub fn collections(&self) -> &std::collections::HashMap<String, Collection> {
        &self.collections
    }

    /// 获取集合数量
    pub fn count(&self) -> usize {
        self.collections.len()
    }
}
