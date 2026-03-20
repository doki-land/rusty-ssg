//! 集合（Collections）功能模块
//!
//! 该模块提供 Jekyll 集合的完整实现，包括：
//! - 集合配置解析
//! - 集合文件加载和处理
//! - 集合数据管理
//! - 集合在模板中的使用
//! - 永久链接生成

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::{
    errors::CollectionError,
    jekyll::{FrontMatter, FrontMatterParser, JekyllConfig, JekyllStructure},
};

/// 集合配置，定义集合的行为和属性
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectionConfig {
    /// 集合名称
    pub name: String,

    /// 是否输出集合页面
    ///
    /// 如果为 true，集合中的每个项都将被渲染为独立的页面
    pub output: bool,

    /// 集合的永久链接格式
    ///
    /// 支持的占位符：
    /// - :collection - 集合名称
    /// - :name - 文件名（不含扩展名）
    /// - :title - 标题（从 front matter 中获取）
    /// - :slug - URL 友好的标题
    /// - :year - 年份（4位）
    /// - :month - 月份（2位）
    /// - :day - 日期（2位）
    /// - :i_month - 月份（无前导零）
    /// - :i_day - 日期（无前导零）
    pub permalink: Option<String>,

    /// 集合的前置内容默认值
    ///
    /// 这些默认值会被应用到集合中的每个项
    pub defaults: Option<HashMap<String, Value>>,

    /// 集合的相对路径
    ///
    /// 相对于站点根目录的路径
    pub path: Option<String>,

    /// 集合项的排序字段
    ///
    /// 默认按文件名排序
    pub sort_by: Option<String>,

    /// 排序方向
    ///
    /// true 为升序，false 为降序，默认为升序
    pub sort_reverse: Option<bool>,

    /// 集合布局模板
    ///
    /// 用于渲染集合项的默认布局
    pub layout: Option<String>,
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
    /// 返回默认配置的集合配置实例
    pub fn new(name: String) -> Self {
        Self {
            name,
            output: false,
            permalink: None,
            defaults: None,
            path: None,
            sort_by: None,
            sort_reverse: None,
            layout: None,
        }
    }

    /// 设置是否输出集合
    ///
    /// # Arguments
    ///
    /// * `output` - 是否输出集合
    ///
    /// # Returns
    ///
    /// 返回修改后的配置实例
    pub fn with_output(mut self, output: bool) -> Self {
        self.output = output;
        self
    }

    /// 设置永久链接格式
    ///
    /// # Arguments
    ///
    /// * `permalink` - 永久链接格式字符串
    ///
    /// # Returns
    ///
    /// 返回修改后的配置实例
    pub fn with_permalink(mut self, permalink: String) -> Self {
        self.permalink = Some(permalink);
        self
    }

    /// 设置前置内容默认值
    ///
    /// # Arguments
    ///
    /// * `defaults` - 默认值哈希映射
    ///
    /// # Returns
    ///
    /// 返回修改后的配置实例
    pub fn with_defaults(mut self, defaults: HashMap<String, Value>) -> Self {
        self.defaults = Some(defaults);
        self
    }

    /// 设置集合路径
    ///
    /// # Arguments
    ///
    /// * `path` - 集合路径
    ///
    /// # Returns
    ///
    /// 返回修改后的配置实例
    pub fn with_path(mut self, path: String) -> Self {
        self.path = Some(path);
        self
    }

    /// 设置排序字段
    ///
    /// # Arguments
    ///
    /// * `sort_by` - 排序字段名
    ///
    /// # Returns
    ///
    /// 返回修改后的配置实例
    pub fn with_sort_by(mut self, sort_by: String) -> Self {
        self.sort_by = Some(sort_by);
        self
    }

    /// 设置排序方向
    ///
    /// # Arguments
    ///
    /// * `reverse` - 是否反向排序
    ///
    /// # Returns
    ///
    /// 返回修改后的配置实例
    pub fn with_sort_reverse(mut self, reverse: bool) -> Self {
        self.sort_reverse = Some(reverse);
        self
    }

    /// 设置默认布局
    ///
    /// # Arguments
    ///
    /// * `layout` - 布局名称
    ///
    /// # Returns
    ///
    /// 返回修改后的配置实例
    pub fn with_layout(mut self, layout: String) -> Self {
        self.layout = Some(layout);
        self
    }

    /// 从 JSON 值解析集合配置
    ///
    /// # Arguments
    ///
    /// * `name` - 集合名称
    /// * `value` - JSON 配置值
    ///
    /// # Returns
    ///
    /// 返回解析后的集合配置
    pub fn from_value(name: String, value: &Value) -> Result<Self, CollectionError> {
        let mut config = Self::new(name);

        if let Some(obj) = value.as_object() {
            if let Some(output) = obj.get("output") {
                if let Some(b) = output.as_bool() {
                    config.output = b;
                }
            }

            if let Some(permalink) = obj.get("permalink") {
                if let Some(s) = permalink.as_str() {
                    config.permalink = Some(s.to_string());
                }
            }

            if let Some(path) = obj.get("path") {
                if let Some(s) = path.as_str() {
                    config.path = Some(s.to_string());
                }
            }

            if let Some(sort_by) = obj.get("sort_by") {
                if let Some(s) = sort_by.as_str() {
                    config.sort_by = Some(s.to_string());
                }
            }

            if let Some(sort_reverse) = obj.get("sort_reverse") {
                if let Some(b) = sort_reverse.as_bool() {
                    config.sort_reverse = Some(b);
                }
            }

            if let Some(layout) = obj.get("layout") {
                if let Some(s) = layout.as_str() {
                    config.layout = Some(s.to_string());
                }
            }

            if let Some(defaults) = obj.get("defaults") {
                if let Some(defaults_obj) = defaults.as_object() {
                    let mut defaults_map: HashMap<String, Value> = HashMap::new();
                    for (k, v) in defaults_obj {
                        defaults_map.insert(k.clone(), v.clone());
                    }
                    config.defaults = Some(defaults_map);
                }
            }
        }

        Ok(config)
    }
}

