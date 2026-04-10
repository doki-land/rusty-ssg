//! 默认主题实现
//! 提供完整的文档站点主题和样式

use crate::{Result, tools::UnifiedTemplateManager, types::MkDocsConfig};
use nargo_template::ToJsonValue;

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
        let mut map = serde_json::Map::new();
        map.insert("text".to_string(), self.text.to_json_value());
        map.insert("items".to_string(), self.items.to_json_value());
        serde_json::Value::Object(map)
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
        let mut map = serde_json::Map::new();
        map.insert("text".to_string(), self.text.to_json_value());
        map.insert("link".to_string(), self.link.to_json_value());
        serde_json::Value::Object(map)
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
        let mut map = serde_json::Map::new();
        map.insert("text".to_string(), self.text.to_json_value());
        map.insert("link".to_string(), self.link.to_json_value());
        map.insert("children".to_string(), self.children.to_json_value());
        serde_json::Value::Object(map)
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
        let mut map = serde_json::Map::new();
        map.insert("page_title".to_string(), self.page_title.to_json_value());
        map.insert("site_title".to_string(), self.site_title.to_json_value());
        map.insert("content".to_string(), self.content.to_json_value());
        map.insert("nav_items".to_string(), self.nav_items.to_json_value());
        map.insert("sidebar_groups".to_string(), self.sidebar_groups.to_json_value());
        map.insert("current_path".to_string(), self.current_path.to_json_value());
        map.insert("has_footer".to_string(), self.has_footer.to_json_value());
        map.insert("has_footer_message".to_string(), self.has_footer_message.to_json_value());
        map.insert("footer_message".to_string(), self.footer_message.to_json_value());
        map.insert("has_footer_copyright".to_string(), self.has_footer_copyright.to_json_value());
        map.insert("footer_copyright".to_string(), self.footer_copyright.to_json_value());
        map.insert("current_lang".to_string(), self.current_lang.to_json_value());
        map.insert("available_locales".to_string(), self.available_locales.to_json_value());
        map.insert("root_path".to_string(), self.root_path.to_json_value());
        serde_json::Value::Object(map)
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
        let mut map = serde_json::Map::new();
        map.insert("code".to_string(), self.code.to_json_value());
        map.insert("label".to_string(), self.label.to_json_value());
        map.insert("is_current".to_string(), self.is_current.to_json_value());
        serde_json::Value::Object(map)
    }
}

/// 主题管理器
pub struct ThemeManager {
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
    /// 主题实例
    theme: Box<dyn Theme>,
}

/// 主题 trait
pub trait Theme {
    /// 渲染页面
    fn render_page(&self, context: &PageContext) -> Result<String>;
    
    /// 获取主题名称
    fn name(&self) -> &str;
    
    /// 获取站点标题
    fn site_title(&self) -> &str;
    
    /// 获取模板引擎类型
    fn engine_type(&self) -> &TemplateEngineType;
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
    pub template_manager: UnifiedTemplateManager,
    /// 自定义主题目录
    custom_dir: Option<String>,
}

impl Theme for DefaultTheme {
    fn render_page(&self, context: &PageContext) -> Result<String> {
        self.render_page_impl(context)
    }
    
    fn name(&self) -> &str {
        match &self.theme_type {
            ThemeType::Default => "default",
            ThemeType::Dark => "dark",
            ThemeType::Tech => "tech",
            ThemeType::Custom(name) => name.as_str(),
        }
    }
    
    fn site_title(&self) -> &str {
        &self.config.site_name
    }
    
    fn engine_type(&self) -> &TemplateEngineType {
        &self.engine_type
    }
}

impl ThemeManager {
    /// 创建新的主题管理器
    pub fn new(config: MkDocsConfig) -> Result<Self> {
        let theme_type = DefaultTheme::get_theme_type(&config);
        let custom_dir = config.theme.custom_dir.clone();
        
        // 创建默认主题实例
        let theme = Box::new(DefaultTheme::new(config.clone())?);
        
        Ok(Self {
            config,
            theme_type,
            engine_type: TemplateEngineType::DejaVu, // 默认使用 DejaVu 引擎
            template_manager: UnifiedTemplateManager::new(),
            custom_dir,
            theme,
        })
    }
    
    /// 渲染页面
    pub fn render_page(&self, context: &PageContext) -> Result<String> {
        self.theme.render_page(context)
    }
    
    /// 获取主题名称
    pub fn theme_name(&self) -> &str {
        self.theme.name()
    }
    
    /// 获取站点标题
    pub fn site_title(&self) -> &str {
        self.theme.site_title()
    }
    
    /// 获取模板引擎类型
    pub fn engine_type(&self) -> &TemplateEngineType {
        self.theme.engine_type()
    }
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
    pub fn with_theme(
        config: MkDocsConfig,
        theme_type: ThemeType,
        engine_type: TemplateEngineType,
        custom_dir: Option<String>,
    ) -> Result<Self> {
        let mut template_manager = UnifiedTemplateManager::new();

        // 首先尝试从自定义目录加载模板
        if let Some(ref dir) = custom_dir {
            if let Err(e) = Self::load_templates_from_custom_dir(&mut template_manager, &engine_type, dir) {
                eprintln!("Warning: Failed to load templates from custom directory: {}", e);
            }
        }

        // 如果自定义目录加载失败，使用内置模板
        Self::load_builtin_templates(&mut template_manager, &engine_type, &theme_type)?;

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
    fn load_templates_from_custom_dir(
        template_manager: &mut UnifiedTemplateManager,
        _engine_type: &TemplateEngineType,
        dir: &str,
    ) -> std::io::Result<()> {
        use std::path::Path;
        let template_dir = Path::new(dir).join("templates");
        if template_dir.exists() {
            template_manager.register_templates_from_dir(&template_dir)?;
        }
        Ok(())
    }

    /// 加载内置模板
    fn load_builtin_templates(
        template_manager: &mut UnifiedTemplateManager,
        _engine_type: &TemplateEngineType,
        theme_type: &ThemeType,
    ) -> std::io::Result<()> {
        let template_content = match theme_type {
            ThemeType::Default => include_str!("../templates/page.dejavu"),
            ThemeType::Dark => include_str!("../templates/page.dejavu"),
            ThemeType::Tech => include_str!("../templates/page.dejavu"),
            ThemeType::Custom(_) => include_str!("../templates/page.dejavu"),
        };

        template_manager.register_template("page", template_content)?;
        Ok(())
    }

    /// 渲染页面实现
    ///
    /// # Arguments
    ///
    /// * `context` - 页面上下文
    ///
    /// # Returns
    ///
    /// 渲染后的 HTML 字符串
    fn render_page_impl(&self, context: &PageContext) -> Result<String> {
        let context_value = context.to_json_value();
        let result = self.template_manager.render("page", &context_value);
        match result {
            Ok(html) => Ok(html),
            Err(_) => Ok(self.render_fallback_page(context)),
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
        format!(
            r#"
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
        nav_items.iter().map(|item| self.render_nav_item(item)).collect::<Vec<_>>().join("")
    }

    /// 渲染单个导航栏项目
    fn render_nav_item(&self, item: &ThemeNavItem) -> String {
        if item.children.is_empty() {
            format!("<li><a href='{}'>{}</a></li>", item.link, item.text)
        }
        else {
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
        sidebar_groups
            .iter()
            .map(|group| {
                let items = group
                    .items
                    .iter()
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
        }
        else {
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
