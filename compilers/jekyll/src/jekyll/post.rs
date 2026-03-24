#![warn(missing_docs)]

//! Jekyll 帖子管理模块
//!
//! 提供 Jekyll 博客帖子的加载、解析和管理功能

use chrono::{DateTime, Datelike, NaiveDate, Utc};
use serde_json::Value;

use crate::errors::{JekyllError, PostError, Result};

use super::{FrontMatterParser, JekyllConfig, JekyllStructure, front_matter::FrontMatter};

/// Jekyll 博客帖子
#[derive(Debug, Clone)]
pub struct Post {
    /// 帖子标题
    pub title: String,
    /// 帖子 slug
    pub slug: String,
    /// 发布日期
    pub date: NaiveDate,
    /// 发布时间
    pub datetime: Option<DateTime<Utc>>,
    /// 分类列表
    pub categories: Vec<String>,
    /// 标签列表
    pub tags: Vec<String>,
    /// 帖子内容
    pub content: String,
    /// 摘要
    pub excerpt: Option<String>,
    /// Front Matter 变量
    pub front_matter: Value,
    /// 永久链接
    pub permalink: String,
    /// 布局名称
    pub layout: Option<String>,
    /// 原始文件路径
    pub path: String,
    /// 是否为草稿
    pub draft: bool,
    /// 上一篇帖子
    pub previous: Option<Box<Post>>,
    /// 下一篇帖子
    pub next: Option<Box<Post>>,
}

