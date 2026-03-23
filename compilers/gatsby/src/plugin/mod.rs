//! Gatsby 插件模块
//! 提供 Gatsby 静态站点生成器的插件系统

use crate::types::Result;
use nargo_types::NargoValue;
use std::collections::HashMap;

/// 插件错误类型
#[derive(Debug)]
pub enum PluginError {
    /// 插件加载错误
    LoadError(String),
    /// 插件执行错误
    ExecuteError(String),
    /// 插件配置错误
    ConfigError(String),
}

impl std::fmt::Display for PluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginError::LoadError(msg) => write!(f, "Plugin load error: {}", msg),
            PluginError::ExecuteError(msg) => write!(f, "Plugin execute error: {}", msg),
            PluginError::ConfigError(msg) => write!(f, "Plugin config error: {}", msg),
        }
    }
}

impl std::error::Error for PluginError {}

/// 插件元数据
#[derive(Debug, Clone)]
pub struct PluginMeta {
    /// 插件名称
    pub name: String,
    /// 插件版本
    pub version: String,
    /// 插件描述
    pub description: String,
}

impl PluginMeta {
    /// 创建新的插件元数据
    pub fn new(name: String, version: String, description: String) -> Self {
        Self { name, version, description }
    }
}

/// 节点类型
#[derive(Debug, Clone)]
pub struct Node {
    /// 节点 ID
    pub id: String,
    /// 节点类型
    pub node_type: String,
    /// 节点数据
    pub data: HashMap<String, NargoValue>,
    /// 内部字段
    pub internal: NodeInternal,
}

impl Node {
    /// 创建新的节点
    pub fn new(id: String, node_type: String) -> Self {
        Self { id, node_type, data: HashMap::new(), internal: NodeInternal::default() }
    }

    /// 设置节点数据
    pub fn with_data(mut self, data: HashMap<String, NargoValue>) -> Self {
        self.data = data;
        self
    }
}

/// 节点内部字段
#[derive(Debug, Clone, Default)]
pub struct NodeInternal {
    /// 内容摘要
    pub content_digest: String,
    /// 类型
    pub type_: String,
    /// 所有者
    pub owner: String,
}

impl NodeInternal {
    /// 创建新的节点内部字段
    pub fn new(content_digest: String, type_: String, owner: String) -> Self {
        Self { content_digest, type_, owner }
    }
}

/// 页面类型
#[derive(Debug, Clone)]
pub struct Page {
    /// 页面路径
    pub path: String,
    /// 页面组件
    pub component: String,
    /// 页面上下文
    pub context: HashMap<String, NargoValue>,
}

impl Page {
    /// 创建新的页面
    pub fn new(path: String, component: String) -> Self {
        Self { path, component, context: HashMap::new() }
    }

    /// 设置页面上下文
    pub fn with_context(mut self, context: HashMap<String, NargoValue>) -> Self {
        self.context = context;
        self
    }
}

/// 插件上下文
#[derive(Debug, Clone)]
pub struct PluginContext {
    /// 站点配置
    pub site_config: HashMap<String, NargoValue>,
    /// 节点存储
    pub nodes: HashMap<String, Node>,
    /// 页面存储
    pub pages: Vec<Page>,
}

impl PluginContext {
    /// 创建新的插件上下文
    pub fn new() -> Self {
        Self { site_config: HashMap::new(), nodes: HashMap::new(), pages: Vec::new() }
    }

    /// 添加节点
    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id.clone(), node);
    }

    /// 获取节点
    pub fn get_node(&self, id: &str) -> Option<&Node> {
        self.nodes.get(id)
    }

    /// 添加页面
    pub fn add_page(&mut self, page: Page) {
        self.pages.push(page);
    }
}

impl Default for PluginContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Gatsby 插件 Trait
/// 定义插件需要实现的生命周期钩子方法
pub trait Plugin: Send + Sync {
    /// 获取插件元数据
    fn meta(&self) -> &PluginMeta;

    /// onPreBootstrap 钩子
    /// 在引导开始前调用，用于插件的初始化
    fn on_pre_bootstrap(&mut self, _context: &mut PluginContext) -> Result<()> {
        Ok(())
    }

    /// onBootstrap 钩子
    /// 在引导过程中调用
    fn on_bootstrap(&mut self, _context: &mut PluginContext) -> Result<()> {
        Ok(())
    }

    /// onPostBootstrap 钩子
    /// 在引导完成后调用
    fn on_post_bootstrap(&mut self, _context: &mut PluginContext) -> Result<()> {
        Ok(())
    }