/// 集合项，表示集合中的单个文档
#[derive(Debug, Clone, PartialEq)]
pub struct CollectionItem {
    /// 集合项文件路径
    pub path: PathBuf,

    /// 集合项名称（文件名不含扩展名）
    pub name: String,

    /// 集合项完整内容（包括 Front Matter）
    pub content: String,

    /// 解析后的 Front Matter
    pub front_matter: FrontMatter,

    /// 生成的永久链接
    pub permalink: String,

    /// 相对于集合目录的路径
    pub relative_path: String,

    /// 所属集合名称
    pub collection: String,

    /// 从文件名解析出的日期（如果有）
    pub date: Option<NaiveDate>,

    /// URL 友好的标题
    pub slug: String,
}

impl CollectionItem {
    /// 从文件创建集合项
    ///
    /// # Arguments
    ///
    /// * `item_path` - 集合项文件路径
    /// * `collection_name` - 集合名称
    /// * `config` - 集合配置
    /// * `site_config` - 站点配置
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
        site_config: &JekyllConfig,
    ) -> Result<Self, CollectionError> {
        let item_path = item_path.as_ref();
        let relative_path = item_path.to_string_lossy().to_string();

        let content = std::fs::read_to_string(item_path)?;

        let mut front_matter = FrontMatterParser::parse(&content)?;

        if let Some(defaults) = &config.defaults {
            for (k, v) in defaults {
                if !front_matter.variables.contains_key(k) {
                    front_matter.variables.insert(k.clone(), v.clone());
                }
            }
        }

        let name = item_path.file_stem().unwrap_or_default().to_string_lossy().to_string();

        let date = Self::parse_date_from_name(&name);

        let slug = Self::generate_slug(&name, &front_matter);

        let permalink = Self::generate_permalink(
            &name,
            collection_name,
            &front_matter,
            date,
            &slug,
            config,
            site_config,
        )?;

        Ok(Self {
            path: item_path.to_path_buf(),
            name,
            content,
            front_matter,
            permalink,
            relative_path,
            collection: collection_name.to_string(),
            date,
            slug,
        })
    }

    /// 从文件名解析日期
    ///
    /// 支持的文件名格式：YYYY-MM-DD-title.md
    ///
    /// # Arguments
    ///
    /// * `name` - 文件名（不含扩展名）
    ///
    /// # Returns
    ///
    /// 返回解析出的日期（如果成功）
    fn parse_date_from_name(name: &str) -> Option<NaiveDate> {
        if name.len() >= 10 {
            if let Ok(date) = NaiveDate::parse_from_str(&name[0..10], "%Y-%m-%d") {
                return Some(date);
            }
        }
        None
    }

    /// 生成 URL 友好的 slug
    ///
    /// # Arguments
    ///
    /// * `name` - 文件名
    /// * `front_matter` - Front Matter
    ///
    /// # Returns
    ///
    /// 返回生成的 slug
    fn generate_slug(name: &str, front_matter: &FrontMatter) -> String {
        let title = front_matter
            .get_str("title")
            .or_else(|| front_matter.get_str("slug"))
            .unwrap_or(name);

        title
            .to_lowercase()
            .replace(|c: char| !c.is_alphanumeric() && c != ' ', "-")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join("-")
            .trim_matches('-')
            .to_string()
    }

    /// 生成永久链接
    ///
    /// # Arguments
    ///
    /// * `name` - 文件名
    /// * `collection_name` - 集合名称
    /// * `front_matter` - Front Matter
    /// * `date` - 解析出的日期
    /// * `slug` - URL 友好的 slug
    /// * `config` - 集合配置
    /// * `site_config` - 站点配置
    ///
    /// # Returns
    ///
    /// 返回生成的永久链接
    ///
    /// # Errors
    ///
    /// 返回 `CollectionError` 如果永久链接生成失败
    fn generate_permalink(
        name: &str,
        collection_name: &str,
        front_matter: &FrontMatter,
        date: Option<NaiveDate>,
        slug: &str,
        config: &CollectionConfig,
        site_config: &JekyllConfig,
    ) -> Result<String, CollectionError> {
        let permalink_format = config
            .permalink
            .as_deref()
            .or_else(|| site_config.permalink.as_deref())
            .unwrap_or("/:collection/:name/");

        let mut permalink = permalink_format.to_string();

        permalink = permalink.replace(":collection", collection_name);
        permalink = permalink.replace(":name", name);
        permalink = permalink.replace(":slug", slug);

        if let Some(title) = front_matter.get_str("title") {
            permalink = permalink.replace(":title", title);
        }

        if let Some(date) = date {
            permalink = permalink.replace(":year", &date.format("%Y").to_string());
            permalink = permalink.replace(":month", &date.format("%m").to_string());
            permalink = permalink.replace(":day", &date.format("%d").to_string());
            permalink = permalink.replace(":i_month", &date.format("%-m").to_string());
            permalink = permalink.replace(":i_day", &date.format("%-d").to_string());
        }

        if !permalink.starts_with('/') {
            permalink = format!("/{}", permalink);
        }

        if !permalink.ends_with('/') && !permalink.contains('.') {
            permalink = format!("{}/", permalink);
        }

        Ok(permalink)
    }

    /// 渲染集合项内容为 HTML
    ///
    /// # Arguments
    ///
    /// * `converter` - Markdown 转换器
    ///
    /// # Returns
    ///
    /// 返回渲染后的 HTML 字符串
    ///
    /// # Errors
    ///
    /// 返回 `CollectionError` 如果渲染失败
    pub fn render_content(
        &self,
        converter: &crate::jekyll::MarkdownConverter,
    ) -> Result<String, CollectionError> {
        converter
            .convert(&self.front_matter.content())
            .map_err(|e| CollectionError::FileParseError(format!("{:?}", e)))
    }

    /// 获取集合项的标题
    ///
    /// 优先从 Front Matter 的 `title` 字段获取，如果不存在则使用文件名
    ///
    /// # Returns
    ///
    /// 返回标题字符串
    pub fn title(&self) -> &str {
        self.front_matter.get_str("title").unwrap_or(&self.name)
    }

    /// 检查集合项是否已发布
    ///
    /// 检查 Front Matter 中的 `published` 字段，默认为 true
    ///
    /// # Returns
    ///
    /// 返回是否已发布
    pub fn is_published(&self) -> bool {
        self.front_matter.get_bool("published").unwrap_or(true)
    }

    /// 获取集合项的布局
    ///
    /// 优先从 Front Matter 的 `layout` 字段获取，如果不存在则使用集合配置的默认布局
    ///
    /// # Arguments
    ///
    /// * `config` - 集合配置
    ///
    /// # Returns
    ///
    /// 返回布局名称（如果有）
    pub fn layout(&self, config: &CollectionConfig) -> Option<String> {
        self.front_matter
            .get_str("layout")
            .map(|s| s.to_string())
            .or_else(|| config.layout.clone())
    }

    /// 将集合项转换为 JSON 值
    ///
    /// 用于在 Liquid 模板中使用
    ///
    /// # Returns
    ///
    /// 返回 JSON 值
    pub fn to_value(&self) -> Value {
        let mut map = serde_json::Map::new();

        map.insert("name".to_string(), Value::String(self.name.clone()));
        map.insert("title".to_string(), Value::String(self.title().to_string()));
        map.insert("slug".to_string(), Value::String(self.slug.clone()));
        map.insert(
            "permalink".to_string(),
            Value::String(self.permalink.clone()),
        );
        map.insert(
            "relative_path".to_string(),
            Value::String(self.relative_path.clone()),
        );
        map.insert(
            "collection".to_string(),
            Value::String(self.collection.clone()),
        );
        map.insert("url".to_string(), Value::String(self.permalink.clone()));

        if let Some(date) = self.date {
            map.insert(
                "date".to_string(),
                Value::String(date.to_string()),
            );
        }

        for (k, v) in &self.front_matter.variables {
            map.insert(k.clone(), v.clone());
        }

        Value::Object(map)
    }
}

