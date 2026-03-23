//! Mermaid 图表渲染插件
//! 
//! 提供对 Markdown 中 Mermaid 图表的支持

use crate::plugin::{PluginContext, PluginMeta, VuePressPlugin};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// 匹配 Mermaid 图表的正则表达式
    static ref MERMAID_BLOCK_RE: Regex = Regex::new(r"```mermaid([\s\S]*?)```").unwrap();
}

/// Mermaid 图表渲染插件
pub struct MermaidPlugin {
    /// 插件元数据
    meta: PluginMeta,
}

impl MermaidPlugin {
    /// 创建新的 Mermaid 插件实例
    pub fn new() -> Self {
        Self {
            meta: PluginMeta::new(
                "vutex-plugin-mermaid".to_string(),
                "0.1.0".to_string(),
                "Mermaid 图表渲染插件，支持流程图和图表".to_string(),
            ),
        }
    }

    /// 处理 Mermaid 图表块
    ///
    /// # Arguments
    ///
    /// * `content` - 包含 Mermaid 图表的文本内容
    ///
    /// # Returns
    ///
    /// 替换后的文本内容
    pub fn process_mermaid_blocks(&self, content: &str) -> String {
        MERMAID_BLOCK_RE
            .replace_all(content, |caps: &regex::Captures| {
                let diagram = &caps[1];
                
                format!(
                    r#"<div class="mermaid">
{}
</div>"#,
                    diagram
                )
            })
            .to_string()
    }
}

impl Default for MermaidPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl VuePressPlugin for MermaidPlugin {
    /// 获取插件元数据
    fn meta(&self) -> &PluginMeta {
        &self.meta
    }

    /// 渲染前钩子，在 Markdown 解析后、HTML 渲染前处理 Mermaid 图表
    ///
    /// # Arguments
    ///
    /// * `context` - 插件上下文，包含文档内容等信息
    ///
    /// # Returns
    ///
    /// 处理后的插件上下文
    fn before_render(&self, context: PluginContext) -> PluginContext {
        let content = self.process_mermaid_blocks(&context.content);

        PluginContext { content, frontmatter: context.frontmatter, path: context.path }
    }

    /// 渲染后钩子，在 HTML 渲染后添加 Mermaid 脚本
    ///
    /// # Arguments
    ///
    /// * `context` - 插件上下文，包含文档内容等信息
    ///
    /// # Returns
    ///
    /// 处理后的插件上下文
    fn after_render(&self, context: PluginContext) -> PluginContext {
        let mut content = context.content;

        // 添加 Mermaid 脚本
        let mermaid_script = r#"
<script src="https://cdn.jsdelivr.net/npm/mermaid@10.4.0/dist/mermaid.min.js"></script>
<script>
mermaid.initialize({ startOnLoad: true });
</script>
"#;

        // 在 </body> 标签前添加 Mermaid 脚本
        if let Some(body_end) = content.find("</body>") {
            content.insert_str(body_end, mermaid_script);
        }

        PluginContext { content, frontmatter: context.frontmatter, path: context.path }
    }
}
