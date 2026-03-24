//! 站点生成模块
//! 提供静态站点生成的核心功能，支持多语言文档

use crate::{
    Document,
    config::types::{SiteLocaleData, Theme},
    tools::{
        Result,
        theme::{DefaultTheme, LocaleInfo, NavItem, PageContext, SidebarGroup, SidebarLink},
    },
    types::VuePressConfig,
};
use std::{collections::HashMap, fs, path::PathBuf};

/// 语言分组的文档映射
pub type LanguageDocuments = HashMap<String, HashMap<String, Document>>;

/// 静态站点生成器
pub struct StaticSiteGenerator {
    /// 配置
    config: VuePressConfig,
    /// 主题
    theme: Box<dyn crate::tools::theme::Theme>,
}

impl StaticSiteGenerator {
    /// 创建新的静态站点生成器
    pub fn new(config: VuePressConfig) -> Result<Self> {
        let theme: Box<dyn crate::tools::theme::Theme> =
            Box::new(DefaultTheme::new(config.clone())?) as Box<dyn crate::tools::theme::Theme>;

        Ok(Self { config, theme })
    }

    /// 生成静态站点
    pub fn generate(&mut self, documents: &HashMap<String, Document>, output_dir: &PathBuf) -> Result<()> {
        if !output_dir.exists() {
            fs::create_dir_all(output_dir)?;
        }

        let locales = self.get_available_locales();
        let default_lang = self.get_default_language();

        let mut lang_keys: Vec<String> = Vec::new();
        let mut lang_documents: HashMap<String, Vec<(String, Document)>> = HashMap::new();

        for (path, doc) in documents {
            let (lang, _) = self.extract_language_from_path(path, &default_lang);
            if !lang_keys.contains(&lang) {
                lang_keys.push(lang.clone());
            }
            lang_documents.entry(lang).or_default().push((path.clone(), doc.clone()));
        }

        // 生成路由配置
        self.generate_routes_config(output_dir, &lang_documents, &default_lang)?;

        // 生成搜索索引
        self.generate_search_index(output_dir, documents)?;

        // 生成站点地图
        self.generate_sitemap(output_dir, documents, &default_lang)?;

        for lang in lang_keys {
            let docs = lang_documents.remove(&lang).unwrap();
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
                    &locales,
                    normalized_path.clone(),
                    html_path.clone(),
                )?;

                fs::write(&output_path, html_content)?;
            }
        }

        self.generate_root_index(output_dir)?;
        self.generate_404_page(output_dir)?;
        self.generate_robots_txt(output_dir)?;

        Ok(())
    }

    /// 生成路由配置文件
    fn generate_routes_config(
        &self,
        output_dir: &PathBuf,
        lang_documents: &HashMap<String, Vec<(String, Document)>>,
        default_lang: &str,
    ) -> Result<()> {
        use serde_json::json;

        let mut routes = Vec::new();

        for (lang, docs) in lang_documents {
            for (path, doc) in docs {
                let (_, normalized_path) = self.extract_language_from_path(path, default_lang);
                let html_path = normalized_path.replace(".md", ".html");
                let route_path = if lang == default_lang {
                    format!("/{}", html_path.replace(".html", ""))
                }
                else {
                    format!("/{}/{}", lang, html_path.replace(".html", ""))
                };

                let route = json!({
                    "path": route_path,
                    "component": "@theme/layouts/Page.vue",
                    "meta": {
                        "title": doc.title().unwrap_or(""),
                        "lang": lang
                    }
                });

                routes.push(route);
            }
        }

        let routes_config = json!({
            "routes": routes
        });

        let routes_path = output_dir.join(".vuepress").join("routes.json");
        if let Some(parent) = routes_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        fs::write(routes_path, serde_json::to_string_pretty(&routes_config)?)?;

        Ok(())
    }

    /// 生成搜索索引
    fn generate_search_index(&self, output_dir: &PathBuf, documents: &HashMap<String, Document>) -> Result<()> {
        use serde_json::json;

        let mut search_index = Vec::new();

        for (path, doc) in documents {
            let item = json!({
                "path": path,
                "title": doc.title().unwrap_or(""),
                "content": doc.content
            });

            search_index.push(item);
        }

        let search_config = json!({
            "index": search_index
        });

        let search_path = output_dir.join(".vuepress").join("search-index.json");
        if let Some(parent) = search_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        fs::write(search_path, serde_json::to_string_pretty(&search_config)?)?;

        Ok(())
    }

    /// 生成站点地图
    fn generate_sitemap(&self, output_dir: &PathBuf, documents: &HashMap<String, Document>, default_lang: &str) -> Result<()> {
        let mut sitemap = String::from(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">",
        );

        for (path, doc) in documents {
            let (lang, normalized_path) = self.extract_language_from_path(path, default_lang);
            let html_path = normalized_path.replace(".md", ".html");
            let url = format!("/{}/{}", lang, html_path);

            sitemap.push_str(&format!(
                "<url><loc>{}</loc><lastmod>{}</lastmod></url>",
                url,
                chrono::Utc::now().format("%Y-%m-%d").to_string()
            ));
        }

        sitemap.push_str("</urlset>");

        let sitemap_path = output_dir.join("sitemap.xml");
        fs::write(sitemap_path, sitemap)?;

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

    /// 生成 404 页面
    fn generate_404_page(&self, output_dir: &PathBuf) -> Result<()> {
        let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>404 - Page Not Found</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
            line-height: 1.6;
            color: #333;
            background: #f5f5f5;
            display: flex;
            justify-content: center;
            align-items: center;
            min-height: 100vh;
            margin: 0;
        }
        .container {
            text-align: center;
            background: #fff;
            padding: 40px;
            border-radius: 8px;
            box-shadow: 0 1px 3px rgba(0,0,0,0.1);
        }
        h1 {
            font-size: 3rem;
            margin-bottom: 16px;
        }
        p {
            font-size: 1.2rem;
            margin-bottom: 24px;
        }
        a {
            color: #007acc;
            text-decoration: none;
        }
        a:hover {
            text-decoration: underline;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>404</h1>
        <p>Page not found</p>
        <p>The page you are looking for does not exist.</p>
        <a href="/">Go back to home</a>
    </div>
</body>
</html>"#;

        let page_404_path = output_dir.join("404.html");
        fs::write(page_404_path, html)?;

        Ok(())
    }

    /// 生成 robots.txt 文件
    fn generate_robots_txt(&self, output_dir: &PathBuf) -> Result<()> {
        let robots_content = r#"User-agent: *
Allow: /

# Sitemap
Sitemap: /sitemap.xml"#;

        let robots_path = output_dir.join("robots.txt");
        fs::write(robots_path, robots_content)?;

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

        if self.config.locales.contains_key(first_part) || first_part.contains('-') || first_part.len() == 2 {
            let normalized_path = parts[1..].join("/");
            (first_part.to_string(), if normalized_path.is_empty() { "index.md".to_string() } else { normalized_path })
        }
        else {
            (default_lang.to_string(), path.to_string())
        }
    }

    /// 获取默认语言
    fn get_default_language(&self) -> String {
        if let Some((lang, _)) = self.config.locales.iter().find(|(_, cfg)| cfg.default.unwrap_or(false)) {
            return lang.to_string();
        }

        self.config.locales.keys().next().cloned().unwrap_or_else(|| "zh-hans".to_string())
    }

    /// 获取可用的语言列表
    fn get_available_locales(&self) -> Vec<(String, crate::types::LocaleConfig)> {
        self.config.locales.iter().map(|(k, v)| (k.to_string(), v.clone())).collect()
    }

    /// 渲染单个页面
    fn render_page(
        &self,
        document: &Document,
        current_lang: &str,
        nav_items: &[NavItem],
        sidebar_groups: &[SidebarGroup],
        locales: &[(String, crate::types::LocaleConfig)],
        current_path: String,
    ) -> Result<String> {
        let doc_title = document.title().unwrap_or("");
        let site_title = self.theme.site_title();

        let page_title = if !doc_title.is_empty() { format!("{} | {}", doc_title, site_title) } else { site_title.to_string() };

        let content = document.rendered_content.as_deref().unwrap_or("");

        let (has_footer, has_footer_message, footer_message, has_footer_copyright, footer_copyright) =
            if let Some(footer) = &self.config.theme.footer {
                (
                    true,
                    footer.message.is_some(),
                    footer.message.as_deref().unwrap_or("").to_string(),
                    footer.copyright.is_some(),
                    footer.copyright.as_deref().unwrap_or("").to_string(),
                )
            }
            else {
                (false, false, String::new(), false, String::new())
            };

        let locale_infos: Vec<LocaleInfo> = locales
            .iter()
            .map(|(code, config)| LocaleInfo {
                code: code.to_string(),
                label: config.label.clone(),
                is_current: code == current_lang,
            })
            .collect();

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
    fn generate_nav_items(&self, lang: &str) -> Vec<NavItem> {
        let nav_source = if let Some(locale_config) = self.config.locales.get(lang) {
            if let Some(ref nav) = locale_config.nav { nav } else { &self.config.theme.nav }
        }
        else {
            &self.config.theme.nav
        };

        nav_source
            .iter()
            .map(|item| NavItem { text: item.text.clone(), link: item.link.as_deref().unwrap_or("#").to_string() })
            .collect()
    }

    /// 生成侧边栏组
    fn generate_sidebar_groups(&self, documents: &HashMap<String, Document>, lang: &str) -> Vec<SidebarGroup> {
        let mut groups = Vec::new();

        let sidebar_source = if let Some(locale_config) = self.config.locales.get(lang) {
            if let Some(ref sidebar) = locale_config.sidebar { sidebar } else { &self.config.theme.sidebar }
        }
        else {
            &self.config.theme.sidebar
        };

        if let Some(sidebar_config) = sidebar_source.get("/") {
            for item in sidebar_config {
                let mut group = SidebarGroup { text: item.text.clone(), items: Vec::new() };

                if let Some(items) = &item.items {
                    for sub_item in items {
                        if let Some(link) = &sub_item.link {
                            let normalized_link =
                                if link.starts_with('/') { format!("{}{}", lang, link) } else { format!("{}/{}", lang, link) }
                                    .replace(".md", ".html");

                            group.items.push(SidebarLink { text: sub_item.text.clone(), link: normalized_link });
                        }
                    }
                }

                groups.push(group);
            }
        }

        if groups.is_empty() {
            let mut default_group = SidebarGroup { text: "文档".to_string(), items: Vec::new() };

            for (path, doc) in documents {
                let title = doc.title().unwrap_or(path).to_string();
                let full_path = format!("{}/{}", lang, path).replace(".md", ".html");
                default_group.items.push(SidebarLink { text: title, link: full_path });
            }

            groups.push(default_group);
        }

        groups
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
        locales: &[(String, crate::types::LocaleConfig)],
        current_full_path: String,
        current_html_path: String,
    ) -> Result<String> {
        let doc_title = document.title().unwrap_or("");
        let site_title = self.theme.site_title();

        let page_title = if !doc_title.is_empty() { format!("{} | {}", doc_title, site_title) } else { site_title.to_string() };

        let content = document.rendered_content.as_deref().unwrap_or("");

        let (has_footer, has_footer_message, footer_message, has_footer_copyright, footer_copyright) =
            if let Some(footer) = &self.config.theme.footer {
                (
                    true,
                    footer.message.is_some(),
                    footer.message.as_deref().unwrap_or("").to_string(),
                    footer.copyright.is_some(),
                    footer.copyright.as_deref().unwrap_or("").to_string(),
                )
            }
            else {
                (false, false, String::new(), false, String::new())
            };

        let locale_infos: Vec<LocaleInfo> = locales
            .iter()
            .map(|(code, config)| LocaleInfo {
                code: code.to_string(),
                label: config.label.clone(),
                is_current: code == current_lang,
            })
            .collect();

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
    pub fn load_from_file(path: &PathBuf) -> Result<VuePressConfig> {
        Ok(VuePressConfig::load_from_file(path)?)
    }

    /// 从目录查找并加载配置
    ///
    /// 按以下顺序查找配置文件：
    /// 1. vuepress.config.toml
    /// 2. vuepress.config.json
    ///
    /// # Arguments
    ///
    /// * `dir` - 要搜索的目录路径
    ///
    /// # Errors
    ///
    /// 返回错误如果配置文件读取、解析或验证失败
    pub fn load_from_dir(dir: &PathBuf) -> Result<VuePressConfig> {
        Ok(VuePressConfig::load_from_dir(dir)?)
    }
}
