//! 文档模块
//! 定义 VuTeX 文档的元信息结构

use nargo_types::{NargoValue, Span};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

/// Front Matter 增强模块
pub mod front_matter_enhancer;
/// Hugo 内容结构和文件系统处理模块
pub mod hugo_content;
/// 国际化翻译模块
pub mod i18n;
/// 相关内容推荐模块
pub mod related_content;
/// 分类系统模块
pub mod taxonomy;

pub use self::{front_matter_enhancer::*, hugo_content::*, i18n::*, related_content::*, taxonomy::*};
/// VuTeX 文档
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct VutexDocument {
    /// 文档元信息
    pub meta: DocumentMeta,
    /// 前置元数据（frontmatter）
    pub frontmatter: FrontMatter,
    /// 文档内容（原始 Markdown）
    pub content: String,
    /// 解析后的内容（可以是 HTML 或结构化数据）
    pub rendered_content: Option<String>,
    /// 位置信息
    pub span: Span,
}

impl VutexDocument {
    /// 创建新的文档
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置文档路径
    pub fn with_path(mut self, path: String) -> Self {
        self.meta.path = path;
        self
    }

    /// 设置前置元数据
    pub fn with_frontmatter(mut self, frontmatter: FrontMatter) -> Self {
        self.frontmatter = frontmatter;
        self
    }

    /// 设置文档内容
    pub fn with_content(mut self, content: String) -> Self {
        self.content = content;
        self
    }

    /// 获取文档标题
    pub fn title(&self) -> Option<&str> {
        self.frontmatter.title.as_deref().or_else(|| self.meta.title.as_deref())
    }

    /// 获取文档描述
    pub fn description(&self) -> Option<&str> {
        self.frontmatter.description.as_deref()
    }

    /// 获取文档标签
    pub fn tags(&self) -> &[String] {
        &self.frontmatter.tags
    }

    /// 序列化为 JSON
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }

    /// 序列化为美化的 JSON
    pub fn to_json_pretty(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}

/// 文档元信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DocumentMeta {
    /// 文档路径
    pub path: String,
    /// 文档标题
    pub title: Option<String>,
    /// 文档语言
    pub lang: Option<String>,
    /// 最后更新时间
    pub last_updated: Option<i64>,
    /// 其他元数据
    pub extra: HashMap<String, NargoValue>,
}

/// 前置元数据（Front Matter）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FrontMatter {
    /// 页面标题
    pub title: Option<String>,
    /// 页面描述
    pub description: Option<String>,
    /// 页面布局
    pub layout: Option<String>,
    /// 页面标签
    pub tags: Vec<String>,
    /// 页面分类
    pub categories: Vec<String>,
    /// 发布日期
    pub date: Option<String>,
    /// 是否为草稿
    pub draft: Option<bool>,
    /// 作者
    pub author: Option<String>,
    /// 权重（用于排序）
    pub weight: Option<i32>,
    /// 菜单配置
    pub menu: Option<HashMap<String, serde_json::Value>>,
    /// 别名
    pub aliases: Vec<String>,
    /// 自定义 URL 片段
    pub slug: Option<String>,
    /// 是否在侧边栏中隐藏
    pub sidebar: Option<bool>,
    /// 侧边栏顺序
    pub sidebar_order: Option<i32>,
    /// 自定义元数据
    pub custom: HashMap<String, NargoValue>,
}

impl FrontMatter {
    /// 创建新的前置元数据
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置标题
    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    /// 设置描述
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// 设置布局
    pub fn with_layout(mut self, layout: String) -> Self {
        self.layout = Some(layout);
        self
    }

    /// 添加标签
    pub fn add_tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }

    /// 添加分类
    pub fn add_category(mut self, category: String) -> Self {
        self.categories.push(category);
        self
    }

    /// 设置日期
    pub fn with_date(mut self, date: String) -> Self {
        self.date = Some(date);
        self
    }

    /// 设置是否为草稿
    pub fn with_draft(mut self, draft: bool) -> Self {
        self.draft = Some(draft);
        self
    }

    /// 设置作者
    pub fn with_author(mut self, author: String) -> Self {
        self.author = Some(author);
        self
    }

    /// 设置权重
    pub fn with_weight(mut self, weight: i32) -> Self {
        self.weight = Some(weight);
        self
    }

    /// 添加别名
    pub fn add_alias(mut self, alias: String) -> Self {
        self.aliases.push(alias);
        self
    }

    /// 设置 slug
    pub fn with_slug(mut self, slug: String) -> Self {
        self.slug = Some(slug);
        self
    }
}