    /// onCreateNode 钩子
    /// 在节点创建时调用
    fn on_create_node(&mut self, _node: &mut Node, _context: &mut PluginContext) -> Result<()> {
        Ok(())
    }

    /// onCreatePage 钩子
    /// 在页面创建时调用
    fn on_create_page(&mut self, _page: &mut Page, _context: &mut PluginContext) -> Result<()> {
        Ok(())
    }

    /// onPreBuild 钩子
    /// 在构建开始前调用
    fn on_pre_build(&mut self, _context: &mut PluginContext) -> Result<()> {
        Ok(())
    }

    /// onPostBuild 钩子
    /// 在构建完成后调用
    fn on_post_build(&mut self, _context: &mut PluginContext) -> Result<()> {
        Ok(())
    }

    /// onPreExtractQueries 钩子
    /// 在提取查询前调用
    fn on_pre_extract_queries(&mut self, _context: &mut PluginContext) -> Result<()> {
        Ok(())
    }

    /// onPostExtractQueries 钩子
    /// 在提取查询后调用
    fn on_post_extract_queries(&mut self, _context: &mut PluginContext) -> Result<()> {
        Ok(())
    }

    /// onPreRenderHTML 钩子
    /// 在渲染 HTML 前调用
    fn on_pre_render_html(&mut self, _context: &mut PluginContext) -> Result<()> {
        Ok(())
    }

    /// onPostRenderHTML 钩子
    /// 在渲染 HTML 后调用
    fn on_post_render_html(&mut self, _context: &mut PluginContext) -> Result<()> {
        Ok(())
    }

    /// onRouteUpdate 钩子
    /// 在路由更新时调用
    fn on_route_update(&mut self, _context: &mut PluginContext) -> Result<()> {
        Ok(())
    }

    /// onClientEntry 钩子
    /// 在客户端入口时调用
    fn on_client_entry(&mut self, _context: &mut PluginContext) -> Result<()> {
        Ok(())
    }

    /// onInitialClientRender 钩子
    /// 在初始客户端渲染时调用
    fn on_initial_client_render(&mut self, _context: &mut PluginContext) -> Result<()> {
        Ok(())
    }

    /// onPreExtractQueries 钩子
    /// 在提取查询前调用
    fn on_extract_queries(&mut self, _context: &mut PluginContext) -> Result<()> {
        Ok(())
    }

    /// onCreateWebpackConfig 钩子
    /// 在创建 Webpack 配置时调用
    fn on_create_webpack_config(&mut self, _context: &mut PluginContext) -> Result<()> {
        Ok(())
    }

    /// onPreSourceNodes 钩子
    /// 在源代码节点前调用
    fn on_pre_source_nodes(&mut self, _context: &mut PluginContext) -> Result<()> {
        Ok(())
    }

    /// onPostSourceNodes 钩子
    /// 在源代码节点后调用
    fn on_post_source_nodes(&mut self, _context: &mut PluginContext) -> Result<()> {
        Ok(())
    }

    /// onPreProcessSource 钩子
    /// 在处理源代码前调用
    fn on_pre_process_source(&mut self, _context: &mut PluginContext) -> Result<()> {
        Ok(())
    }

    /// onPostProcessSource 钩子
    /// 在处理源代码后调用
    fn on_post_process_source(&mut self, _context: &mut PluginContext) -> Result<()> {
        Ok(())
    }
}

/// 插件注册表
/// 用于管理和注册 Gatsby 插件
pub struct PluginRegistry {
    /// 已注册的插件列表
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginRegistry {
    /// 创建新的插件注册表
    pub fn new() -> Self {
        Self { plugins: Vec::new() }
    }

    /// 注册一个插件
    pub fn register<P: Plugin + 'static>(&mut self, plugin: P) {
        self.plugins.push(Box::new(plugin));
    }

    /// 注册一个 Box 包装的插件
    pub fn register_box(&mut self, plugin: Box<dyn Plugin + 'static>) {
        self.plugins.push(plugin);
    }

    /// 从配置加载插件
    ///
    /// # Arguments
    ///
    /// * `plugins_config` - 插件配置列表
    ///
    /// # Returns
    ///
    /// 加载结果
    pub fn load_from_config(&mut self, plugins_config: &[crate::config::PluginConfig]) -> Result<()> {
        println!("🔌 Loading plugins...");

        for plugin_config in plugins_config {
            let plugin_name = plugin_config.name();
            println!("📦 Loading plugin: {}", plugin_name);

            // 尝试从本地文件系统加载插件
            match self.load_plugin_from_filesystem(plugin_name, plugin_config) {
                Ok(plugin) => {
                    self.register_box(plugin);
                    println!("✅ Plugin {} loaded successfully", plugin_name);
                },
                Err(e) => {
                    // 如果加载失败，使用模拟插件
                    println!("⚠️  Failed to load plugin {}, using mock instead: {:?}", plugin_name, e);
                    let plugin = Self::create_mock_plugin(plugin_name.to_string());
                    self.register(plugin);
                }
            }
        }

        Ok(())
    }

