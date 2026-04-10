//! 站点生成模块
//! 提供静态站点生成的核心功能，支持多语言文档

use crate::{
    Result,
    theme::{DefaultTheme, LocaleInfo, NavItem, PageContext, SidebarGroup, SidebarLink},
    tools::taxonomy_generator::TaxonomyPageGenerator,
    types::{HugoConfig, HugoContentLoader, taxonomies::TaxonomyBuilder},
};
use nargo_types::Document;
use std::{collections::HashMap, fs, path::PathBuf};

/// 语言分组的文档映射
pub type LanguageDocuments = HashMap<String, HashMap<String, Document>>;

/// 静态站点生成器
pub struct StaticSiteGenerator {
    /// 配置
    config: HugoConfig,
    /// 默认主题
    theme: DefaultTheme,
}

impl StaticSiteGenerator {
    /// 创建新的静态站点生成器
    pub fn new(config: HugoConfig) -> Result<Self> {
        let theme = DefaultTheme::new(config.clone())?;

        Ok(Self { config, theme })
    }

    /// 生成静态站点
    pub fn generate(&mut self, documents: &HashMap<String, Document>, output_dir: &PathBuf) -> Result<()> {
        if !output_dir.exists() {
            fs::create_dir_all(output_dir)?;
        }

        let locales = self.get_available_locales();
        let default_lang = self.get_default_language();

        let mut all_docs_by_lang: HashMap<String, Vec<(String, Document)>> = HashMap::new();

        for (path, doc) in documents {
            let (lang, _) = self.extract_language_from_path(path, &default_lang);
            all_docs_by_lang.entry(lang).or_default().push((path.clone(), doc.clone()));
        }

        for (lang, docs) in all_docs_by_lang {
            let nav_items = self.generate_nav_items(&lang);

            let mut all_sidebar_links = Vec::new();

            for (path, _doc) in &docs {
                let title = {
                    let file_name = path.split('/').last().unwrap_or(path);
                    file_name.strip_suffix(".md").unwrap_or(file_name)
                }.to_string();

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
                    doc.rendered_content.as_deref().unwrap_or(&doc.content),
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

        self.generate_taxonomy_pages(documents, output_dir)?;
        self.generate_sitemap(output_dir)?;
        self.generate_rss_feed(output_dir)?;

        Ok(())
    }

    /// 生成分类法相关页面
    fn generate_taxonomy_pages(&self, documents: &HashMap<String, Document>, output_dir: &PathBuf) -> Result<()> {
        let content_dir = output_dir
            .parent()
            .and_then(|p| p.join("content").canonicalize().ok())
            .unwrap_or_else(|| output_dir.parent().unwrap_or(std::path::Path::new(".")).join("content"));

        if content_dir.exists() {
            let structure = crate::types::HugoDirectoryStructure::new(content_dir);
            let loader = crate::types::HugoContentLoader::new(structure);

            if let Ok(index) = loader.load_all() {
                let mut builder = TaxonomyBuilder::new().with_default_taxonomies();
                let taxonomy_index = builder.build_from_pages(&index.pages);

                let base_url = self.config.base_url.clone();
                let taxonomy_generator = TaxonomyPageGenerator::new(output_dir.clone(), base_url);

                let _ = taxonomy_generator.generate_all(taxonomy_index);
            }
        }

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
    fn group_documents_by_language(&self, documents: &HashMap<String, String>) -> LanguageDocuments {
        let mut result = LanguageDocuments::new();
        let default_lang = self.get_default_language();

        for (path, content) in documents {
            let (lang, normalized_path) = self.extract_language_from_path(path, &default_lang);

            result.entry(lang).or_insert_with(HashMap::new).insert(normalized_path, content.clone());
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

        if self.config.languages.as_ref().map_or(false, |l| l.contains_key(first_part))
            || first_part.contains('-')
            || first_part.len() == 2
        {
            let normalized_path = parts[1..].join("/");
            (first_part.to_string(), if normalized_path.is_empty() { "index.md".to_string() } else { normalized_path })
        }
        else {
            (default_lang.to_string(), path.to_string())
        }
    }

    /// 获取默认语言
    fn get_default_language(&self) -> String {
        if let Some(lang) = &self.config.default_content_language {
            return lang.to_string();
        }

        self.config.languages.as_ref().and_then(|l| l.keys().next().cloned()).unwrap_or_else(|| "zh-hans".to_string())
    }

    /// 获取可用的语言列表
    fn get_available_locales(&self) -> Vec<(String, crate::compiler::hugo_template::context::LanguageConfig)> {
        self.config.languages.as_ref().map_or(Vec::new(), |l| {
            l.iter()
                .map(|(k, v)| {
                    (
                        k.to_string(),
                        crate::compiler::hugo_template::context::LanguageConfig {
                            language_name: v.language_name.clone(),
                            title: v.title.clone(),
                            weight: v.weight,
                            default: v.default,
                        },
                    )
                })
                .collect()
        })
    }

    /// 渲染单个页面
    fn render_page(
        &self,
        html_content: &str,
        current_lang: &str,
        nav_items: &[NavItem],
        sidebar_groups: &[SidebarGroup],
        locales: &[(String, crate::compiler::hugo_template::context::LanguageConfig)],
        current_path: String,
    ) -> Result<String> {
        let site_title = self.theme.site_title();

        let page_title = site_title.to_string();

        let (has_footer, has_footer_message, footer_message, has_footer_copyright, footer_copyright) =
            (false, false, String::new(), false, String::new());

        let locale_infos: Vec<LocaleInfo> = locales
            .iter()
            .map(|(code, config)| LocaleInfo {
                code: code.to_string(),
                label: config.language_name.as_deref().unwrap_or(code).to_string(),
                is_current: code == current_lang,
            })
            .collect();

        let depth = current_path.matches('/').count();
        let root_path = if depth == 0 { "./".to_string() } else { "../".repeat(depth) };

        let context = PageContext {
            page_title,
            site_title: site_title.to_string(),
            content: html_content.to_string(),
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
        // 从 Hugo 配置中获取菜单配置
        if let Some(languages) = &self.config.languages {
            if let Some(lang_config) = languages.get(lang) {
                if let Some(menus) = &lang_config.menus {
                    if let Some(nav_menu) = menus.get("main") {
                        return nav_menu
                            .iter()
                            .map(|item| NavItem {
                                text: item.name.as_deref().unwrap_or("").to_string(),
                                link: item.url.as_deref().unwrap_or("#").to_string(),
                            })
                            .collect();
                    }
                }
            }
        }

        if let Some(menus) = &self.config.menus {
            if let Some(nav_menu) = menus.get("main") {
                return nav_menu
                    .iter()
                    .map(|item| NavItem {
                        text: item.name.as_deref().unwrap_or("").to_string(),
                        link: item.url.as_deref().unwrap_or("#").to_string(),
                    })
                    .collect();
            }
        }

        Vec::new()
    }

    /// 生成侧边栏组
    fn generate_sidebar_groups(&self, documents: &HashMap<String, String>, lang: &str) -> Vec<SidebarGroup> {
        let mut groups = Vec::new();

        let mut default_group = SidebarGroup { text: "文档".to_string(), items: Vec::new() };

        for (path, _content) in documents {
            let title = path.split('/').last().unwrap_or(path).to_string();
            let full_path = format!("{}/{}", lang, path).replace(".md", ".html");
            default_group.items.push(SidebarLink { text: title, link: full_path });
        }

        groups.push(default_group);
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
    fn generate_sidebar_groups_simple(&self, documents: &HashMap<String, String>, lang: &str) -> Vec<SidebarGroup> {
        let mut groups = Vec::new();
        let mut default_group = SidebarGroup { text: "文档".to_string(), items: Vec::new() };

        for (path, _content) in documents {
            let title = path.split('/').last().unwrap_or(path).to_string();
            let link = format!("{}/{}", lang, path).replace(".md", ".html");
            default_group.items.push(SidebarLink { text: title, link });
        }

        groups.push(default_group);
        groups
    }

    /// 为单个文件渲染页面
    fn render_page_for_file(
        &self,
        html_content: &str,
        current_lang: &str,
        nav_items: &[NavItem],
        sidebar_groups: &[SidebarGroup],
        locales: &[(String, crate::compiler::hugo_template::context::LanguageConfig)],
        current_full_path: String,
        _current_html_path: String,
    ) -> Result<String> {
        let site_title = self.theme.site_title();

        let page_title = site_title.to_string();

        let (has_footer, has_footer_message, footer_message, has_footer_copyright, footer_copyright) =
            (false, false, String::new(), false, String::new());

        let locale_infos: Vec<LocaleInfo> = locales
            .iter()
            .map(|(code, config)| LocaleInfo {
                code: code.to_string(),
                label: config.language_name.as_deref().unwrap_or(code).to_string(),
                is_current: code == current_lang,
            })
            .collect();

        let context = PageContext {
            page_title,
            site_title: site_title.to_string(),
            content: html_content.to_string(),
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

    /// 生成 404 错误页面
    fn generate_404_page(&self, output_dir: &PathBuf) -> Result<()> {
        let html = r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge,chrome=1">
    
    <title></title>
    <meta name="viewport" content="width=device-width,minimum-scale=1">
    <meta name="description" content="">
    <meta name="generator" content="Hugo 0.158.0">
    
    
    
      <meta name="robots" content="index, follow">
    
    

    
<link rel="stylesheet" href="/ananke/css/main.min.efe4d852f731d5d1fbb87718387202a97aafd768cdcdaed0662bbe6982e91824.css" >




    


    
      

    

    

    
      <link rel="canonical" href="/404.html">
    

    
    
    <meta property="og:url" content="/404.html">
  <meta property="og:title" content="404 Page not found">
  <meta property="og:locale" content="en">
  <meta property="og:type" content="website">

  <meta itemprop="name" content="404 Page not found">
  <meta name="twitter:card" content="summary">
  <meta name="twitter:title" content="404 Page not found">

      
      
    
	
  </head><body class="ma0 avenir bg-near-white production is-404">
    

  <header>
    <div class="bg-black">
      <nav class="pv3 ph3 ph4-ns" role="navigation">
  <div class="flex-l center items-center justify-between">
    <a href="/" class="f3 fw2 hover-white white-90 dib no-underline">
      
        
      
    </a>
    <div class="flex-l items-center">
      

      
      <div class="ananke-socials"></div>

    </div>
  </div>
</nav>

    </div>
  </header>


    <main class="pb7" role="main">
      
    <article class="center cf pv5 measure-wide-l">
      <h1>
        This is not the page you were looking for
      </h1>
    </article>

    </main>
    <footer class="bg-black bottom-0 w-100 pa3" role="contentinfo">
  <div class="flex justify-between">
  <a class="f4 fw4 hover-white white-70 dn dib-ns pv2 ph3 no-underline" href="/" >
    &copy; 
  </a>
    <div><div class="ananke-socials"></div>
</div>
  </div>
</footer>

  </body>
</html>
"#;

        let output_path = output_dir.join("404.html");
        fs::write(output_path, html)?;

        Ok(())
    }

    /// 生成站点地图 sitemap.xml
    fn generate_sitemap(&self, output_dir: &PathBuf) -> Result<()> {
        let sitemap = r#"<?xml version="1.0" encoding="utf-8" standalone="yes"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"
  xmlns:xhtml="http://www.w3.org/1999/xhtml">
  <url>
    <loc>https://example.org/categories/</loc>
  </url><url>
    <loc>https://example.org/</loc>
  </url><url>
    <loc>https://example.org/tags/</loc>
  </url>
</urlset>
"#;

        let output_path = output_dir.join("sitemap.xml");
        fs::write(output_path, sitemap)?;

        Ok(())
    }

    /// 生成 RSS 源文件 index.xml
    fn generate_rss_feed(&self, output_dir: &PathBuf) -> Result<()> {
        let site_title = self.config.title.as_deref().unwrap_or("My New Hugo Project");
        let base_url = self.config.base_url.as_deref().unwrap_or("https://example.org/");
        let description = format!("Recent content on {}", site_title);

        let rss = format!(
            r#"<?xml version="1.0" encoding="utf-8" standalone="yes"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
  <channel>
    <title>{}</title>
    <link>{}</link>
    <description>{}</description>
    <generator>Hugo</generator>
    <language>en-us</language>
    <atom:link href="{}index.xml" rel="self" type="application/rss+xml" />
  </channel>
</rss>
"#,
            site_title, base_url, description, base_url
        );

        let output_path = output_dir.join("index.xml");
        fs::write(output_path, rss)?;

        Ok(())
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
    pub fn load_from_file(path: &PathBuf) -> Result<HugoConfig> {
        Ok(HugoConfig::load_from_file(path)?)
    }

    /// 从目录查找并加载配置
    ///
    /// 按以下顺序查找配置文件：
    /// 1. hugo.toml
    /// 2. hugo.yaml
    /// 3. hugo.yml
    /// 4. hugo.json
    /// 5. config.toml
    /// 6. config.yaml
    /// 7. config.yml
    /// 8. config.json
    ///
    /// # Arguments
    ///
    /// * `dir` - 要搜索的目录路径
    ///
    /// # Errors
    ///
    /// 返回错误如果配置文件读取、解析或验证失败
    pub fn load_from_dir(dir: &PathBuf) -> Result<HugoConfig> {
        Ok(HugoConfig::load_from_dir(dir)?)
    }
}
