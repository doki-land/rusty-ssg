//! 配置模块
//! 定义 Hexo 博客框架的配置结构，支持从 YAML 文件加载配置

use oak_yaml;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    error::Error,
    fmt::{self, Display, Formatter},
    path::Path,
};

/// 配置加载和验证相关的错误类型
#[derive(Debug, Clone)]
pub enum ConfigError {
    /// 文件读取错误
    FileReadError {
        /// 错误信息
        cause: String,
    },

    /// YAML 解析错误
    YamlParseError {
        /// 错误原因
        cause: String,
    },

    /// 配置验证错误
    ValidationError {
        /// 错误原因
        cause: String,
    },

    /// 不支持的配置文件格式
    UnsupportedFormat {
        /// 格式名称
        format: String,
    },
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::FileReadError { cause } => write!(f, "Failed to read config file: {}", cause),
            ConfigError::YamlParseError { cause } => write!(f, "Failed to parse YAML config: {}", cause),
            ConfigError::ValidationError { cause } => write!(f, "Config validation error: {}", cause),
            ConfigError::UnsupportedFormat { format } => write!(f, "Unsupported config file format: {}", format),
        }
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(source: std::io::Error) -> Self {
        ConfigError::FileReadError { cause: source.to_string() }
    }
}

impl ConfigError {
    /// 获取错误的 i18n key
    pub fn i18n_key(&self) -> &'static str {
        match self {
            ConfigError::FileReadError { cause: _ } => "hexo.error.config.file_read",
            ConfigError::YamlParseError { .. } => "hexo.error.config.yaml_parse",
            ConfigError::ValidationError { .. } => "hexo.error.config.validation",
            ConfigError::UnsupportedFormat { .. } => "hexo.error.config.unsupported_format",
        }
    }

    /// 获取错误参数
    pub fn params(&self) -> Vec<(&'static str, String)> {
        match self {
            ConfigError::FileReadError { cause } => {
                vec![("error", cause.clone())]
            }
            ConfigError::YamlParseError { cause } => {
                vec![("cause", cause.clone())]
            }
            ConfigError::ValidationError { cause } => {
                vec![("cause", cause.clone())]
            }
            ConfigError::UnsupportedFormat { format } => {
                vec![("format", format.clone())]
            }
        }
    }

    /// 创建 YAML 解析错误
    pub fn yaml_parse_error(cause: String) -> Self {
        ConfigError::YamlParseError { cause }
    }

    /// 创建验证错误
    pub fn validation_error(cause: String) -> Self {
        ConfigError::ValidationError { cause }
    }

    /// 创建不支持的格式错误
    pub fn unsupported_format(format: String) -> Self {
        ConfigError::UnsupportedFormat { format }
    }
}

/// 配置验证 trait
pub trait ConfigValidation {
    /// 验证配置的有效性
    ///
    /// # Errors
    ///
    /// 返回 `ConfigError::ValidationError` 如果配置无效
    fn validate(&self) -> Result<(), ConfigError>;
}

/// Hexo 配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct HexoConfig {
    /// 网站信息
    pub site: Option<SiteConfig>,
    /// 网址配置
    pub url: Option<String>,
    /// 根目录
    pub root: Option<String>,
    /// 主题配置
    pub theme: Option<String>,
    /// 发布配置
    pub deploy: Option<DeployConfig>,
    /// 写作配置
    pub writing: Option<WritingConfig>,
    /// 服务器配置
    pub server: Option<ServerConfig>,
    /// 日期格式配置
    pub date_format: Option<String>,
    /// 时间格式配置
    pub time_format: Option<String>,
    /// 分页配置
    pub pagination: Option<PaginationConfig>,
    /// 目录配置
    pub directory: Option<DirectoryConfig>,
    /// 插件配置
    pub plugins: Option<Vec<String>>,
    /// 主题配置
    pub theme_config: Option<HashMap<String, serde_json::Value>>,
}

