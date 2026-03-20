//! 主题变量系统

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 站点信息
#[derive(Debug, Deserialize, Serialize)]
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
    /// 分类
    pub categories: Vec<Category>,
    /// 标签
    pub tags: Vec<Tag>,
    /// 文章
    pub posts: Vec<Post>,
    /// 页面
    pub pages: Vec<Page>,
}

/// 分类
#[derive(Debug, Deserialize, Serialize)]
pub struct Category {
    /// 分类名称
    pub name: String,
    /// 分类路径
    pub path: String,
    /// 分类下的文章
    pub posts: Vec<Post>,
}

/// 标签
#[derive(Debug, Deserialize, Serialize)]
pub struct Tag {
    /// 标签名称
    pub name: String,
    /// 标签路径
    pub path: String,
    /// 标签下的文章
    pub posts: Vec<Post>,
}

/// 文章
#[derive(Debug, Deserialize, Serialize)]
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
    pub front_matter: serde_json::Value,
}

/// 页面
#[derive(Debug, Deserialize, Serialize)]
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
    /// 页面前置元数据
    pub front_matter: serde_json::Value,
}

/// 主题变量
#[derive(Debug, Deserialize, Serialize)]
pub struct ThemeVariables {
    /// 站点信息
    pub site: Site,
    /// 文章信息
    pub page: Option<Post>,
    /// 主题配置
    pub theme: serde_json::Value,
    /// 配置信息
    pub config: serde_json::Value,
    /// 分页信息
    pub pagination: Option<Pagination>,
    /// 其他变量
    pub __: HashMap<String, serde_json::Value>,
}

/// 分页信息
#[derive(Debug, Deserialize, Serialize)]
pub struct Pagination {
    /// 当前页码
    pub current: u32,
    /// 每页数量
    pub per_page: u32,
    /// 总页数
    pub total: u32,
    /// 上一页
    pub prev: Option<u32>,
    /// 下一页
    pub next: Option<u32>,
    /// 分页路径
    pub path: String,
}

/// 生成主题变量
pub fn generate_variables(
    site: Site,
    page: Option<Post>,
    theme_config: serde_json::Value,
    config: serde_json::Value,
) -> ThemeVariables {
    ThemeVariables { site, page, theme: theme_config, config, pagination: None, __: HashMap::new() }
}
