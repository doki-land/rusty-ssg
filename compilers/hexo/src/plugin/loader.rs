//! 插件加载器

use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use crate::types::{HexoError, Result};

/// 插件结构
#[derive(Debug)]
pub struct Plugin {
    /// 插件名称
    pub name: String,
    /// 插件路径
    pub path: PathBuf,
    /// 插件配置
    pub config: serde_json::Value,
    /// 插件类型
    pub plugin_type: PluginType,
}

/// 插件类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PluginType {
    /// JavaScript 插件
    JavaScript,
    /// Rust 插件
    Rust,
    /// 脚本插件
    Script,
}

impl Plugin {
    /// 从路径加载插件
    pub fn load(path: &Path) -> Result<Self> {
        let name = path
            .file_name()
            .ok_or_else(|| HexoError::custom_error("Plugin path must have a file name".to_string()))?
            .to_string_lossy()
            .to_string();

        // 检测插件类型
        let plugin_type = if path.is_file() && path.extension().map_or(false, |ext| ext == "js") {
            PluginType::Script
        }
        else if path.join("package.json").exists() {
            PluginType::JavaScript
        }
        else {
            PluginType::Rust
        };

        // 加载插件配置
        let config = if plugin_type == PluginType::JavaScript {
            let package_path = path.join("package.json");
            let mut file = File::open(&package_path)
                .map_err(|e| HexoError::io_error(Some(package_path.to_string_lossy().to_string()), e.to_string()))?;
            let mut content = String::new();
            file.read_to_string(&mut content)
                .map_err(|e| HexoError::io_error(Some(package_path.to_string_lossy().to_string()), e.to_string()))?;
            serde_json::from_str(&content).map_err(|e| HexoError::serde_error("package.json".to_string(), e.to_string()))?
        }
        else {
            serde_json::Value::Object(serde_json::Map::new())
        };

        Ok(Self { name, path: path.to_path_buf(), config, plugin_type })
    }

    /// 初始化插件
    pub fn init(&self) -> Result<()> {
        // 根据插件类型初始化
        match self.plugin_type {
            PluginType::JavaScript => self.init_javascript(),
            PluginType::Rust => self.init_rust(),
            PluginType::Script => self.init_script(),
        }
    }

    /// 初始化 JavaScript 插件
    fn init_javascript(&self) -> Result<()> {
        // 这里需要实现 JavaScript 插件的初始化
        // 可以使用 Node.js 嵌入或 WASM
        Ok(())
    }

    /// 初始化 Rust 插件
    fn init_rust(&self) -> Result<()> {
        // 这里需要实现 Rust 插件的初始化
        Ok(())
    }

    /// 初始化脚本插件
    fn init_script(&self) -> Result<()> {
        // 这里需要实现脚本插件的初始化
        Ok(())
    }
}

/// 插件管理器
#[derive(Debug)]
pub struct PluginManager {
    /// 已加载的插件
    pub plugins: Vec<Plugin>,
    /// 插件搜索路径
    pub search_paths: Vec<PathBuf>,
}

impl PluginManager {
    /// 创建插件管理器
    pub fn new() -> Self {
        Self { plugins: vec![], search_paths: vec![] }
    }

    /// 添加插件搜索路径
    pub fn add_search_path(&mut self, path: &Path) {
        self.search_paths.push(path.to_path_buf());
    }

    /// 加载插件
    pub fn load_plugins(&mut self) -> Result<()> {
        // 搜索并加载插件
        for search_path in &self.search_paths {
            if !search_path.exists() || !search_path.is_dir() {
                continue;
            }

            // 遍历目录
            for entry in std::fs::read_dir(search_path)
                .map_err(|e| HexoError::io_error(Some(search_path.to_string_lossy().to_string()), e.to_string()))?
            {
                let entry =
                    entry.map_err(|e| HexoError::io_error(Some(search_path.to_string_lossy().to_string()), e.to_string()))?;
                let path = entry.path();

                if path.is_dir() || (path.is_file() && path.extension().map_or(false, |ext| ext == "js")) {
                    let plugin = Plugin::load(&path)?;
                    plugin.init()?;
                    self.plugins.push(plugin);
                }
            }
        }

        Ok(())
    }

    /// 获取已加载的插件
    pub fn get_plugins(&self) -> &Vec<Plugin> {
        &self.plugins
    }
}
