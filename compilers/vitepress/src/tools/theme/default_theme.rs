//! 默认主题实现
//! 提供完整的文档站点主题和样式

use crate::{Result, types::VutexConfig};

/// 语言信息，包含语言代码、标签和是否为当前语言的状态
#[derive(Debug, Clone)]
pub struct LocaleInfo {
    /// 语言代码，如 `en`、`zh-CN`
    pub code: String,
    /// 语言标签，用于在界面上显示，如 "English"、"中文"
    pub label: String,
    /// 是否为当前语言
    pub is_current: bool,
}

/// 侧边栏组，用于组织侧边栏链接
#[derive(Debug, Clone)]
pub struct SidebarGroup {
    /// 侧边栏组的标题文本
    pub text: String,
    /// 侧边栏组内的链接项目列表
    pub items: Vec<SidebarLink>,
}

/// 侧边栏链接，表示侧边栏中的单个可点击链接
#[derive(Debug, Clone)]
pub struct SidebarLink {
    /// 侧边栏链接的显示文本
    pub text: String,
    /// 侧边栏链接的目标地址
    pub link: String,
}

/// 导航栏项，表示顶部导航栏中的单个导航项
#[derive(Debug, Clone)]
pub struct NavItem {
    /// 导航栏项的显示文本
    pub text: String,
    /// 导航栏项的链接地址
    pub link: String,
}

/// 社交链接，表示站点社交信息的链接
#[derive(Debug, Clone)]
pub struct SocialLink {
    /// 社交链接的图标名称或标识符
    pub icon: String,
    /// 社交链接的目标地址
    pub link: String,
}

/// 页面模板上下文，包含渲染页面所需的所有数据
#[derive(Debug, Clone)]
pub struct PageContext {
    /// 当前页面的完整标题，通常包含页面标题和站点标题
    pub page_title: String,
    /// 站点的标题
    pub site_title: String,
    /// 页面的主要内容，通常是经过 Markdown 渲染后的 HTML
    pub content: String,
    /// 导航栏项目列表
    pub nav_items: Vec<NavItem>,
    /// 侧边栏组列表
    pub sidebar_groups: Vec<SidebarGroup>,
    /// 社交链接列表
    pub social_links: Vec<SocialLink>,
    /// 当前页面的路径
    pub current_path: String,
    /// 是否显示页脚
    pub has_footer: bool,
    /// 是否显示页脚消息
    pub has_footer_message: bool,
    /// 页脚的消息文本
    pub footer_message: String,
    /// 是否显示页脚版权信息
    pub has_footer_copyright: bool,
    /// 页脚的版权信息文本
    pub footer_copyright: String,
    /// 当前语言代码
    pub current_lang: String,
    /// 可用的语言信息列表，用于语言切换功能
    pub available_locales: Vec<LocaleInfo>,
    /// 相对于根目录的路径前缀，用于生成相对路径链接
    pub root_path: String,
}

/// 默认主题，用于渲染文档站点的页面
pub struct DefaultTheme {
    /// 主题配置，包含站点的配置信息
    config: VutexConfig,
}

impl DefaultTheme {
    /// 创建新的默认主题实例
    ///
    /// # Arguments
    ///
    /// * `config` - 站点配置，包含主题配置信息
    ///
    /// # Returns
    ///
    /// 新的默认主题实例，如果创建成功则返回 `Ok(DefaultTheme)`，否则返回错误
    pub fn new(config: VutexConfig) -> Result<Self> {
        Ok(Self { config })
    }

    /// 使用页面上下文渲染完整的 HTML 页面
    ///
    /// # Arguments
    ///
    /// * `context` - 页面上下文，包含渲染页面所需的所有数据
    ///
    /// # Returns
    ///
    /// 渲染后的 HTML 字符串，如果渲染成功则返回 `Ok(String)`，否则返回错误
    pub fn render_page(&self, context: &PageContext) -> Result<String> {
        let template = include_str!("../templates/page.html");

        let mut html = template.to_string();

        html = html.replace("{{ page_title }}", &context.page_title);
        html = html.replace("{{ site_title }}", &context.site_title);

        let nav_html = context
            .nav_items
            .iter()
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

        let social_html = context
            .social_links
            .iter()
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
    /// 站点标题字符串，如果未配置则返回默认标题 "VitePress Documentation"
    pub fn site_title(&self) -> &str {
        self.config.title.as_deref().unwrap_or("VitePress Documentation")
    }

    /// 获取主题配置
    ///
    /// # Returns
    ///
    /// 站点配置的不可变引用
    pub fn config(&self) -> &VutexConfig {
        &self.config
    }
}
