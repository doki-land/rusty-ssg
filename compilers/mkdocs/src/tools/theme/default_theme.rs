//! 默认主题实现
//! 提供完整的文档站点主题和样式

use crate::tools::UnifiedTemplateManager;
use crate::{Result, types::MkDocsConfig};
use nargo_template::{TemplateEngine, ToJsonValue};
use serde_json::json;
use std::collections::HashMap;

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
}

impl ToJsonValue for ThemeNavItem {
    fn to_json_value(&self) -> serde_json::Value {
        let mut map = HashMap::new();
        map.insert("text", self.text.clone());
        map.insert("link", self.link.clone());
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
    /// 模板引擎类型
    engine_type: TemplateEngineType,
    /// 模板管理器
    template_manager: UnifiedTemplateManager,
}

impl DefaultTheme {
    /// 创建新的默认主题实例（使用 DejaVu 引擎）
    pub fn new(config: MkDocsConfig) -> Result<Self> {
        Self::with_engine(config, TemplateEngineType::DejaVu)
    }

    /// 创建指定模板引擎的默认主题实例
    ///
    /// # Arguments
    ///
    /// * `config` - 主题配置
    /// * `engine_type` - 模板引擎类型
    ///
    /// # Returns
    ///
    /// 新的默认主题实例
    pub fn with_engine(config: MkDocsConfig, engine_type: TemplateEngineType) -> Result<Self> {
        let mut template_manager = UnifiedTemplateManager::new();

        match engine_type {
            TemplateEngineType::DejaVu => {
                let template_content = include_str!("../templates/page.dejavu");
                template_manager.register_template(TemplateEngine::DejaVu, "page", template_content)?;
            }
            TemplateEngineType::Handlebars => {
                let template_content = include_str!("../templates/page.dejavu");
                template_manager.register_template(TemplateEngine::Handlebars, "page", template_content)?;
            }
            TemplateEngineType::Jinja2 => {
                let template_content = include_str!("../templates/page.dejavu");
                template_manager.register_template(TemplateEngine::Jinja2, "page", template_content)?;
            }
        }

        Ok(Self { config, engine_type, template_manager })
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
            TemplateEngineType::DejaVu => self
                .template_manager
                .render(TemplateEngine::DejaVu, "page", context)
                .map_err(|e| crate::types::MkDocsError::from(e)),
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