impl Post {
    /// 从文件创建帖子
    ///
    /// # Arguments
    ///
    /// * `path` - 帖子文件路径
    /// * `config` - Jekyll 配置
    /// * `is_draft` - 是否为草稿
    ///
    /// # Returns
    ///
    /// 返回帖子或错误
    pub fn from_file<P: AsRef<std::path::Path>>(path: P, config: &JekyllConfig, is_draft: bool) -> Result<Self> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path).map_err(JekyllError::from)?;

        let front_matter = FrontMatterParser::parse(&content)?;

        let (title_from_filename, date_from_filename) = Self::parse_filename(path)?;

        let title = front_matter.get_str("title").map(|s| s.to_string()).unwrap_or(title_from_filename);

        let date_str = front_matter.get_str("date").unwrap_or("");
        let date = if !date_str.is_empty() {
            NaiveDate::parse_from_str(date_str, "%Y-%m-%d").map_err(|e| PostError::DateParseError(e.to_string()))?
        }
        else {
            date_from_filename
        };

        let categories = Self::extract_categories(&front_matter, path);
        let tags = Self::extract_tags(&front_matter);

        let slug = Self::slugify(&title);

        let permalink = Self::generate_permalink(&title, &date, &categories, config)?;

        let layout = front_matter.get_str("layout").map(|s| s.to_string());
        
        // 提取摘要
        let excerpt = Self::extract_excerpt(front_matter.content());

        Ok(Self {
            title,
            slug,
            date,
            datetime: None,
            categories,
            tags,
            content: front_matter.content().to_string(),
            excerpt,
            front_matter: front_matter.variables().clone(),
            permalink,
            layout,
            path: path.display().to_string(),
            draft: is_draft,
            previous: None,
            next: None,
        })
    }

    /// 解析帖子文件名
    ///
    /// Jekyll 帖子文件名格式：YYYY-MM-DD-title.md
    ///
    /// # Arguments
    ///
    /// * `path` - 文件路径
    ///
    /// # Returns
    ///
    /// 返回 (标题, 日期) 或错误
    pub fn parse_filename<P: AsRef<std::path::Path>>(path: P) -> Result<(String, NaiveDate)> {
        let path = path.as_ref();
        let filename = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| PostError::InvalidPostFilename(path.display().to_string()))?;

        let parts: Vec<&str> = filename.splitn(4, '-').collect();

        if parts.len() < 4 {
            return Err(PostError::InvalidPostFilename(format!(
                "Post filename must be in format YYYY-MM-DD-title.md, got: {}",
                filename
            ))
            .into());
        }

        let year: i32 = parts[0].parse().map_err(|_| PostError::DateParseError(format!("Invalid year: {}", parts[0])))?;
        let month: u32 = parts[1].parse().map_err(|_| PostError::DateParseError(format!("Invalid month: {}", parts[1])))?;
        let day: u32 = parts[2].parse().map_err(|_| PostError::DateParseError(format!("Invalid day: {}", parts[2])))?;

        let date = NaiveDate::from_ymd_opt(year, month, day)
            .ok_or_else(|| PostError::DateParseError(format!("Invalid date: {}-{}-{}", year, month, day)))?;

        let title = parts[3].replace('-', " ");

        Ok((title, date))
    }

    /// 从 front matter 和路径提取分类
    ///
    /// # Arguments
    ///
    /// * `front_matter` - Front Matter 解析结果
    /// * `path` - 文件路径
    ///
    /// # Returns
    ///
    /// 返回分类列表
    pub fn extract_categories(front_matter: &FrontMatter, path: &std::path::Path) -> Vec<String> {
        let mut categories = Vec::new();

        if let Some(fm_categories) = front_matter.get("categories") {
            match fm_categories {
                Value::String(s) => {
                    // 处理字符串类型的分类
                    categories.push(s.clone());
                }
                Value::Array(arr) => {
                    // 处理数组类型的分类
                    for item in arr {
                        if let Value::String(s) = item {
                            categories.push(s.clone());
                        }
                    }
                }
                Value::Object(obj) => {
                    // 处理对象类型的分类（可能是解析错误）
                    for (key, value) in obj {
                        if let Value::String(s) = value {
                            categories.push(s.clone());
                        }
                    }
                }
                _ => {
                    // 其他类型，尝试转换为字符串
                    categories.push(fm_categories.to_string());
                }
            }
        }

        // 过滤掉空字符串
        categories = categories.into_iter().filter(|s| !s.is_empty()).collect();

        if categories.is_empty() {
            // 从路径中提取所有父目录作为分类
            let mut current = path.parent();
            let mut path_categories = Vec::new();

            while let Some(parent) = current {
                if let Some(dir_name) = parent.file_name().and_then(|n| n.to_str()) {
                    if dir_name == "_posts" {
                        break;
                    }
                    if !dir_name.starts_with('_') {
                        path_categories.push(dir_name.to_string());
                    }
                }
                current = parent.parent();
            }

            // 反转顺序，因为我们是从子目录向上遍历的
            path_categories.reverse();
            categories.extend(path_categories);
        }

        categories
    }

    /// 从 front matter 提取标签
    ///
    /// # Arguments
    ///
    /// * `front_matter` - Front Matter 解析结果
    ///
    /// # Returns
    ///
    /// 返回标签列表
    pub fn extract_tags(front_matter: &FrontMatter) -> Vec<String> {
        let mut tags = Vec::new();

        if let Some(fm_tags) = front_matter.get("tags") {
            match fm_tags {
                Value::String(s) => {
                    for tag in s.split(',') {
                        tags.push(tag.trim().to_string());
                    }
                }
                Value::Array(arr) => {
                    for item in arr {
                        if let Value::String(s) = item {
                            tags.push(s.clone());
                        }
                    }
                }
                _ => {}
            }
        }

        tags
    }

    /// 生成永久链接
    ///
    /// # Arguments
    ///
    /// * `title` - 帖子标题
    /// * `date` - 发布日期
    /// * `categories` - 分类列表
    /// * `config` - Jekyll 配置
    ///
    /// # Returns
    ///
    /// 返回永久链接或错误
    pub fn generate_permalink(title: &str, date: &NaiveDate, categories: &[String], config: &JekyllConfig) -> Result<String> {
        let slug = Self::slugify(title);

        let format = config.permalink.as_deref().unwrap_or("/:categories/:year/:month/:day/:title/");

        let mut permalink = format.to_string();

        permalink = permalink.replace(":title", &slug);
        permalink = permalink.replace(":year", &date.format("%Y").to_string());
        permalink = permalink.replace(":month", &date.format("%m").to_string());
        permalink = permalink.replace(":day", &date.format("%d").to_string());

        let categories_path = if categories.is_empty() { String::new() } else { categories.join("/") };
        permalink = permalink.replace(":categories", &categories_path);

        if permalink.starts_with('/') { Ok(permalink) } else { Ok(format!("/{}", permalink)) }
    }

    /// 将标题转换为 slug
    ///
    /// # Arguments
    ///
    /// * `title` - 标题
    ///
    /// # Returns
    ///
    /// 返回 slug
    pub fn slugify(title: &str) -> String {
        let slug = slug::slugify(title);
        slug.to_lowercase()
    }

    /// 获取发布年份
    pub fn year(&self) -> i32 {
        self.date.year()
    }

    /// 获取发布月份
    pub fn month(&self) -> u32 {
        self.date.month()
    }

    /// 获取发布日
    pub fn day(&self) -> u32 {
        self.date.day()
    }

    /// 检查是否包含指定标签
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t.eq_ignore_ascii_case(tag))
    }

    /// 检查是否属于指定分类
    pub fn has_category(&self, category: &str) -> bool {
        self.categories.iter().any(|c| c.eq_ignore_ascii_case(category))
    }

    /// 提取摘要
    ///
    /// # Arguments
    ///
    /// * `content` - 帖子内容
    ///
    /// # Returns
    ///
    /// 返回摘要或 None
    pub fn extract_excerpt(content: &str) -> Option<String> {
        // 检查是否有 <!-- more --> 分隔符
        if let Some(index) = content.find("<!-- more -->") {
            let excerpt = content[0..index].trim().to_string();
            if !excerpt.is_empty() {
                return Some(excerpt);
            }
        }

        // 否则，提取前 200 个字符作为摘要
        let trimmed = content.trim();
        if trimmed.len() > 200 {
            let excerpt = trimmed[0..200].to_string() + "...";
            Some(excerpt)
        } else if !trimmed.is_empty() {
            Some(trimmed.to_string())
        } else {
            None
        }
    }
}

