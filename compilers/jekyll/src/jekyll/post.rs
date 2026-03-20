//! 帖子（Posts）处理模块
//!
//! 该模块提供 Jekyll 帖子的处理功能，包括：
//! - 帖子文件名日期解析（YYYY-MM-DD-title.md）
//! - 帖子的排序和分组
//! - 帖子的永久链接（permalink）生成
//! - 帖子的元数据处理

use crate::{
    errors::PostError,
    jekyll::{FrontMatter, JekyllConfig, JekyllError, JekyllStructure, MarkdownConverter},
};
use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike};
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{BTreeMap, HashMap},
    path::{Path, PathBuf},
};

/// 帖子结构体
#[derive(Debug, Clone, PartialEq)]
pub struct Post {
    /// 帖子文件路径
    pub path: PathBuf,
    /// 帖子标题
    pub title: String,
    /// 帖子日期
    pub date: NaiveDate,
    /// 帖子内容
    pub content: String,
    /// 前置内容
    pub front_matter: FrontMatter,
    /// 永久链接
    pub permalink: String,
    /// 分类
    pub categories: Vec<String>,
    /// 标签
    pub tags: Vec<String>,
    /// 布局
    pub layout: Option<String>,
    /// 相对路径
    pub relative_path: String,
}

impl Post {
    /// 从文件创建帖子
    ///
    /// # Arguments
    ///
    /// * `post_path` - 帖子文件路径
    /// * `config` - Jekyll 配置
    ///
    /// # Returns
    ///
    /// 返回创建的 Post 实例
    ///
    /// # Errors
    ///
    /// 返回 `PostError` 如果文件读取或解析失败
    pub fn from_file<P: AsRef<Path>>(post_path: P, config: &JekyllConfig) -> Result<Self, PostError> {
        let post_path = post_path.as_ref();
        let relative_path = post_path.to_string_lossy().to_string();

        // 解析文件名
        let (title, date) = Self::parse_filename(post_path)?;

        // 读取文件内容
        let content = std::fs::read_to_string(post_path)?;

        // 解析 Front Matter
        let front_matter = crate::jekyll::FrontMatterParser::parse(&content)?;

        // 提取分类和标签
        let categories = Self::extract_categories(&front_matter, post_path);
        let tags = Self::extract_tags(&front_matter);

        // 提取布局
        let layout = front_matter.get("layout").and_then(|v| v.as_str()).map(|s| s.to_string());

        // 生成永久链接
        let permalink = Self::generate_permalink(&title, &date, &categories, config)?;

        Ok(Self {
            path: post_path.to_path_buf(),
            title: title.to_string(),
            date,
            content,
            front_matter,
            permalink,
            categories,
            tags,
            layout,
            relative_path,
        })
    }

    /// 解析帖子文件名
    ///
    /// # Arguments
    ///
    /// * `path` - 帖子文件路径
    ///
    /// # Returns
    ///
    /// 返回 (标题, 日期) 元组
    ///
    /// # Errors
    ///
    /// 返回 `PostError` 如果文件名格式无效
    fn parse_filename(path: &Path) -> Result<(String, NaiveDate), PostError> {
        lazy_static! {
            static ref POST_FILENAME_REGEX: Regex =
                Regex::new(r"^(\d{4})-(\d{2})-(\d{2})-(.+?)\.\w+$").expect("Invalid regex pattern");
        }

        let filename = path.file_name().unwrap_or_default().to_string_lossy();

        if let Some(captures) = POST_FILENAME_REGEX.captures(&filename) {
            let year = captures[1].parse::<i32>().map_err(|e| PostError::DateParseError(e.to_string()))?;
            let month = captures[2].parse::<u32>().map_err(|e| PostError::DateParseError(e.to_string()))?;
            let day = captures[3].parse::<u32>().map_err(|e| PostError::DateParseError(e.to_string()))?;
            let title = captures[4].replace('-', " ");

            let date = NaiveDate::from_ymd_opt(year, month, day)
                .ok_or_else(|| PostError::DateParseError("Invalid date".to_string()))?;

            Ok((title, date))
        }
        else {
            Err(PostError::InvalidFilename(filename.to_string()))
        }
    }

