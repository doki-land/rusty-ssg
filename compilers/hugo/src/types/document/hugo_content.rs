//! Hugo 内容结构和文件系统处理模块
//!
//! 提供 Hugo 标准目录结构识别、内容文件遍历、Front Matter 解析
//! 以及 Sections、Bundles 等概念的实现

use oak_yaml;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    collections::HashMap,
    error::Error,
    fmt::{self, Display, Formatter},
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

/// Hugo 内容处理相关的错误类型
#[derive(Debug, Clone)]
pub enum HugoContentError {
    /// 文件读取错误
    FileReadError(String),

    /// Front Matter 解析错误
    FrontMatterParseError {
        /// 错误原因
        cause: String,
    },

    /// YAML 解析错误
    YamlParseError {
        /// 错误原因
        cause: String,
    },

    /// TOML 解析错误
    TomlParseError {
        /// 错误原因
        cause: String,
    },

    /// JSON 解析错误
    JsonParseError {
        /// 错误原因
        cause: String,
    },

    /// 无效的内容路径
    InvalidPath {
        /// 错误原因
        cause: String,
    },
}

impl Display for HugoContentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            HugoContentError::FileReadError(error) => write!(f, "Failed to read file: {}", error),
            HugoContentError::FrontMatterParseError { cause } => write!(f, "Failed to parse front matter: {}", cause),
            HugoContentError::YamlParseError { cause } => write!(f, "YAML parse error: {}", cause),
            HugoContentError::TomlParseError { cause } => write!(f, "TOML parse error: {}", cause),
            HugoContentError::JsonParseError { cause } => write!(f, "JSON parse error: {}", cause),
            HugoContentError::InvalidPath { cause } => write!(f, "Invalid content path: {}", cause),
        }
    }
}

impl Error for HugoContentError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl From<std::io::Error> for HugoContentError {
    fn from(error: std::io::Error) -> Self {
        HugoContentError::FileReadError(error.to_string())
    }
}

impl HugoContentError {
    /// 获取错误的 i18n key
    pub fn i18n_key(&self) -> &'static str {
        match self {
            HugoContentError::FileReadError(_) => "hugo.error.content.file_read",
            HugoContentError::FrontMatterParseError { .. } => "hugo.error.content.front_matter_parse",
            HugoContentError::YamlParseError { .. } => "hugo.error.content.yaml_parse",
            HugoContentError::TomlParseError { .. } => "hugo.error.content.toml_parse",
            HugoContentError::JsonParseError { .. } => "hugo.error.content.json_parse",
            HugoContentError::InvalidPath { .. } => "hugo.error.content.invalid_path",
        }
    }

    /// 获取错误参数
    pub fn params(&self) -> Vec<(&'static str, String)> {
        match self {
            HugoContentError::FileReadError(error) => {
                vec![("error", error.to_string())]
            }
            HugoContentError::FrontMatterParseError { cause } => {
                vec![("cause", cause.clone())]
            }
            HugoContentError::YamlParseError { cause } => {
                vec![("cause", cause.clone())]
            }
            HugoContentError::TomlParseError { cause } => {
                vec![("cause", cause.clone())]
            }
            HugoContentError::JsonParseError { cause } => {
                vec![("cause", cause.clone())]
            }
            HugoContentError::InvalidPath { cause } => {
                vec![("cause", cause.clone())]
            }
        }
    }

    /// 创建 Front Matter 解析错误
    pub fn front_matter_parse_error(cause: String) -> Self {
        HugoContentError::FrontMatterParseError { cause }
    }

    /// 创建 YAML 解析错误
    pub fn yaml_parse_error(cause: String) -> Self {
        HugoContentError::YamlParseError { cause }
    }

    /// 创建 TOML 解析错误
    pub fn toml_parse_error(cause: String) -> Self {
        HugoContentError::TomlParseError { cause }
    }

    /// 创建 JSON 解析错误
    pub fn json_parse_error(cause: String) -> Self {
        HugoContentError::JsonParseError { cause }
    }

    /// 创建无效路径错误
    pub fn invalid_path(cause: String) -> Self {
        HugoContentError::InvalidPath { cause }
    }
}

