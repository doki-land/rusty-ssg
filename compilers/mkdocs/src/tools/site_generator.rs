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
    /// 主题管理器
    theme_manager: ThemeManager,
}

impl StaticSiteGenerator {
    /// 创建新的静态站点生成器
    pub fn new(config: MkDocsConfig) -> Result<Self> {
        let theme_manager = ThemeManager::new(config.clone())?;

        Ok(Self { config, theme_manager })
    }

    /// 生成静态站点
    pub fn generate(&mut self, documents: &HashMap<String, String>, output_dir: &PathBuf) -> Result<()> {
        if !output_dir.exists() {
            fs::create_dir_all(output_dir)?;
        }

        // 验证配置（严格模式下会检查更多内容）
        self.validate_config()?;

        // 收集所有文档信息用于侧边栏
        let mut all_sidebar_links = Vec::new();

        for (path, content) in documents {
            let title = Self::extract_title(content, path);
            let html_path = self.generate_html_path(path);
            all_sidebar_links.push((title, html_path.clone()));
        }

        // 编译所有文档
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

            // 生成侧边栏
            let mut sidebar_links = Vec::new();
            for (title, link) in &all_sidebar_links {
                let relative_link = format!("{}{}", root_path, link);
                sidebar_links.push(ThemeSidebarLink { text: title.clone(), link: relative_link });
            }

            let sidebar_group = ThemeSidebarGroup { text: "文档".to_string(), items: sidebar_links };
            let sidebar_groups = vec![sidebar_group];

            // 生成导航栏
            let nav_items = Self::generate_nav_items(&self.config);

            // 渲染页面
            let html_content = self.render_page_for_file(content, path, &nav_items, &sidebar_groups, html_path.clone())?;

            // 写入文件
            fs::write(&output_path, html_content)?;
        }

        // 复制静态资源
        self.copy_static_files(output_dir)?;

        // 验证链接
        self.validate_links(documents)?;

        Ok(())
    }

    /// 验证配置
    fn validate_config(&self) -> Result<()> {
        // 检查必填字段
        if self.config.site_name().is_empty() {
            return Err(crate::types::MkDocsError::ConfigValidationError {
                message: "site_name cannot be empty".to_string(),
            });
        }

        // 严格模式下的额外验证
        if self.config.strict {
            // 检查文档目录是否存在
            let docs_dir = PathBuf::from(self.config.docs_dir());
            if !docs_dir.exists() {
                return Err(crate::types::MkDocsError::ConfigValidationError {
                    message: format!("docs_dir '{}' does not exist", self.config.docs_dir()),
                });
            }

            // 检查主题配置
            if self.config.theme.name().is_empty() {
                return Err(crate::types::MkDocsError::ConfigValidationError {
                    message: "theme name cannot be empty in strict mode".to_string(),
                });
            }
        }

        Ok(())
    }

    /// 验证链接
    fn validate_links(&self, documents: &HashMap<String, String>) -> Result<()> {
        // 创建链接验证器
        let mut validator = crate::tools::link_validator::LinkValidator::new(self.config.clone());

        // 添加所有文档
        for path in documents.keys() {
            validator.add_file(path);
        }

        // 验证所有链接
        validator.validate_all(documents);

        // 获取验证结果
        let result = validator.result();

        // 严格模式下，任何错误或警告都会导致构建失败
        if self.config.strict {
            if result.has_errors() || result.has_warnings() {
                result.print();
                return Err(crate::types::MkDocsError::LinkValidationError {
                    message: "Link validation failed in strict mode".to_string(),
                });
            }
        }
        // 非严格模式下，只打印警告和错误
        else {
            result.print();
        }

        Ok(())
    }

    /// 复制静态资源文件
    fn copy_static_files(&self, output_dir: &PathBuf) -> Result<()> {
        // 复制主题静态资源
        self.copy_theme_static_files(output_dir)?;
        
        // 复制配置中指定的额外静态资源
        self.copy_extra_static_files(output_dir)?;
        
        Ok(())
    }

    /// 复制主题静态资源
    fn copy_theme_static_files(&self, output_dir: &PathBuf) -> Result<()> {
        // 这里可以实现主题静态资源的复制逻辑
        Ok(())
    }

    /// 复制额外的静态资源
    fn copy_extra_static_files(&self, output_dir: &PathBuf) -> Result<()> {
        // 复制额外的 CSS 文件
        for css_file in &self.config.extra_css {
            self.copy_static_file(css_file, output_dir, "css")?;
        }
        
        // 复制额外的 JavaScript 文件
        for js_file in &self.config.extra_javascript {
            match js_file {
                crate::types::ExtraJavaScript::String(path) => {
                    self.copy_static_file(path, output_dir, "js")?;
                }
                crate::types::ExtraJavaScript::Object(config) => {
                    self.copy_static_file(&config.path, output_dir, "js")?;
                }
            }
        }
        
        Ok(())
    }

    /// 复制单个静态文件
    fn copy_static_file(&self, file_path: &str, output_dir: &PathBuf, subdir: &str) -> Result<()> {
        let src_path = PathBuf::from(file_path);
        if src_path.exists() {
            let dest_dir = output_dir.join(subdir);
            fs::create_dir_all(&dest_dir)?;
            
            let dest_path = dest_dir.join(src_path.file_name().unwrap());
            fs::copy(&src_path, &dest_path)?;
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
        let site_title = self.theme_manager.site_title();

        let page_title = if !doc_title.is_empty() { format!("{} | {}", doc_title, site_title) } else { site_title.to_string() };

        // 使用 nargo-document 渲染 Markdown 内容
        use nargo_document::MarkdownRenderer;
        let renderer = MarkdownRenderer::new();
        let html_content = match renderer.render(content) {
            Ok(html) => html,
            Err(_) => Self::simple_markdown_to_html(content),
        };

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

        self.theme_manager.render_page(&context)
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