impl HexoConfig {
    /// 从文件加载配置，根据文件扩展名自动选择解析器
    ///
    /// # Arguments
    ///
    /// * `path` - 配置文件的路径
    ///
    /// # Errors
    ///
    /// 返回 `ConfigError` 如果文件读取或解析失败
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)?;

        match path.extension().and_then(|ext| ext.to_str()) {
            Some("yml") | Some("yaml") => Self::load_from_yaml_str(&content),
            Some(ext) => Err(ConfigError::unsupported_format(ext.to_string())),
            None => Err(ConfigError::unsupported_format("no extension".to_string())),
        }
    }

    /// 从 YAML 字符串加载配置
    ///
    /// # Arguments
    ///
    /// * `yaml_str` - YAML 格式的配置字符串
    ///
    /// # Errors
    ///
    /// 返回 `ConfigError::YamlParseError` 如果 YAML 解析失败
    pub fn load_from_yaml_str(yaml_str: &str) -> Result<Self, ConfigError> {
        let config: Self = oak_yaml::language::from_str(yaml_str).map_err(|e| ConfigError::yaml_parse_error(e.to_string()))?;
        config.validate()?;
        Ok(config)
    }

    /// 从目录中查找并加载配置文件
    ///
    /// 按以下顺序查找配置文件：
    /// 1. _config.yml
    /// 2. _config.yaml
    ///
    /// # Arguments
    ///
    /// * `dir` - 要搜索的目录路径
    ///
    /// # Errors
    ///
    /// 返回 `ConfigError` 如果配置文件读取或解析失败
    pub fn load_from_dir<P: AsRef<Path>>(dir: P) -> Result<Self, ConfigError> {
        let dir = dir.as_ref();

        let yml_path = dir.join("_config.yml");
        if yml_path.exists() {
            return Self::load_from_file(yml_path);
        }

        let yaml_path = dir.join("_config.yaml");
        if yaml_path.exists() {
            return Self::load_from_file(yaml_path);
        }

        Ok(Self::default())
    }

    /// 将配置序列化为 YAML 字符串
    ///
    /// # Errors
    ///
    /// 返回 `ConfigError::YamlParseError` 如果序列化失败
    pub fn to_yaml(&self) -> Result<String, ConfigError> {
        oak_yaml::language::to_string(self).map_err(|e| ConfigError::yaml_parse_error(e.to_string()))
    }
}

impl ConfigValidation for HexoConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        // 验证网站信息
        if let Some(site) = &self.site {
            site.validate()?;
        }

        // 验证发布配置
        if let Some(deploy) = &self.deploy {
            deploy.validate()?;
        }

        // 验证写作配置
        if let Some(writing) = &self.writing {
            writing.validate()?;
        }

        // 验证服务器配置
        if let Some(server) = &self.server {
            server.validate()?;
        }

        // 验证分页配置
        if let Some(pagination) = &self.pagination {
            pagination.validate()?;
        }

        // 验证目录配置
        if let Some(directory) = &self.directory {
            directory.validate()?;
        }

        Ok(())
    }
}

/// 网站信息配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SiteConfig {
    /// 网站标题
    pub title: Option<String>,
    /// 网站副标题
    pub subtitle: Option<String>,
    /// 网站描述
    pub description: Option<String>,
    /// 网站作者
    pub author: Option<String>,
    /// 网站语言
    pub language: Option<String>,
    /// 网站时区
    pub timezone: Option<String>,
}

impl ConfigValidation for SiteConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        Ok(())
    }
}

/// 发布配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DeployConfig {
    /// 类型
    pub r#type: Option<String>,
    /// 仓库地址
    pub repo: Option<String>,
    /// 分支
    pub branch: Option<String>,
    /// 消息
    pub message: Option<String>,
}

impl ConfigValidation for DeployConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        if let Some(r#type) = &self.r#type {
            if r#type.is_empty() {
                return Err(ConfigError::validation_error("Deploy type cannot be empty".to_string()));
            }
        }
        Ok(())
    }
}

