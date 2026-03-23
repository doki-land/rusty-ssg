//! KaTeX 数学公式渲染插件
//!
//! 提供对 Markdown 中 LaTeX 数学公式的支持，包括行内公式 `$...$` 和块级公式 `$$...$$`

use crate::plugin::{PluginContext, PluginMeta, PluginType, VitePressPlugin};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// 匹配块级公式 `$$...$$` 的正则表达式
    static ref BLOCK_MATH_RE: Regex = Regex::new(r"\$\$([\s\S]*?)\$\$").unwrap();
    /// 匹配行内公式 `$...$` 的正则表达式
    static ref INLINE_MATH_RE: Regex = Regex::new(r"\$([^$\n]+?)\$").unwrap();
}

/// KaTeX 数学公式渲染插件
pub struct KaTeXPlugin {
    /// 插件元数据
    meta: PluginMeta,
}

impl KaTeXPlugin {
    /// 创建新的 KaTeX 插件实例
    pub fn new() -> Self {
        Self {
            meta: PluginMeta::new(
                "vutex-plugin-katex".to_string(),
                "0.1.0".to_string(),
                "KaTeX 数学公式渲染插件，支持行内公式和块级公式".to_string(),
                PluginType::Markdown,
            ),
        }
    }

    /// 处理块级公式，将 `$$...$$` 替换为 `&lt;div class="katex-block"&gt;...&lt;/div&gt;`
    ///
    /// # Arguments
    ///
    /// * `content` - 包含数学公式的文本内容
    ///
    /// # Returns
    ///
    /// 替换后的文本内容
    fn process_block_math(&self, content: &str) -> String {
        BLOCK_MATH_RE
            .replace_all(content, |caps: &regex::Captures| {
                let math = &caps[1];
                format!("<div class=\"katex-block\">{}</div>", math.trim())
            })
            .to_string()
    }

    /// 处理行内公式，将 `$...$` 替换为 `&lt;span class="katex-inline"&gt;...&lt;/span&gt;`
    ///
    /// # Arguments
    ///
    /// * `content` - 包含数学公式的文本内容
    ///
    /// # Returns
    ///
    /// 替换后的文本内容
    fn process_inline_math(&self, content: &str) -> String {
        // 先处理块级公式，将其替换为临时标记
        let block_processed = self.process_block_math(content);
        // 再处理行内公式
        INLINE_MATH_RE
            .replace_all(&block_processed, |caps: &regex::Captures| {
                let math = &caps[1];
                format!("<span class=\"katex-inline\">{}</span>", math)
            })
            .to_string()
    }
}

impl Default for KaTeXPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl VitePressPlugin for KaTeXPlugin {
    /// 获取插件元数据
    fn meta(&self) -> &PluginMeta {
        &self.meta
    }

    /// 渲染前钩子，在 Markdown 解析后、HTML 渲染前处理数学公式
    ///
    /// # Arguments
    ///
    /// * `context` - 插件上下文，包含文档内容等信息
    ///
    /// # Returns
    ///
    /// 处理后的插件上下文
    fn before_render(&self, context: PluginContext) -> PluginContext {
        let content = self.process_inline_math(&context.content);
        PluginContext {
            content,
            frontmatter: context.frontmatter,
            path: context.path,
            config: context.config,
            site_data: context.site_data,
        }
    }
}
