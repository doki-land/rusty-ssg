//! VuePress 配置文件解析器
//!
//! 支持解析 CommonJS 和 ES 模块格式的配置文件，处理配置文件中的导入和依赖

use std::{fs::File, io::Read, path::Path};

use crate::config::types::VuePressConfig;
use lazy_static::lazy_static;
use oak_toml;
use regex::Regex;

/// 配置文件解析器
pub struct ConfigParser {
    /// 配置文件路径
    config_path: String,
}

impl ConfigParser {
    /// 创建新的配置解析器
    pub fn new(config_path: &str) -> Self {
        Self { config_path: config_path.to_string() }
    }

    /// 解析配置文件
    pub fn parse(&self) -> Result<VuePressConfig, Box<dyn std::error::Error>> {
        let path = Path::new(&self.config_path);
        if !path.exists() {
            return Err(format!("Configuration file not found: {}", self.config_path).into());
        }

        let extension = path.extension().and_then(|ext| ext.to_str());

        let config = match extension {
            Some("js") | Some("ts") => self.parse_js_module()?,
            Some("json") => self.parse_json()?,
            Some("toml") => self.parse_toml()?,
            _ => return Err("Unsupported configuration file format".into()),
        };

        // 验证配置
        self.validate(&config)?;

        Ok(config)
    }

    /// 解析 JavaScript/TypeScript 模块
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
    fn parse_js_content(&self, content: &str) -> Result<VuePressConfig, Box<dyn std::error::Error>> {
        // 尝试提取配置对象
        // 处理 CommonJS 导出: module.exports = { ... }
        // 处理 ES 模块导出: export default { ... }
        lazy_static! {
            static ref CJS_EXPORT: Regex = Regex::new(r#"module\.exports\s*=\s*({[\s\S]*?})\s*;?\s*$"#).unwrap();
            static ref ES_EXPORT: Regex = Regex::new(r#"export\s+default\s*({[\s\S]*?})\s*;?\s*$"#).unwrap();
        }

        // 尝试匹配 CommonJS 导出
        if let Some(captures) = CJS_EXPORT.captures(content) {
            if let Some(config_str) = captures.get(1) {
                // 尝试将提取的配置字符串转换为 JSON
                // 注意：这只能处理简单的配置对象，不支持复杂的 JavaScript 表达式
                let config_str = config_str.as_str();
                let config: VuePressConfig = self.parse_config_string(config_str)?;
                return Ok(config);
            }
        }

        // 尝试匹配 ES 模块导出
        if let Some(captures) = ES_EXPORT.captures(content) {
            if let Some(config_str) = captures.get(1) {
                // 尝试将提取的配置字符串转换为 JSON
                let config_str = config_str.as_str();
                let config: VuePressConfig = self.parse_config_string(config_str)?;
                return Ok(config);
            }
        }

        // 如果没有找到导出，返回默认配置
        let config = VuePressConfig::new();
        Ok(config)
    }

    /// 解析配置字符串
    fn parse_config_string(&self, config_str: &str) -> Result<VuePressConfig, Box<dyn std::error::Error>> {
        // 尝试直接解析为 JSON
        match serde_json::from_str(config_str) {
            Ok(config) => Ok(config),
            Err(e) => {
                // 如果直接解析失败，尝试处理 JavaScript 对象语法
                let json_str = self.convert_js_to_json(config_str)?;
                let config: VuePressConfig = serde_json::from_str(&json_str)?;
                Ok(config)
            }
        }
    }

    /// 将 JavaScript 对象语法转换为 JSON
    fn convert_js_to_json(&self, js_str: &str) -> Result<String, Box<dyn std::error::Error>> {
        // 简单的转换逻辑，处理一些常见的 JavaScript 语法
        let mut json_str = js_str.to_string();
        
        // 移除注释
        json_str = Regex::new(r#"//.*$"#).unwrap().replace_all(&json_str, "").to_string();
        json_str = Regex::new(r#"/\*[\s\S]*?\*/"#).unwrap().replace_all(&json_str, "").to_string();
        
        // 处理单引号
        json_str = Regex::new(r#"'([^']*)'"#).unwrap().replace_all(&json_str, r#""$1""#).to_string();
        
        // 处理尾随逗号
        json_str = Regex::new(r#,\s*([\]}])"#).unwrap().replace_all(&json_str, r#" $1"#).to_string();
        
        // 处理属性名没有引号的情况
        json_str = Regex::new(r#"([{,])\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*:"#).unwrap().replace_all(&json_str, r#"$1 "$2":"#).to_string();
        
        Ok(json_str)
    }

    /// 解析 JSON 配置文件
    fn parse_json(&self) -> Result<VuePressConfig, Box<dyn std::error::Error>> {
        let file = File::open(&self.config_path)?;
        let config: VuePressConfig = serde_json::from_reader(file)?;
        Ok(config)
    }

    /// 解析 TOML 配置文件
    pub fn parse_toml(&self) -> Result<VuePressConfig, Box<dyn std::error::Error>> {
        let mut file = File::open(&self.config_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let config: VuePressConfig = oak_toml::from_str(&content)?;
        Ok(config)
    }

    /// 验证配置
    pub fn validate(&self, config: &VuePressConfig) -> Result<(), Box<dyn std::error::Error>> {
        // 验证基础配置
        if let Some(base) = &config.base {
            if !base.starts_with('/') {
                return Err("Base URL must start with '/'".into());
            }
        }

        // 验证端口配置
        if let Some(port) = config.port {
            if port < 1 || port > 65535 {
                return Err("Port must be between 1 and 65535".into());
            }
        }

        // 验证输出目录
        if let Some(dest) = &config.dest {
            let dest_path = Path::new(dest);
            if !dest_path.is_absolute() {
                // 相对路径是允许的
            }
        }

        Ok(())
    }
}
