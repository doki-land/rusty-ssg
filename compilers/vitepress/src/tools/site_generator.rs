//! 站点生成模块
//! 提供静态站点生成的核心功能，支持多语言文档

use crate::Result;
use std::{collections::HashMap, fs, path::PathBuf, path::Path};
use crate::types::{LocaleConfig, VutexConfig};
use crate::tools::theme::{DefaultTheme, LocaleInfo, NavItem, PageContext, SidebarGroup, SidebarLink, SocialLink};
use walkdir::WalkDir;
use crate::Document;

/// 语言分组的文档映射
/// 键为语言代码，值为该语言下的文档映射（路径 -> 文档）
pub type LanguageDocuments = HashMap<String, HashMap<String, Document>>;

/// 静态站点生成器
pub struct StaticSiteGenerator {
    /// 配置
    config: VutexConfig,
    /// 默认主题
    theme: DefaultTheme,
    /// 源目录路径
    source_dir: Option<PathBuf>,
}

impl StaticSiteGenerator {
    /// 创建新的静态站点生成器
    ///
    /// # Arguments
    ///
    /// * `config` - 站点配置
    ///
    /// # Returns
    ///
    /// 新的静态站点生成器实例
    pub fn new(config: VutexConfig) -> Result<Self> {
        let theme = DefaultTheme::new(config.clone())?;

        Ok(Self { config, theme, source_dir: None })
    }

    /// 创建带源目录的新的静态站点生成器
    ///
    /// # Arguments
    ///
    /// * `config` - 站点配置
    /// * `source_dir` - 源目录路径
    ///
    /// # Returns
    ///
    /// 新的静态站点生成器实例
    pub fn with_source_dir(config: VutexConfig, source_dir: PathBuf) -> Result<Self> {
        let theme = DefaultTheme::new(config.clone())?;

        Ok(Self { config, theme, source_dir: Some(source_dir) })
    }

    /// 生成静态站点
    ///
    /// # Arguments
    ///
    /// * `documents` - 文档映射（路径 -> 文档）
    /// * `output_dir` - 输出目录路径
    ///
    /// # Errors
    ///
    /// 返回错误如果文件系统操作失败
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

        if let Some(ref source_dir) = self.source_dir {
            self.copy_resources(source_dir, output_dir)?;
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

    /// 从路径中提取语言代码
    fn extract_language_from_path(&self, path: &str, default_lang: &str) -> (String, String) {
        let parts: Vec<&str> = path.split('/').collect();

        if parts.is_empty() {
            return (default_lang.to_string(), path.to_string());
        }

        let first_part = parts[0];
        let locales = self.config.locales.as_ref().map(|l| l.keys().collect::<Vec<_>>()).unwrap_or_default();
        
        if locales.contains(&&first_part.to_string()) || first_part.contains('-') || first_part.len() == 2 {
            let normalized_path = parts[1..].join("/");
            (first_part.to_string(), if normalized_path.is_empty() { "index.md".to_string() } else { normalized_path })
        }
        else {
            (default_lang.to_string(), path.to_string())
        }
    }

    /// 获取默认语言
    fn get_default_language(&self) -> String {
        if let Some(locales) = &self.config.locales {
            if let Some((lang, _)) = locales.iter().find(|(_, cfg)| cfg.default.unwrap_or(false)) {
                return lang.to_string();
            }
            locales.keys().next().cloned().unwrap_or_else(|| "zh-hans".to_string())
        } else {
            "zh-hans".to_string()
        }
    }

    /// 获取可用的语言列表
    fn get_available_locales(&self) -> Vec<(String, LocaleConfig)> {
        if let Some(locales) = &self.config.locales {
            locales.iter().map(|(k, v)| (k.to_string(), v.clone())).collect()
        } else {
            Vec::new()
        }
    }

    /// 为单个文件渲染页面
    fn render_page_for_file(
        &self,
        document: &Document,
        current_lang: &str,
        nav_items: &[NavItem],
        sidebar_groups: &[SidebarGroup],
        locales: &[(String, LocaleConfig)],
        current_full_path: String,
        _current_html_path: String,
    ) -> Result<String> {
        let doc_title = document.title().unwrap_or("");
        let site_title = self.theme.site_title();

        let page_title = if !doc_title.is_empty() { format!("{} | {}", doc_title, site_title) } else { site_title.to_string() };

        let content = document.rendered_content.as_deref().unwrap_or("");

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
            social_links: Vec::new(),
            current_path: current_full_path,
            has_footer: false,
            has_footer_message: false,
            footer_message: String::new(),
            has_footer_copyright: false,
            footer_copyright: String::new(),
            current_lang: current_lang.to_string(),
            available_locales: locale_infos,
            root_path: String::new(),
        };

        self.theme.render_page(&context)
    }

