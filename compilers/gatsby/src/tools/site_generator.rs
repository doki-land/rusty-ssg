//! 站点生成模块
//! 提供静态站点生成的核心功能

use crate::{
    tools::theme::{DefaultTheme, LocaleInfo, NavItem, PageContext, SidebarGroup, SidebarLink},
    types::{GatsbyConfig, Result},
};
use nargo_types::Document;
use std::{collections::HashMap, fs, path::PathBuf};

/// 语言分组的文档映射
pub type LanguageDocuments = HashMap<String, HashMap<String, Document>>;

/// 静态站点生成器
pub struct StaticSiteGenerator {
    /// 配置
    config: GatsbyConfig,
    /// 默认主题
    theme: DefaultTheme,
}

impl StaticSiteGenerator {
    /// 创建新的静态站点生成器
    pub fn new(config: GatsbyConfig) -> Result<Self> {
        let theme = DefaultTheme::new(config.clone())?;

        Ok(Self { config, theme })
    }

    /// 生成静态站点
    pub fn generate(&mut self, documents: &HashMap<String, Document>, output_dir: &PathBuf) -> Result<()> {
        if !output_dir.exists() {
            fs::create_dir_all(output_dir)?;
        }

        let default_lang = self.get_default_language();

        let mut all_docs_by_lang: HashMap<String, Vec<(String, Document)>> = HashMap::new();

        for (path, doc) in documents {
            let (lang, _) = self.extract_language_from_path(path, &default_lang);
            all_docs_by_lang.entry(lang).or_default().push((path.clone(), doc.clone()));
        }

        for (lang, docs) in all_docs_by_lang {
            let nav_items = self.generate_nav_items(&lang);

            let mut all_sidebar_links = Vec::new();

            for (path, doc) in &docs {
                let title = doc
                    .title()
                    .unwrap_or_else(|| {
                        let file_name = path.split('/').last().unwrap_or(path);
                        file_name.strip_suffix(".md").unwrap_or(file_name)
                    })
                    .to_string();

                let (_, normalized_path) = self.extract_language_from_path(path, &default_lang);
                let html_path = normalized_path.replace(".md", ".html");
                all_sidebar_links.push((title, html_path));
            }

            for (path, doc) in &docs {
                let (_, normalized_path) = self.extract_language_from_path(path, &default_lang);
                let html_path = normalized_path.replace(".md", ".html");
                let full_html_path = format!("{}/{}", lang, html_path);
                let output_path = output_dir.join(&full_html_path);

                if let Some(parent) = output_path.parent() {
                    if !parent.exists() {
                        fs::create_dir_all(parent)?;
                    }
                }

                let depth = normalized_path.matches('/').count();
                let root_path = if depth == 0 { "./".to_string() } else { "../".repeat(depth) };

                let mut sidebar_links = Vec::new();
                for (title, link) in &all_sidebar_links {
                    let relative_link = format!("{}{}", root_path, link);
                    sidebar_links.push(SidebarLink { text: title.clone(), link: relative_link });
                }

                let sidebar_group = SidebarGroup { text: "文档".to_string(), items: sidebar_links };
                let sidebar_groups = vec![sidebar_group];

                let html_content = self.render_page_for_file(
                    doc,
                    &lang,
                    &nav_items,
                    &sidebar_groups,
                    normalized_path.clone(),
                    html_path.clone(),
                )?;

                fs::write(&output_path, html_content)?;
            }
        }

        self.generate_root_index(output_dir)?;

        Ok(())
    }

    /// 生成根目录 index.html，重定向到默认语言页面
    fn generate_root_index(&self, output_dir: &PathBuf) -> Result<()> {
        let default_lang = self.get_default_language();
        let redirect_url = format!("./{}/index.html", default_lang);

        let html = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <meta http-equiv="refresh" content="0; url={redirect_url}" />
    <title>Redirecting...</title>
</head>
<body>
    <p>Redirecting to documentation... <a href="{redirect_url}">Click here if not redirected</a></p>
</body>
</html>
"#
        );

        let root_index_path = output_dir.join("index.html");
        fs::write(root_index_path, html)?;

