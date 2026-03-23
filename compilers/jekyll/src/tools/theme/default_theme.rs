//! 默认主题实现
//! 提供完整的文档站点主题和样式

use crate::jekyll::JekyllConfig;
use crate::types::Result;

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

/// 侧边栏组
#[derive(Debug, Clone)]
pub struct SidebarGroup {
    /// 组标题
    pub text: String,
    /// 组内项目
    pub items: Vec<SidebarLink>,
}

/// 侧边栏链接
#[derive(Debug, Clone)]
pub struct SidebarLink {
    /// 链接文本
    pub text: String,
    /// 链接地址
    pub link: String,
}

/// 导航栏项
#[derive(Debug, Clone)]
pub struct NavItem {
    /// 显示文本
    pub text: String,
    /// 链接
    pub link: String,
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

/// 默认主题
pub struct DefaultTheme {
    /// 主题配置
    config: JekyllConfig,
}

impl DefaultTheme {
    /// 创建新的默认主题实例
    pub fn new(config: JekyllConfig) -> Result<Self> {
        Ok(Self { config })
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
        // 使用简单的字符串替换实现模板渲染
        let template_content = include_str!("../templates/page.html");
        let mut result = template_content.to_string();

        // 简单的变量替换
        result = result.replace("{{ page_title }}", &context.page_title);
        result = result.replace("{{ site_title }}", &context.site_title);
        result = result.replace("{{ content }}", &context.content);

        Ok(result)
    }

    /// 获取站点标题
    ///
    /// # Returns
    ///
    /// 站点标题字符串
    pub fn site_title(&self) -> &str {
        self.config.title.as_deref().unwrap_or("Jekyll Site")
    }
}
