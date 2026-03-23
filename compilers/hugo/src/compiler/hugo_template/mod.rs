//! Hugo 模板引擎模块
//! 提供 Hugo 兼容的模板渲染功能，包括模板查找、变量系统和内置函数

use std::{collections::HashMap, error::Error, fmt, path::Path};

use nargo_template::{TemplateEngine, TemplateManager};
use serde_json::Value;

pub mod context;
pub mod resolver;

pub use context::{HugoPage, HugoSite, HugoTemplateContext, LanguageConfig, PageParams, SiteParams};
pub use resolver::{TemplateResolver, TemplateResolverError};

/// Hugo 模板引擎错误类型
#[derive(Debug)]
pub enum HugoTemplateError {
    /// 模板解析错误
    ParseError {
        /// 错误消息
        message: String,
    },

    /// 模板渲染错误
    RenderError {
        /// 错误消息
        message: String,
    },

    /// 模板查找错误
    ResolveError {
        /// 底层解析错误
        source: TemplateResolverError,
    },

    /// 文件系统错误
    IoError {
        /// 底层 IO 错误
        source: std::io::Error,
    },
}

impl HugoTemplateError {
    /// 获取错误的 i18n 键
    pub fn i18n_key(&self) -> &'static str {
        match self {
            HugoTemplateError::ParseError { .. } => "hugo.error.template.parse",
            HugoTemplateError::RenderError { .. } => "hugo.error.template.render",
            HugoTemplateError::ResolveError { .. } => "hugo.error.template.resolve",
            HugoTemplateError::IoError { .. } => "hugo.error.template.io",
        }
    }

    /// 获取错误的参数
    pub fn params(&self) -> Vec<(String, String)> {
        match self {
            HugoTemplateError::ParseError { message } => vec![("message".to_string(), message.clone())],
            HugoTemplateError::RenderError { message } => vec![("message".to_string(), message.clone())],
            HugoTemplateError::ResolveError { source } => source.params(),
            HugoTemplateError::IoError { source } => vec![("message".to_string(), source.to_string())],
        }
    }
}

impl fmt::Display for HugoTemplateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HugoTemplateError::ParseError { message } => write!(f, "Template parse error: {}", message),
            HugoTemplateError::RenderError { message } => write!(f, "Template render error: {}", message),
            HugoTemplateError::ResolveError { source } => write!(f, "Template resolve error: {}", source),
            HugoTemplateError::IoError { source } => write!(f, "File system error: {}", source),
        }
    }
}

impl Error for HugoTemplateError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            HugoTemplateError::ResolveError { source } => Some(source),
            HugoTemplateError::IoError { source } => Some(source),
            _ => None,
        }
    }
}

impl From<TemplateResolverError> for HugoTemplateError {
    fn from(source: TemplateResolverError) -> Self {
        HugoTemplateError::ResolveError { source }
    }
}

impl From<std::io::Error> for HugoTemplateError {
    fn from(source: std::io::Error) -> Self {
        HugoTemplateError::IoError { source }
    }
}

/// Hugo 模板引擎
///
/// 负责管理模板加载、解析和渲染，提供 Hugo 兼容的模板环境
pub struct HugoTemplateEngine {
    /// 模板解析器
    resolver: TemplateResolver,
    /// 模板管理器
    template_manager: TemplateManager,
    /// 站点配置
    site: HugoSite,
}

impl HugoTemplateEngine {
    /// 创建新的 Hugo 模板引擎
    ///
    /// # Arguments
    ///
    /// * `root_dir` - 项目根目录
    /// * `site` - 站点配置
    pub fn new(root_dir: impl AsRef<Path>, site: HugoSite) -> Result<Self, HugoTemplateError> {
        let resolver = TemplateResolver::new(root_dir);
        let template_manager = TemplateManager::new();

        Ok(Self { resolver, template_manager, site })
    }

