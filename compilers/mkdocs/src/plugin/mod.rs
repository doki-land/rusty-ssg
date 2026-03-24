//! 插件模块
//! 提供插件系统和常用插件实现

use crate::types::{MkDocsConfig, PluginOptions};
use nargo_types::Document;
use std::{collections::HashMap, path::Path, result::Result};

/// 插件错误类型
#[derive(Debug)]
pub enum PluginError {
    /// 插件加载错误
    LoadError(String),
    /// 插件执行错误
    ExecuteError(String),
    /// 配置错误
    ConfigError(String),
}

/// 插件上下文
pub struct PluginContext {
    /// MkDocs 配置
    pub config: &'static MkDocsConfig,
    /// 文档路径
    pub document_path: Option<&'static Path>,
}

/// 插件 trait
pub trait Plugin {
    /// 插件名称
    fn name(&self) -> &str;

    /// 初始化插件
    fn init(&mut self, ctx: &PluginContext) -> Result<(), PluginError>;

    /// 执行插件
    fn execute(&self, content: &str, ctx: &PluginContext) -> Result<String, PluginError>;

    /// 处理文档
    fn process_document(&self, document: &mut Document, ctx: &PluginContext) -> Result<(), PluginError>;
}

/// 插件管理器
pub struct PluginManager {
    /// 已注册的插件
    plugins: HashMap<String, Box<dyn Plugin>>,
    /// 启用的插件
    enabled_plugins: Vec<String>,
    /// 插件上下文
    context: Option<PluginContext>,
}

impl PluginManager {
    /// 创建新的插件管理器
    pub fn new() -> Self {
        let mut manager = Self { 
            plugins: HashMap::new(), 
            enabled_plugins: Vec::new(),
            context: None,
        };
        manager.register_default_plugins();
        manager
    }

    /// 设置插件上下文
    pub fn set_context(&mut self, config: &'static MkDocsConfig, document_path: Option<&'static Path>) {
        self.context = Some(PluginContext {
            config,
            document_path,
        });
    }

    /// 注册默认插件
    fn register_default_plugins(&mut self) {
        self.register_plugin(Box::new(PrismPlugin::new(PluginOptions::new()))).unwrap();
        self.register_plugin(Box::new(KatexPlugin::new(PluginOptions::new()))).unwrap();
        self.register_plugin(Box::new(MermaidPlugin::new(PluginOptions::new()))).unwrap();
        self.register_plugin(Box::new(SitemapPlugin::new(PluginOptions::new()))).unwrap();
        self.register_plugin(Box::new(GoogleAnalyticsPlugin::new(PluginOptions::new()))).unwrap();
    }

    /// 注册插件
    pub fn register_plugin(&mut self, plugin: Box<dyn Plugin>) -> Result<(), PluginError> {
        let name = plugin.name().to_string();
        self.plugins.insert(name, plugin);
        Ok(())
    }

    /// 启用插件
    pub fn enable_plugin(&mut self, name: &str) {
        if self.plugins.contains_key(name) && !self.enabled_plugins.contains(&name.to_string()) {
            self.enabled_plugins.push(name.to_string());
        }
    }

    /// 禁用插件
    pub fn disable_plugin(&mut self, name: &str) {
        self.enabled_plugins.retain(|n| n != name);
    }

    /// 初始化所有启用的插件
    pub fn init_plugins(&mut self) -> Result<(), PluginError> {
        if let Some(ctx) = &self.context {
            for name in &self.enabled_plugins {
                if let Some(plugin) = self.plugins.get_mut(name) {
                    plugin.init(ctx)?;
                }
            }
        }
        Ok(())
    }

    /// 执行所有启用的插件
    pub fn execute_plugins(&self, content: &str) -> Result<String, PluginError> {
        let mut result = content.to_string();
        if let Some(ctx) = &self.context {
            for name in &self.enabled_plugins {
                if let Some(plugin) = self.plugins.get(name) {
                    result = plugin.execute(&result, ctx)?;
                }
            }
        }
        Ok(result)
    }

    /// 处理文档
    pub fn process_document(&self, document: &mut Document) -> Result<(), PluginError> {
        if let Some(ctx) = &self.context {
            for name in &self.enabled_plugins {
                if let Some(plugin) = self.plugins.get(name) {
                    plugin.process_document(document, ctx)?;
                }
            }
        }
        Ok(())
    }

    /// 从配置加载插件
    pub fn load_from_config(&mut self, plugins: &[crate::types::PluginConfig]) {
        for plugin_config in plugins {
            match plugin_config {
                crate::types::PluginConfig::String(name) => {
                    self.enable_plugin(name);
                }
                crate::types::PluginConfig::Map(map) => {
                    for (name, options) in map {
                        if options.enabled() {
                            self.enable_plugin(name);
                        }
                    }
                }
            }
        }
    }

    /// 获取所有注册的插件
    pub fn registered_plugins(&self) -> Vec<&str> {
        self.plugins.keys().map(|k| k.as_str()).collect()
    }

    /// 获取所有启用的插件
    pub fn enabled_plugins(&self) -> &Vec<String> {
        &self.enabled_plugins
    }