/// 帖子管理器
#[derive(Debug)]
pub struct PostManager {
    /// Jekyll 目录结构
    structure: JekyllStructure,
    /// Jekyll 配置
    config: JekyllConfig,
    /// 已加载的帖子列表
    posts: Vec<Post>,
    /// 是否包含草稿
    include_drafts: bool,
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
    /// 返回帖子管理器实例
    pub fn new(structure: JekyllStructure, config: JekyllConfig) -> Self {
        Self { structure, config, posts: Vec::new(), include_drafts: false }
    }

    /// 设置是否包含草稿
    pub fn with_drafts(mut self, include: bool) -> Self {
        self.include_drafts = include;
        self
    }

    /// 加载所有帖子
    ///
    /// # Returns
    ///
    /// 返回加载的帖子数量或错误
    pub fn load_posts(&mut self) -> Result<usize> {
        let mut count = 0;

        {
            if let Some(posts_dir) = self.structure.posts_dir() {
                let posts_dir = posts_dir.to_path_buf();
                count += self.load_posts_from_dir(&posts_dir, false)?;
            }
        }

        if self.include_drafts {
            {
                if let Some(drafts_dir) = self.structure.drafts_dir() {
                    let drafts_dir = drafts_dir.to_path_buf();
                    count += self.load_posts_from_dir(&drafts_dir, true)?;
                }
            }
        }

        self.sort_posts();

        Ok(count)
    }

    /// 从目录加载帖子
    fn load_posts_from_dir<P: AsRef<std::path::Path>>(&mut self, dir: P, is_draft: bool) -> Result<usize> {
        let dir = dir.as_ref();
        let mut count = 0;

        if !dir.exists() {
            return Ok(0);
        }

        for entry in walkdir::WalkDir::new(dir).into_iter().filter_map(|e| e.ok()).filter(|e| e.file_type().is_file()) {
            let path = entry.path();

            if let Some(ext) = path.extension() {
                if ext == "md" || ext == "markdown" || ext == "html" {
                    match Post::from_file(path, &self.config, is_draft) {
                        Ok(post) => {
                            self.posts.push(post);
                            count += 1;
                        }
                        Err(e) => {
                            eprintln!("Warning: Failed to load post {}: {}", path.display(), e);
                        }
                    }
                }
            }
        }

        Ok(count)
    }

    /// 对帖子按日期排序（降序）
    fn sort_posts(&mut self) {
        self.posts.sort_by(|a, b| b.date.cmp(&a.date));
        self.set_previous_next_posts();
    }

