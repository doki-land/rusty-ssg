//! 默认主题实现
//! 提供完整的文档站点主题和样式

use crate::{Result, tools::UnifiedTemplateManager, types::MkDocsConfig};
use nargo_template::{TemplateEngine, ToJsonValue};
use serde_json::json;
use std::collections::HashMap;

/// 主题类型
/// 支持多种内置主题
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThemeType {
    /// 默认主题
    Default,
    /// 暗黑主题
    Dark,
    /// 技术文档主题
    Tech,
    /// 自定义主题
    Custom(String),
}

/// 模板引擎类型
/// 支持多种模板引擎
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateEngineType {
    /// DejaVu 模板引擎
    DejaVu,
    /// Handlebars 模板引擎
    Handlebars,
    /// Jinja2 模板引擎
    Jinja2,
}

/// 侧边栏组
#[derive(Debug, Clone)]
pub struct ThemeSidebarGroup {
    /// 组标题
    pub text: String,
    /// 组内项目
    pub items: Vec<ThemeSidebarLink>,
}

impl ToJsonValue for ThemeSidebarGroup {
    fn to_json_value(&self) -> serde_json::Value {
        let mut map = HashMap::new();
        map.insert("text", self.text.clone());
        map.insert("items", self.items.to_json_value().to_string());
        json!(map)
    }
}

/// 侧边栏链接
#[derive(Debug, Clone)]
pub struct ThemeSidebarLink {
    /// 链接文本
    pub text: String,
    /// 链接地址
    pub link: String,
}

impl ToJsonValue for ThemeSidebarLink {
    fn to_json_value(&self) -> serde_json::Value {
        let mut map = HashMap::new();
        map.insert("text", self.text.clone());
        map.insert("link", self.link.clone());
        json!(map)
    }
}

/// 导航栏项
#[derive(Debug, Clone)]
pub struct ThemeNavItem {
    /// 显示文本
    pub text: String,
    /// 链接
    pub link: String,
    /// 子导航项
    pub children: Vec<ThemeNavItem>,
}

impl ToJsonValue for ThemeNavItem {
    fn to_json_value(&self) -> serde_json::Value {
        let mut map = HashMap::new();
        map.insert("text", self.text.clone());
        map.insert("link", self.link.clone());
        map.insert("children", self.children.to_json_value().to_string());
        json!(map)
    }
}

/// 页面模板上下文
#[derive(Debug, Clone)]
pub struct PageContext {
    /// 页面标题
    pub page_title: String,
    /// 站点标题
    pub site_title: String,
    /// 页面内容
    pub content: String,
    /// 导航栏项目
    pub nav_items: Vec<ThemeNavItem>,
    /// 侧边栏组
    pub sidebar_groups: Vec<ThemeSidebarGroup>,
    /// 当前页面路径
    pub current_path: String,
    /// 是否有页脚
    pub has_footer: bool,
    /// 是否有页脚消息
    pub has_footer_message: bool,
    /// 页脚消息
    pub footer_message: String,
    /// 是否有页脚版权
    pub has_footer_copyright: bool,
    /// 页脚版权
    pub footer_copyright: String,
    /// 当前语言
    pub current_lang: String,
    /// 可用语言列表
    pub available_locales: Vec<LocaleInfo>,
    /// 相对于根目录的路径前缀
    pub root_path: String,
}

impl ToJsonValue for PageContext {
    fn to_json_value(&self) -> serde_json::Value {
        let mut map = HashMap::new();
        map.insert("page_title", self.page_title.clone());
        map.insert("site_title", self.site_title.clone());
        map.insert("content", self.content.clone());
        map.insert("nav_items", self.nav_items.to_json_value().to_string());
        map.insert("sidebar_groups", self.sidebar_groups.to_json_value().to_string());
        map.insert("current_path", self.current_path.clone());
        map.insert("has_footer", self.has_footer.to_string());
        map.insert("has_footer_message", self.has_footer_message.to_string());
        map.insert("footer_message", self.footer_message.clone());
        map.insert("has_footer_copyright", self.has_footer_copyright.to_string());
        map.insert("footer_copyright", self.footer_copyright.clone());
        map.insert("current_lang", self.current_lang.clone());
        map.insert("available_locales", self.available_locales.to_json_value().to_string());
        map.insert("root_path", self.root_path.clone());
        json!(map)
    }
}

