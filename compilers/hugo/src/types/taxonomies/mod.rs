//! 分类法（Taxonomies）模块
//!
//! 提供 Hugo 兼容的分类法系统，包括标签、分类和自定义分类法，
//! 支持分类页面和术语页面的生成。

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::types::document::hugo_content::HugoPage;

/// 分类法术语（单个分类项）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaxonomyTerm {
    /// 术语名称
    pub name: String,
    /// 术语 slug（URL 友好名称）
    pub slug: String,
    /// 术语权重
    pub weight: Option<i32>,
    /// 关联的页面
    pub pages: Vec<HugoPage>,
}

impl TaxonomyTerm {
    /// 创建新的分类法术语
    pub fn new(name: String, slug: String) -> Self {
        Self { name, slug, weight: None, pages: Vec::new() }
    }

    /// 添加关联页面
    pub fn add_page(&mut self, page: HugoPage) {
        self.pages.push(page);
    }

    /// 设置术语权重
    pub fn with_weight(mut self, weight: i32) -> Self {
        self.weight = Some(weight);
        self
    }
}

/// 分类法（如 tags、categories）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Taxonomy {
    /// 分类法名称（复数形式，如 tags）
    pub name: String,
    /// 分类法单数形式（如 tag）
    pub singular: String,
    /// 术语集合
    pub terms: HashMap<String, TaxonomyTerm>,
    /// 是否禁用此分类法
    pub disabled: bool,
}

impl Taxonomy {
    /// 创建新的分类法
    pub fn new(name: String, singular: String) -> Self {
        Self { name, singular, terms: HashMap::new(), disabled: false }
    }

    /// 添加术语
    pub fn add_term(&mut self, term: TaxonomyTerm) {
        self.terms.insert(term.slug.clone(), term);
    }

    /// 获取术语
    pub fn get_term(&self, slug: &str) -> Option<&TaxonomyTerm> {
        self.terms.get(slug)
    }

    /// 获取可变术语引用
    pub fn get_term_mut(&mut self, slug: &str) -> Option<&mut TaxonomyTerm> {
        self.terms.get_mut(slug)
    }

    /// 获取所有术语
    pub fn get_all_terms(&self) -> Vec<&TaxonomyTerm> {
        self.terms.values().collect()
    }

    /// 获取按名称排序的所有术语
    pub fn get_sorted_terms(&self) -> Vec<&TaxonomyTerm> {
        let mut terms: Vec<_> = self.terms.values().collect();
        terms.sort_by(|a, b| a.name.cmp(&b.name));
        terms
    }

