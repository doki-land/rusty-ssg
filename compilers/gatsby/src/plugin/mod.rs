//! Gatsby 插件模块
//! 提供 Gatsby 静态站点生成器的插件系统

use std::collections::HashMap;
use nargo_types::NargoValue;
use crate::types::Result;

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
        Self {
            id,
            node_type,
            data: HashMap::new(),
            internal: NodeInternal::default(),
        }
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
        Self {
            path,
            component,
            context: HashMap::new(),
        }
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
        Self {
            site_config: HashMap::new(),
            nodes: HashMap::new(),
            pages: Vec::new(),
        }
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