/// 集合，表示一组相关的文档
#[derive(Debug, Clone, PartialEq)]
pub struct Collection {
    /// 集合名称
    pub name: String,

    /// 集合配置
    pub config: CollectionConfig,

    /// 集合中的所有项
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
    ///
    /// # Returns
    ///
    /// 返回新的集合实例
    pub fn new(name: String, config: CollectionConfig, directory: PathBuf) -> Self {
        Self {
            name,
            config,
            items: Vec::new(),
            directory,
        }
    }

    /// 加载集合中的所有项
    ///
    /// # Arguments
    ///
    /// * `site_config` - 站点配置
    ///
    /// # Returns
    ///
    /// 返回加载的集合项数量
    ///
    /// # Errors
    ///
    /// 返回 `CollectionError` 如果加载失败
    pub fn load_items(&mut self, site_config: &JekyllConfig) -> Result<usize, CollectionError> {
        let mut count = 0;

        if let Ok(entries) = std::fs::read_dir(&self.directory) {
            for entry in entries {
                let entry = entry?;
                let path = entry.path();

                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext == "md" || ext == "markdown" {
                            match CollectionItem::from_file(&path, &self.name, &self.config, site_config) {
                                Ok(item) => {
                                    if item.is_published() {
                                        self.items.push(item);
                                        count += 1;
                                    }
                                }
                                Err(e) => {
                                    eprintln!(
                                        "Error loading collection item {}: {:?}",
                                        path.to_string_lossy(),
                                        e
                                    );
                                }
                            }
                        }
                    }
                } else if path.is_dir() {
                    let sub_dir_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                    let sub_collection_name = format!("{}/{}", self.name, sub_dir_name);
                    let mut sub_config = self.config.clone();
                    sub_config.name = sub_collection_name.clone();

                    let mut sub_collection = Self::new(sub_collection_name, sub_config, path);

                    let sub_count = sub_collection.load_items(site_config)?;
                    count += sub_count;

                    self.items.extend(sub_collection.items);
                }
            }
        }

        self.sort_items();

        Ok(count)
    }

    /// 根据配置排序集合项
    pub fn sort_items(&mut self) {
        let sort_by = self.config.sort_by.as_deref().unwrap_or("name");
        let reverse = self.config.sort_reverse.unwrap_or(false);

        self.items.sort_by(|a, b| {
            let ordering = match sort_by {
                "date" => {
                    match (a.date, b.date) {
                        (Some(d1), Some(d2)) => d1.cmp(&d2),
                        (Some(_), None) => std::cmp::Ordering::Less,
                        (None, Some(_)) => std::cmp::Ordering::Greater,
                        (None, None) => a.name.cmp(&b.name),
                    }
                }
                "title" => a.title().cmp(b.title()),
                _ => a.name.cmp(&b.name),
            };

            if reverse {
                ordering.reverse()
            } else {
                ordering
            }
        });
    }

    /// 获取集合项的不可变引用
    ///
    /// # Returns
    ///
    /// 返回集合项的切片
    pub fn items(&self) -> &[CollectionItem] {
        &self.items
    }

    /// 获取集合项的可变引用
    ///
    /// # Returns
    ///
    /// 返回集合项的可变向量引用
    pub fn items_mut(&mut self) -> &mut Vec<CollectionItem> {
        &mut self.items
    }

    /// 根据名称获取集合项
    ///
    /// # Arguments
    ///
    /// * `name` - 集合项名称
    ///
    /// # Returns
    ///
    /// 返回匹配的集合项引用（如果存在）
    pub fn get_item_by_name(&self, name: &str) -> Option<&CollectionItem> {
        self.items.iter().find(|item| item.name == name)
    }

    /// 根据 slug 获取集合项
    ///
    /// # Arguments
    ///
    /// * `slug` - URL 友好的 slug
    ///
    /// # Returns
    ///
    /// 返回匹配的集合项引用（如果存在）
    pub fn get_item_by_slug(&self, slug: &str) -> Option<&CollectionItem> {
        self.items.iter().find(|item| item.slug == slug)
    }

    /// 使用自定义比较函数排序集合项
    ///
    /// # Arguments
    ///
    /// * `f` - 比较函数
    pub fn sort_items_by<F>(&mut self, f: F)
    where
        F: FnMut(&CollectionItem, &CollectionItem) -> std::cmp::Ordering,
    {
        self.items.sort_by(f);
    }

    /// 检查集合是否应该输出
    ///
    /// # Returns
    ///
    /// 返回是否应该输出集合
    pub fn should_output(&self) -> bool {
        self.config.output
    }

    /// 将集合转换为 JSON 值
    ///
    /// 用于在 Liquid 模板中使用
    ///
    /// # Returns
    ///
    /// 返回 JSON 值
    pub fn to_value(&self) -> Value {
        let mut map = serde_json::Map::new();

        map.insert("name".to_string(), Value::String(self.name.clone()));
        map.insert("output".to_string(), Value::Bool(self.config.output));

        let items_array: Vec<Value> = self.items.iter().map(|item| item.to_value()).collect();
        map.insert("items".to_string(), Value::Array(items_array.clone()));
        map.insert("docs".to_string(), Value::Array(items_array));

        Value::Object(map)
    }
}