/// Front Matter 格式类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrontMatterFormat {
    /// YAML 格式
    Yaml,
    /// TOML 格式
    Toml,
    /// JSON 格式
    Json,
}

/// 内容类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContentType {
    /// 普通页面
    Page,
    /// Section 页面 (_index.md)
    Section,
    /// 叶子包（Leaf Bundle）
    LeafBundle,
    /// 分支包（Branch Bundle）
    BranchBundle,
}

/// 内容权重
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContentWeight {
    /// 按日期排序
    Date,
    /// 按权重排序
    Weight(i32),
    /// 按标题排序
    Title,
    /// 按链接标题排序
    LinkTitle,
    /// 按文件系统名称排序
    FileName,
    /// 按长度排序
    Length,
}

/// Hugo 内容文件的 Front Matter
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HugoFrontMatter {
    /// 页面标题
    pub title: Option<String>,
    /// 页面链接标题
    pub link_title: Option<String>,
    /// 页面描述
    pub description: Option<String>,
    /// 页面摘要
    pub summary: Option<String>,
    /// 页面布局
    pub layout: Option<String>,
    /// 内容类型
    pub r#type: Option<String>,
    /// 是否为草稿
    pub draft: Option<bool>,
    /// 发布日期
    pub date: Option<String>,
    /// 最后修改日期
    pub lastmod: Option<String>,
    /// 过期日期
    pub expiry_date: Option<String>,
    /// 发布日期
    pub publish_date: Option<String>,
    /// 页面权重
    pub weight: Option<i32>,
    /// 页面标签
    pub tags: Option<Vec<String>>,
    /// 页面分类
    pub categories: Option<Vec<String>>,
    /// 页面系列
    pub series: Option<Vec<String>>,
    /// 页面关键词
    pub keywords: Option<Vec<String>>,
    /// 作者信息
    pub authors: Option<Vec<String>>,
    /// 封面图像
    pub images: Option<Vec<String>>,
    /// slug
    pub slug: Option<String>,
    /// 页面 URL
    pub url: Option<String>,
    /// 永久链接
    pub permalink: Option<String>,
    /// 是否在搜索结果中隐藏
    pub noindex: Option<bool>,
    /// 是否启用评论
    pub comments: Option<bool>,
    /// 菜单配置
    pub menu: Option<HashMap<String, MenuItem>>,
    /// 别名
    pub aliases: Option<Vec<String>>,
    /// 自定义参数
    pub params: Option<HashMap<String, serde_json::Value>>,
    /// 是否生成目录
    pub toc: Option<bool>,
    /// 资源配置
    pub resources: Option<Vec<ResourceConfig>>,
    /// 自定义分类法
    #[serde(flatten)]
    pub custom_taxonomies: HashMap<String, serde_json::Value>,
}

/// 菜单项
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MenuItem {
    /// 菜单标识符
    pub identifier: Option<String>,
    /// 菜单名称
    pub name: Option<String>,
    /// 菜单标题
    pub title: Option<String>,
    /// 菜单 URL
    pub url: Option<String>,
    /// 菜单权重
    pub weight: Option<i32>,
    /// 父菜单标识符
    pub parent: Option<String>,
    /// 前置文本
    pub pre: Option<String>,
    /// 后置文本
    pub post: Option<String>,
}

/// 资源配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceConfig {
    /// 资源名称
    pub name: Option<String>,
    /// 资源标题
    pub title: Option<String>,
    /// 资源参数
    pub params: Option<HashMap<String, serde_json::Value>>,
}

impl HugoFrontMatter {
    /// 创建新的 Front Matter
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置标题
    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    /// 设置日期
    pub fn with_date(mut self, date: String) -> Self {
        self.date = Some(date);
        self
    }

    /// 设置权重
    pub fn with_weight(mut self, weight: i32) -> Self {
        self.weight = Some(weight);
        self
    }

    /// 检查是否为草稿
    pub fn is_draft(&self) -> bool {
        self.draft.unwrap_or(false)
    }
}