    /// 禁用此分类法
    pub fn disable(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// 分类法索引
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TaxonomyIndex {
    /// 所有分类法
    pub taxonomies: HashMap<String, Taxonomy>,
}

impl TaxonomyIndex {
    /// 创建新的分类法索引
    pub fn new() -> Self {
        Self::default()
    }

    /// 添加分类法
    pub fn add_taxonomy(&mut self, taxonomy: Taxonomy) {
        self.taxonomies.insert(taxonomy.name.clone(), taxonomy);
    }

    /// 获取分类法
    pub fn get_taxonomy(&self, name: &str) -> Option<&Taxonomy> {
        self.taxonomies.get(name)
    }

    /// 获取可变分类法引用
    pub fn get_taxonomy_mut(&mut self, name: &str) -> Option<&mut Taxonomy> {
        self.taxonomies.get_mut(name)
    }

    /// 获取所有分类法
    pub fn get_all_taxonomies(&self) -> Vec<&Taxonomy> {
        self.taxonomies.values().collect()
    }

    /// 为页面收集所有分类法
    pub fn get_page_taxonomies(&self, page: &HugoPage) -> HashMap<String, Vec<String>> {
        let mut result = HashMap::new();

        for (taxonomy_name, taxonomy) in &self.taxonomies {
            let mut terms = Vec::new();
            for term in taxonomy.terms.values() {
                if term.pages.iter().any(|p| p.path == page.path) {
                    terms.push(term.name.clone());
                }
            }
            if !terms.is_empty() {
                result.insert(taxonomy_name.clone(), terms);
            }
        }

        result
    }
}

/// 分类法构建器
#[derive(Debug, Clone)]
pub struct TaxonomyBuilder {
    /// 索引
    index: TaxonomyIndex,
    /// 已使用的 slugs
    used_slugs: HashSet<String>,
}

impl TaxonomyBuilder {
    /// 创建新的分类法构建器
    pub fn new() -> Self {
        Self { index: TaxonomyIndex::new(), used_slugs: HashSet::new() }
    }

    /// 注册默认分类法（tags 和 categories）
    pub fn with_default_taxonomies(mut self) -> Self {
        self.register_taxonomy("tags".to_string(), "tag".to_string());
        self.register_taxonomy("categories".to_string(), "category".to_string());
        self
    }

    /// 注册自定义分类法
    pub fn register_taxonomy(&mut self, name: String, singular: String) {
        let taxonomy = Taxonomy::new(name.clone(), singular);
        self.index.add_taxonomy(taxonomy);
    }

    /// 从页面集合构建分类法索引
    pub fn build_from_pages(&mut self, pages: &[HugoPage]) -> &TaxonomyIndex {
        for page in pages {
            self.index_page(page);
        }
        &self.index
    }

    /// 索引单个页面
    fn index_page(&mut self, page: &HugoPage) {
        let fm = &page.frontmatter;

        if let Some(tags) = &fm.tags {
            self.add_page_to_taxonomy("tags", tags, page.clone());
        }

        if let Some(categories) = &fm.categories {
            self.add_page_to_taxonomy("categories", categories, page.clone());
        }

        if let Some(series) = &fm.series {
            if !self.index.taxonomies.contains_key("series") {
                self.register_taxonomy("series".to_string(), "series".to_string());
            }
            self.add_page_to_taxonomy("series", series, page.clone());
        }

        if let Some(authors) = &fm.authors {
            if !self.index.taxonomies.contains_key("authors") {
                self.register_taxonomy("authors".to_string(), "author".to_string());
            }
            self.add_page_to_taxonomy("authors", authors, page.clone());
        }

        for (taxonomy_name, value) in &fm.custom_taxonomies {
            if let Ok(terms) = serde_json::from_value::<Vec<String>>(value.clone()) {
                if !self.index.taxonomies.contains_key(taxonomy_name) {
                    let singular = taxonomy_name.trim_end_matches('s').to_string();
                    self.register_taxonomy(taxonomy_name.clone(), singular);
                }
                self.add_page_to_taxonomy(taxonomy_name, &terms, page.clone());
            }
        }
    }

    /// 将页面添加到指定分类法
    fn add_page_to_taxonomy(&mut self, taxonomy_name: &str, terms: &[String], page: HugoPage) {
        if let Some(taxonomy) = self.index.get_taxonomy_mut(taxonomy_name) {
            for term_name in terms {
                let slug = Self::slugify(term_name);
                if let Some(term) = taxonomy.get_term_mut(&slug) {
                    term.add_page(page.clone());
                }
                else {
                    let mut term = TaxonomyTerm::new(term_name.clone(), slug);
                    term.add_page(page.clone());
                    taxonomy.add_term(term);
                }
            }
        }
    }

    /// 将字符串转换为 slug（URL 友好格式）
    fn slugify(s: &str) -> String {
        s.to_lowercase().replace(|c: char| !c.is_alphanumeric(), "-").replace("--", "-").trim_matches('-').to_string()
    }

    /// 获取构建完成的索引
    pub fn finish(self) -> TaxonomyIndex {
        self.index
    }
}

impl Default for TaxonomyBuilder {
    fn default() -> Self {
        Self::new()
    }
}