/// 集合管理器，负责管理所有集合
#[derive(Debug)]
pub struct CollectionManager {
    /// 所有集合的映射
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
    ///
    /// # Returns
    ///
    /// 返回新的集合管理器实例
    pub fn new(structure: JekyllStructure, config: JekyllConfig) -> Self {
        Self {
            collections: HashMap::new(),
            structure,
            config,
        }
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
        self.parse_collections_from_config()?;

        for collection in self.collections.values_mut() {
            collection.load_items(&self.config)?;
        }

        Ok(self.collections.len())
    }

    /// 从配置中解析集合
    ///
    /// # Errors
    ///
    /// 返回 `CollectionError` 如果解析失败
    fn parse_collections_from_config(&mut self) -> Result<(), CollectionError> {
        if let Some(collections_config) = self.config.collections.as_ref() {
            for (name, collection_value) in collections_config {
                let config = CollectionConfig::from_value(name.clone(), collection_value)?;

                let collection_dir = if let Some(path) = &config.path {
                    self.structure.root().join(path)
                } else {
                    let collections_dir = self
                        .config
                        .collections_dir
                        .as_deref()
                        .unwrap_or("");
                    if collections_dir.is_empty() {
                        self.structure.root().join(format!("_{}", name))
                    } else {
                        self.structure
                            .root()
                            .join(collections_dir)
                            .join(name)
                    }
                };

                if collection_dir.exists() && collection_dir.is_dir() {
                    let collection = Collection::new(name.clone(), config, collection_dir);
                    self.collections.insert(name.clone(), collection);
                }
            }
        }

        Ok(())
    }