/// 内容资源
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentResource {
    /// 资源路径
    pub path: PathBuf,
    /// 资源名称
    pub name: String,
    /// 资源标题
    pub title: Option<String>,
    /// 资源大小（字节）
    pub size: Option<u64>,
}

/// Hugo 内容页面
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HugoPage {
    /// 页面路径
    pub path: PathBuf,
    /// 页面相对路径
    pub relative_path: PathBuf,
    /// 页面内容类型
    pub content_type: ContentType,
    /// 页面 Front Matter
    pub frontmatter: HugoFrontMatter,
    /// 页面原始内容（不含 Front Matter）
    pub content: String,
    /// 关联的资源
    pub resources: Vec<ContentResource>,
    /// 子页面
    pub children: Vec<HugoPage>,
    /// 所属的 Section
    pub section: Option<String>,
    /// 页面语言
    pub lang: Option<String>,
}

impl HugoPage {
    /// 创建新的 Hugo 页面
    pub fn new(path: PathBuf, relative_path: PathBuf) -> Self {
        Self {
            path,
            relative_path,
            content_type: ContentType::Page,
            frontmatter: HugoFrontMatter::new(),
            content: String::new(),
            resources: Vec::new(),
            children: Vec::new(),
            section: None,
            lang: None,
        }
    }

    /// 获取页面标题
    pub fn title(&self) -> Option<&str> {
        self.frontmatter.title.as_deref().or_else(|| self.frontmatter.link_title.as_deref())
    }

    /// 获取页面权重
    pub fn weight(&self) -> i32 {
        self.frontmatter.weight.unwrap_or(0)
    }

    /// 检查是否为草稿
    pub fn is_draft(&self) -> bool {
        self.frontmatter.is_draft()
    }
}

/// Hugo 目录结构
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HugoDirectoryStructure {
    /// 内容目录
    pub content_dir: PathBuf,
    /// 静态文件目录
    pub static_dir: Option<PathBuf>,
    /// 布局目录
    pub layouts_dir: Option<PathBuf>,
    /// 资源目录
    pub assets_dir: Option<PathBuf>,
    /// 数据目录
    pub data_dir: Option<PathBuf>,
    /// 主题目录
    pub themes_dir: Option<PathBuf>,
    /// 发布目录
    pub publish_dir: Option<PathBuf>,
}

impl HugoDirectoryStructure {
    /// 创建新的目录结构
    pub fn new(content_dir: PathBuf) -> Self {
        Self { content_dir, ..Default::default() }
    }

    /// 从项目根目录自动发现 Hugo 目录结构
    pub fn discover(root: impl AsRef<Path>) -> Self {
        let root = root.as_ref();
        let mut structure = Self::default();

        let content_dir = root.join("content");
        if content_dir.exists() {
            structure.content_dir = content_dir;
        }

        let static_dir = root.join("static");
        if static_dir.exists() {
            structure.static_dir = Some(static_dir);
        }

        let layouts_dir = root.join("layouts");
        if layouts_dir.exists() {
            structure.layouts_dir = Some(layouts_dir);
        }

        let assets_dir = root.join("assets");
        if assets_dir.exists() {
            structure.assets_dir = Some(assets_dir);
        }

        let data_dir = root.join("data");
        if data_dir.exists() {
            structure.data_dir = Some(data_dir);
        }

        let themes_dir = root.join("themes");
        if themes_dir.exists() {
            structure.themes_dir = Some(themes_dir);
        }

        let public_dir = root.join("public");
        if public_dir.exists() {
            structure.publish_dir = Some(public_dir);
        }

        structure
    }
}

/// Hugo 内容索引
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HugoContentIndex {
    /// 所有页面
    pub pages: Vec<HugoPage>,
    /// 按路径索引的页面
    pub page_map: HashMap<PathBuf, HugoPage>,
    /// Sections
    pub sections: HashMap<String, HugoPage>,
    /// 所有资源
    pub resources: Vec<ContentResource>,
}

impl HugoContentIndex {
    /// 创建新的内容索引
    pub fn new() -> Self {
        Self::default()
    }

    /// 添加页面到索引
    pub fn add_page(&mut self, page: HugoPage) {
        self.page_map.insert(page.path.clone(), page.clone());
        self.pages.push(page);
    }

