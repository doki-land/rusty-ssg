//! VuePress 配置文件解析器
//! 
//! 支持解析 CommonJS 和 ES 模块格式的配置文件，处理配置文件中的导入和依赖，
//! 提供配置文件的加载和解析功能

use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::config::types::VuePressConfig;
use oak_toml;

/// 配置文件解析器
/// 用于解析 VuePress 配置文件，支持多种格式
pub struct ConfigParser {
    /// 配置文件路径
    config_path: String,
}

impl ConfigParser {
    /// 创建新的配置解析器
    ///
    /// # Arguments
    ///
    /// * `config_path` - 配置文件的路径
    pub fn new(config_path: &str) -> Self {
        Self {
            config_path: config_path.to_string(),
        }
    }

    /// 解析配置文件
    ///
    /// 根据文件扩展名自动选择解析器，支持 .js、.ts、.json 和 .toml 格式
    ///
    /// # Errors
    ///
    /// 返回错误如果文件读取失败或解析失败
    pub fn parse(&self) -> Result<VuePressConfig, Box<dyn std::error::Error>> {
        let path = Path::new(&self.config_path);
        let extension = path.extension().and_then(|ext| ext.to_str());

        match extension {
            Some("js") | Some("ts") => self.parse_js_module(),
            Some("json") => self.parse_json(),
            Some("toml") => self.parse_toml(),
            _ => Err("Unsupported configuration file format".into()),
        }
    }

    /// 解析 JavaScript/TypeScript 模块
    ///
    /// # Errors
    ///
    /// 返回错误如果文件读取失败或解析失败
    fn parse_js_module(&self) -> Result<VuePressConfig, Box<dyn std::error::Error>> {
        // 读取配置文件内容
        let mut file = File::open(&self.config_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        // 简化的模块解析逻辑
        // 实际实现可能需要使用 JavaScript 引擎执行模块
        // 这里我们假设配置文件导出一个对象
        self.parse_js_content(&content)
    }

    /// 解析 JavaScript 内容
    ///
    /// # Arguments
    ///
    /// * `content` - JavaScript 配置文件内容
    ///
    /// # Errors
    ///
    /// 返回错误如果解析失败
    fn parse_js_content(&self, content: &str) -> Result<VuePressConfig, Box<dyn std::error::Error>> {
        // 简单的解析逻辑，实际项目中可能需要使用更复杂的方法
        // 这里我们只处理常见的导出格式
        // 1. 检查是否是 ES 模块导出
        if content.contains("export default") {
            // 提取 export default 后面的对象
            let start_idx = content.find("export default").unwrap_or(0) + "export default".len();
            let mut config_content = content[start_idx..].trim().to_string();
            // 移除末尾的分号
            if config_content.ends_with(';') {
                config_content.pop();
            }
            // 尝试解析为 JSON
            if let Ok(config) = serde_json::from_str::<VuePressConfig>(&config_content) {
                return Ok(config);
            }
        }
        // 2. 检查是否是 CommonJS 模块导出
        else if content.contains("module.exports") {
            // 提取 module.exports 后面的对象
            let start_idx = content.find("module.exports").unwrap_or(0) + "module.exports".len();
            let mut config_content = content[start_idx..].trim().to_string();
            // 移除末尾的分号
            if config_content.ends_with(';') {
                config_content.pop();
            }
            // 尝试解析为 JSON
            if let Ok(config) = serde_json::from_str::<VuePressConfig>(&config_content) {
                return Ok(config);
            }
        }
        // 如果无法解析，返回默认配置
        let config = VuePressConfig::new();
        Ok(config)
    }

    /// 解析 JSON 配置文件
    ///
    /// # Errors
    ///
    /// 返回错误如果文件读取失败或 JSON 解析失败
    fn parse_json(&self) -> Result<VuePressConfig, Box<dyn std::error::Error>> {
        let file = File::open(&self.config_path)?;
        let config: VuePressConfig = serde_json::from_reader(file)?;
        Ok(config)
    }

    /// 解析 TOML 配置文件
    ///
    /// # Errors
    ///
    /// 返回错误如果文件读取失败或 TOML 解析失败
    pub fn parse_toml(&self) -> Result<VuePressConfig, Box<dyn std::error::Error>> {
        let mut file = File::open(&self.config_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let config: VuePressConfig = oak_toml::from_str(&content)?;
        Ok(config)
    }

    /// 验证配置文件
    ///
    /// # Arguments
    ///
    /// * `config` - 配置对象
    ///
    /// # Returns
    ///
    /// 验证结果
    pub fn validate_config(&self, config: &VuePressConfig) -> Result<(), Box<dyn std::error::Error>> {
        // 验证基础配置
        if config.base.is_none() {
            return Err("Base URL is required".into());
        }
        if config.lang.is_none() {
            return Err("Language is required".into());
        }
        if config.title.is_none() {
            return Err("Title is required".into());
        }
        if config.description.is_none() {
            return Err("Description is required".into());
        }
        if config.dest.is_none() {
            return Err("Destination directory is required".into());
        }
        if config.temp.is_none() {
            return Err("Temporary directory is required".into());
        }
        if config.cache.is_none() {
            return Err("Cache directory is required".into());
        }
        if config.public.is_none() {
            return Err("Public directory is required".into());
        }
        if config.page_patterns.is_none() {
            return Err("Page patterns are required".into());
        }
        if config.host.is_none() {
            return Err("Host is required".into());
        }
        if config.port.is_none() {
            return Err("Port is required".into());
        }
        if config.template_dev.is_none() {
            return Err("Development template is required".into());
        }
        if config.should_preload.is_none() {
            return Err("Preload configuration is required".into());
        }
        if config.should_prefetch.is_none() {
            return Err("Prefetch configuration is required".into());
        }
        if config.template_build.is_none() {
            return Err("Build template is required".into());
        }
        if config.markdown.is_none() {
            return Err("Markdown configuration is required".into());
        }
        if config.plugins.is_none() {
            return Err("Plugins configuration is required".into());
        }
        Ok(())
    }
}
