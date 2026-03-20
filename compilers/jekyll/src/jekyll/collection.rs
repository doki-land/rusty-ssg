//! 集合（Collections）功能模块
//!
//! 该模块提供 Jekyll 集合的完整实现，包括：
//! - 集合配置解析
//! - 集合文件加载和处理
//! - 集合数据管理
//! - 集合在模板中的使用

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::{
    collections::{BTreeMap, HashMap},
    path::{Path, PathBuf},
};

use crate::{
    errors::{CollectionError, JekyllError, MarkdownError},
    jekyll::{FrontMatter, FrontMatterParser, JekyllConfig, JekyllStructure},
};

/// 集合配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectionConfig {
    /// 集合名称
    pub name: String,
    /// 是否输出集合
    pub output: bool,
    /// 集合的永久链接格式
    pub permalink: Option<String>,
    /// 集合的前置内容默认值
    pub defaults: Option<HashMap<String, Value>>,
    /// 集合的相对路径
    pub path: Option<String>,
}

impl CollectionConfig {
    /// 创建新的集合配置
    pub fn new(name: String) -> Self {
        Self { name, output: false, permalink: None, defaults: None, path: None }
    }

    /// 设置是否输出集合
    pub fn with_output(mut self, output: bool) -> Self {
        self.output = output;
        self
    }

    /// 设置永久链接格式
    pub fn with_permalink(mut self, permalink: String) -> Self {
        self.permalink = Some(permalink);
        self
    }

    /// 设置前置内容默认值
    pub fn with_defaults(mut self, defaults: HashMap<String, Value>) -> Self {
        self.defaults = Some(defaults);
        self
    }

    /// 设置集合路径
    pub fn with_path(mut self, path: String) -> Self {
        self.path = Some(path);
        self
    }
}

/// 集合项
#[derive(Debug, Clone, PartialEq)]
pub struct CollectionItem {
    /// 集合项文件路径
    pub path: PathBuf,
    /// 集合项名称
    pub name: String,
    /// 集合项内容
    pub content: String,
    /// 前置内容
    pub front_matter: FrontMatter,
    /// 永久链接
    pub permalink: String,
    /// 相对路径
    pub relative_path: String,
    /// 所属集合
    pub collection: String,
}

impl CollectionItem {
    /// 从文件创建集合项
    ///
    /// # Arguments
    ///
    /// * `item_path` - 集合项文件路径
    /// * `collection_name` - 集合名称
    /// * `config` - 集合配置
    ///
    /// # Returns
    ///
    /// 返回创建的 CollectionItem 实例
    ///
    /// # Errors
    ///
    /// 返回 `CollectionError` 如果文件读取或解析失败
    pub fn from_file<P: AsRef<Path>>(
        item_path: P,
        collection_name: &str,
        config: &CollectionConfig,
    ) -> Result<Self, CollectionError> {
        let item_path = item_path.as_ref();
        let relative_path = item_path.to_string_lossy().to_string();

        // 读取文件内容
        let content = std::fs::read_to_string(item_path)?;

        // 解析 Front Matter
        let front_matter = FrontMatterParser::parse(&content)?;

        // 提取名称（从文件名）
        let name = item_path.file_stem().unwrap_or_default().to_string_lossy().to_string();

        // 生成永久链接
        let permalink = Self::generate_permalink(&name, collection_name, config)?;

        Ok(Self {
            path: item_path.to_path_buf(),
            name,
            content,
            front_matter,
            permalink,
            relative_path,
            collection: collection_name.to_string(),
        })
    }

