#![warn(missing_docs)]

//! Jekyll 帖子管理模块
//!
//! 提供 Jekyll 博客帖子的加载、解析和管理功能

use chrono::{DateTime, NaiveDate, Utc};
use serde_json::Value;

use crate::errors::{JekyllError, PostError, Result};

use super::{FrontMatter, FrontMatterParser, JekyllConfig, JekyllStructure};

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

        let title = front_matter
            .get_str("title")
            .map(|s| s.to_string())
            .unwrap_or(title_from_filename);

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

        Ok(Self {
            title,
            slug,
            date,
            datetime: None,
            categories,
            tags,
            content: front_matter.content().to_string(),
            front_matter: front_matter.variables().clone(),
            permalink,
            layout,
            path: path.display().to_string(),
            draft: is_draft,
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
        let filename = path.file_stem().and_then(|s| s.to_str()).ok_or_else(|| {
            PostError::InvalidFilename(path.display().to_string())
        })?;

        let parts: Vec<&str> = filename.splitn(4, '-').collect();

        if parts.len() < 4 {
            return Err(PostError::InvalidFilename(format!(
                "Post filename must be in format YYYY-MM-DD-title.md, got: {}",
                filename
            ))
            .into());
        }

        let year: i32 = parts[0]
            .parse()
            .map_err(|_| PostError::DateParseError(format!("Invalid year: {}", parts[0])))?;
        let month: u32 = parts[1]
            .parse()
            .map_err(|_| PostError::DateParseError(format!("Invalid month: {}", parts[1])))?;
        let day: u32 = parts[2]
            .parse()
            .map_err(|_| PostError::DateParseError(format!("Invalid day: {}", parts[2])))?;

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
    pub fn extract_categories(front_matter: &super::FrontMatter, path: &std::path::Path) -> Vec<String> {
        let mut categories = Vec::new();

        if let Some(fm_categories) = front_matter.get("categories") {
            match fm_categories {
                Value::String(s) => {
                    categories.push(s.clone());
                }
                Value::Array(arr) => {
                    for item in arr {
                        if let Value::String(s) = item {
                            categories.push(s.clone());
                        }
                    }
                }
                _ => {}
            }
        }

        if categories.is_empty() {
            if let Some(parent) = path.parent() {
                if let Some(dir_name) = parent.file_name().and_then(|n| n.to_str()) {
                    if dir_name != "_posts" && !dir_name.starts_with('_') {
                        categories.push(dir_name.to_string());
                    }
                }
            }
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
    pub fn extract_tags(front_matter: &super::FrontMatter) -> Vec<String> {
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

        let categories_path = if categories.is_empty() {
            String::new()
        }
        else {
            categories.join("/")
        };
        permalink = permalink.replace(":categories", &categories_path);

        if permalink.starts_with('/') {
            Ok(permalink)
        }
        else {
            Ok(format!("/{}", permalink))
        }
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

        if let Some(posts_dir) = self.structure.posts_dir() {
            count += self.load_posts_from_dir(posts_dir, false)?;
        }

        if self.include_drafts {
            if let Some(drafts_dir) = self.structure.drafts_dir() {
                count += self.load_posts_from_dir(drafts_dir, true)?;
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

        for entry in walkdir::WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_parse_filename() {
        let temp_dir = tempdir().unwrap();
        let post_path = temp_dir.path().join("2024-01-01-test-post.md");

        let (title, date) = Post::parse_filename(&post_path).unwrap();
        assert_eq!(title, "test post");
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), 1);
        assert_eq!(date.day(), 1);
    }

    #[test]
    fn test_slugify() {
        let title = "Test Post With Special Characters!";
        let slug = Post::slugify(title);
        assert_eq!(slug, "test-post-with-special-characters");
    }

    #[test]
    fn test_generate_permalink() {
        let config = JekyllConfig::new().with_permalink("/:categories/:year/:month/:day/:title/".to_string());
        let title = "Test Post";
        let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let categories = vec!["programming".to_string(), "rust".to_string()];

        let permalink = Post::generate_permalink(title, &date, &categories, &config).unwrap();
        assert_eq!(permalink, "/programming/rust/2024/01/01/test-post/");
    }

    #[test]
    fn test_post_manager() {
        let temp_dir = tempdir().unwrap();

        let posts_dir = temp_dir.path().join("_posts");
        fs::create_dir_all(&posts_dir).unwrap();

        let post_content1 = r#"---
title: First Post
categories: programming
---
First post content."#;
        fs::write(posts_dir.join("2024-01-01-first-post.md"), post_content1).unwrap();

        let post_content2 = r#"---
title: Second Post
categories: programming
---
Second post content."#;
        fs::write(posts_dir.join("2024-01-02-second-post.md"), post_content2).unwrap();

        let structure = JekyllStructure::new(temp_dir.path()).unwrap();
        let config = JekyllConfig::new();

        let mut manager = PostManager::new(structure, config);
        let count = manager.load_posts().unwrap();

        assert_eq!(count, 2);
        assert_eq!(manager.posts().len(), 2);

        assert!(manager.posts()[0].date >= manager.posts()[1].date);

        let programming_posts = manager.get_posts_by_category("programming");
        assert_eq!(programming_posts.len(), 2);
    }
}