    /// 生成导航栏项目
    fn generate_nav_items(&self, _lang: &str) -> Vec<NavItem> {
        if let Some(ref theme) = self.config.theme {
            if let Some(ref nav) = theme.nav {
                nav.iter()
                    .filter_map(|item| match item {
                        crate::types::NavItem::WithLink(link) => Some(NavItem {
                            text: link.text.clone(),
                            link: link.link.clone(),
                        }),
                        crate::types::NavItem::WithChildren(_) => None,
                    })
                    .collect()
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    }

    /// 复制资源文件到输出目录
    ///
    /// # Arguments
    ///
    /// * `source_dir` - 源目录路径
    /// * `output_dir` - 输出目录路径
    ///
    /// # Errors
    ///
    /// 返回错误如果文件系统操作失败
    fn copy_resources(&self, source_dir: &Path, output_dir: &Path) -> Result<()> {
        let public_dir = if let Some(ref public_path) = self.config.public {
            source_dir.join(public_path)
        } else {
            source_dir.join("public")
        };

        if public_dir.exists() && public_dir.is_dir() {
            self.copy_directory(&public_dir, output_dir)?;
        }

        for entry in WalkDir::new(source_dir) {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                let rel_path = path.strip_prefix(source_dir).unwrap_or(path);
                
                let path_str = rel_path.to_string_lossy();
                let components: Vec<&str> = path_str.split(std::path::MAIN_SEPARATOR).collect();
                
                if components.iter().any(|&c| c == "node_modules" || c == ".git" || c == "dist" || c == ".vitepress" || c == ".vutex") {
                    continue;
                }
                
                if let Some(ext) = path.extension() {
                    let ext_str = ext.to_string_lossy().to_lowercase();
                    
                    if ext_str == "md" {
                        continue;
                    }
                    
                    if self.is_resource_file(&ext_str) {
                        let dest_path = output_dir.join(rel_path);
                        
                        if let Some(parent) = dest_path.parent() {
                            if !parent.exists() {
                                fs::create_dir_all(parent)?;
                            }
                        }
                        
                        fs::copy(path, dest_path)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// 检查文件扩展名是否为资源文件
    ///
    /// # Arguments
    ///
    /// * `ext` - 文件扩展名（小写）
    ///
    /// # Returns
    ///
    /// 如果是资源文件则返回 true，否则返回 false
    fn is_resource_file(&self, ext: &str) -> bool {
        let resource_exts = [
            "css", "js", "jpg", "jpeg", "png", "gif", "svg", "ico", "webp",
            "woff", "woff2", "ttf", "eot", "otf",
            "pdf", "zip", "tar", "gz",
            "html", "htm",
        ];
        
        resource_exts.contains(&ext)
    }

    /// 递归复制目录
    ///
    /// # Arguments
    ///
    /// * `src` - 源目录路径
    /// * `dest` - 目标目录路径
    ///
    /// # Errors
    ///
    /// 返回错误如果文件系统操作失败
    fn copy_directory(&self, src: &Path, dest: &Path) -> Result<()> {
        if !dest.exists() {
            fs::create_dir_all(dest)?;
        }

        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let dest_path = dest.join(entry.file_name());

            if file_type.is_dir() {
                self.copy_directory(&entry.path(), &dest_path)?;
            } else {
                fs::copy(entry.path(), dest_path)?;
            }
        }

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
    pub fn load_from_file(path: &PathBuf) -> Result<VutexConfig> {
        Ok(VutexConfig::load_from_file(path)?)
    }

    /// 从目录查找并加载配置
    ///
    /// 按以下顺序查找配置文件：
    /// 1. vutex.config.toml
    /// 2. vutex.config.json
    ///
    /// # Arguments
    ///
    /// * `dir` - 要搜索的目录路径
    ///
    /// # Errors
    ///
    /// 返回错误如果配置文件读取、解析或验证失败
    pub fn load_from_dir(dir: &PathBuf) -> Result<VutexConfig> {
        Ok(VutexConfig::load_from_dir(dir)?)
    }
}
