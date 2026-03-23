//! 编译器模块
//! 提供 VuePress 文档编译器的核心功能

use crate::types::{
    Result, VuePressConfig,
    ipc::{InvokePluginRequest, PluginContext},
};
use nargo_parser::parse_document;
use nargo_types::Document;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

mod html_renderer;
mod parser;
pub use html_renderer::{HtmlRenderer, HtmlRendererConfig};
pub use parser::*;

use crate::plugin_host::PluginHost;

/// VuePress 文档编译器
///
/// 负责将 Markdown 文档编译为 VuePress 文档格式，支持通过 Node.js 混合执行模式调用插件
pub struct VuePressCompiler {
    /// 编译器配置
    config: VuePressConfig,
    /// HTML 渲染器
    html_renderer: HtmlRenderer,
    /// 编译缓存
    cache: HashMap<String, Document>,
    /// 插件宿主（可选）
    plugin_host: Option<PluginHost>,
}

impl VuePressCompiler {
    /// 创建新的编译器（无插件支持，降级模式）
    pub fn new() -> Self {
        let config = VuePressConfig::new();
        Self { config: config.clone(), html_renderer: HtmlRenderer::new(config), cache: HashMap::new(), plugin_host: None }
    }

    /// 创建带配置的编译器（无插件支持，降级模式）
    ///
    /// # Arguments
    ///
    /// * `config` - 编译器配置
    pub fn with_config(config: VuePressConfig) -> Self {
        Self { config: config.clone(), html_renderer: HtmlRenderer::new(config), cache: HashMap::new(), plugin_host: None }
    }

    /// 创建带 PluginHost 的编译器（支持 Node.js 混合执行模式）
    ///
    /// # Arguments
    ///
    /// * `plugin_host` - 插件宿主实例
    pub fn with_plugin_host(plugin_host: PluginHost) -> Self {
        let config = VuePressConfig::new();
        Self {
            config: config.clone(),
            html_renderer: HtmlRenderer::new(config),
            cache: HashMap::new(),
            plugin_host: Some(plugin_host),
        }
    }

    /// 创建带配置和 PluginHost 的编译器（支持 Node.js 混合执行模式）
    ///
    /// # Arguments
    ///
    /// * `config` - 编译器配置
    /// * `plugin_host` - 插件宿主实例
    pub fn with_config_and_plugin_host(config: VuePressConfig, plugin_host: PluginHost) -> Self {
        Self {
            config: config.clone(),
            html_renderer: HtmlRenderer::new(config),
            cache: HashMap::new(),
            plugin_host: Some(plugin_host),
        }
    }

    /// 获取编译器配置
    pub fn config(&self) -> &VuePressConfig {
        &self.config
    }

    /// 获取可变的编译器配置
    pub fn config_mut(&mut self) -> &mut VuePressConfig {
        &mut self.config
    }

    /// 获取 HTML 渲染器
    pub fn html_renderer(&self) -> &HtmlRenderer {
        &self.html_renderer
    }

    /// 获取可变的 HTML 渲染器
    pub fn html_renderer_mut(&mut self) -> &mut HtmlRenderer {
        &mut self.html_renderer
    }

    /// 获取插件宿主的可变引用
    pub fn plugin_host_mut(&mut self) -> Option<&mut PluginHost> {
        self.plugin_host.as_mut()
    }

    /// 获取插件宿主的不可变引用
    pub fn plugin_host(&self) -> Option<&PluginHost> {
        self.plugin_host.as_ref()
    }

    /// 设置插件宿主
    ///
    /// # Arguments
    ///
    /// * `plugin_host` - 插件宿主实例
    pub fn set_plugin_host(&mut self, plugin_host: PluginHost) {
        self.plugin_host = Some(plugin_host);
    }

    /// 移除插件宿主，进入降级模式
    pub fn remove_plugin_host(&mut self) {
        self.plugin_host = None;
    }

    /// 将 frontmatter 转换为 HashMap<String, String>
    ///
    /// # Arguments
    ///
    /// * `doc` - 文档对象
    fn convert_frontmatter_to_map(&self, _doc: &nargo_types::Document) -> HashMap<String, String> {
        let mut map = HashMap::new();
        // 暂时返回空 map，因为新的 Document 结构没有 frontmatter 字段
        map
    }

