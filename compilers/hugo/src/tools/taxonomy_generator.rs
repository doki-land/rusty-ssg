//! 分类法页面生成器模块
//!
//! 提供分类页面和术语页面的生成功能，包括 HTML 渲染和文件输出。

use crate::types::document::hugo_content::HugoPage;
use crate::types::taxonomies::{Taxonomy, TaxonomyIndex, TaxonomyTerm};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// 术语页面上下文
#[derive(Debug, Clone)]
pub struct TermPageContext {
    /// 术语名称
    pub term_name: String,
    /// 术语 slug
    pub term_slug: String,
    /// 分类法名称
    pub taxonomy_name: String,
    /// 分类法单数形式
    pub taxonomy_singular: String,
    /// 关联的页面
    pub pages: Vec<PageInfo>,
}

/// 页面信息
#[derive(Debug, Clone)]
pub struct PageInfo {
    /// 页面标题
    pub title: String,
    /// 页面 URL
    pub url: String,
    /// 页面日期
    pub date: Option<String>,
}

/// 分类页面上下文
#[derive(Debug, Clone)]
pub struct TaxonomyPageContext {
    /// 分类法名称
    pub taxonomy_name: String,
    /// 分类法单数形式
    pub taxonomy_singular: String,
    /// 所有术语
    pub terms: Vec<TermInfo>,
}

/// 术语信息
#[derive(Debug, Clone)]
pub struct TermInfo {
    /// 术语名称
    pub name: String,
    /// 术语 slug
    pub slug: String,
    /// 术语 URL
    pub url: String,
    /// 页面数量
    pub page_count: usize,
}

/// 分类法页面生成器
pub struct TaxonomyPageGenerator {
    /// 输出目录
    output_dir: PathBuf,
    /// 基础 URL
    base_url: Option<String>,
}

impl TaxonomyPageGenerator {
    /// 创建新的分类法页面生成器
    pub fn new(output_dir: PathBuf, base_url: Option<String>) -> Self {
        Self {
            output_dir,
            base_url,
        }
    }

    /// 生成所有分类法页面
    pub fn generate_all(&self, taxonomy_index: &TaxonomyIndex) -> Result<Vec<PathBuf>> {
        let mut generated_paths = Vec::new();

        for taxonomy in taxonomy_index.get_all_taxonomies() {
            if !taxonomy.disabled {
                let paths = self.generate_taxonomy_pages(taxonomy)?;
                generated_paths.extend(paths);
            }
        }

        Ok(generated_paths)
    }

    /// 生成单个分类法的页面（分类页面和所有术语页面）
    pub fn generate_taxonomy_pages(&self, taxonomy: &Taxonomy) -> Result<Vec<PathBuf>> {
        let mut generated_paths = Vec::new();

        let taxonomy_page_path = self.generate_taxonomy_page(taxonomy)?;
        generated_paths.push(taxonomy_page_path);

        for term in taxonomy.get_all_terms() {
            let term_page_path = self.generate_term_page(taxonomy, term)?;
            generated_paths.push(term_page_path);
        }

        Ok(generated_paths)
    }

    /// 生成分类页面（显示该分类法的所有术语）
    pub fn generate_taxonomy_page(&self, taxonomy: &Taxonomy) -> Result<PathBuf> {
        let context = self.build_taxonomy_context(taxonomy);
        let html = self.render_taxonomy_page(&context);

        let file_path = self.output_dir
            .join(&taxonomy.name)
            .join("index.html");

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&file_path, html)?;

        Ok(file_path)
    }

    /// 生成术语页面（显示该术语下的所有页面）
    pub fn generate_term_page(&self, taxonomy: &Taxonomy, term: &TaxonomyTerm) -> Result<PathBuf> {
        let context = self.build_term_context(taxonomy, term);
        let html = self.render_term_page(&context);

        let file_path = self.output_dir
            .join(&taxonomy.name)
            .join(&term.slug)
            .join("index.html");

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&file_path, html)?;

        Ok(file_path)
    }

    /// 构建分类页面上下文
    fn build_taxonomy_context(&self, taxonomy: &Taxonomy) -> TaxonomyPageContext {
        let terms: Vec<TermInfo> = taxonomy.get_sorted_terms()
            .iter()
            .map(|term| TermInfo {
                name: term.name.clone(),
                slug: term.slug.clone(),
                url: format!("/{}/{}/", taxonomy.name, term.slug),
                page_count: term.pages.len(),
            })
            .collect();

        TaxonomyPageContext {
            taxonomy_name: taxonomy.name.clone(),
            taxonomy_singular: taxonomy.singular.clone(),
            terms,
        }
    }

    /// 构建术语页面上下文
    fn build_term_context(&self, taxonomy: &Taxonomy, term: &TaxonomyTerm) -> TermPageContext {
        let pages: Vec<PageInfo> = term.pages
            .iter()
            .map(|page| PageInfo {
                title: page.title().unwrap_or("Untitled").to_string(),
                url: self.get_page_url(page),
                date: page.frontmatter.date.clone(),
            })
            .collect();

        TermPageContext {
            term_name: term.name.clone(),
            term_slug: term.slug.clone(),
            taxonomy_name: taxonomy.name.clone(),
            taxonomy_singular: taxonomy.singular.clone(),
            pages,
        }
    }

    /// 获取页面 URL
    fn get_page_url(&self, page: &HugoPage) -> String {
        let relative_path = page.relative_path.to_string_lossy().to_string();
        let html_path = relative_path.replace(".md", ".html");
        format!("/{}", html_path)
    }

    /// 渲染分类页面 HTML
    fn render_taxonomy_page(&self, context: &TaxonomyPageContext) -> String {
        let terms_html: Vec<String> = context.terms
            .iter()
            .map(|term| format!(
                r#"<li><a href="{}">{}</a> ({})</li>"#,
                term.url, term.name, term.page_count
            ))
            .collect();

        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} - {}</title>
</head>
<body>
    <h1>All {}</h1>
    <ul>
        {}
    </ul>
</body>
</html>"#,
            context.taxonomy_name,
            "Site",
            context.taxonomy_name,
            terms_html.join("\n        ")
        )
    }

    /// 渲染术语页面 HTML
    fn render_term_page(&self, context: &TermPageContext) -> String {
        let pages_html: Vec<String> = context.pages
            .iter()
            .map(|page| {
                let date_str = page.date.as_ref().map(|d| format!(" - {}", d)).unwrap_or_default();
                format!(
                    r#"<li><a href="{}">{}{}</a></li>"#,
                    page.url, page.title, date_str
                )
            })
            .collect();

        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}: {} - {}</title>
</head>
<body>
    <h1>{}: {}</h1>
    <p>{} pages</p>
    <ul>
        {}
    </ul>
</body>
</html>"#,
            context.taxonomy_singular,
            context.term_name,
            "Site",
            context.taxonomy_singular,
            context.term_name,
            context.pages.len(),
            pages_html.join("\n        ")
        )
    }
}

/// 分类法生成器错误
#[derive(Debug)]
pub enum TaxonomyGeneratorError {
    /// 文件系统错误
    IoError(std::io::Error),
}

impl From<std::io::Error> for TaxonomyGeneratorError {
    fn from(error: std::io::Error) -> Self {
        TaxonomyGeneratorError::IoError(error)
    }
}

impl std::fmt::Display for TaxonomyGeneratorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaxonomyGeneratorError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for TaxonomyGeneratorError {}

/// 分类法生成器结果类型
pub type Result<T> = std::result::Result<T, TaxonomyGeneratorError>;
