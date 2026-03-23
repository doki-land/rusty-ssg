//! 搜索插件
//! 
//! 提供对文档的搜索功能

use crate::plugin::{PluginContext, PluginMeta, VuePressPlugin};

/// 搜索插件
pub struct SearchPlugin {
    /// 插件元数据
    meta: PluginMeta,
}

impl SearchPlugin {
    /// 创建新的搜索插件实例
    pub fn new() -> Self {
        Self {
            meta: PluginMeta::new(
                "vutex-plugin-search".to_string(),
                "0.1.0".to_string(),
                "文档搜索插件，支持全文搜索".to_string(),
            ),
        }
    }
}

impl Default for SearchPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl VuePressPlugin for SearchPlugin {
    /// 获取插件元数据
    fn meta(&self) -> &PluginMeta {
        &self.meta
    }

    /// 渲染后钩子，添加搜索功能
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

        // 添加搜索脚本
        let search_script = r#"
<script>
// 简单的搜索功能
const searchIndex = [
    // 这里会被站点生成器填充
];

function search(query) {
    const results = searchIndex.filter(item => 
        item.title.toLowerCase().includes(query.toLowerCase()) || 
        item.content.toLowerCase().includes(query.toLowerCase())
    );
    return results;
}

// 初始化搜索
function initSearch() {
    const searchInput = document.getElementById('search-input');
    const searchResults = document.getElementById('search-results');
    
    if (searchInput && searchResults) {
        searchInput.addEventListener('input', (e) => {
            const query = e.target.value;
            const results = search(query);
            
            searchResults.innerHTML = results.map(item => 
                `<a href="${item.path}" class="search-result">${item.title}</a>`
            ).join('');
        });
    }
}

// 页面加载完成后初始化搜索
window.addEventListener('DOMContentLoaded', initSearch);
</script>
"#;

        // 在 </body> 标签前添加搜索脚本
        if let Some(body_end) = content.find("</body>") {
            content.insert_str(body_end, search_script);
        }

        PluginContext { content, frontmatter: context.frontmatter, path: context.path }
    }
}