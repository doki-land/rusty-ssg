//! Rusty Hexo 类型定义模块

use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    path::PathBuf,
};

/// 配置模块
pub mod config;

/// 通用结果类型
pub type Result<T> = std::result::Result<T, HexoError>;

/// Hexo 错误类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HexoError {
    /// IO 错误
    IoError {
        /// 错误参数
        path: Option<String>,
        /// 错误原因
        cause: String,
    },
    /// 序列化/反序列化错误
    SerdeError {
        /// 错误参数
        source: String,
        /// 错误原因
        cause: String,
    },
    /// TOML 解析错误
    TomlError {
        /// 错误参数
        path: Option<String>,
        /// 错误原因
        cause: String,
    },
    /// YAML 解析错误
    YamlError {
        /// 错误参数
        path: Option<String>,
        /// 错误原因
        cause: String,
    },
    /// 配置错误
    ConfigError {
        /// 错误参数
        key: Option<String>,
        /// 错误原因
        cause: String,
    },
    /// 文档未找到错误
    DocumentNotFound {
        /// 错误参数
        path: String,
    },
    /// 插件错误
    PluginError {
        /// 错误参数
        name: String,
        /// 错误原因
        cause: String,
    },
    /// 主题错误
    ThemeError {
        /// 错误参数
        name: String,
        /// 错误原因
        cause: String,
    },
    /// 部署错误
    DeployError {
        /// 错误参数
        strategy: String,
        /// 错误原因
        cause: String,
    },
    /// 自定义错误
    CustomError {
        /// 错误参数
        message: String,
    },
}

impl HexoError {
    /// 获取错误的 i18n key
    pub fn i18n_key(&self) -> &'static str {
        match self {
            HexoError::IoError { .. } => "hexo.error.io",
            HexoError::SerdeError { .. } => "hexo.error.serde",
            HexoError::TomlError { .. } => "hexo.error.toml",
            HexoError::YamlError { .. } => "hexo.error.yaml",
            HexoError::ConfigError { .. } => "hexo.error.config",
            HexoError::DocumentNotFound { .. } => "hexo.error.document_not_found",
            HexoError::PluginError { .. } => "hexo.error.plugin",
            HexoError::ThemeError { .. } => "hexo.error.theme",
            HexoError::DeployError { .. } => "hexo.error.deploy",
            HexoError::CustomError { .. } => "hexo.error.custom",
        }
    }

    /// 创建 IO 错误
    pub fn io_error(path: Option<String>, cause: String) -> Self {
        HexoError::IoError { path, cause }
    }

    /// 创建 Serde 错误
    pub fn serde_error(source: String, cause: String) -> Self {
        HexoError::SerdeError { source, cause }
    }

    /// 创建 TOML 错误
    pub fn toml_error(path: Option<String>, cause: String) -> Self {
        HexoError::TomlError { path, cause }
    }

    /// 创建 YAML 错误
    pub fn yaml_error(path: Option<String>, cause: String) -> Self {
        HexoError::YamlError { path, cause }
    }

    /// 创建配置错误
    pub fn config_error(key: Option<String>, cause: String) -> Self {
        HexoError::ConfigError { key, cause }
    }

    /// 创建文档未找到错误
    pub fn document_not_found(path: String) -> Self {
        HexoError::DocumentNotFound { path }
    }

    /// 创建插件错误
    pub fn plugin_error(name: String, cause: String) -> Self {
        HexoError::PluginError { name, cause }
    }

    /// 创建主题错误
    pub fn theme_error(name: String, cause: String) -> Self {
        HexoError::ThemeError { name, cause }
    }

    /// 创建部署错误
    pub fn deploy_error(strategy: String, cause: String) -> Self {
        HexoError::DeployError { strategy, cause }
    }

    /// 创建自定义错误
    pub fn custom_error(message: String) -> Self {
        HexoError::CustomError { message }
    }
}

impl Display for HexoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            HexoError::IoError { path, cause } => {
                if let Some(path) = path {
                    write!(f, "IO error at {}: {}", path, cause)
                }
                else {
                    write!(f, "IO error: {}", cause)
                }
            }
            HexoError::SerdeError { source, cause } => {
                write!(f, "Serde error in {}: {}", source, cause)
            }
            HexoError::TomlError { path, cause } => {
                if let Some(path) = path {
                    write!(f, "TOML error in {}: {}", path, cause)
                }
                else {
                    write!(f, "TOML error: {}", cause)
                }
            }
            HexoError::YamlError { path, cause } => {
                if let Some(path) = path {
                    write!(f, "YAML error in {}: {}", path, cause)
                }
                else {
                    write!(f, "YAML error: {}", cause)
                }
            }
            HexoError::ConfigError { key, cause } => {
                if let Some(key) = key {
                    write!(f, "Config error for {}: {}", key, cause)
                }
                else {
                    write!(f, "Config error: {}", cause)
                }
            }
            HexoError::DocumentNotFound { path } => {
                write!(f, "Document not found: {}", path)
            }
            HexoError::PluginError { name, cause } => {
                write!(f, "Plugin error in {}: {}", name, cause)
            }
            HexoError::ThemeError { name, cause } => {
                write!(f, "Theme error in {}: {}", name, cause)
            }
            HexoError::DeployError { strategy, cause } => {
                write!(f, "Deploy error with {}: {}", strategy, cause)
            }
            HexoError::CustomError { message } => {
                write!(f, "{}", message)
            }
        }
    }
}

impl Error for HexoError {}

impl From<std::io::Error> for HexoError {
    fn from(e: std::io::Error) -> Self {
        HexoError::io_error(None, e.to_string())
    }
}

impl From<serde_json::Error> for HexoError {
    fn from(e: serde_json::Error) -> Self {
        HexoError::serde_error("JSON".to_string(), e.to_string())
    }
}

impl From<walkdir::Error> for HexoError {
    fn from(e: walkdir::Error) -> Self {
        HexoError::io_error(None, e.to_string())
    }
}

/// 从 config 模块重新导出配置类型
pub use config::{
    ConfigError, ConfigValidation, DeployConfig, DirectoryConfig, ExternalLinkConfig, HexoConfig, PaginationConfig,
    ServerConfig, SiteConfig, WritingConfig,
};

/// 文章前置元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontMatter {
    /// 文章标题
    pub title: String,
    /// 文章日期
    pub date: Option<String>,
    /// 文章更新日期
    pub updated: Option<String>,
    /// 文章作者
    pub author: Option<String>,
    /// 文章分类
    pub categories: Option<Vec<String>>,
    /// 文章标签
    pub tags: Option<Vec<String>>,
    /// 文章 permalink
    pub permalink: Option<String>,
    /// 文章是否发布
    pub published: Option<bool>,
}

/// 编译选项
#[derive(Debug, Clone)]
pub struct CompileOptions {
    /// 源目录
    pub source_dir: PathBuf,
    /// 输出目录
    pub output_dir: PathBuf,
    /// 是否清理输出目录
    pub clean: bool,
}

/// 编译结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileResult {
    /// 编译的文件数
    pub files: usize,
    /// 编译时间（毫秒）
    pub compile_time_ms: u64,
    /// 是否成功
    pub success: bool,
    /// 错误信息
    pub errors: Vec<String>,
}

impl CompileResult {
    /// 创建成功的编译结果
    pub fn success(files: usize, compile_time_ms: u64) -> Self {
        Self { files, compile_time_ms, success: true, errors: Vec::new() }
    }

    /// 创建失败的编译结果
    pub fn failure(errors: Vec<String>, compile_time_ms: u64) -> Self {
        Self { files: 0, compile_time_ms, success: false, errors }
    }
}
