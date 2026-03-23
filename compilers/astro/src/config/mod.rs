//! 配置模块

use serde::Deserialize;
use std::{fs, path::Path};

/// Astro 配置结构
#[derive(Debug, Deserialize, Default, Clone)]
pub struct AstroConfig {
    /// 站点基本 URL
    #[serde(default = "default_base_url")]
    pub base_url: String,
    
    /// 输出目录
    #[serde(default = "default_out_dir")]
    pub out_dir: String,
    
    /// 构建配置
    #[serde(default)]
    pub build: BuildConfig,
    
    /// 开发服务器配置
    #[serde(default)]
    pub dev: DevConfig,
    
    /// 集成配置
    #[serde(default)]
    pub integrations: Vec<String>,
    
    /// 页面配置
    #[serde(default)]
    pub pages: PagesConfig,
}

/// 构建配置
#[derive(Debug, Deserialize, Default, Clone)]
pub struct BuildConfig {
    /// 构建格式
    #[serde(default = "default_build_format")]
    pub format: String,
    
    /// 是否启用源映射
    #[serde(default)]
    pub sourcemap: bool,
    
    /// 是否压缩输出
    #[serde(default = "default_compress")]
    pub compress: bool,
}

/// 开发服务器配置
#[derive(Debug, Deserialize, Default, Clone)]
pub struct DevConfig {
    /// 开发服务器端口
    #[serde(default = "default_dev_port")]
    pub port: u16,
    
    /// 开发服务器主机
    #[serde(default = "default_dev_host")]
    pub host: String,
}

/// 页面配置
#[derive(Debug, Deserialize, Default, Clone)]
pub struct PagesConfig {
    /// 页面目录
    #[serde(default = "default_pages_dir")]
    pub dir: String,
}

/// 默认基本 URL
fn default_base_url() -> String {
    "/".to_string()
}

/// 默认输出目录
fn default_out_dir() -> String {
    "dist".to_string()
}

/// 默认构建格式
fn default_build_format() -> String {
    "static".to_string()
}

/// 默认压缩设置
fn default_compress() -> bool {
    true
}

/// 默认开发服务器端口
fn default_dev_port() -> u16 {
    3000
}

/// 默认开发服务器主机
fn default_dev_host() -> String {
    "localhost".to_string()
}

/// 默认页面目录
fn default_pages_dir() -> String {
    "src/pages".to_string()
}

/// 配置管理器
pub struct ConfigManager {
    config: AstroConfig,
}

impl ConfigManager {
    /// 创建新的配置管理器
    pub fn new() -> Self {
        Self {
            config: AstroConfig::default(),
        }
    }
    
    /// 从项目目录加载配置
    pub fn load_from_project(&mut self, project_path: &Path) -> Result<AstroConfig, String> {
        // 尝试加载不同格式的配置文件
        let config_files = [
            "astro.config.mjs",
            "astro.config.js",
            "astro.config.ts",
            "astro.config.json",
        ];
        
        for config_file in &config_files {
            let config_path = project_path.join(config_file);
            if config_path.exists() {
                return self.load_config(&config_path);
            }
        }
        
        // 如果没有找到配置文件，返回默认配置
        Ok(self.config.clone())
    }
    
    /// 加载配置文件
    fn load_config(&mut self, config_path: &Path) -> Result<AstroConfig, String> {
        // 读取配置文件内容
        let content = fs::read_to_string(config_path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;
        
        // 这里简化处理，实际应该根据文件类型使用不同的解析方法
        // 对于 JSON 文件，直接使用 serde_json 解析
        if config_path.extension().unwrap_or_default() == "json" {
            self.config = serde_json::from_str(&content)
                .map_err(|e| format!("Failed to parse JSON config: {}", e))?;
        } else {
            // 对于 JavaScript/TypeScript 文件，这里使用简化处理
            // 实际应该使用 Node.js 来执行配置文件
            // 这里暂时返回默认配置
            println!("Warning: Using default config for non-JSON config file");
        }
        
        Ok(self.config.clone())
    }
    
    /// 获取配置
    pub fn config(&self) -> &AstroConfig {
        &self.config
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new()
    }
}
