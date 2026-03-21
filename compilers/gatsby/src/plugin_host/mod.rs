//! Gatsby 插件宿主模块
//! 提供插件的加载、管理和执行环境

use crate::{
    plugin::{Node, Page, Plugin, PluginContext, PluginRegistry},
    types::Result,
};
use nargo_types::NargoValue;
use std::collections::HashMap;

/// 插件宿主错误类型
#[derive(Debug)]
pub enum PluginHostError {
    /// 宿主启动错误
    StartError(String),
    /// 宿主通信错误
    CommunicationError(String),
    /// 插件初始化错误
    PluginInitError(String),
}

impl std::fmt::Display for PluginHostError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginHostError::StartError(msg) => write!(f, "Plugin host start error: {}", msg),
            PluginHostError::CommunicationError(msg) => write!(f, "Plugin host communication error: {}", msg),
            PluginHostError::PluginInitError(msg) => write!(f, "Plugin initialization error: {}", msg),
        }
    }
}

impl std::error::Error for PluginHostError {}

/// 插件宿主
/// 管理插件的生命周期和执行
pub struct PluginHost {
    /// 插件注册表
    registry: PluginRegistry,
    /// 插件上下文
    context: PluginContext,
    /// 宿主配置
    config: HashMap<String, NargoValue>,
}

impl PluginHost {
    /// 创建新的插件宿主
    pub fn new() -> Self {
        Self { registry: PluginRegistry::new(), context: PluginContext::new(), config: HashMap::new() }
    }

    /// 从配置创建新的插件宿主
    pub fn with_config(config: HashMap<String, NargoValue>) -> Self {
        let mut host = Self::new();
        host.config = config;
        host.context.site_config = host.config.clone();
        host
    }

    /// 注册一个插件
    pub fn register_plugin<P: Plugin + 'static>(&mut self, plugin: P) {
        self.registry.register(plugin);
    }

    /// 启动插件宿主
    pub fn start(&mut self) -> Result<()> {
        self.registry.on_pre_bootstrap_all(&mut self.context)?;
        self.registry.on_bootstrap_all(&mut self.context)?;
        self.registry.on_post_bootstrap_all(&mut self.context)?;
        Ok(())
    }

    /// 创建并处理一个节点
    pub fn create_node(&mut self, mut node: Node) -> Result<Node> {
        self.registry.on_create_node_all(&mut node, &mut self.context)?;
        self.context.add_node(node.clone());
        Ok(node)
    }

    /// 创建并处理一个页面
    pub fn create_page(&mut self, mut page: Page) -> Result<Page> {
        self.registry.on_create_page_all(&mut page, &mut self.context)?;
        self.context.add_page(page.clone());
        Ok(page)
    }

    /// 执行预构建钩子
    pub fn pre_build(&mut self) -> Result<()> {
        self.registry.on_pre_extract_queries_all(&mut self.context)?;
        self.registry.on_pre_build_all(&mut self.context)?;
        Ok(())
    }

    /// 执行后构建钩子
    pub fn post_build(&mut self) -> Result<()> {
        self.registry.on_post_extract_queries_all(&mut self.context)?;
        self.registry.on_post_build_all(&mut self.context)?;
        Ok(())
    }

    /// 获取插件上下文
    pub fn context(&self) -> &PluginContext {
        &self.context
    }

    /// 获取可变的插件上下文
    pub fn context_mut(&mut self) -> &mut PluginContext {
        &mut self.context
    }

    /// 获取插件注册表
    pub fn registry(&self) -> &PluginRegistry {
        &self.registry
    }

    /// 获取已注册插件的数量
    pub fn plugin_count(&self) -> usize {
        self.registry.plugin_count()
    }

    /// 获取所有已注册插件的元数据
    pub fn plugin_metas(&self) -> Vec<&crate::plugin::PluginMeta> {
        self.registry.plugin_metas()
    }

    /// 获取所有节点
    pub fn nodes(&self) -> &HashMap<String, Node> {
        &self.context.nodes
    }

    /// 获取所有页面
    pub fn pages(&self) -> &[Page] {
        &self.context.pages
    }

    /// 根据 ID 获取节点
    pub fn get_node(&self, id: &str) -> Option<&Node> {
        self.context.get_node(id)
    }
}

impl Default for PluginHost {
    fn default() -> Self {
        Self::new()
    }
}