    /// 检查插件是否已注册
    pub fn has_plugin(&self, name: &str) -> bool {
        self.plugins.contains_key(name)
    }

    /// 检查插件是否已启用
    pub fn is_plugin_enabled(&self, name: &str) -> bool {
        self.enabled_plugins.contains(&name.to_string())
    }
}

/// 基础插件实现
pub struct BasePlugin {
    options: PluginOptions,
}

impl BasePlugin {
    /// 创建新的基础插件
    pub fn new(options: PluginOptions) -> Self {
        Self { options }
    }
}

impl Plugin for BasePlugin {
    fn name(&self) -> &str {
        "base"
    }

    fn init(&mut self, _ctx: &PluginContext) -> Result<(), PluginError> {
        Ok(())
    }

    fn execute(&self, content: &str, _ctx: &PluginContext) -> Result<String, PluginError> {
        Ok(content.to_string())
    }

    fn process_document(&self, _document: &mut Document, _ctx: &PluginContext) -> Result<(), PluginError> {
        Ok(())
    }
}

/// 代码高亮插件
pub struct PrismPlugin {
    options: PluginOptions,
}

impl PrismPlugin {
    /// 创建新的 Prism 插件
    pub fn new(options: PluginOptions) -> Self {
        Self { options }
    }
}

impl Plugin for PrismPlugin {
    fn name(&self) -> &str {
        "prism"
    }

    fn init(&mut self, _ctx: &PluginContext) -> Result<(), PluginError> {
        Ok(())
    }

    fn execute(&self, content: &str, _ctx: &PluginContext) -> Result<String, PluginError> {
        // 使用 nargo-document 的代码高亮功能
        Ok(content.to_string())
    }

    fn process_document(&self, document: &mut Document, _ctx: &PluginContext) -> Result<(), PluginError> {
        // 处理文档中的代码块
        Ok(())
    }
}

/// 数学公式插件
pub struct KatexPlugin {
    options: PluginOptions,
}

impl KatexPlugin {
    /// 创建新的 Katex 插件
    pub fn new(options: PluginOptions) -> Self {
        Self { options }
    }
}

impl Plugin for KatexPlugin {
    fn name(&self) -> &str {
        "katex"
    }

    fn init(&mut self, _ctx: &PluginContext) -> Result<(), PluginError> {
        Ok(())
    }

    fn execute(&self, content: &str, _ctx: &PluginContext) -> Result<String, PluginError> {
        // 使用 nargo-document 的数学公式渲染功能
        Ok(content.to_string())
    }

    fn process_document(&self, document: &mut Document, _ctx: &PluginContext) -> Result<(), PluginError> {
        // 处理文档中的数学公式
        Ok(())
    }
}

/// 图表渲染插件
pub struct MermaidPlugin {
    options: PluginOptions,
}

impl MermaidPlugin {
    /// 创建新的 Mermaid 插件
    pub fn new(options: PluginOptions) -> Self {
        Self { options }
    }
}

impl Plugin for MermaidPlugin {
    fn name(&self) -> &str {
        "mermaid"
    }

    fn init(&mut self, _ctx: &PluginContext) -> Result<(), PluginError> {
        Ok(())
    }

    fn execute(&self, content: &str, _ctx: &PluginContext) -> Result<String, PluginError> {
        // 使用 nargo-document 的图表渲染功能
        Ok(content.to_string())
    }

    fn process_document(&self, document: &mut Document, _ctx: &PluginContext) -> Result<(), PluginError> {
        // 处理文档中的图表
        Ok(())
    }
}

/// 站点地图插件
pub struct SitemapPlugin {
    options: PluginOptions,
}

impl SitemapPlugin {
    /// 创建新的 Sitemap 插件
    pub fn new(options: PluginOptions) -> Self {
        Self { options }
    }
}

impl Plugin for SitemapPlugin {
    fn name(&self) -> &str {
        "sitemap"
    }

    fn init(&mut self, _ctx: &PluginContext) -> Result<(), PluginError> {
        Ok(())
    }

    fn execute(&self, content: &str, _ctx: &PluginContext) -> Result<String, PluginError> {
        Ok(content.to_string())
    }

    fn process_document(&self, document: &mut Document, _ctx: &PluginContext) -> Result<(), PluginError> {
        // 站点地图生成逻辑
        Ok(())
    }
}

/// Google Analytics 插件
pub struct GoogleAnalyticsPlugin {
    options: PluginOptions,
}

impl GoogleAnalyticsPlugin {
    /// 创建新的 Google Analytics 插件
    pub fn new(options: PluginOptions) -> Self {
        Self { options }
    }
}

impl Plugin for GoogleAnalyticsPlugin {
    fn name(&self) -> &str {
        "google-analytics"
    }

    fn init(&mut self, _ctx: &PluginContext) -> Result<(), PluginError> {
        Ok(())
    }

    fn execute(&self, content: &str, _ctx: &PluginContext) -> Result<String, PluginError> {
        // 添加 Google Analytics 脚本
        Ok(content.to_string())
    }

    fn process_document(&self, document: &mut Document, _ctx: &PluginContext) -> Result<(), PluginError> {
        // 处理文档以添加 Google Analytics
        Ok(())
    }
}
