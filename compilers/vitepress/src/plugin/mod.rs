//! 插件模块
//! 提供 VitePress 文档编译器的插件系统

use nargo_types::NargoValue;
use std::{collections::HashMap, hash::Hash};

pub mod katex;
pub mod prism;

/// 插件类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PluginType {
    /// 核心插件
    Core,
    /// Markdown 插件
    Markdown,
    /// 主题插件
    Theme,
    /// 构建插件
    Build,
    /// 其他插件
    Other,
}

/// 插件元数据
#[derive(Debug, Clone)]
pub struct PluginMeta {
    /// 插件名称
    pub name: String,
    /// 插件版本
    pub version: String,
    /// 插件描述
    pub description: String,
    /// 插件类型
    pub plugin_type: PluginType,
    /// 插件作者
    pub author: Option<String>,
    /// 插件主页
    pub homepage: Option<String>,
}

impl PluginMeta {
    /// 创建新的插件元数据
    pub fn new(name: String, version: String, description: String, plugin_type: PluginType) -> Self {
        Self { name, version, description, plugin_type, author: None, homepage: None }
    }

    /// 设置插件作者
    pub fn with_author(mut self, author: String) -> Self {
        self.author = Some(author);
        self
    }

    /// 设置插件主页
    pub fn with_homepage(mut self, homepage: String) -> Self {
        self.homepage = Some(homepage);
        self
    }
}

/// 插件上下文
/// 提供给插件处理的文档信息
#[derive(Debug, Clone)]
pub struct PluginContext {
    /// 文档内容
    pub content: String,
    /// 前置元数据
    pub frontmatter: HashMap<String, NargoValue>,
    /// 文档路径
    pub path: String,
    /// 全局配置
    pub config: Option<HashMap<String, NargoValue>>,
    /// 站点数据
    pub site_data: Option<HashMap<String, NargoValue>>,
}

impl PluginContext {
    /// 创建新的插件上下文
    pub fn new(content: String, frontmatter: HashMap<String, NargoValue>, path: String) -> Self {
        Self { content, frontmatter, path, config: None, site_data: None }
    }

    /// 从文档内容创建插件上下文
    pub fn from_content(content: String, path: String) -> Self {
        Self { content, frontmatter: HashMap::new(), path, config: None, site_data: None }
    }

    /// 设置全局配置
    pub fn with_config(mut self, config: HashMap<String, NargoValue>) -> Self {
        self.config = Some(config);
        self
    }

    /// 设置站点数据
    pub fn with_site_data(mut self, site_data: HashMap<String, NargoValue>) -> Self {
        self.site_data = Some(site_data);
        self
    }
}

/// VitePress 插件 Trait
/// 定义插件需要实现的钩子方法
pub trait VitePressPlugin: Send + Sync {
    /// 获取插件元数据
    fn meta(&self) -> &PluginMeta;

    /// 插件初始化钩子
    /// 在编译开始前调用，用于插件的初始化
    fn setup(&mut self, config: Option<HashMap<String, NargoValue>>) {
        let _ = config;
    }

    /// 编译前钩子
    /// 在编译开始前调用，用于准备编译环境
    fn before_compile(&self) -> Result<(), crate::types::VitePressError> {
        Ok(())
    }

    /// 解析前钩子
    /// 在 Markdown 解析前调用，用于修改原始内容
    fn before_parse(&self, context: PluginContext) -> PluginContext {
        context
    }

    /// 渲染前钩子
    /// 在 Markdown 解析后、HTML 渲染前调用，用于修改文档内容
    fn before_render(&self, context: PluginContext) -> PluginContext {
        context
    }

    /// 渲染后钩子
    /// 在 HTML 渲染后调用，用于修改渲染后的内容
    fn after_render(&self, context: PluginContext) -> PluginContext {
        context
    }

    /// 编译后钩子
    /// 在编译完成后调用，用于清理资源或生成额外文件
    fn after_compile(&self) -> Result<(), crate::types::VitePressError> {
        Ok(())
    }

