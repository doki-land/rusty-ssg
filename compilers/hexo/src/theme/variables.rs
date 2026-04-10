//! 主题变量模块

use serde_json::Value;

/// 站点信息
pub struct Site {
    /// 站点标题
    pub title: String,
    /// 站点描述
    pub description: String,
    /// 站点作者
    pub author: String,
    /// 站点 URL
    pub url: String,
    /// 站点根路径
    pub root: String,
    /// 分类列表
    pub categories: Vec<Category>,
    /// 标签列表
    pub tags: Vec<Tag>,
    /// 文章列表
    pub posts: Vec<Post>,
    /// 页面列表
    pub pages: Vec<Page>,
}

/// 文章信息
pub struct Post {
    /// 文章标题
    pub title: String,
    /// 文章日期
    pub date: String,
    /// 文章更新日期
    pub updated: Option<String>,
    /// 文章路径
    pub path: String,
    /// 文章内容
    pub content: String,
    /// 文章摘要
    pub excerpt: String,
    /// 文章分类
    pub categories: Vec<Category>,
    /// 文章标签
    pub tags: Vec<Tag>,
    /// 文章前置元数据
    pub front_matter: Value,
}

/// 页面信息
pub struct Page {
    /// 页面标题
    pub title: String,
    /// 页面日期
    pub date: String,
    /// 页面更新日期
    pub updated: Option<String>,
    /// 页面路径
    pub path: String,
    /// 页面内容
    pub content: String,
    /// 页面摘要
    pub excerpt: String,
    /// 页面分类
    pub categories: Vec<Category>,
    /// 页面标签
    pub tags: Vec<Tag>,
    /// 页面前置元数据
    pub front_matter: Value,
}

impl Page {
    /// 从文章创建页面
    pub fn from_post(post: Post) -> Self {
        Self {
            title: post.title,
            date: post.date,
            updated: post.updated,
            path: post.path,
            content: post.content,
            excerpt: post.excerpt,
            categories: post.categories,
            tags: post.tags,
            front_matter: post.front_matter,
        }
    }
}

/// 分类信息
pub struct Category {
    /// 分类名称
    pub name: String,
    /// 分类路径
    pub path: String,
    /// 分类文章数
    pub posts: Vec<Post>,
}

/// 标签信息
pub struct Tag {
    /// 标签名称
    pub name: String,
    /// 标签路径
    pub path: String,
    /// 标签文章数
    pub posts: Vec<Post>,
}