    /// 提取分类
    ///
    /// # Arguments
    ///
    /// * `front_matter` - 前置内容
    /// * `path` - 帖子文件路径
    ///
    /// # Returns
    ///
    /// 返回分类列表
    fn extract_categories(front_matter: &FrontMatter, path: &Path) -> Vec<String> {
        // 优先从 Front Matter 中提取
        if let Some(categories) = front_matter.get("categories") {
            if let Some(cat_array) = categories.as_array() {
                return cat_array.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect();
            }
            else if let Some(cat_str) = categories.as_str() {
                return cat_str.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
            }
        }

        // 从路径中提取分类（Jekyll 风格）
        let mut categories = Vec::new();
        let mut current_path = path.parent().unwrap_or(path);

        // 向上遍历目录，直到找到 _posts 目录
        while let Some(parent) = current_path.parent() {
            if parent.file_name().unwrap_or_default() == "_posts" {
                break;
            }
            if let Some(dir_name) = current_path.file_name() {
                categories.insert(0, dir_name.to_string_lossy().to_string());
            }
            current_path = parent;
        }

        categories
    }

    /// 提取标签
    ///
    /// # Arguments
    ///
    /// * `front_matter` - 前置内容
    ///
    /// # Returns
    ///
    /// 返回标签列表
    fn extract_tags(front_matter: &FrontMatter) -> Vec<String> {
        if let Some(tags) = front_matter.get("tags") {
            if let Some(tag_array) = tags.as_array() {
                return tag_array.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect();
            }
            else if let Some(tag_str) = tags.as_str() {
                return tag_str.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
            }
        }

        Vec::new()
    }

    /// 生成永久链接
    ///
    /// # Arguments
    ///
    /// * `title` - 帖子标题
    /// * `date` - 帖子日期
    /// * `categories` - 分类列表
    /// * `config` - Jekyll 配置
    ///
    /// # Returns
    ///
    /// 返回生成的永久链接
    ///
    /// # Errors
    ///
    /// 返回 `PostError` 如果永久链接生成失败
    fn generate_permalink(
        title: &str,
        date: &NaiveDate,
        categories: &[String],
        config: &JekyllConfig,
    ) -> Result<String, PostError> {
        let permalink_format = config.permalink.as_deref().unwrap_or("/:categories/:year/:month/:day/:title/");

        let mut permalink = permalink_format.to_string();

        // 替换类别
        let categories_str = categories.join("/");
        permalink = permalink.replace(":categories", &categories_str);

        // 替换日期部分
        permalink = permalink.replace(":year", &date.year().to_string());
        permalink = permalink.replace(":month", &format!("{:02}", date.month()));
        permalink = permalink.replace(":day", &format!("{:02}", date.day()));

        // 替换标题（slug 化）
        let slugified_title = Self::slugify(title);
        permalink = permalink.replace(":title", &slugified_title);

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

    /// 将字符串转换为 slug
    ///
    /// # Arguments
    ///
    /// * `text` - 要转换的文本
    ///
    /// # Returns
    ///
    /// 返回 slug 化的字符串
    fn slugify(text: &str) -> String {
        text.to_lowercase().replace(|c: char| !c.is_alphanumeric() && c != ' ', "-").trim_matches('-').to_string()
    }

    /// 渲染帖子内容
    ///
    /// # Arguments
    ///
    /// * `converter` - Markdown 转换器
    ///
    /// # Returns
    ///
    /// 返回渲染后的 HTML
    pub fn render_content(&self, converter: &MarkdownConverter) -> Result<String, PostError> {
        converter
            .convert(&self.front_matter.content())
            .map_err(|e| PostError::JekyllError(JekyllError::FrontMatterParseError(e.to_string())))
    }

    /// 获取帖子的最后修改时间
    ///
    /// # Returns
    ///
    /// 返回最后修改时间
    pub fn last_modified(&self) -> Result<NaiveDateTime, PostError> {
        let metadata = std::fs::metadata(&self.path)?;
        let modified = metadata.modified()?;
        let modified: chrono::DateTime<chrono::Local> = chrono::DateTime::from(modified);
        Ok(modified.naive_local())
    }
}

/// 帖子管理器
pub struct PostManager {
    /// Jekyll 目录结构
    structure: JekyllStructure,
    /// Jekyll 配置
    config: JekyllConfig,
    /// 帖子列表
    posts: Vec<Post>,
    /// 按日期分组的帖子
    posts_by_date: BTreeMap<NaiveDate, Vec<Post>>,
    /// 按分类分组的帖子
    posts_by_category: HashMap<String, Vec<Post>>,
    /// 按标签分组的帖子
    posts_by_tag: HashMap<String, Vec<Post>>,
}

impl PostManager {
    /// 创建新的帖子管理器
    ///
    /// # Arguments
    ///
    /// * `structure` - Jekyll 目录结构
    /// * `config` - Jekyll 配置
    ///
    /// # Returns
    ///
    /// 返回新创建的 PostManager 实例
    pub fn new(structure: JekyllStructure, config: JekyllConfig) -> Self {
        Self {
            structure,
            config,
            posts: Vec::new(),
            posts_by_date: BTreeMap::new(),
            posts_by_category: HashMap::new(),
            posts_by_tag: HashMap::new(),
        }
    }

