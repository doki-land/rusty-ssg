//! 默认主题实现
//! 提供完整的文档站点主题和样式

use crate::{Result, tools::UnifiedTemplateManager, types::VuePressConfig};
use nargo_template::ToJsonValue;
use serde_json::json;

/// 模板引擎类型
/// 支持 Askama 和 Dejavu 两种模板引擎
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateEngineType {
    /// Askama 模板引擎
    Askama,
    /// Dejavu 模板引擎
    Dejavu,
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
        json!({
            "code": self.code.clone(),
            "label": self.label.clone(),
            "is_current": self.is_current
        })
    }
}

/// 侧边栏组
#[derive(Debug, Clone)]
pub struct SidebarGroup {
    /// 组标题
    pub text: String,
    /// 组内项目
    pub items: Vec<SidebarLink>,
}

impl ToJsonValue for SidebarGroup {
    fn to_json_value(&self) -> serde_json::Value {
        json!({
            "text": self.text.clone(),
            "items": self.items.to_json_value()
        })
    }
}

/// 侧边栏链接
#[derive(Debug, Clone)]
pub struct SidebarLink {
    /// 链接文本
    pub text: String,
    /// 链接地址
    pub link: String,
}

impl ToJsonValue for SidebarLink {
    fn to_json_value(&self) -> serde_json::Value {
        json!({
            "text": self.text.clone(),
            "link": self.link.clone()
        })
    }
}

/// 导航栏项
#[derive(Debug, Clone)]
pub struct NavItem {
    /// 显示文本
    pub text: String,
    /// 链接
    pub link: String,
}

impl ToJsonValue for NavItem {
    fn to_json_value(&self) -> serde_json::Value {
        json!({
            "text": self.text.clone(),
            "link": self.link.clone()
        })
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
    pub nav_items: Vec<NavItem>,
    /// 侧边栏组
    pub sidebar_groups: Vec<SidebarGroup>,
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
        json!({
            "page_title": self.page_title.clone(),
            "site_title": self.site_title.clone(),
            "content": self.content.clone(),
            "nav_items": self.nav_items.to_json_value(),
            "sidebar_groups": self.sidebar_groups.to_json_value(),
            "current_path": self.current_path.clone(),
            "has_footer": self.has_footer,
            "has_footer_message": self.has_footer_message,
            "footer_message": self.footer_message.clone(),
            "has_footer_copyright": self.has_footer_copyright,
            "footer_copyright": self.footer_copyright.clone(),
            "current_lang": self.current_lang.clone(),
            "available_locales": self.available_locales.to_json_value(),
            "root_path": self.root_path.clone()
        })
    }
}

/// 主题接口
pub trait Theme {
    /// 渲染页面
    fn render_page(&self, context: &PageContext) -> Result<String>;

    /// 获取站点标题
    fn site_title(&self) -> &str;
}

/// 默认主题
pub struct DefaultTheme {
    /// 主题配置
    config: VuePressConfig,
    /// 模板引擎类型
    engine_type: TemplateEngineType,
    /// 模板管理器
    template_manager: UnifiedTemplateManager,
}

impl Theme for DefaultTheme {
    /// 渲染页面
    fn render_page(&self, context: &PageContext) -> Result<String> {
        let json_context = context.to_json_value();
        self.template_manager
            .render("page", &json_context)
            .map_err(|e| crate::types::VutexError::from(e))
    }

    /// 获取站点标题
    fn site_title(&self) -> &str {
        self.config.title.as_deref().unwrap_or("VuePress Documentation")
    }
}

impl DefaultTheme {
    /// 创建新的默认主题实例（使用 Dejavu 引擎）
    pub fn new(config: VuePressConfig) -> Result<Self> {
        Self::with_engine(config, TemplateEngineType::Dejavu)
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
    pub fn with_engine(config: VuePressConfig, engine_type: TemplateEngineType) -> Result<Self> {
        let mut template_manager = UnifiedTemplateManager::new();

        let template_content = include_str!("../templates/page.dejavu");
        template_manager.register_template("page", template_content)?;

        Ok(Self { config, engine_type, template_manager })
    }

    /// 获取当前使用的模板引擎类型
    ///
    /// # Returns
    ///
    /// 模板引擎类型
    pub fn engine_type(&self) -> &TemplateEngineType {
        &self.engine_type
    }

    /// 从文件加载自定义主题
    ///
    /// # Arguments
    ///
    /// * `config` - 主题配置
    /// * `theme_path` - 主题目录路径
    ///
    /// # Returns
    ///
    /// 新的主题实例
    pub fn from_path(config: VuePressConfig, theme_path: &std::path::Path) -> Result<Self> {
        let mut template_manager = UnifiedTemplateManager::new();

        let page_template_path = theme_path.join("templates").join("page.dejavu");
        if page_template_path.exists() {
            let template_content = std::fs::read_to_string(page_template_path)?;
            template_manager.register_template("page", &template_content)?;
        }
        else {
            let template_content = include_str!("../templates/page.dejavu");
            template_manager.register_template("page", template_content)?;
        }

        Ok(Self { config, engine_type: TemplateEngineType::Dejavu, template_manager })
    }
}
