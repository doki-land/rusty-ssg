//! 默认主题实现
//! 提供完整的文档站点主题和样式

use crate::{Result, types::VutexConfig};
use nargo_template::{TemplateEngine, ToJsonValue};
use serde_json::{json, Value};

/// 模板引擎类型
/// 支持多种模板引擎
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateEngineType {
    /// Askama 模板引擎
    Askama,
    /// DejaVu 模板引擎
    DejaVu,
    /// Handlebars 模板引擎
    Handlebars,
    /// Jinja2 模板引擎
    Jinja2,
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
            "code": self.code,
            "label": self.label,
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
            "text": self.text,
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
            "text": self.text,
            "link": self.link
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
            "text": self.text,
            "link": self.link
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
            "page_title": self.page_title,
            "site_title": self.site_title,
            "content": self.content,
            "nav_items": self.nav_items.to_json_value(),
            "sidebar_groups": self.sidebar_groups.to_json_value(),
            "current_path": self.current_path,
            "has_footer": self.has_footer,
            "has_footer_message": self.has_footer_message,
            "footer_message": self.footer_message,
            "has_footer_copyright": self.has_footer_copyright,
            "footer_copyright": self.footer_copyright,
            "current_lang": self.current_lang,
            "available_locales": self.available_locales.to_json_value(),
            "root_path": self.root_path
        })
    }
}

/// 统一模板管理器
pub struct UnifiedTemplateManager {
    manager: nargo_template::TemplateManager,
}

impl UnifiedTemplateManager {
    /// 创建新的统一模板管理器
    pub fn new() -> Self {
        Self {
            manager: nargo_template::TemplateManager::new(),
        }
    }

    /// 注册模板
    pub fn register_template(&mut self, engine: TemplateEngine, name: &str, content: &str) -> Result<()> {
        self.manager.register_template(engine, name, content)
            .map_err(|e| crate::types::errors::VutexError::ConfigError { message: e.to_string() })
    }

    /// 渲染模板
    pub fn render<T: ToJsonValue>(&self, engine: TemplateEngine, template_name: &str, context: &T) -> Result<String> {
        let json_context = context.to_json_value();
        self.manager.render(engine, template_name, &json_context)
            .map_err(|e| crate::types::errors::VutexError::ConfigError { message: e.to_string() })
    }
}

impl Default for UnifiedTemplateManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 默认主题
pub struct DefaultTheme {
    /// 主题配置
    config: VutexConfig,
    /// 模板引擎类型
    engine_type: TemplateEngineType,
    /// 模板管理器
    template_manager: UnifiedTemplateManager,
}

impl DefaultTheme {
    /// 创建新的默认主题实例（使用 DejaVu 引擎）
    pub fn new(config: VutexConfig) -> Result<Self> {
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
    pub fn with_engine(config: VutexConfig, engine_type: TemplateEngineType) -> Result<Self> {
        let mut template_manager = UnifiedTemplateManager::new();
        
        // 注册模板
        match engine_type {
            TemplateEngineType::DejaVu => {
                let template_content = include_str!("../templates/page.dejavu");
                template_manager.register_template(TemplateEngine::DejaVu, "page", template_content)?;
            }
            TemplateEngineType::Handlebars => {
                let template_content = include_str!("../templates/page.html");
                template_manager.register_template(TemplateEngine::Handlebars, "page", template_content)?;
            }
            TemplateEngineType::Jinja2 => {
                let template_content = include_str!("../templates/page.html");
                template_manager.register_template(TemplateEngine::Jinja2, "page", template_content)?;
            }
            TemplateEngineType::Askama => {
                // 保持 Askama 模板的兼容性
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
            TemplateEngineType::DejaVu => {
                self.template_manager.render(TemplateEngine::DejaVu, "page", context)
            }
            TemplateEngineType::Handlebars => {
                self.template_manager.render(TemplateEngine::Handlebars, "page", context)
            }
            TemplateEngineType::Jinja2 => {
                self.template_manager.render(TemplateEngine::Jinja2, "page", context)
            }
            TemplateEngineType::Askama => {
                // 保持 Askama 模板的兼容性
                // 这里可以使用临时的字符串替换实现
                let template_content = include_str!("../templates/page.html");
                let mut result = template_content.to_string();
                
                // 简单的变量替换
                result = result.replace("{{ page_title }}", &context.page_title);
                result = result.replace("{{ site_title }}", &context.site_title);
                result = result.replace("{{ content }}", &context.content);
                
                Ok(result)
            }
        }
    }

    /// 获取站点标题
    ///
    /// # Returns
    ///
    /// 站点标题字符串
    pub fn site_title(&self) -> &str {
        self.config.title.as_deref().unwrap_or("VuTeX Documentation")
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