    /// 开发服务器启动前钩子
    fn before_dev_server(&self, config: Option<HashMap<String, NargoValue>>) -> Result<(), crate::types::VitePressError> {
        let _ = config;
        Ok(())
    }

    /// 开发服务器关闭后钩子
    fn after_dev_server(&self) -> Result<(), crate::types::VitePressError> {
        Ok(())
    }
}

/// 插件注册表
/// 用于管理和注册 VitePress 插件
pub struct PluginRegistry {
    /// 已注册的插件列表
    plugins: Vec<Box<dyn VitePressPlugin>>,
    /// 按类型分类的插件索引
    plugins_by_type: HashMap<PluginType, Vec<usize>>,
}

impl PluginRegistry {
    /// 创建新的插件注册表
    pub fn new() -> Self {
        Self { plugins: Vec::new(), plugins_by_type: HashMap::new() }
    }

    /// 注册一个插件
    pub fn register<P: VitePressPlugin + 'static>(&mut self, plugin: P) {
        let plugin_box = Box::new(plugin);
        let plugin_type = plugin_box.meta().plugin_type.clone();
        let index = self.plugins.len();

        self.plugins.push(plugin_box);
        self.plugins_by_type.entry(plugin_type).or_insert_with(Vec::new).push(index);
    }

    /// 初始化所有已注册的插件
    pub fn setup_all(&mut self, config: Option<HashMap<String, NargoValue>>) {
        for plugin in &mut self.plugins {
            plugin.setup(config.clone());
        }
    }

    /// 对所有已注册的插件调用编译前钩子
    pub fn before_compile_all(&self) -> Result<(), crate::types::VitePressError> {
        for plugin in &self.plugins {
            plugin.before_compile()?;
        }
        Ok(())
    }

    /// 对所有已注册的插件调用解析前钩子
    pub fn before_parse_all(&self, context: PluginContext) -> PluginContext {
        let mut current_context = context;
        for plugin in &self.plugins {
            current_context = plugin.before_parse(current_context);
        }
        current_context
    }

    /// 对所有已注册的插件调用渲染前钩子
    pub fn before_render_all(&self, context: PluginContext) -> PluginContext {
        let mut current_context = context;
        for plugin in &self.plugins {
            current_context = plugin.before_render(current_context);
        }
        current_context
    }

    /// 对所有已注册的插件调用渲染后钩子
    pub fn after_render_all(&self, context: PluginContext) -> PluginContext {
        let mut current_context = context;
        for plugin in &self.plugins {
            current_context = plugin.after_render(current_context);
        }
        current_context
    }

    /// 对所有已注册的插件调用编译后钩子
    pub fn after_compile_all(&self) -> Result<(), crate::types::VitePressError> {
        for plugin in &self.plugins {
            plugin.after_compile()?;
        }
        Ok(())
    }

    /// 对所有已注册的插件调用开发服务器启动前钩子
    pub fn before_dev_server_all(
        &self,
        config: Option<HashMap<String, NargoValue>>,
    ) -> Result<(), crate::types::VitePressError> {
        for plugin in &self.plugins {
            plugin.before_dev_server(config.clone())?;
        }
        Ok(())
    }

    /// 对所有已注册的插件调用开发服务器关闭后钩子
    pub fn after_dev_server_all(&self) -> Result<(), crate::types::VitePressError> {
        for plugin in &self.plugins {
            plugin.after_dev_server()?;
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

    /// 按类型获取插件
    pub fn plugins_by_type(&self, plugin_type: PluginType) -> Option<Vec<&Box<dyn VitePressPlugin>>> {
        self.plugins_by_type
            .get(&plugin_type)
            .map(|indices| indices.iter().filter_map(|&index| self.plugins.get(index)).collect())
    }

    /// 检查是否注册了指定名称的插件
    pub fn has_plugin(&self, name: &str) -> bool {
        self.plugins.iter().any(|p| p.meta().name == name)
    }

    /// 获取指定名称的插件
    pub fn get_plugin(&self, name: &str) -> Option<&Box<dyn VitePressPlugin>> {
        self.plugins.iter().find(|p| p.meta().name == name)
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}