    /// 设置前一篇和后一篇帖子
    fn set_previous_next_posts(&mut self) {
        for i in 0..self.posts.len() {
            // 设置前一篇帖子
            if i > 0 {
                self.posts[i].previous = Some(Box::new(self.posts[i-1].clone()));
            }
            
            // 设置后一篇帖子
            if i < self.posts.len() - 1 {
                self.posts[i].next = Some(Box::new(self.posts[i+1].clone()));
            }
        }
    }

    /// 获取所有帖子
    pub fn posts(&self) -> &[Post] {
        &self.posts
    }

    /// 获取已发布的帖子
    pub fn published_posts(&self) -> Vec<&Post> {
        self.posts.iter().filter(|p| !p.draft).collect()
    }

    /// 获取草稿帖子
    pub fn draft_posts(&self) -> Vec<&Post> {
        self.posts.iter().filter(|p| p.draft).collect()
    }

    /// 获取指定分类的帖子
    pub fn get_posts_by_category(&self, category: &str) -> Vec<&Post> {
        self.posts.iter().filter(|p| p.has_category(category)).collect()
    }

    /// 获取指定标签的帖子
    pub fn get_posts_by_tag(&self, tag: &str) -> Vec<&Post> {
        self.posts.iter().filter(|p| p.has_tag(tag)).collect()
    }

    /// 获取最新的 N 篇帖子
    pub fn get_latest_posts(&self, n: usize) -> Vec<&Post> {
        self.posts.iter().take(n).collect()
    }

    /// 获取所有分类
    pub fn all_categories(&self) -> Vec<String> {
        let mut categories: Vec<String> = self.posts.iter().flat_map(|p| p.categories.clone()).collect();
        categories.sort();
        categories.dedup();
        categories
    }

    /// 获取所有标签
    pub fn all_tags(&self) -> Vec<String> {
        let mut tags: Vec<String> = self.posts.iter().flat_map(|p| p.tags.clone()).collect();
        tags.sort();
        tags.dedup();
        tags
    }

    /// 获取帖子数量
    pub fn count(&self) -> usize {
        self.posts.len()
    }

    /// 清除所有帖子
    pub fn clear(&mut self) {
        self.posts.clear();
    }

    /// 按年份和月份分组帖子
    pub fn posts_by_year_month(&self) -> std::collections::HashMap<i32, std::collections::HashMap<u32, Vec<&Post>>> {
        let mut result = std::collections::HashMap::new();

        for post in &self.posts {
            let year = post.date.year();
            let month = post.date.month();

            let month_map = result.entry(year).or_insert(std::collections::HashMap::new());
            month_map.entry(month).or_insert(Vec::new()).push(post);
        }

        result
    }

    /// 按标签分组帖子
    pub fn posts_by_tag(&self) -> std::collections::HashMap<String, Vec<&Post>> {
        let mut result = std::collections::HashMap::new();

        for post in &self.posts {
            for tag in &post.tags {
                result.entry(tag.clone()).or_insert(Vec::new()).push(post);
            }
        }

        result
    }

    /// 按分类分组帖子
    pub fn posts_by_category(&self) -> std::collections::HashMap<String, Vec<&Post>> {
        let mut result = std::collections::HashMap::new();

        for post in &self.posts {
            for category in &post.categories {
                result.entry(category.clone()).or_insert(Vec::new()).push(post);
            }
        }

        result
    }

    /// 搜索帖子
    pub fn search_posts(&self, query: &str) -> Vec<&Post> {
        let query_lower = query.to_lowercase();
        self.posts
            .iter()
            .filter(|post| {
                post.title.to_lowercase().contains(&query_lower)
                    || post.content.to_lowercase().contains(&query_lower)
                    || post.tags.iter().any(|tag| tag.to_lowercase().contains(&query_lower))
                    || post.categories.iter().any(|cat| cat.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    /// 分页获取帖子
    pub fn paginate_posts(&self, page: usize, per_page: usize) -> Vec<&Post> {
        let start = (page - 1) * per_page;
        let end = start + per_page;
        self.posts.iter().skip(start).take(per_page).collect()
    }

    /// 获取帖子总数
    pub fn total_posts(&self) -> usize {
        self.posts.len()
    }

    /// 获取总页数
    pub fn total_pages(&self, per_page: usize) -> usize {
        (self.posts.len() + per_page - 1) / per_page
    }
}