/// 语言信息
#[derive(Debug, Clone)]
pub struct LocaleInfo {
    /// 语言代码
    pub code: String,
    /// 语言标签
    pub label: String,
    /// 是否为当前语言
    pub is_current: bool,
}

impl ToJsonValue for LocaleInfo {
    fn to_json_value(&self) -> serde_json::Value {
        let mut map = HashMap::new();
        map.insert("code", self.code.clone());
        map.insert("label", self.label.clone());
        map.insert("is_current", self.is_current.to_string());
        json!(map)
    }
}

/// 默认主题
pub struct DefaultTheme {
    /// 主题配置
    config: MkDocsConfig,
    /// 主题类型
    theme_type: ThemeType,
    /// 模板引擎类型
    engine_type: TemplateEngineType,
    /// 模板管理器
    template_manager: UnifiedTemplateManager,
    /// 自定义主题目录
    custom_dir: Option<String>,
}

impl DefaultTheme {
    /// 创建新的默认主题实例（使用 DejaVu 引擎）
    pub fn new(config: MkDocsConfig) -> Result<Self> {
        let theme_type = Self::get_theme_type(&config);
        let custom_dir = config.theme.custom_dir.clone();
        Self::with_theme(config, theme_type, TemplateEngineType::DejaVu, custom_dir)
    }

    /// 创建指定主题类型和模板引擎的默认主题实例
    ///
    /// # Arguments
    ///
    /// * `config` - 主题配置
    /// * `theme_type` - 主题类型
    /// * `engine_type` - 模板引擎类型
    /// * `custom_dir` - 自定义主题目录
    ///
    /// # Returns
    ///
    /// 新的默认主题实例
    pub fn with_theme(config: MkDocsConfig, theme_type: ThemeType, engine_type: TemplateEngineType, custom_dir: Option<String>) -> Result<Self> {
        let mut template_manager = UnifiedTemplateManager::new();

        // 首先尝试从自定义目录加载模板
        if let Some(ref dir) = custom_dir {
            if let Err(e) = Self::load_templates_from_custom_dir(&mut template_manager, engine_type, dir) {
                eprintln!("Warning: Failed to load templates from custom directory: {}", e);
            }
        }

        // 如果自定义目录加载失败，使用内置模板
        Self::load_builtin_templates(&mut template_manager, engine_type, &theme_type)?;

        Ok(Self { config, theme_type, engine_type, template_manager, custom_dir })
    }

    /// 从配置中获取主题类型
    fn get_theme_type(config: &MkDocsConfig) -> ThemeType {
        match config.theme.name.as_str() {
            "default" => ThemeType::Default,
            "dark" => ThemeType::Dark,
            "tech" => ThemeType::Tech,
            custom => ThemeType::Custom(custom.to_string()),
        }
    }

    /// 从自定义目录加载模板
    fn load_templates_from_custom_dir(template_manager: &mut UnifiedTemplateManager, engine_type: TemplateEngineType, dir: &str) -> std::io::Result<()> {
        use std::path::Path;
        let template_dir = Path::new(dir).join("templates");
        if template_dir.exists() {
            template_manager.register_templates_from_dir(engine_type, &template_dir)?;
        }
        Ok(())
    }

    /// 加载内置模板
    fn load_builtin_templates(template_manager: &mut UnifiedTemplateManager, engine_type: TemplateEngineType, theme_type: &ThemeType) -> std::io::Result<()> {
        let template_content = match theme_type {
            ThemeType::Default => include_str!("../templates/page.dejavu"),
            ThemeType::Dark => include_str!("../templates/page.dejavu"), // 后续添加暗黑主题模板
            ThemeType::Tech => include_str!("../templates/page.dejavu"), // 后续添加技术主题模板
            ThemeType::Custom(_) => include_str!("../templates/page.dejavu"), // 自定义主题回退到默认模板
        };

        template_manager.register_template(engine_type, "page", template_content)?;
        Ok(())
    }

    /// 渲染页面
    ///
    /// # Arguments
    ///
    /// * `context` - 页面上下文
    ///
    /// # Returns
    ///
    /// 渲染后的 HTML 字符串
    pub fn render_page(&self, context: &PageContext) -> Result<String> {
        match self.engine_type {
            TemplateEngineType::DejaVu => {
                let result = self.template_manager.render(TemplateEngine::DejaVu, "page", context);
                match result {
                    Ok(html) => Ok(html),
                    Err(e) => {
                        // 回退到简单的 HTML 渲染
                        Ok(self.render_fallback_page(context))
                    }
                }
            },
            TemplateEngineType::Handlebars => self
                .template_manager
                .render(TemplateEngine::Handlebars, "page", context)
                .map_err(|e| crate::types::MkDocsError::from(e)),
            TemplateEngineType::Jinja2 => self
                .template_manager
                .render(TemplateEngine::Jinja2, "page", context)
                .map_err(|e| crate::types::MkDocsError::from(e)),
        }
    }