    /// 从文件系统加载插件
    ///
    /// # Arguments
    ///
    /// * `plugin_name` - 插件名称
    /// * `plugin_config` - 插件配置
    ///
    /// # Returns
    ///
    /// 加载的插件或错误
    fn load_plugin_from_filesystem(&self, plugin_name: &str, plugin_config: &crate::config::PluginConfig) -> Result<Box<dyn Plugin + 'static>> {
        use std::path::Path;

        // 尝试在 node_modules 中查找插件
        let plugin_paths = [
            Path::new("./node_modules/").join(plugin_name),
            Path::new("./plugins/").join(plugin_name),
        ];

        for path in &plugin_paths {
            if path.exists() {
                // 检查插件是否包含 package.json
                let package_json_path = path.join("package.json");
                if package_json_path.exists() {
                    // 这里可以实现更复杂的插件加载逻辑
                    // 目前只是创建一个基于配置的插件
                    let plugin = self.create_plugin_from_config(plugin_name, plugin_config);
                    return Ok(plugin);
                }
            }
        }

        Err(crate::types::GatsbyError::plugin(format!("Plugin {} not found in node_modules or plugins directory", plugin_name)))
    }

    /// 从配置创建插件
    ///
    /// # Arguments
    ///
    /// * `plugin_name` - 插件名称
    /// * `plugin_config` - 插件配置
    ///
    /// # Returns
    ///
    /// 创建的插件
    fn create_plugin_from_config(&self, plugin_name: &str, plugin_config: &crate::config::PluginConfig) -> Box<dyn Plugin + 'static> {
        struct ConfiguredPlugin {
            meta: PluginMeta,
            options: Option<std::collections::HashMap<String, serde_json::Value>>,
        }

        impl Plugin for ConfiguredPlugin {
            fn meta(&self) -> &PluginMeta {
                &self.meta
            }

            // 可以根据插件配置实现特定的钩子方法
        }

        Box::new(ConfiguredPlugin {
            meta: PluginMeta::new(plugin_name.to_string(), "1.0.0".to_string(), format!("Plugin {}", plugin_name)),
            options: plugin_config.options().cloned(),
        })
    }

    /// 创建模拟插件（用于测试）
    fn create_mock_plugin(name: String) -> impl Plugin {
        struct MockPlugin {
            meta: PluginMeta,
        }

        impl Plugin for MockPlugin {
            fn meta(&self) -> &PluginMeta {
                &self.meta
            }
        }

        MockPlugin { meta: PluginMeta::new(name, "1.0.0".to_string(), "Mock plugin for testing".to_string()) }
    }

    /// 调用所有插件的 on_pre_bootstrap 钩子
    pub fn on_pre_bootstrap_all(&mut self, context: &mut PluginContext) -> Result<()> {
        for plugin in &mut self.plugins {
            plugin.on_pre_bootstrap(context)?;
        }
        Ok(())
    }

    /// 调用所有插件的 on_bootstrap 钩子
    pub fn on_bootstrap_all(&mut self, context: &mut PluginContext) -> Result<()> {
        for plugin in &mut self.plugins {
            plugin.on_bootstrap(context)?;
        }
        Ok(())
    }

    /// 调用所有插件的 on_post_bootstrap 钩子
    pub fn on_post_bootstrap_all(&mut self, context: &mut PluginContext) -> Result<()> {
        for plugin in &mut self.plugins {
            plugin.on_post_bootstrap(context)?;
        }
        Ok(())
    }

    /// 调用所有插件的 on_create_node 钩子
    pub fn on_create_node_all(&mut self, node: &mut Node, context: &mut PluginContext) -> Result<()> {
        for plugin in &mut self.plugins {
            plugin.on_create_node(node, context)?;
        }
        Ok(())
    }

    /// 调用所有插件的 on_create_page 钩子
    pub fn on_create_page_all(&mut self, page: &mut Page, context: &mut PluginContext) -> Result<()> {
        for plugin in &mut self.plugins {
            plugin.on_create_page(page, context)?;
        }
        Ok(())
    }

    /// 调用所有插件的 on_pre_build 钩子
    pub fn on_pre_build_all(&mut self, context: &mut PluginContext) -> Result<()> {
        for plugin in &mut self.plugins {
            plugin.on_pre_build(context)?;
        }
        Ok(())
    }

    /// 调用所有插件的 on_post_build 钩子
    pub fn on_post_build_all(&mut self, context: &mut PluginContext) -> Result<()> {
        for plugin in &mut self.plugins {
            plugin.on_post_build(context)?;
        }
        Ok(())
    }

    /// 调用所有插件的 on_pre_extract_queries 钩子
    pub fn on_pre_extract_queries_all(&mut self, context: &mut PluginContext) -> Result<()> {
        for plugin in &mut self.plugins {
            plugin.on_pre_extract_queries(context)?;
        }
        Ok(())
    }

    /// 调用所有插件的 on_post_extract_queries 钩子
    pub fn on_post_extract_queries_all(&mut self, context: &mut PluginContext) -> Result<()> {
        for plugin in &mut self.plugins {
            plugin.on_post_extract_queries(context)?;
        }
        Ok(())
    }

    /// 调用所有插件的 on_pre_render_html 钩子
    pub fn on_pre_render_html_all(&mut self, context: &mut PluginContext) -> Result<()> {
        for plugin in &mut self.plugins {
            plugin.on_pre_render_html(context)?;
        }
        Ok(())
    }

    /// 调用所有插件的 on_post_render_html 钩子
    pub fn on_post_render_html_all(&mut self, context: &mut PluginContext) -> Result<()> {
        for plugin in &mut self.plugins {
            plugin.on_post_render_html(context)?;
        }
        Ok(())
    }

    /// 调用所有插件的 on_route_update 钩子
    pub fn on_route_update_all(&mut self, context: &mut PluginContext) -> Result<()> {
        for plugin in &mut self.plugins {
            plugin.on_route_update(context)?;
        }
        Ok(())
    }

    /// 调用所有插件的 on_client_entry 钩子
    pub fn on_client_entry_all(&mut self, context: &mut PluginContext) -> Result<()> {
        for plugin in &mut self.plugins {
            plugin.on_client_entry(context)?;
        }
        Ok(())
    }

    /// 调用所有插件的 on_initial_client_render 钩子
    pub fn on_initial_client_render_all(&mut self, context: &mut PluginContext) -> Result<()> {
        for plugin in &mut self.plugins {
            plugin.on_initial_client_render(context)?;
        }
        Ok(())
    }

    /// 调用所有插件的 on_extract_queries 钩子
    pub fn on_extract_queries_all(&mut self, context: &mut PluginContext) -> Result<()> {
        for plugin in &mut self.plugins {
            plugin.on_extract_queries(context)?;
        }
        Ok(())
    }

    /// 调用所有插件的 on_create_webpack_config 钩子
    pub fn on_create_webpack_config_all(&mut self, context: &mut PluginContext) -> Result<()> {
        for plugin in &mut self.plugins {
            plugin.on_create_webpack_config(context)?;
        }
        Ok(())
    }

    /// 调用所有插件的 on_pre_source_nodes 钩子
    pub fn on_pre_source_nodes_all(&mut self, context: &mut PluginContext) -> Result<()> {
        for plugin in &mut self.plugins {
            plugin.on_pre_source_nodes(context)?;
        }
        Ok(())
    }

    /// 调用所有插件的 on_post_source_nodes 钩子
    pub fn on_post_source_nodes_all(&mut self, context: &mut PluginContext) -> Result<()> {
        for plugin in &mut self.plugins {
            plugin.on_post_source_nodes(context)?;
        }
        Ok(())
    }

    /// 调用所有插件的 on_pre_process_source 钩子
    pub fn on_pre_process_source_all(&mut self, context: &mut PluginContext) -> Result<()> {
        for plugin in &mut self.plugins {
            plugin.on_pre_process_source(context)?;
        }
        Ok(())
    }

    /// 调用所有插件的 on_post_process_source 钩子
    pub fn on_post_process_source_all(&mut self, context: &mut PluginContext) -> Result<()> {
        for plugin in &mut self.plugins {
            plugin.on_post_process_source(context)?;
        }
        Ok(())
    }

    /// 获取已注册插件的数量
    pub fn plugin_count(&self) -> usize {
        self.plugins.len()
    }

    /// 获取所有已注册插件的元数据
    pub fn plugin_metas(&self) -> Vec<&PluginMeta> {
        self.plugins.iter().map(|p| p.meta()).collect()
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}