    /// 生成永久链接
    ///
    /// # Arguments
    ///
    /// * `name` - 集合项名称
    /// * `collection_name` - 集合名称
    /// * `config` - 集合配置
    ///
    /// # Returns
    ///
    /// 返回生成的永久链接
    ///
    /// # Errors
    ///
    /// 返回 `CollectionError` 如果永久链接生成失败
    fn generate_permalink(name: &str, collection_name: &str, config: &CollectionConfig) -> Result<String, CollectionError> {
        let permalink_format = config.permalink.as_deref().unwrap_or_else(|| "/:collection/:name/");

        let mut permalink = permalink_format.to_string();

        // 替换集合名称
        permalink = permalink.replace(":collection", collection_name);

        // 替换名称
        let slugified_name =
            name.to_lowercase().replace(|c: char| !c.is_alphanumeric() && c != ' ', "-").trim_matches('-').to_string();
        permalink = permalink.replace(":name", &slugified_name);

        // 确保以斜杠开头
        if !permalink.starts_with('/') {
            permalink = format!("/{}", permalink);
        }

        // 确保以斜杠结尾
        if !permalink.ends_with('/') {
            permalink = format!("{}/", permalink);
        }

        Ok(permalink)
    }

    /// 渲染集合项内容
    ///
    /// # Arguments
    ///
    /// * `converter` - Markdown 转换器
    ///
    /// # Returns
    ///
    /// 返回渲染后的 HTML
    pub fn render_content(&self, converter: &crate::jekyll::MarkdownConverter) -> Result<String, CollectionError> {
        converter.convert(&self.front_matter.content()).map_err(|e| CollectionError::FileParseError(format!("{:?}", e)))
    }
}

/// 集合
#[derive(Debug, Clone, PartialEq)]
pub struct Collection {
    /// 集合名称
    pub name: String,
    /// 集合配置
    pub config: CollectionConfig,
    /// 集合项
    pub items: Vec<CollectionItem>,
    /// 集合目录路径
    pub directory: PathBuf,
}

impl Collection {
    /// 创建新的集合
    ///
    /// # Arguments
    ///
    /// * `name` - 集合名称
    /// * `config` - 集合配置
    /// * `directory` - 集合目录路径
    pub fn new(name: String, config: CollectionConfig, directory: PathBuf) -> Self {
        Self { name, config, items: Vec::new(), directory }
    }

    /// 加载集合项
    ///
    /// # Returns
    ///
    /// 返回加载的集合项数量
    ///
    /// # Errors
    ///
    /// 返回 `CollectionError` 如果加载失败
    pub fn load_items(&mut self) -> Result<usize, CollectionError> {
        let mut count = 0;

        // 遍历集合目录
        if let Ok(entries) = std::fs::read_dir(&self.directory) {
            for entry in entries {
                let entry = entry?;
                let path = entry.path();

                if path.is_file() {
                    // 只处理 Markdown 文件
                    if let Some(ext) = path.extension() {
                        if ext == "md" || ext == "markdown" {
                            match CollectionItem::from_file(&path, &self.name, &self.config) {
                                Ok(item) => {
                                    self.items.push(item);
                                    count += 1;
                                }
                                Err(e) => {
                                    // 记录错误但继续处理其他项
                                    eprintln!("Error loading collection item {}: {:?}", path.to_string_lossy(), e);
                                }
                            }
                        }
                    }
                }
                else if path.is_dir() {
                    // 递归处理子目录
                    let sub_dir_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                    let sub_collection_name = format!("{}/{}", self.name, sub_dir_name);
                    let sub_config = CollectionConfig::new(sub_collection_name.clone());

                    let mut sub_collection = Self::new(sub_collection_name, sub_config, path);

                    let sub_count = sub_collection.load_items()?;
                    count += sub_count;

                    // 将子集合的项添加到当前集合
                    self.items.extend(sub_collection.items);
                }
            }
        }

        Ok(count)
    }

    /// 获取集合项
    pub fn items(&self) -> &[CollectionItem] {
        &self.items
    }

    /// 获取集合项的可变引用
    pub fn items_mut(&mut self) -> &mut Vec<CollectionItem> {
        &mut self.items
    }

    /// 根据名称获取集合项
    pub fn get_item_by_name(&self, name: &str) -> Option<&CollectionItem> {
        self.items.iter().find(|item| item.name == name)
    }