    /// 根据路径获取页面
    pub fn get_page(&self, path: &Path) -> Option<&HugoPage> {
        self.page_map.get(path)
    }
}

/// Front Matter 解析器
pub struct FrontMatterParser;

impl FrontMatterParser {
    /// 从内容字符串中解析 Front Matter
    ///
    /// 返回 (FrontMatter, 剩余内容, FrontMatter格式)
    pub fn parse(content: &str) -> Result<(HugoFrontMatter, String, FrontMatterFormat), HugoContentError> {
        let trimmed = content.trim_start();

        if trimmed.starts_with("---") {
            Self::parse_yaml(trimmed)
        }
        else if trimmed.starts_with("+++") {
            Self::parse_toml(trimmed)
        }
        else if trimmed.starts_with("{") {
            Self::parse_json(trimmed)
        }
        else {
            Ok((HugoFrontMatter::new(), content.to_string(), FrontMatterFormat::Yaml))
        }
    }

    /// 解析 YAML 格式的 Front Matter
    fn parse_yaml(content: &str) -> Result<(HugoFrontMatter, String, FrontMatterFormat), HugoContentError> {
        let content = content.strip_prefix("---").unwrap_or(content);
        let end_marker = content
            .find("---")
            .ok_or_else(|| HugoContentError::front_matter_parse_error("YAML front matter missing closing ---".to_string()))?;

        let yaml_content = &content[..end_marker];
        let remaining_content = content[end_marker + 3..].to_string();

        let frontmatter: HugoFrontMatter =
            oak_yaml::from_str(yaml_content).map_err(|e| HugoContentError::yaml_parse_error(e.to_string()))?;

        Ok((frontmatter, remaining_content, FrontMatterFormat::Yaml))
    }

    /// 解析 TOML 格式的 Front Matter
    fn parse_toml(content: &str) -> Result<(HugoFrontMatter, String, FrontMatterFormat), HugoContentError> {
        let content = content.strip_prefix("+++").unwrap_or(content);
        let end_marker = content
            .find("+++")
            .ok_or_else(|| HugoContentError::front_matter_parse_error("TOML front matter missing closing +++".to_string()))?;

        let toml_content = &content[..end_marker];
        let remaining_content = content[end_marker + 3..].to_string();

        let frontmatter: HugoFrontMatter =
            oak_toml::language::from_str(toml_content).map_err(|e| HugoContentError::toml_parse_error(e.to_string()))?;

        Ok((frontmatter, remaining_content, FrontMatterFormat::Toml))
    }

    /// 解析 JSON 格式的 Front Matter
    fn parse_json(content: &str) -> Result<(HugoFrontMatter, String, FrontMatterFormat), HugoContentError> {
        let mut brace_count = 0;
        let mut end_index = 0;
        let mut in_string = false;
        let mut escape = false;

        for (i, c) in content.char_indices() {
            if escape {
                escape = false;
                continue;
            }

            match c {
                '\\' if in_string => escape = true,
                '"' => in_string = !in_string,
                '{' if !in_string => brace_count += 1,
                '}' if !in_string => {
                    brace_count -= 1;
                    if brace_count == 0 {
                        end_index = i + 1;
                        break;
                    }
                }
                _ => {}
            }
        }

        if end_index == 0 {
            return Err(HugoContentError::front_matter_parse_error("JSON front matter missing closing }".to_string()));
        }

        let json_content = &content[..end_index];
        let remaining_content = content[end_index..].to_string();

        let frontmatter: HugoFrontMatter =
            serde_json::from_str(json_content).map_err(|e| HugoContentError::json_parse_error(e.to_string()))?;

        Ok((frontmatter, remaining_content, FrontMatterFormat::Json))
    }
}

/// Hugo 内容加载器
pub struct HugoContentLoader {
    /// 目录结构
    structure: HugoDirectoryStructure,
}

impl HugoContentLoader {
    /// 创建新的内容加载器
    pub fn new(structure: HugoDirectoryStructure) -> Self {
        Self { structure }
    }

    /// 从项目根目录创建内容加载器
    pub fn from_root(root: impl AsRef<Path>) -> Self {
        Self::new(HugoDirectoryStructure::discover(root))
    }

