//! 插件模块
//! 提供插件系统和常用插件实现

use crate::types::PluginOptions;
use std::{collections::HashMap, result::Result};

/// 插件错误类型
#[derive(Debug)]
pub enum PluginError {
    /// 插件加载错误
    LoadError(String),
    /// 插件执行错误
    ExecuteError(String),
}

/// 插件 trait
pub trait Plugin {
    /// 插件名称
    fn name(&self) -> &str;

    /// 初始化插件
    fn init(&mut self) -> Result<(), PluginError>;

    /// 执行插件
    fn execute(&self, content: &str) -> Result<String, PluginError>;
}

/// 插件管理器
pub struct PluginManager {
    /// 已注册的插件
    plugins: HashMap<String, Box<dyn Plugin>>,
    /// 启用的插件
    enabled_plugins: Vec<String>,
}

impl PluginManager {
    /// 创建新的插件管理器
    pub fn new() -> Self {
        let mut manager = Self { plugins: HashMap::new(), enabled_plugins: Vec::new() };
        manager.register_default_plugins();
        manager
    }

    /// 注册默认插件
    fn register_default_plugins(&mut self) {
        self.register_plugin(Box::new(PrismPlugin::new(PluginOptions::new()))).unwrap();
        self.register_plugin(Box::new(KatexPlugin::new(PluginOptions::new()))).unwrap();
        self.register_plugin(Box::new(MermaidPlugin::new(PluginOptions::new()))).unwrap();
        self.register_plugin(Box::new(SitemapPlugin::new(PluginOptions::new()))).unwrap();
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
        for name in &self.enabled_plugins {
            if let Some(plugin) = self.plugins.get_mut(name) {
                plugin.init()?;
            }
        }
        Ok(())
    }

    /// 执行所有启用的插件
    pub fn execute_plugins(&self, content: &str) -> Result<String, PluginError> {
        let mut result = content.to_string();
        for name in &self.enabled_plugins {
            if let Some(plugin) = self.plugins.get(name) {
                result = plugin.execute(&result)?;
            }
        }
        Ok(result)
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

    fn init(&mut self) -> Result<(), PluginError> {
        Ok(())
    }

    fn execute(&self, content: &str) -> Result<String, PluginError> {
        // 简单的代码高亮处理
        let result = content.replace("```", "<pre><code>").replace("```", "</code></pre>");
        Ok(result)
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

    fn init(&mut self) -> Result<(), PluginError> {
        Ok(())
    }

    fn execute(&self, content: &str) -> Result<String, PluginError> {
        // 简单的数学公式处理
        let result = content.replace("$$", "<span class='math'>").replace("$$", "</span>");
        Ok(result)
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

    fn init(&mut self) -> Result<(), PluginError> {
        Ok(())
    }

    fn execute(&self, content: &str) -> Result<String, PluginError> {
        // 简单的图表渲染处理
        let result = content.replace("```mermaid", "<div class='mermaid'>").replace("```", "</div>");
        Ok(result)
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

    fn init(&mut self) -> Result<(), PluginError> {
        Ok(())
    }

    fn execute(&self, content: &str) -> Result<String, PluginError> {
        // 站点地图生成逻辑
        Ok(content.to_string())
    }
}