    /// 获取所有集合的不可变引用
    ///
    /// # Returns
    ///
    /// 返回集合映射的引用
    pub fn collections(&self) -> &HashMap<String, Collection> {
        &self.collections
    }

    /// 根据名称获取集合
    ///
    /// # Arguments
    ///
    /// * `name` - 集合名称
    ///
    /// # Returns
    ///
    /// 返回集合的引用（如果存在）
    pub fn get_collection(&self, name: &str) -> Option<&Collection> {
        self.collections.get(name)
    }

    /// 根据名称获取集合的可变引用
    ///
    /// # Arguments
    ///
    /// * `name` - 集合名称
    ///
    /// # Returns
    ///
    /// 返回集合的可变引用（如果存在）
    pub fn get_collection_mut(&mut self, name: &str) -> Option<&mut Collection> {
        self.collections.get_mut(name)
    }

    /// 检查集合是否存在
    ///
    /// # Arguments
    ///
    /// * `name` - 集合名称
    ///
    /// # Returns
    ///
    /// 返回集合是否存在
    pub fn has_collection(&self, name: &str) -> bool {
        self.collections.contains_key(name)
    }

    /// 清除所有集合
    pub fn clear(&mut self) {
        self.collections.clear();
    }

    /// 将所有集合转换为 JSON 值
    ///
    /// 用于在 Liquid 模板中使用
    ///
    /// # Returns
    ///
    /// 返回 JSON 值映射
    pub fn to_value_map(&self) -> HashMap<String, Value> {
        let mut map = HashMap::new();

        for (name, collection) in &self.collections {
            map.insert(name.clone(), collection.to_value());
        }

        map
    }
}

impl Clone for CollectionManager {
    fn clone(&self) -> Self {
        Self {
            collections: self.collections.clone(),
            structure: self.structure.clone(),
            config: self.config.clone(),
        }
    }
}
