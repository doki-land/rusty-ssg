//! 站点生成模块
//! 提供静态站点生成的核心功能，支持多语言文档

use crate::{
    Result,
    tools::theme::{DefaultTheme, LocaleInfo, NavItem, PageContext, SidebarGroup, SidebarLink},
    types::{LocaleConfig, VitePressConfig},
};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

/// 语言分组的文档映射
/// 键为语言代码，值为该语言下的文档映射（路径 -> 文档）
pub type LanguageDocuments = HashMap<String, HashMap<String, String>>;

/// 静态站点生成器，负责将文档转换为完整的静态站点
pub struct StaticSiteGenerator {
    /// 站点配置
    config: VitePressConfig,
    /// 默认主题实例，用于渲染页面
    theme: DefaultTheme,
    /// 源目录路径，用于查找资源文件
    source_dir: Option<PathBuf>,
}

impl StaticSiteGenerator {
    /// 创建新的静态站点生成器实例
    ///
    /// # Arguments
    ///
    /// * `config` - 站点配置，包含所有必要的配置信息
    ///
    /// # Returns
    ///
    /// 新的静态站点生成器实例，如果创建成功则返回 `Ok(Self)`，否则返回错误
    pub fn new(config: VitePressConfig) -> Result<Self> {
        let theme = DefaultTheme::new(config.clone())?;

        Ok(Self { config, theme, source_dir: None })
    }

    /// 创建带源目录的新的静态站点生成器实例
    ///
    /// # Arguments
    ///
    /// * `config` - 站点配置，包含所有必要的配置信息
    /// * `source_dir` - 源目录路径，用于查找和复制资源文件
    ///
    /// # Returns
    ///
    /// 新的静态站点生成器实例，如果创建成功则返回 `Ok(Self)`，否则返回错误
    pub fn with_source_dir(config: VitePressConfig, source_dir: PathBuf) -> Result<Self> {
        let theme = DefaultTheme::new(config.clone())?;

        Ok(Self { config, theme, source_dir: Some(source_dir) })
    }