        Ok(())
    }

    /// 按语言分组文档
    fn group_documents_by_language(&self, documents: &HashMap<String, Document>) -> LanguageDocuments {
        let mut result = LanguageDocuments::new();
        let default_lang = self.get_default_language();

        for (path, document) in documents {
            let (lang, normalized_path) = self.extract_language_from_path(path, &default_lang);

            result.entry(lang).or_insert_with(HashMap::new).insert(normalized_path, document.clone());
        }

        result
    }

    /// 从路径中提取语言代码
    fn extract_language_from_path(&self, path: &str, default_lang: &str) -> (String, String) {
        let parts: Vec<&str> = path.split('/').collect();

        if parts.is_empty() {
            return (default_lang.to_string(), path.to_string());
        }

        let first_part = parts[0];

        if first_part.contains('-') || first_part.len() == 2 {
            let normalized_path = parts[1..].join("/");
            (first_part.to_string(), if normalized_path.is_empty() { "index.md".to_string() } else { normalized_path })
        }
        else {
            (default_lang.to_string(), path.to_string())
        }
    }

    /// 获取默认语言
    fn get_default_language(&self) -> String {
        "zh-hans".to_string()
    }

    /// 获取可用的语言列表
    fn get_available_locales(&self) -> Vec<(String, String)> {
        vec![]
    }

    /// 渲染单个页面
    fn render_page(
        &self,
        document: &Document,
        current_lang: &str,
        nav_items: &[NavItem],
        sidebar_groups: &[SidebarGroup],
        current_path: String,
    ) -> Result<String> {
        let doc_title = document.title().unwrap_or("");
        let site_title = self.theme.site_title();

        let page_title = if !doc_title.is_empty() { format!("{} | {}", doc_title, site_title) } else { site_title.to_string() };

        let content = document.rendered_content.as_deref().unwrap_or("");

        let (has_footer, has_footer_message, footer_message, has_footer_copyright, footer_copyright) =
            (false, false, String::new(), false, String::new());

        let locale_infos: Vec<LocaleInfo> = vec![];

        let depth = current_path.matches('/').count();
        let root_path = if depth == 0 { "./".to_string() } else { "../".repeat(depth) };

        let context = PageContext {
            page_title,
            site_title: site_title.to_string(),
            content: content.to_string(),
            nav_items: nav_items.to_vec(),
            sidebar_groups: sidebar_groups.to_vec(),
            current_path,
            has_footer,
            has_footer_message,
            footer_message,
            has_footer_copyright,
            footer_copyright,
            current_lang: current_lang.to_string(),
            available_locales: locale_infos,
            root_path: root_path.clone(),
        };

        self.theme.render_page(&context)
    }

    /// 生成导航栏项目
    fn generate_nav_items(&self, _lang: &str) -> Vec<NavItem> {
        Vec::new()
    }

    /// 获取输出文件路径
    fn get_output_path(&self, source_path: &str, output_dir: &PathBuf, lang: &str) -> PathBuf {
        let html_path = self.get_html_path(source_path);
        output_dir.join(lang).join(html_path.trim_start_matches('/'))
    }

    /// 获取 HTML 文件路径（相对路径）
    fn get_html_path(&self, source_path: &str) -> String {
        source_path.replace(".md", ".html")
    }

    /// 简单版本的侧边栏生成
    fn generate_sidebar_groups_simple(&self, documents: &HashMap<String, Document>, lang: &str) -> Vec<SidebarGroup> {
        let mut groups = Vec::new();
        let mut default_group = SidebarGroup { text: "文档".to_string(), items: Vec::new() };

        for (path, doc) in documents {
            let title = doc.title().unwrap_or(path).to_string();
            let link = format!("{}/{}", lang, path).replace(".md", ".html");
            default_group.items.push(SidebarLink { text: title, link });
        }

        groups.push(default_group);
        groups
    }

    /// 为单个文件渲染页面
    fn render_page_for_file(
        &self,
        document: &Document,
        current_lang: &str,
        nav_items: &[NavItem],
        sidebar_groups: &[SidebarGroup],
        current_full_path: String,
        _current_html_path: String,
    ) -> Result<String> {
        let doc_title = document.title().unwrap_or("");
        let site_title = self.theme.site_title();

        let page_title = if !doc_title.is_empty() { format!("{} | {}", doc_title, site_title) } else { site_title.to_string() };

        let content = document.rendered_content.as_deref().unwrap_or("");

        let (has_footer, has_footer_message, footer_message, has_footer_copyright, footer_copyright) =
            (false, false, String::new(), false, String::new());

        let locale_infos: Vec<LocaleInfo> = vec![];

        let context = PageContext {
            page_title,
            site_title: site_title.to_string(),
            content: content.to_string(),
            nav_items: nav_items.to_vec(),
            sidebar_groups: sidebar_groups.to_vec(),
            current_path: current_full_path,
            has_footer,
            has_footer_message,
            footer_message,
            has_footer_copyright,
            footer_copyright,
            current_lang: current_lang.to_string(),
            available_locales: locale_infos,
            root_path: "".to_string(),
        };

        self.theme.render_page(&context)
    }
}

/// 配置加载器
pub struct ConfigLoader;

impl ConfigLoader {
    /// 从文件加载配置
    ///
    /// # Arguments
    ///
    /// * `path` - 配置文件的路径
    ///
    /// # Errors
    ///
    /// 返回错误如果文件读取、解析或验证失败
    pub fn load_from_file(path: &PathBuf) -> Result<GatsbyConfig> {
        Ok(GatsbyConfig::load_from_file(path)?)
    }

    /// 从目录查找并加载配置
    ///
    /// 按以下顺序查找配置文件：
    /// 1. gatsby-config.js
    /// 2. gatsby-config.json
    /// 3. gatsby-config.yaml
    /// 4. gatsby-config.yml
    /// 5. gatsby-config.toml
    ///
    /// # Arguments
    ///
    /// * `dir` - 要搜索的目录路径
    ///
    /// # Errors
    ///
    /// 返回错误如果配置文件读取、解析或验证失败
    pub fn load_from_dir(dir: &PathBuf) -> Result<GatsbyConfig> {
        Ok(GatsbyConfig::load_from_dir(dir)?)
    }
}