    /// 应用主题模板
    ///
    /// # Arguments
    ///
    /// * `content` - 渲染后的内容
    /// * `title` - 页面标题
    ///
    /// # Returns
    ///
    /// 应用模板后的完整 HTML
    fn apply_theme_template(&self, content: &str, title: &str) -> String {
        use crate::tools::theme::{DefaultTheme, NavItem, PageContext, SidebarGroup, SidebarLink};
        
        // 创建默认主题实例
        let theme = DefaultTheme::new(self.config.clone()).unwrap();
        
        // 构建页面上下文
        let page_context = PageContext {
            page_title: title.to_string(),
            site_title: theme.site_title().to_string(),
            content: content.to_string(),
            nav_items: vec![
                NavItem { text: "Home".to_string(), link: "/".to_string() },
                NavItem { text: "Guide".to_string(), link: "/guide/".to_string() },
                NavItem { text: "API".to_string(), link: "/api/".to_string() },
            ],
            sidebar_groups: vec![
                SidebarGroup {
                    text: "Getting Started".to_string(),
                    items: vec![
                        SidebarLink { text: "Introduction".to_string(), link: "/guide/introduction/".to_string() },
                        SidebarLink { text: "Installation".to_string(), link: "/guide/installation/".to_string() },
                        SidebarLink { text: "Quick Start".to_string(), link: "/guide/quick-start/".to_string() },
                    ],
                },
                SidebarGroup {
                    text: "Advanced".to_string(),
                    items: vec![
                        SidebarLink { text: "Configuration".to_string(), link: "/guide/configuration/".to_string() },
                        SidebarLink { text: "Themes".to_string(), link: "/guide/themes/".to_string() },
                        SidebarLink { text: "Plugins".to_string(), link: "/guide/plugins/".to_string() },
                    ],
                },
            ],
            current_path: "/".to_string(),
            has_footer: true,
            has_footer_message: true,
            footer_message: "Powered by VuePress".to_string(),
            has_footer_copyright: true,
            footer_copyright: "© 2024 VuePress".to_string(),
            current_lang: "en-US".to_string(),
            available_locales: vec![],
            root_path: "/".to_string(),
        };
        
        // 渲染页面
        match theme.render_page(&page_context) {
            Ok(html) => {
                // 添加 Vue 运行时
                let html_with_vue = html.replace("</body>", &format!(
                    r#"
    <!-- Vue 运行时 -->
    <script src="https://cdn.jsdelivr.net/npm/vue@3.3.4/dist/vue.global.prod.js"></script>
    <script>
        const {{ createApp }} = Vue
        createApp({{}}).mount('#app')
    </script>
</body>"#
                ));
                
                // 修复模板语法错误
                let html_with_vue_fixed = html_with_vue
                    .replace("const {{ createApp }}", "const { createApp }")
                    .replace("createApp({{}})", "createApp({})");
                html_with_vue_fixed
            },
            Err(_) => {
                // 回退到简单模板
                format!(
                    r#"<!DOCTYPE html>
<html lang="{lang}">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title}</title>
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@vuepress/theme-default@latest/style.css">
</head>
<body>
    <div id="app">
        <div class="container">
            <header class="header">
                <h1>{title}</h1>
            </header>
            <main class="content">
                {content}
            </main>
            <footer class="footer">
                <p>Powered by VuePress</p>
            </footer>
        </div>
    </div>
    <!-- Vue 运行时 -->
    <script src="https://cdn.jsdelivr.net/npm/vue@3.3.4/dist/vue.global.prod.js"></script>
    <script>
        const { createApp } = Vue
        createApp({}).mount('#app')
    </script>
</body>
</html>"#,
                    lang = "en-US",
                    title = title,
                    content = content
                )
            }
        }
    }

    /// 编译单个文档
    ///
    /// # Arguments
    ///
    /// * `source` - 文档内容
    /// * `path` - 文档路径
    ///
    /// # Returns
    ///
    /// 编译后的文档
    pub fn compile_document(&mut self, source: &str, path: &str) -> Result<Document> {
        if let Some(cached) = self.cache.get(path) {
            return Ok(cached.clone());
        }

        let mut doc = crate::compiler::parser::parse_content_file(source, path)?;

        let frontmatter_map = self.convert_frontmatter_to_map(&doc);

        let mut content = doc.content.clone();

        if let Some(ref mut plugin_host) = self.plugin_host {
            let context = PluginContext::new(content.clone(), frontmatter_map.clone(), path.to_string());

            content = Self::invoke_hook(plugin_host, "before_render", context)?;
        }

        let rendered_html = self.html_renderer.render(&content)?;
        let mut final_html = rendered_html;

        if let Some(ref mut plugin_host) = self.plugin_host {
            let context = PluginContext::new(final_html.clone(), frontmatter_map, path.to_string());

            final_html = Self::invoke_hook(plugin_host, "after_render", context)?;
        }

        // 应用主题模板
        let page_title = path.split('/').last().unwrap_or(path).replace(".md", "").replace(".vue", "");
        let templated_html = self.apply_theme_template(&final_html, &page_title);

        doc.rendered_content = Some(templated_html);
        self.cache.insert(path.to_string(), doc.clone());

        Ok(doc)
    }

    /// 调用插件钩子
    ///
    /// # Arguments
    ///
    /// * `plugin_host` - 插件宿主实例
    /// * `hook_name` - 钩子名称
    /// * `context` - 插件调用上下文
    fn invoke_hook(plugin_host: &mut PluginHost, hook_name: &str, context: PluginContext) -> Result<String> {
        let request = InvokePluginRequest::new(hook_name.to_string(), "*".to_string(), context);

        let timeout = Duration::from_secs(30);
        let response = plugin_host
            .invoke_plugin(request, timeout)
            .map_err(|e| crate::types::VutexError::ConfigError { message: format!("{}", e) })?;

        if response.success {
            Ok(response.content.unwrap_or_default())
        }
        else {
            Err(crate::types::VutexError::ConfigError {
                message: response.error.unwrap_or_else(|| "Unknown plugin error".to_string()),
            })
        }
    }

    /// 批量编译文档
    ///
    /// # Arguments
    ///
    /// * `documents` - 文档映射（路径 -> 内容）
    ///
    /// # Returns
    ///
    /// 编译结果
    pub fn compile_batch(&mut self, documents: &HashMap<String, String>) -> super::CompileResult {
        let start_time = Instant::now();
        let mut compiled_docs = HashMap::new();
        let mut errors = Vec::new();

        for (path, source) in documents {
            match self.compile_document(source, path) {
                Ok(doc) => {
                    compiled_docs.insert(path.to_string(), doc);
                }
                Err(e) => {
                    errors.push(e);
                }
            }
        }

        let compile_time_ms = start_time.elapsed().as_millis() as u64;

        if errors.is_empty() {
            super::CompileResult::success(compiled_docs, compile_time_ms)
        }
        else {
            super::CompileResult::from_errors(errors, compile_time_ms)
        }
    }

    /// 清除编译缓存
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// 从缓存中获取文档
    pub fn get_cached(&self, path: &str) -> Option<&Document> {
        self.cache.get(path)
    }
}

impl Default for VuePressCompiler {
    fn default() -> Self {
        Self::new()
    }
}
