//! 默认主题实现
//! 提供完整的文档站点主题和样式

use crate::Result;
use crate::types::VutexError;
use crate::types::VutexConfig;
use std::collections::HashMap;

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

/// 社交链接
#[derive(Debug, Clone)]
pub struct SocialLink {
    /// 图标名称
    pub icon: String,
    /// 链接地址
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
    /// 社交链接
    pub social_links: Vec<SocialLink>,
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
    config: VutexConfig,
}

impl DefaultTheme {
    /// 创建新的默认主题实例
    ///
    /// # Arguments
    ///
    /// * `config` - 主题配置
    ///
    /// # Returns
    ///
    /// 新的默认主题实例
    pub fn new(config: VutexConfig) -> Result<Self> {
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
        let template = include_str!("../templates/page.html");
        
        let mut html = template.to_string();
        
        html = html.replace("{{ page_title }}", &context.page_title);
        html = html.replace("{{ site_title }}", &context.site_title);
        
        let nav_html = context.nav_items.iter()
            .map(|item| format!("<li><a href=\"{}\">{}</a></li>", item.link, item.text))
            .collect::<Vec<_>>()
            .join("\n");
        html = html.replace("{% for item in &nav_items %}\n                    <li><a href=\"{{ item.link }}\">{{ item.text }}</a></li>\n                    {% endfor %}", &nav_html);
        
        let mut sidebar_html = String::new();
        for group in &context.sidebar_groups {
            if !group.items.is_empty() {
                sidebar_html.push_str(&format!("<li class=\"group-title\">{}</li>\n", group.text));
                for item in &group.items {
                    let active_class = if item.link == context.current_path { " class=\"active\"" } else { "" };
                    sidebar_html.push_str(&format!("<li><a href=\"{}\"{active_class}>{}</a></li>\n", item.link, item.text));
                }
            }
        }
        html = html.replace("{% for group in &sidebar_groups %}\n                    {% if !group.items.is_empty() %}\n                    <li class=\"group-title\">{{ group.text }}</li>\n                    {% for item in &group.items %}\n                    <li>\n                        <a href=\"{{ item.link }}\"{% if item.link == current_path %} class=\"active\"{% endif %}>{{ item.text }}</a>\n                    </li>\n                    {% endfor %}\n                    {% endif %}\n                    {% endfor %}", &sidebar_html);
        
        let social_html = context.social_links.iter()
            .map(|link| format!("<a href=\"{}\" class=\"social-link\">{}</a>", link.link, link.icon))
            .collect::<Vec<_>>()
            .join("\n");
        html = html.replace("{% for link in &social_links %}\n                    <a href=\"{{ link.link }}\" class=\"social-link\">{{ link.icon }}</a>\n                    {% endfor %}", &social_html);
        
        html = html.replace("{{ content|safe }}", &context.content);
        
        let mut footer_html = String::new();
        if context.has_footer {
            footer_html.push_str("<footer class=\"footer\">\n");
            if context.has_footer_message {
                footer_html.push_str(&format!("<p>{}</p>\n", context.footer_message));
            }
            if context.has_footer_copyright {
                footer_html.push_str(&format!("<p>{}</p>\n", context.footer_copyright));
            }
            footer_html.push_str("</footer>");
        }
        html = html.replace("{% if has_footer %}\n                <footer class=\"footer\">\n                    {% if has_footer_message %}\n                    <p>{{ footer_message }}</p>\n                    {% endif %}\n                    {% if has_footer_copyright %}\n                    <p>{{ footer_copyright }}</p>\n                    {% endif %}\n                </footer>\n                {% endif %}", &footer_html);
        
        Ok(html)
    }

    /// 获取站点标题
    ///
    /// # Returns
    ///
    /// 站点标题字符串
    pub fn site_title(&self) -> &str {
        self.config.title.as_deref().unwrap_or("VitePress Documentation")
    }

    /// 获取主题配置
    ///
    /// # Returns
    ///
    /// 主题配置引用
    pub fn config(&self) -> &VutexConfig {
        &self.config
    }
}