    /// 排序集合项
    pub fn sort_items_by<F>(&mut self, f: F)
    where
        F: FnMut(&CollectionItem, &CollectionItem) -> std::cmp::Ordering,
    {
        self.items.sort_by(f);
    }
}

/// 集合管理器
pub struct CollectionManager {
    /// 集合映射
    collections: HashMap<String, Collection>,
    /// Jekyll 目录结构
    structure: JekyllStructure,
    /// Jekyll 配置
    config: JekyllConfig,
}

impl CollectionManager {
    /// 创建新的集合管理器
    ///
    /// # Arguments
    ///
    /// * `structure` - Jekyll 目录结构
    /// * `config` - Jekyll 配置
    pub fn new(structure: JekyllStructure, config: JekyllConfig) -> Self {
        Self { collections: HashMap::new(), structure, config }
    }

    /// 加载所有集合
    ///
    /// # Returns
    ///
    /// 返回加载的集合数量
    ///
    /// # Errors
    ///
    /// 返回 `CollectionError` 如果加载失败
    pub fn load_collections(&mut self) -> Result<usize, CollectionError> {
        // 从配置中解析集合
        self.parse_collections_from_config()?;

        // 加载集合项
        for collection in self.collections.values_mut() {
            collection.load_items()?;
        }

        Ok(self.collections.len())
    }

    /// 从配置中解析集合
    ///
    /// # Errors
    ///
    /// 返回 `CollectionError` 如果解析失败
    fn parse_collections_from_config(&mut self) -> Result<(), CollectionError> {
        // 从配置中获取集合配置
        if let Some(collections_config) = self.config.collections.as_ref() {
            for (name, collection_config) in collections_config {
                // 解析集合配置
                let mut config = CollectionConfig::new(name.clone());

                // 解析 output
                if let Some(output) = collection_config.get("output") {
                    if let Some(output_bool) = output.as_bool() {
                        config.output = output_bool;
                    }
                }

                // 解析 permalink
                if let Some(permalink) = collection_config.get("permalink") {
                    if let Some(permalink_str) = permalink.as_str() {
                        config.permalink = Some(permalink_str.to_string());
                    }
                }

                // 解析 path
                if let Some(path) = collection_config.get("path") {
                    if let Some(path_str) = path.as_str() {
                        config.path = Some(path_str.to_string());
                    }
                }

                // 解析 defaults
                if let Some(defaults) = collection_config.get("defaults") {
                    if let Some(defaults_obj) = defaults.as_object() {
                        let mut defaults_map: HashMap<String, Value> = HashMap::new();
                        for (k, v) in defaults_obj {
                            defaults_map.insert(k.clone(), v.clone());
                        }
                        config.defaults = Some(defaults_map);
                    }
                }

                // 确定集合目录路径
                let collection_dir = if let Some(path) = &config.path {
                    self.structure.root().join(path)
                }
                else {
                    self.structure.root().join(format!("_{}", name))
                };

                // 检查目录是否存在
                if collection_dir.exists() && collection_dir.is_dir() {
                    let collection = Collection::new(name.clone(), config, collection_dir);
                    self.collections.insert(name.clone(), collection);
                }
            }
        }

        Ok(())
    }

    /// 获取所有集合
    pub fn collections(&self) -> &HashMap<String, Collection> {
        &self.collections
    }

    /// 获取集合
    pub fn get_collection(&self, name: &str) -> Option<&Collection> {
        self.collections.get(name)
    }

    /// 获取集合的可变引用
    pub fn get_collection_mut(&mut self, name: &str) -> Option<&mut Collection> {
        self.collections.get_mut(name)
    }

    /// 检查集合是否存在
    pub fn has_collection(&self, name: &str) -> bool {
        self.collections.contains_key(name)
    }

    /// 清除所有集合
    pub fn clear(&mut self) {
        self.collections.clear();
    }
}
