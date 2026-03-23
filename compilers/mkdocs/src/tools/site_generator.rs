//! 站点生成模块
//! 提供静态站点生成的核心功能

use crate::{
    Result,
    tools::theme::{
        DefaultTheme, PageContext,
        default_theme::{ThemeNavItem, ThemeSidebarGroup, ThemeSidebarLink},
    },
    types::MkDocsConfig,
};
use std::{collections::HashMap, fs, path::PathBuf};

/// 静态站点生成器
pub struct StaticSiteGenerator {
    /// 配置
    config: MkDocsConfig,
    /// 默认主题
    theme: DefaultTheme,
}

impl StaticSiteGenerator {
    /// 创建新的静态站点生成器
    pub fn new(config: MkDocsConfig) -> Result<Self> {
        let theme = DefaultTheme::new(config.clone())?;

        Ok(Self { config, theme })
    }

    /// 生成静态站点
    pub fn generate(&mut self, documents: &HashMap<String, String>, output_dir: &PathBuf) -> Result<()> {
        if !output_dir.exists() {
            fs::create_dir_all(output_dir)?;
        }

        let mut all_sidebar_links = Vec::new();

        for (path, content) in documents {
            let title = Self::extract_title(content, path);
            let html_path = self.generate_html_path(path);
            all_sidebar_links.push((title, html_path.clone()));
        }

        for (path, content) in documents {
            let html_path = self.generate_html_path(path);
            let output_path = output_dir.join(&html_path);

            if let Some(parent) = output_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
            }

            let depth = html_path.matches('/').count();
            let root_path = if depth == 0 { "./".to_string() } else { "../".repeat(depth) };

            let mut sidebar_links = Vec::new();
            for (title, link) in &all_sidebar_links {
                let relative_link = format!("{}{}", root_path, link);
                sidebar_links.push(ThemeSidebarLink { text: title.clone(), link: relative_link });
            }

            let sidebar_group = ThemeSidebarGroup { text: "文档".to_string(), items: sidebar_links };
            let sidebar_groups = vec![sidebar_group];

            let nav_items = Self::generate_nav_items(&self.config);

            let html_content = self.render_page_for_file(content, path, &nav_items, &sidebar_groups, html_path.clone())?;

            fs::write(&output_path, html_content)?;
        }

