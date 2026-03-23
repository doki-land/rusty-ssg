//! 分类系统模块
//! 提供分类和标签的处理功能

use super::{HugoContentIndex, HugoPage};
use std::{collections::HashMap, path::PathBuf};

/// 分类项
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaxonomyTerm {
    /// 分类名称
    pub name: String,
    /// 分类别名
    pub slug: String,
    /// 分类描述
    pub description: Option<String>,
    /// 分类下的页面数量
    pub count: usize,
    /// 分类下的页面
    pub pages: Vec<HugoPage>,
}

/// 分类类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaxonomyType {
    /// 标签
    Tags,
    /// 分类
    Categories,
    /// 自定义分类
    Custom(String),
}

/// 分类系统
pub struct TaxonomySystem {
    /// 内容索引
    content_index: &'static HugoContentIndex,
}

impl TaxonomySystem {
    /// 创建新的分类系统
    pub fn new(content_index: &'static HugoContentIndex) -> Self {
        Self { content_index }
    }

    /// 获取所有标签
    pub fn get_tags(&self) -> Vec<TaxonomyTerm> {
        let tag_groups = self.content_index.group_by_tag();
        self.convert_to_terms(tag_groups)
    }

    /// 获取所有分类
    pub fn get_categories(&self) -> Vec<TaxonomyTerm> {
        let category_groups = self.content_index.group_by_category();
        self.convert_to_terms(category_groups)
    }

    /// 转换分组为分类项
    fn convert_to_terms(&self, groups: HashMap<String, Vec<&HugoPage>>) -> Vec<TaxonomyTerm> {
        groups
            .into_iter()
            .map(|(name, pages)| TaxonomyTerm {
                name: name.clone(),
                slug: name.to_lowercase().replace(' ', "-"),
                description: None,
                count: pages.len(),
                pages: pages.into_iter().cloned().collect(),
            })
            .collect()
    }

    /// 生成分类列表页面
    pub fn generate_taxonomy_pages(&self) -> Vec<HugoPage> {
        let mut pages = Vec::new();

        // 生成标签列表页面
        pages.extend(self.generate_tag_pages());

        // 生成分类列表页面
        pages.extend(self.generate_category_pages());

        pages
    }

    /// 生成标签列表页面
    fn generate_tag_pages(&self) -> Vec<HugoPage> {
        let tags = self.get_tags();
        let mut pages = Vec::new();

        // 生成标签索引页面
        let tag_index_page = self.generate_taxonomy_index_page(TaxonomyType::Tags, &tags);
        pages.push(tag_index_page);

        // 生成每个标签的列表页面
        for tag in &tags {
            let tag_page = self.generate_taxonomy_term_page(TaxonomyType::Tags, tag);
            pages.push(tag_page);
        }

        pages
    }

    /// 生成分类列表页面
    fn generate_category_pages(&self) -> Vec<HugoPage> {
        let categories = self.get_categories();
        let mut pages = Vec::new();

        // 生成分类索引页面
        let category_index_page = self.generate_taxonomy_index_page(TaxonomyType::Categories, &categories);
        pages.push(category_index_page);

        // 生成每个分类的列表页面
        for category in &categories {
            let category_page = self.generate_taxonomy_term_page(TaxonomyType::Categories, category);
            pages.push(category_page);
        }

        pages
    }

    /// 生成分类索引页面
    fn generate_taxonomy_index_page(&self, taxonomy_type: TaxonomyType, terms: &Vec<TaxonomyTerm>) -> HugoPage {
        let (name, path) = match taxonomy_type {
            TaxonomyType::Tags => ("Tags", PathBuf::from("tags/_index.md")),
            TaxonomyType::Categories => ("Categories", PathBuf::from("categories/_index.md")),
            TaxonomyType::Custom(custom) => (custom.as_str(), PathBuf::from(format!("{}/_index.md", custom))),
        };

        let mut frontmatter = super::HugoFrontMatter::new();
        frontmatter.title = Some(name.to_string());
        frontmatter.description = Some(format!("All {} on this site", name.to_lowercase()));

        let content = format!("# {}\n\nThis page lists all {} on the site.\n", name, name.to_lowercase());

        let mut page = HugoPage::new(path.clone(), path);
        page.frontmatter = frontmatter;
        page.content = content;
        page.content_type = super::ContentType::Section;

        page
    }

    /// 生成分类项页面
    fn generate_taxonomy_term_page(&self, taxonomy_type: TaxonomyType, term: &TaxonomyTerm) -> HugoPage {
        let (base_path, plural_name) = match taxonomy_type {
            TaxonomyType::Tags => ("tags", "tags"),
            TaxonomyType::Categories => ("categories", "categories"),
            TaxonomyType::Custom(custom) => (custom.as_str(), custom.as_str()),
        };

        let path = PathBuf::from(format!("{}/{}/_index.md", base_path, term.slug));

        let mut frontmatter = super::HugoFrontMatter::new();
        frontmatter.title = Some(term.name.clone());
        frontmatter.description = Some(format!("All posts tagged with '{}'", term.name));

        let content = format!("# {}\n\nThis page lists all posts tagged with '{}'.\n", term.name, term.name);

        let mut page = HugoPage::new(path.clone(), path);
        page.frontmatter = frontmatter;
        page.content = content;
        page.content_type = super::ContentType::Section;

        page
    }
}