/// 写作配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct WritingConfig {
    /// 新文章默认布局
    pub new_post_name: Option<String>,
    /// 默认布局
    pub default_layout: Option<String>,
    /// 标题格式
    pub titlecase: Option<bool>,
    /// 外部链接新窗口打开
    pub external_link: Option<ExternalLinkConfig>,
    /// 文件名大小写
    pub filename_case: Option<i32>,
    /// 渲染草稿
    pub render_drafts: Option<bool>,
    /// 博客文章的 permalink 格式
    pub permalink: Option<String>,
    /// 永久链接参数
    pub permalink_defaults: Option<HashMap<String, String>>,
    /// 变量模板
    pub template_dir: Option<String>,
    /// 资产文件夹
    pub asset_folder: Option<bool>,
    /// 相对链接
    pub relative_link: Option<bool>,
    /// 代码块缩进
    pub code_dir: Option<String>,
    /// 草稿文件夹
    pub draft_dir: Option<String>,
    /// 发布文件夹
    pub post_dir: Option<String>,
    /// 分页目录
    pub page_dir: Option<String>,
    /// 存档目录
    pub archive_dir: Option<String>,
    /// 分类目录
    pub category_dir: Option<String>,
    /// 标签目录
    pub tag_dir: Option<String>,
    /// 文档目录
    pub docs_dir: Option<String>,
    /// 跳过渲染的文件
    pub skip_render: Option<Vec<String>>,
}

impl ConfigValidation for WritingConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        Ok(())
    }
}

/// 外部链接配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ExternalLinkConfig {
    /// 启用
    pub enable: Option<bool>,
    /// 排除的域名
    pub field: Option<String>,
    /// 排除的域名
    pub exclude: Option<Vec<String>>,
}

/// 服务器配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerConfig {
    /// 端口
    pub port: Option<u16>,
    /// 主机
    pub host: Option<String>,
    /// 生成静态文件
    pub static_: Option<bool>,
    /// 压缩
    pub compress: Option<bool>,
    /// 缓存
    pub cache: Option<bool>,
    /// 延迟加载
    pub defer: Option<bool>,
    /// 日志格式
    pub log: Option<String>,
    /// 进程名称
    pub process: Option<String>,
    /// 标题
    pub title: Option<bool>,
}

impl ConfigValidation for ServerConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        Ok(())
    }
}

/// 分页配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PaginationConfig {
    /// 每页文章数
    pub per_page: Option<u32>,
    /// 分页目录
    pub dir: Option<String>,
    /// 布局
    pub layout: Option<Vec<String>>,
    /// 格式
    pub format: Option<String>,
    /// 别名
    pub alias: Option<String>,
}

impl ConfigValidation for PaginationConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        Ok(())
    }
}

/// 目录配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DirectoryConfig {
    /// 源文件目录
    pub source_dir: Option<String>,
    /// 公共文件目录
    pub public_dir: Option<String>,
    /// 标签目录
    pub tag_dir: Option<String>,
    /// 分类目录
    pub category_dir: Option<String>,
    /// 代码目录
    pub code_dir: Option<String>,
    /// 国际化目录
    pub i18n_dir: Option<String>,
    /// 插件目录
    pub plugin_dir: Option<String>,
    /// 脚本目录
    pub scripts_dir: Option<String>,
    /// 主题目录
    pub themes_dir: Option<String>,
    /// 新文章模板
    pub new_post_name: Option<String>,
    /// 草稿目录
    pub draft_dir: Option<String>,
    /// 发布目录
    pub post_dir: Option<String>,
    /// 页面目录
    pub page_dir: Option<String>,
    /// 存档目录
    pub archive_dir: Option<String>,
    /// 资源目录
    pub asset_dir: Option<String>,
    /// 数据目录
    pub data_dir: Option<String>,
    /// 布局目录
    pub layout_dir: Option<String>,
    /// 语言目录
    pub languages_dir: Option<String>,
    /// 配置目录
    pub config_dir: Option<String>,
}

impl ConfigValidation for DirectoryConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        Ok(())
    }
}