        Ok(())
    }

    /// 生成 HTML 文件路径
    /// 根据 use_directory_urls 配置决定是否使用目录 URL
    fn generate_html_path(&self, path: &str) -> String {
        if self.config.use_directory_urls() {
            // 使用目录 URL: page.md -> page/index.html
            if path.ends_with(".md") {
                let base_path = path.strip_suffix(".md").unwrap_or(path);
                format!("{}/index.html", base_path)
            }
            else {
                path.to_string()
            }
        }
        else {
            // 不使用目录 URL: page.md -> page.html
            path.replace(".md", ".html")
        }
    }

    /// 从内容中提取标题
    fn extract_title(content: &str, path: &str) -> String {
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("# ") {
                return line[2..].trim().to_string();
            }
            else if line.starts_with("title:") {
                let title_part = line[6..].trim();
                let title = if title_part.starts_with('"') || title_part.starts_with('\'') {
                    &title_part[1..title_part.len() - 1]
                }
                else {
                    title_part
                };
                return title.to_string();
            }
        }

        let file_name = path.split('/').last().unwrap_or(path);
        file_name.strip_suffix(".md").unwrap_or(file_name).to_string()
    }

    /// 生成导航栏项目
    fn generate_nav_items(config: &MkDocsConfig) -> Vec<ThemeNavItem> {
        let mut nav_items = Vec::new();

        for item in &config.nav {
            match item {
                crate::types::NavItem::String(text) => {
                    nav_items.push(ThemeNavItem { text: text.clone(), link: "#".to_string(), children: Vec::new() });
                }
                crate::types::NavItem::Map(map) => {
                    for (key, value) in map {
                        match value {
                            crate::types::NavValue::String(s) => {
                                let link = Self::generate_nav_link(s, config.use_directory_urls());
                                nav_items.push(ThemeNavItem { text: key.clone(), link, children: Vec::new() });
                            }
                            crate::types::NavValue::List(items) => {
                                let children = Self::generate_sub_nav_items(items, config.use_directory_urls());
                                nav_items.push(ThemeNavItem { text: key.clone(), link: "#".to_string(), children });
                            }
                        }
                    }
                }
            }
        }

        nav_items
    }

    /// 生成子导航栏项目
    fn generate_sub_nav_items(items: &[crate::types::NavItem], use_directory_urls: bool) -> Vec<ThemeNavItem> {
        let mut sub_items = Vec::new();

        for item in items {
            match item {
                crate::types::NavItem::String(text) => {
                    sub_items.push(ThemeNavItem { text: text.clone(), link: "#".to_string(), children: Vec::new() });
                }
                crate::types::NavItem::Map(map) => {
                    for (key, value) in map {
                        match value {
                            crate::types::NavValue::String(s) => {
                                let link = Self::generate_nav_link(s, use_directory_urls);
                                sub_items.push(ThemeNavItem { text: key.clone(), link, children: Vec::new() });
                            }
                            crate::types::NavValue::List(grandchildren) => {
                                let children = Self::generate_sub_nav_items(grandchildren, use_directory_urls);
                                sub_items.push(ThemeNavItem { text: key.clone(), link: "#".to_string(), children });
                            }
                        }
                    }
                }
            }
        }

        sub_items
    }

    /// 生成导航链接
    fn generate_nav_link(path: &str, use_directory_urls: bool) -> String {
        if use_directory_urls && path.ends_with(".md") {
            // 使用目录 URL: page.md -> page/
            path.strip_suffix(".md").unwrap_or(path).to_string() + "/"
        }
        else if path.ends_with(".md") {
            // 不使用目录 URL: page.md -> page.html
            path.replace(".md", ".html")
        }
        else {
            path.to_string()
        }
    }

    /// 为单个文件渲染页面
    fn render_page_for_file(
        &self,
        content: &str,
        current_full_path: &str,
        nav_items: &[ThemeNavItem],
        sidebar_groups: &[ThemeSidebarGroup],
        current_html_path: String,
    ) -> Result<String> {
        let doc_title = Self::extract_title(content, current_full_path);
        let site_title = self.theme.site_title();

        let page_title = if !doc_title.is_empty() { format!("{} | {}", doc_title, site_title) } else { site_title.to_string() };

        let html_content = Self::simple_markdown_to_html(content);

        let (has_footer, has_footer_message, footer_message, has_footer_copyright, footer_copyright) =
            (false, false, String::new(), false, String::new());

        let context = PageContext {
            page_title,
            site_title: site_title.to_string(),
            content: html_content,
            nav_items: nav_items.to_vec(),
            sidebar_groups: sidebar_groups.to_vec(),
            current_path: current_full_path.to_string(),
            has_footer,
            has_footer_message,
            footer_message,
            has_footer_copyright,
            footer_copyright,
            current_lang: "zh-CN".to_string(),
            available_locales: Vec::new(),
            root_path: "".to_string(),
        };

        self.theme.render_page(&context)
    }

    /// 简单的 Markdown 到 HTML 转换
    fn simple_markdown_to_html(content: &str) -> String {
        let mut html = String::new();
        let mut in_code_block = false;

        for line in content.lines() {
            let line = line.trim();

            if line.starts_with("---") && !in_code_block {
                continue;
            }

            if line.starts_with("```") {
                in_code_block = !in_code_block;
                if in_code_block {
                    html.push_str("<pre><code>");
                }
                else {
                    html.push_str("</code></pre>\n");
                }
                continue;
            }

            if in_code_block {
                html.push_str(&html_escape(line));
                html.push('\n');
                continue;
            }

            if line.is_empty() {
                html.push_str("<p></p>\n");
                continue;
            }

            if line.starts_with("# ") {
                html.push_str(&format!("<h1>{}</h1>\n", &line[2..]));
            }
            else if line.starts_with("## ") {
                html.push_str(&format!("<h2>{}</h2>\n", &line[3..]));
            }
            else if line.starts_with("### ") {
                html.push_str(&format!("<h3>{}</h3>\n", &line[4..]));
            }
            else if line.starts_with("- ") || line.starts_with("* ") {
                html.push_str(&format!("<li>{}</li>\n", &line[2..]));
            }
            else if line.starts_with("> ") {
                html.push_str(&format!("<blockquote>{}</blockquote>\n", &line[2..]));
            }
            else {
                html.push_str(&format!("<p>{}</p>\n", line));
            }
        }

        html
    }
}

/// HTML 转义
fn html_escape(s: &str) -> String {
    s.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;").replace("\"", "&quot;").replace("'", "&#39;")
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
    pub fn load_from_file(path: &PathBuf) -> Result<MkDocsConfig> {
        Ok(MkDocsConfig::load_from_file(path)?)
    }

    /// 从目录查找并加载配置
    ///
    /// 按以下顺序查找配置文件：
    /// 1. mkdocs.yml
    /// 2. mkdocs.yaml
    ///
    /// # Arguments
    ///
    /// * `dir` - 要搜索的目录路径
    ///
    /// # Errors
    ///
    /// 返回错误如果配置文件读取、解析或验证失败
    pub fn load_from_dir(dir: &PathBuf) -> Result<MkDocsConfig> {
        Ok(MkDocsConfig::load_from_dir(dir)?)
    }
}