    /// 生成完整的静态站点
    ///
    /// 该方法会：
    /// 1. 创建输出目录（如果不存在）
    /// 2. 按语言分组文档
    /// 3. 为每种语言生成对应的页面
    /// 4. 生成根目录的重定向页面
    /// 5. 复制资源文件
    ///
    /// # Arguments
    ///
    /// * `documents` - 文档映射，键为文件路径，值为对应的文档对象
    /// * `output_dir` - 输出目录路径，生成的站点将保存到此目录
    ///
    /// # Errors
    ///
    /// 如果文件系统操作失败，返回相应的错误
    pub fn generate(&mut self, documents: &HashMap<String, String>, output_dir: &PathBuf) -> Result<()> {
        if !output_dir.exists() {
            fs::create_dir_all(output_dir)?;
        }

        let locales = self.get_available_locales();
        let default_lang = self.get_default_language();

        let mut all_docs_by_lang: HashMap<String, Vec<(String, String)>> = HashMap::new();

        for (path, content) in documents {
            let (lang, _) = self.extract_language_from_path(path, &default_lang);
            all_docs_by_lang.entry(lang).or_default().push((path.clone(), content.clone()));
        }

        for (lang, docs) in all_docs_by_lang {
            let nav_items = self.generate_nav_items(&lang);

            let mut all_sidebar_links = Vec::new();

            for (path, _content) in &docs {
                let title = {
                    let file_name = path.split('/').last().unwrap_or(path);
                    file_name.strip_suffix(".md").unwrap_or(file_name)
                }.to_string();

                let (_, normalized_path) = self.extract_language_from_path(path, &default_lang);
                let html_path = normalized_path.replace(".md", ".html");
                all_sidebar_links.push((title, html_path));
            }

            let sidebar_items = self.generate_sidebar_items(&lang);

            for (path, content) in &docs {
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

                let sidebar_groups = if !sidebar_items.is_empty() {
                    sidebar_items.clone()
                }
                else {
                    let mut sidebar_links = Vec::new();
                    for (title, link) in &all_sidebar_links {
                        let relative_link = format!("{}{}", root_path, link);
                        sidebar_links.push(SidebarLink { text: title.clone(), link: relative_link });
                    }
                    vec![SidebarGroup { text: "文档".to_string(), items: sidebar_links }]
                };

                let html_content = self.render_page_for_file(
                    content,
                    &lang,
                    &nav_items,
                    &sidebar_groups,
                    &locales,
                    normalized_path.clone(),
                    html_path.clone(),
                    &root_path,
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

    /// 生成根目录 index.html，自动重定向到默认语言的首页
    ///
    /// # Arguments
    ///
    /// * `output_dir` - 输出目录路径
    ///
    /// # Errors
    ///
    /// 如果文件写入失败，返回相应的错误
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

    /// 从文件路径中提取语言代码
    ///
    /// 如果路径的第一个部分是有效的语言代码（在配置的 locales 中存在，或符合语言代码格式），
    /// 则提取该语言代码，否则返回默认语言。
    ///
    /// # Arguments
    ///
    /// * `path` - 文件路径，例如 "zh-CN/index.md" 或 "index.md"
    /// * `default_lang` - 默认语言代码
    ///
    /// # Returns
    ///
    /// 一个元组，包含 (语言代码, 移除语言前缀后的路径)
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

    /// 获取默认语言代码
    ///
    /// 首先查找标记为 `default: true` 的语言，如果没有找到则返回第一个配置的语言，
    /// 如果没有配置任何语言则返回 "zh-hans"。
    ///
    /// # Returns
    ///
    /// 默认语言代码字符串
    fn get_default_language(&self) -> String {
        if let Some(locales) = &self.config.locales {
            if let Some((lang, _)) = locales.iter().find(|(_, cfg)| cfg.default.unwrap_or(false)) {
                return lang.to_string();
            }
            locales.keys().next().cloned().unwrap_or_else(|| "zh-hans".to_string())
        }
        else {
            "zh-hans".to_string()
        }
    }

    /// 获取所有可用的语言配置列表
    ///
    /// # Returns
    ///
    /// 一个向量，包含 (语言代码, 语言配置) 元组
    fn get_available_locales(&self) -> Vec<(String, LocaleConfig)> {
        if let Some(locales) = &self.config.locales {
            locales.iter().map(|(k, v)| (k.to_string(), v.clone())).collect()
        }
        else {
            Vec::new()
        }
    }

    /// 为单个文档文件渲染页面
    ///
    /// # Arguments
    ///
    /// * `document` - 要渲染的文档对象
    /// * `current_lang` - 当前语言代码
    /// * `nav_items` - 导航栏项目列表
    /// * `sidebar_groups` - 侧边栏组列表
    /// * `locales` - 所有可用语言配置列表
    /// * `current_full_path` - 当前页面的完整路径
    /// * `_current_html_path` - 当前页面的 HTML 路径（未使用）
    /// * `root_path` - 相对于根目录的路径前缀
    ///
    /// # Returns
    ///
    /// 渲染后的 HTML 字符串，如果渲染成功则返回 `Ok(String)`，否则返回错误
    fn render_page_for_file(
        &self,
        html_content: &str,
        current_lang: &str,
        nav_items: &[NavItem],
        sidebar_groups: &[SidebarGroup],
        locales: &[(String, LocaleConfig)],
        current_full_path: String,
        _current_html_path: String,
        root_path: &str,
    ) -> Result<String> {
        let site_title = self.get_site_title_for_lang(current_lang);

        let page_title = site_title.to_string();

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
            content: html_content.to_string(),
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
            root_path: root_path.to_string(),
        };

        self.theme.render_page(&context)
    }

    /// 获取指定语言的站点标题
    ///
    /// 如果该语言配置了特定的标题，则返回该标题，否则返回全局配置的标题。
    ///
    /// # Arguments
    ///
    /// * `lang` - 语言代码
    ///
    /// # Returns
    ///
    /// 站点标题字符串
    fn get_site_title_for_lang(&self, lang: &str) -> &str {
        if let Some(locales) = &self.config.locales {
            if let Some(locale_config) = locales.get(lang) {
                if let Some(title) = &locale_config.title {
                    return title;
                }
            }
        }
        self.config.title.as_deref().unwrap_or("VitePress Documentation")
    }

    /// 生成指定语言的导航栏项目
    ///
    /// 首先尝试使用该语言特定的导航栏配置，如果不存在则使用全局配置。
    ///
    /// # Arguments
    ///
    /// * `lang` - 语言代码
    ///
    /// # Returns
    ///
    /// 导航栏项目向量
    fn generate_nav_items(&self, lang: &str) -> Vec<NavItem> {
        let nav_config = if let Some(locales) = &self.config.locales {
            if let Some(locale_config) = locales.get(lang) { locale_config.nav.as_ref() } else { None }
        }
        else {
            None
        };

        let nav_config = nav_config.or_else(|| self.config.theme.as_ref().and_then(|theme| theme.nav.as_ref()));

        if let Some(nav) = nav_config {
            nav.iter()
                .filter_map(|item| match item {
                    crate::types::NavItem::WithLink(link) => Some(NavItem { text: link.text.clone(), link: link.link.clone() }),
                    crate::types::NavItem::WithChildren(_) => None,
                })
                .collect()
        }
        else {
            Vec::new()
        }
    }

    /// 生成指定语言的侧边栏项目
    ///
    /// 首先尝试使用该语言特定的侧边栏配置，如果不存在则使用全局配置。
    ///
    /// # Arguments
    ///
    /// * `lang` - 语言代码
    ///
    /// # Returns
    ///
    /// 侧边栏组向量
    fn generate_sidebar_items(&self, lang: &str) -> Vec<SidebarGroup> {
        let sidebar_config = if let Some(locales) = &self.config.locales {
            if let Some(locale_config) = locales.get(lang) { locale_config.sidebar.as_ref() } else { None }
        }
        else {
            None
        };

        let sidebar_config = sidebar_config.or_else(|| self.config.theme.as_ref().and_then(|theme| theme.sidebar.as_ref()));

        if let Some(sidebar) = sidebar_config {
            match sidebar {
                crate::types::Sidebar::List(items) => self.convert_sidebar_items(items),
                crate::types::Sidebar::Multi(_) => Vec::new(),
            }
        }
        else {
            Vec::new()
        }
    }

    /// 将配置中的侧边栏项转换为主题可用的侧边栏组
    ///
    /// # Arguments
    ///
    /// * `items` - 配置中的侧边栏项列表
    ///
    /// # Returns
    ///
    /// 转换后的侧边栏组向量
    fn convert_sidebar_items(&self, items: &[crate::types::SidebarItem]) -> Vec<SidebarGroup> {
        let mut groups = Vec::new();
        let mut current_group = SidebarGroup { text: String::new(), items: Vec::new() };
        let mut has_items = false;

        for item in items {
            if let Some(text) = &item.text {
                if has_items {
                    groups.push(current_group);
                    current_group = SidebarGroup { text: text.clone(), items: Vec::new() };
                }
                else {
                    current_group.text = text.clone();
                }
                has_items = true;
            }

            if let Some(link) = &item.link {
                if let Some(text) = &item.text {
                    current_group.items.push(SidebarLink { text: text.clone(), link: link.clone() });
                }
            }

            if let Some(sub_items) = &item.items {
                if !sub_items.is_empty() {
                    if has_items && !current_group.items.is_empty() {
                        groups.push(current_group);
                        current_group = SidebarGroup { text: item.text.clone().unwrap_or_default(), items: Vec::new() };
                    }
                    for sub_item in sub_items {
                        if let (Some(text), Some(link)) = (&sub_item.text, &sub_item.link) {
                            current_group.items.push(SidebarLink { text: text.clone(), link: link.clone() });
                        }
                    }
                    if !current_group.items.is_empty() {
                        groups.push(current_group);
                        current_group = SidebarGroup { text: String::new(), items: Vec::new() };
                        has_items = false;
                    }
                }
            }
        }

        if !current_group.items.is_empty() || !current_group.text.is_empty() {
            groups.push(current_group);
        }

        groups
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
        }
        else {
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

                if components
                    .iter()
                    .any(|&c| c == "node_modules" || c == ".git" || c == "dist" || c == ".vitepress" || c == ".vutex")
                {
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
            "css", "js", "jpg", "jpeg", "png", "gif", "svg", "ico", "webp", "woff", "woff2", "ttf", "eot", "otf", "pdf", "zip",
            "tar", "gz", "html", "htm",
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
            }
            else {
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
    pub fn load_from_file(path: &PathBuf) -> Result<crate::config::VitePressConfig> {
        Ok(crate::config::VitePressConfig::load_from_file(path)?)
    }

    /// 从目录查找并加载配置
    ///
    /// 按以下顺序查找配置文件：
    /// 1. vitepress.config.toml
    /// 2. vitepress.config.json
    ///
    /// # Arguments
    ///
    /// * `dir` - 要搜索的目录路径
    ///
    /// # Errors
    ///
    /// 返回错误如果配置文件读取、解析或验证失败
    pub fn load_from_dir(dir: &PathBuf) -> Result<crate::config::VitePressConfig> {
        Ok(crate::config::VitePressConfig::load_from_dir(dir)?)
    }
}