    /// 设置主题
    ///
    /// # Arguments
    ///
    /// * `theme_name` - 主题名称
    pub fn with_theme(mut self, theme_name: String) -> Self {
        self.resolver.set_theme(theme_name);
        self
    }

    /// 加载并解析模板
    ///
    /// # Arguments
    ///
    /// * `template_name` - 模板名称（如 "baseof.html" 或 "partials/header.html"）
    pub fn load_template(&mut self, template_name: &str) -> Result<(), HugoTemplateError> {
        let (name, content) = self.resolver.resolve_template(template_name)?;
        self.template_manager
            .register_template(TemplateEngine::Liquid, &name, &content)
            .map_err(|e| HugoTemplateError::ParseError { message: e.to_string() })?;
        Ok(())
    }

    /// 批量加载模板
    ///
    /// # Arguments
    ///
    /// * `template_names` - 模板名称列表
    pub fn load_templates(&mut self, template_names: &[&str]) -> Result<(), HugoTemplateError> {
        for name in template_names {
            self.load_template(name)?;
        }
        Ok(())
    }

    /// 从字符串添加模板
    ///
    /// # Arguments
    ///
    /// * `name` - 模板名称
    /// * `content` - 模板内容
    pub fn add_template(&mut self, name: &str, content: &str) -> Result<(), HugoTemplateError> {
        self.template_manager
            .register_template(TemplateEngine::Liquid, name, content)
            .map_err(|e| HugoTemplateError::ParseError { message: e.to_string() })?;
        Ok(())
    }

    /// 模板渲染
    ///
    /// # Arguments
    ///
    /// * `template_name` - 要渲染的模板名称
    /// * `page` - 页面上下文
    ///
    /// # Returns
    ///
    /// 渲染后的 HTML 字符串
    pub fn render(&mut self, template_name: &str, page: HugoPage) -> Result<String, HugoTemplateError> {
        // 检查是否需要使用 baseof 模板
        let template_to_render = self.resolve_template_with_baseof(template_name)?;

        let context = HugoTemplateContext {
            site: self.site.clone(),
            page,
            env: crate::compiler::hugo_template::context::EnvironmentInfo::new(),
        };
        let json_value =
            serde_json::to_value(context).map_err(|e| HugoTemplateError::RenderError { message: e.to_string() })?;

        self.template_manager
            .render(TemplateEngine::Liquid, &template_to_render, &json_value)
            .map_err(|e| HugoTemplateError::RenderError { message: e.to_string() })
    }

    /// 解析模板，考虑 baseof 继承
    ///
    /// # Arguments
    ///
    /// * `template_name` - 模板名称
    ///
    /// # Returns
    ///
    /// 最终要渲染的模板名称
    fn resolve_template_with_baseof(&mut self, template_name: &str) -> Result<String, HugoTemplateError> {
        // 检查模板是否存在
        if self.resolver.template_exists(template_name) {
            // 尝试加载模板
            self.load_template(template_name)?;

            // 检查模板是否引用了 baseof
            // 这里简化实现，实际应该解析模板内容
            // 暂时直接返回模板名称
            Ok(template_name.to_string())
        }
        else {
            // 尝试使用 baseof 模板
            let baseof_name = "baseof.html";
            if self.resolver.template_exists(baseof_name) {
                self.load_template(baseof_name)?;
                Ok(baseof_name.to_string())
            }
            else {
                Err(HugoTemplateError::ResolveError {
                    source: TemplateResolverError::TemplateNotFound { name: template_name.to_string() },
                })
            }
        }
    }

    /// 获取站点配置的引用
    pub fn site(&self) -> &HugoSite {
        &self.site
    }

    /// 获取可变的站点配置引用
    pub fn site_mut(&mut self) -> &mut HugoSite {
        &mut self.site
    }

    /// 获取模板解析器的引用
    pub fn resolver(&self) -> &TemplateResolver {
        &self.resolver
    }

    /// 获取可变的模板解析器引用
    pub fn resolver_mut(&mut self) -> &mut TemplateResolver {
        &mut self.resolver
    }
}
