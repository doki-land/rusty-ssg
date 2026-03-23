//! Prism 代码高亮插件
//! 
//! 提供对 Markdown 中代码块的语法高亮支持

use crate::plugin::{PluginContext, PluginMeta, VuePressPlugin};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// 匹配代码块的正则表达式
    static ref CODE_BLOCK_RE: Regex = Regex::new(r"```([a-zA-Z0-9_-]*)([\s\S]*?)```").unwrap();
}

/// Prism 代码高亮插件
pub struct PrismPlugin {
    /// 插件元数据
    meta: PluginMeta,
}

impl PrismPlugin {
    /// 创建新的 Prism 插件实例
    pub fn new() -> Self {
        Self {
            meta: PluginMeta::new(
                "vutex-plugin-prism".to_string(),
                "0.1.0".to_string(),
                "Prism 代码高亮插件，支持代码块语法高亮".to_string(),
            ),
        }
    }

    /// 处理代码块，添加 Prism 语法高亮
    ///
    /// # Arguments
    ///
    /// * `content` - 包含代码块的文本内容
    ///
    /// # Returns
    ///
    /// 替换后的文本内容
    pub fn process_code_blocks(&self, content: &str) -> String {
        CODE_BLOCK_RE
            .replace_all(content, |caps: &regex::Captures| {
                let language = &caps[1];
                let code = &caps[2];
                
                if language.is_empty() {
                    format!("```
{}
```", code)
                } else {
                    format!("``` {}
{}
```", language, code)
                }
            })
            .to_string()
    }
}

impl Default for PrismPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl VuePressPlugin for PrismPlugin {
    /// 获取插件元数据
    fn meta(&self) -> &PluginMeta {
        &self.meta
    }

    /// 渲染前钩子，在 Markdown 解析后、HTML 渲染前处理代码块
    ///
    /// # Arguments
    ///
    /// * `context` - 插件上下文，包含文档内容等信息
    ///
    /// # Returns
    ///
    /// 处理后的插件上下文
    fn before_render(&self, context: PluginContext) -> PluginContext {
        let content = self.process_code_blocks(&context.content);
        
        PluginContext { content, frontmatter: context.frontmatter, path: context.path }
    }

    /// 渲染后钩子，在 HTML 渲染后添加 Prism 脚本
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
        
        // 添加 Prism 脚本和样式
        let prism_scripts = r#"
<script src="https://cdn.jsdelivr.net/npm/prismjs@1.29.0/prism.min.js"></script>
<script src="https://cdn.jsdelivr.net/npm/prismjs@1.29.0/components/prism-bash.min.js"></script>
<script src="https://cdn.jsdelivr.net/npm/prismjs@1.29.0/components/prism-javascript.min.js"></script>
<script src="https://cdn.jsdelivr.net/npm/prismjs@1.29.0/components/prism-typescript.min.js"></script>
<script src="https://cdn.jsdelivr.net/npm/prismjs@1.29.0/components/prism-rust.min.js"></script>
<script src="https://cdn.jsdelivr.net/npm/prismjs@1.29.0/components/prism-python.min.js"></script>
<script src="https://cdn.jsdelivr.net/npm/prismjs@1.29.0/components/prism-java.min.js"></script>
<script src="https://cdn.jsdelivr.net/npm/prismjs@1.29.0/components/prism-css.min.js"></script>
<script src="https://cdn.jsdelivr.net/npm/prismjs@1.29.0/components/prism-markdown.min.js"></script>
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/prismjs@1.29.0/themes/prism.min.css">
"#;
        
        // 在 </body> 标签前添加 Prism 脚本
        if let Some(body_end) = content.find("</body>") {
            content.insert_str(body_end, prism_scripts);
        }
        
        PluginContext { content, frontmatter: context.frontmatter, path: context.path }
    }
}
