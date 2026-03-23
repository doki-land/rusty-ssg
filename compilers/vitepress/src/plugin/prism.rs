//! Prism 代码高亮插件
//! 提供代码块的语法高亮功能

use super::{PluginContext, PluginMeta, PluginType, VitePressPlugin};
use nargo_types::NargoValue;
use std::collections::HashMap;

/// Prism 插件配置
#[derive(Debug, Clone)]
pub struct PrismPluginConfig {
    /// 是否启用行号
    pub line_numbers: bool,
    /// 代码主题
    pub theme: String,
    /// 额外的语言支持
    pub additional_languages: Vec<String>,
}

impl Default for PrismPluginConfig {
    fn default() -> Self {
        Self { line_numbers: true, theme: "prism-tomorrow".to_string(), additional_languages: Vec::new() }
    }
}

/// Prism 代码高亮插件
pub struct PrismPlugin {
    /// 插件元数据
    meta: PluginMeta,
    /// 插件配置
    config: PrismPluginConfig,
}

impl PrismPlugin {
    /// 创建新的 Prism 插件
    pub fn new() -> Self {
        Self {
            meta: PluginMeta::new("prism".to_string(), "0.1.0".to_string(), "Code syntax highlighting with Prism".to_string(), PluginType::Markdown),
            config: PrismPluginConfig::default(),
        }
    }

    /// 创建带配置的 Prism 插件
    pub fn with_config(config: PrismPluginConfig) -> Self {
        Self {
            meta: PluginMeta::new("prism".to_string(), "0.1.0".to_string(), "Code syntax highlighting with Prism".to_string(), PluginType::Markdown),
            config,
        }
    }
}

impl VitePressPlugin for PrismPlugin {
    /// 获取插件元数据
    fn meta(&self) -> &PluginMeta {
        &self.meta
    }

    /// 插件初始化钩子
    /// 在编译开始前调用，用于插件的初始化
    fn setup(&mut self, config: Option<HashMap<String, NargoValue>>) {
        if let Some(config) = config {
            if let Some(NargoValue::Bool(line_numbers)) = config.get("line_numbers") {
                self.config.line_numbers = *line_numbers;
            }
            if let Some(NargoValue::String(theme)) = config.get("theme") {
                self.config.theme = theme.clone();
            }
            if let Some(NargoValue::Array(languages)) = config.get("additional_languages") {
                self.config.additional_languages = languages
                    .iter()
                    .filter_map(|v| if let NargoValue::String(s) = v { Some(s.clone()) } else { None })
                    .collect();
            }
        }
    }

    /// 渲染后钩子
    /// 在 HTML 渲染后调用，用于添加代码高亮
    fn after_render(&self, context: PluginContext) -> PluginContext {
        let mut content = context.content;

        // 为代码块添加 Prism 样式和脚本
        if content.contains("<pre><code") {
            // 添加 Prism CSS
            let css_link = format!(
                "<link rel=\"stylesheet\" href=\"https://cdn.jsdelivr.net/npm/prismjs@1.29.0/themes/{}.min.css\">",
                self.config.theme
            );
            content = content.replace("<head>", &format!("<head>{}", css_link));

            // 添加 Prism JS
            let mut js_scripts =
                "<script src=\"https://cdn.jsdelivr.net/npm/prismjs@1.29.0/prism.min.js\"></script>".to_string();

            // 添加额外的语言支持
            for lang in &self.config.additional_languages {
                js_scripts.push_str(&format!(
                    "<script src=\"https://cdn.jsdelivr.net/npm/prismjs@1.29.0/components/prism-{}.min.js\"></script>",
                    lang
                ));
            }

            // 添加行号插件
            if self.config.line_numbers {
                js_scripts.push_str("<script src=\"https://cdn.jsdelivr.net/npm/prismjs@1.29.0/plugins/line-numbers/prism-line-numbers.min.js\"></script>");
                js_scripts.push_str("<link rel=\"stylesheet\" href=\"https://cdn.jsdelivr.net/npm/prismjs@1.29.0/plugins/line-numbers/prism-line-numbers.min.css\">");
                // 为代码块添加 line-numbers 类
                content = content.replace("<pre><code", "<pre class=\"line-numbers\"><code");
            }

            content = content.replace("</body>", &format!("{}</body>", js_scripts));
        }

        PluginContext { content, frontmatter: context.frontmatter, path: context.path, config: context.config, site_data: context.site_data }
    }
}