    /// 加载所有内容
    pub fn load_all(&self) -> Result<HugoContentIndex, HugoContentError> {
        let mut index = HugoContentIndex::new();

        if !self.structure.content_dir.exists() {
            return Ok(index);
        }

        self.load_content_dir(&mut index, &self.structure.content_dir)?;

        Ok(index)
    }

    /// 加载内容目录
    fn load_content_dir(&self, index: &mut HugoContentIndex, content_dir: &Path) -> Result<(), HugoContentError> {
        for entry in WalkDir::new(content_dir).follow_links(true).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "md" || ext == "markdown" {
                        if let Ok(page) = self.load_page(path, content_dir) {
                            index.add_page(page);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// 加载单个页面
    fn load_page(&self, path: &Path, content_dir: &Path) -> Result<HugoPage, HugoContentError> {
        let content = std::fs::read_to_string(path)?;
        let (frontmatter, body_content, _) = FrontMatterParser::parse(&content)?;

        // 增强 Front Matter
        let mut enhanced_frontmatter = FrontMatterEnhancer::enhance(frontmatter, path)?;
        
        // 生成摘要
        if enhanced_frontmatter.summary.is_none() {
            if let Some(summary) = Self::generate_summary(&body_content) {
                enhanced_frontmatter.summary = Some(summary);
            }
        }
        
        // 验证 Front Matter
        FrontMatterEnhancer::validate(&enhanced_frontmatter)?;

        let relative_path = path
            .strip_prefix(content_dir)
            .map_err(|_| HugoContentError::invalid_path("Cannot get relative path".to_string()))?;

        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        let content_type = if file_name == "_index.md" {
            ContentType::Section
        }
        else if file_name == "index.md" {
            if let Some(parent) = path.parent() {
                if parent.join("_index.md").exists() { ContentType::BranchBundle } else { ContentType::LeafBundle }
            }
            else {
                ContentType::LeafBundle
            }
        }
        else {
            ContentType::Page
        };

        let section = relative_path.components().next().and_then(|c| c.as_os_str().to_str()).map(|s| s.to_string());

        let mut page = HugoPage::new(path.to_path_buf(), relative_path.to_path_buf());
        page.content_type = content_type;
        page.frontmatter = enhanced_frontmatter;
        page.content = body_content;
        page.section = section;

        Ok(page)
    }
    
    /// 生成内容摘要
    ///
    /// 1. 首先检查是否有 <!--more--> 分隔符
    /// 2. 如果没有，提取前 70 个单词作为摘要
    pub fn generate_summary(content: &str) -> Option<String> {
        // 检查是否有 <!--more--> 分隔符
        if let Some(more_index) = content.find("<!--more-->") {
            let summary = content[..more_index].trim();
            if !summary.is_empty() {
                return Some(summary.to_string());
            }
        }
        
        // 提取前 70 个单词作为摘要
        let words: Vec<&str> = content
            .split_whitespace()
            .take(70)
            .collect();
        
        if !words.is_empty() {
            let summary = words.join(" ");
            Some(summary)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_generate_summary_with_more_separator() {
        let content = "This is the summary<!--more-->This is the rest of the content";
        let summary = HugoContentLoader::generate_summary(content);
        assert_eq!(summary, Some("This is the summary".to_string()));
    }

    #[test]
    fn test_generate_summary_without_more_separator() {
        let content = "This is a test content with multiple words to test the automatic summary generation. "
            .repeat(10);
        let summary = HugoContentLoader::generate_summary(&content);
        assert!(summary.is_some());
        let summary_str = summary.unwrap();
        let words: Vec<&str> = summary_str.split_whitespace().collect();
        assert!(words.len() <= 70);
    }

    #[test]
    fn test_generate_summary_empty_content() {
        let content = "";
        let summary = HugoContentLoader::generate_summary(content);
        assert_eq!(summary, None);
    }

    #[test]
    fn test_generate_summary_short_content() {
        let content = "Short content";
        let summary = HugoContentLoader::generate_summary(content);
        assert_eq!(summary, Some("Short content".to_string()));
    }
}