    /// 回退页面渲染方法
    ///
    /// # Arguments
    ///
    /// * `context` - 页面上下文
    ///
    /// # Returns
    ///
    /// 简单的 HTML 页面字符串
    fn render_fallback_page(&self, context: &PageContext) -> String {
        format!(r#"
<!DOCTYPE html>
<html lang="{}">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
            line-height: 1.6;
            color: #333;
            max-width: 960px;
            margin: 0 auto;
            padding: 20px;
        }}
        header {{
            border-bottom: 1px solid #eee;
            padding-bottom: 20px;
            margin-bottom: 20px;
        }}
        h1, h2, h3 {{
            color: #2c3e50;
        }}
        nav ul {{
            list-style: none;
            padding: 0;
            display: flex;
            gap: 20px;
        }}
        nav a {{
            text-decoration: none;
            color: #3498db;
        }}
        nav a:hover {{
            text-decoration: underline;
        }}
        .sidebar {{
            float: left;
            width: 200px;
            margin-right: 20px;
        }}
        .content {{
            overflow: hidden;
        }}
        footer {{
            margin-top: 40px;
            padding-top: 20px;
            border-top: 1px solid #eee;
            text-align: center;
            color: #777;
        }}
    </style>
</head>
<body>
    <header>
        <h1>{}</h1>
        <nav>
            <ul>
                {}
            </ul>
        </nav>
    </header>
    <div class="container">
        <div class="sidebar">
            {}
        </div>
        <div class="content">
            {}
        </div>
    </div>
    <footer>
        {}
    </footer>
</body>
</html>
"#, 
            context.current_lang,
            context.page_title,
            context.site_title,
            self.render_nav_items(&context.nav_items),
            self.render_sidebar(&context.sidebar_groups),
            context.content,
            self.render_footer(context)
        )
    }

    /// 渲染导航栏项目
    fn render_nav_items(&self, nav_items: &[ThemeNavItem]) -> String {
        nav_items.iter()
            .map(|item| self.render_nav_item(item))
            .collect::<Vec<_>>()
            .join("")
    }

    /// 渲染单个导航栏项目
    fn render_nav_item(&self, item: &ThemeNavItem) -> String {
        if item.children.is_empty() {
            format!("<li><a href='{}'>{}</a></li>", item.link, item.text)
        } else {
            let children = self.render_nav_items(&item.children);
            format!(
                r#"<li>
                    <a href='{}'>{}</a>
                    <ul class='sub-nav'>{}</ul>
                </li>"#,
                item.link, item.text, children
            )
        }
    }

    /// 渲染侧边栏
    fn render_sidebar(&self, sidebar_groups: &[ThemeSidebarGroup]) -> String {
        sidebar_groups.iter()
            .map(|group| {
                let items = group.items.iter()
                    .map(|item| format!("<li><a href='{}'>{}</a></li>", item.link, item.text))
                    .collect::<Vec<_>>()
                    .join("");
                format!("<div class='sidebar-group'><h3>{}</h3><ul>{}</ul></div>", group.text, items)
            })
            .collect::<Vec<_>>()
            .join("")
    }

    /// 渲染页脚
    fn render_footer(&self, context: &PageContext) -> String {
        if context.has_footer {
            let mut footer = String::new();
            if context.has_footer_message {
                footer.push_str(&format!("<p>{}</p>", context.footer_message));
            }
            if context.has_footer_copyright {
                footer.push_str(&format!("<p>{}</p>", context.footer_copyright));
            }
            footer
        } else {
            "<p>&copy; 2026 MkDocs</p>".to_string()
        }
    }

    /// 获取站点标题
    ///
    /// # Returns
    ///
    /// 站点标题字符串
    pub fn site_title(&self) -> &str {
        &self.config.site_name
    }

    /// 获取当前使用的模板引擎类型
    ///
    /// # Returns
    ///
    /// 模板引擎类型
    pub fn engine_type(&self) -> &TemplateEngineType {
        &self.engine_type
    }
}