    /// 加载所有帖子
    ///
    /// # Returns
    ///
    /// 返回加载的帖子数量
    ///
    /// # Errors
    ///
    /// 返回 `PostError` 如果加载失败
    pub fn load_posts(&mut self) -> Result<usize, PostError> {
        // 收集所有 Markdown 文件
        let markdown_files = self.structure.collect_markdown_files()?;

        // 过滤出帖子文件
        let post_files = markdown_files
            .into_iter()
            .filter(|path| {
                // 检查路径是否在 _posts 目录中
                let mut current_path = path.parent().unwrap_or(path);
                while let Some(parent) = current_path.parent() {
                    if parent.file_name().unwrap_or_default() == "_posts" {
                        return true;
                    }
                    current_path = parent;
                }
                false
            })
            .collect::<Vec<_>>();

        // 加载每个帖子
        for post_path in post_files {
            match Post::from_file(&post_path, &self.config) {
                Ok(post) => {
                    self.posts.push(post);
                }
                Err(e) => {
                    // 记录错误但继续处理其他帖子
                    eprintln!("Error loading post {}: {:?}", post_path.to_string_lossy(), e);
                }
            }
        }

        // 排序帖子（按日期降序）
        self.posts.sort_by(|a, b| b.date.cmp(&a.date));

        // 分组帖子
        self.group_posts();

        Ok(self.posts.len())
    }

    /// 分组帖子
    fn group_posts(&mut self) {
        // 按日期分组
        for post in &self.posts {
            self.posts_by_date.entry(post.date).or_default().push(post.clone());
        }

        // 按分类分组
        for post in &self.posts {
            for category in &post.categories {
                self.posts_by_category.entry(category.clone()).or_default().push(post.clone());
            }
        }

        // 按标签分组
        for post in &self.posts {
            for tag in &post.tags {
                self.posts_by_tag.entry(tag.clone()).or_default().push(post.clone());
            }
        }
    }

    /// 获取所有帖子
    pub fn posts(&self) -> &[Post] {
        &self.posts
    }

    /// 获取按日期分组的帖子
    pub fn posts_by_date(&self) -> &BTreeMap<NaiveDate, Vec<Post>> {
        &self.posts_by_date
    }

    /// 获取按分类分组的帖子
    pub fn posts_by_category(&self) -> &HashMap<String, Vec<Post>> {
        &self.posts_by_category
    }

    /// 获取按标签分组的帖子
    pub fn posts_by_tag(&self) -> &HashMap<String, Vec<Post>> {
        &self.posts_by_tag
    }

    /// 根据分类获取帖子
    ///
    /// # Arguments
    ///
    /// * `category` - 分类名称
    ///
    /// # Returns
    ///
    /// 返回该分类的帖子列表
    pub fn get_posts_by_category(&self, category: &str) -> Option<&Vec<Post>> {
        self.posts_by_category.get(category)
    }

    /// 根据标签获取帖子
    ///
    /// # Arguments
    ///
    /// * `tag` - 标签名称
    ///
    /// # Returns
    ///
    /// 返回该标签的帖子列表
    pub fn get_posts_by_tag(&self, tag: &str) -> Option<&Vec<Post>> {
        self.posts_by_tag.get(tag)
    }

    /// 获取最新的帖子
    ///
    /// # Arguments
    ///
    /// * `limit` - 限制数量
    ///
    /// # Returns
    ///
    /// 返回最新的帖子列表
    pub fn get_latest_posts(&self, limit: usize) -> &[Post] {
        if limit >= self.posts.len() { &self.posts } else { &self.posts[..limit] }
    }

    /// 搜索帖子
    ///
    /// # Arguments
    ///
    /// * `query` - 搜索查询
    ///
    /// # Returns
    ///
    /// 返回匹配的帖子列表
    pub fn search_posts(&self, query: &str) -> Vec<&Post> {
        let query_lower = query.to_lowercase();
        self.posts
            .iter()
            .filter(|post| {
                post.title.to_lowercase().contains(&query_lower)
                    || post.content.to_lowercase().contains(&query_lower)
                    || post.categories.iter().any(|c| c.to_lowercase().contains(&query_lower))
                    || post.tags.iter().any(|t| t.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    /// 清除所有帖子
    pub fn clear(&mut self) {
        self.posts.clear();
        self.posts_by_date.clear();
        self.posts_by_category.clear();
        self.posts_by_tag.clear();
    }

    /// 获取 Jekyll 目录结构
    pub fn structure(&self) -> &JekyllStructure {
        &self.structure
    }

    /// 获取 Jekyll 配置
    pub fn config(&self) -> &JekyllConfig {
        &self.config
    }
}
