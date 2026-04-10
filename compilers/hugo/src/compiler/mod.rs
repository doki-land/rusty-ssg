//! 编译器模块
//! 提供 VuTeX 文档编译器的核心功能

use crate::types::{HugoConfig, Result};
use nargo_document::generator::markdown::MarkdownRenderer;
use nargo_parser::parse_document;
use std::{collections::HashMap, time::Instant};

mod renderer;

/// Hugo 模板引擎模块
pub mod hugo_template;
pub use hugo_template::{
    HugoTemplateEngine, HugoTemplateError,
    context::{HugoPage, HugoSite, HugoTemplateContext, LanguageConfig, PageParams, SiteParams},
    resolver::{TemplateResolver, TemplateResolverError},
};

/// 短代码系统模块
pub mod shortcodes;

/// 模板函数模块
pub mod template_functions;

use crate::compiler::shortcodes::{ShortcodeContext, ShortcodeParser, ShortcodeRegistry, parser::TextFragment};

/// VuTeX 文档编译器
///
/// 负责将 Markdown 文档编译为 VuTeX 文档格式
pub struct VutexCompiler {
    /// 编译器配置
    config: HugoConfig,
    /// Markdown 渲染器
    markdown_renderer: MarkdownRenderer,
    /// 编译缓存
    cache: HashMap<String, Document>,
    /// 短代码注册表
    shortcode_registry: ShortcodeRegistry,
}

impl VutexCompiler {
    /// 创建新的编译器
    pub fn new() -> Self {
        Self {
            config: HugoConfig::new(),
            markdown_renderer: MarkdownRenderer::new(),
            cache: HashMap::new(),
            shortcode_registry: ShortcodeRegistry::default(),
        }
    }

    /// 创建带配置的编译器
    ///
    /// # Arguments
    ///
    /// * `config` - 编译器配置
    pub fn with_config(config: HugoConfig) -> Self {
        Self {
            config,
            markdown_renderer: MarkdownRenderer::new(),
            cache: HashMap::new(),
            shortcode_registry: ShortcodeRegistry::default(),
        }
    }

    /// 获取短代码注册表的可变引用
    pub fn shortcode_registry_mut(&mut self) -> &mut ShortcodeRegistry {
        &mut self.shortcode_registry
    }

    /// 获取短代码注册表的不可变引用
    pub fn shortcode_registry(&self) -> &ShortcodeRegistry {
        &self.shortcode_registry
    }

    /// 获取编译器配置
    pub fn config(&self) -> &HugoConfig {
        &self.config
    }

    /// 获取可变的编译器配置
    pub fn config_mut(&mut self) -> &mut HugoConfig {
        &mut self.config
    }

    /// 编译单个文档
    ///
    /// # Arguments
    ///
    /// * `source` - 文档内容
    /// * `path` - 文档路径
    ///
    /// # Returns
    ///
    /// 编译后的文档
    pub fn compile_document(&mut self, source: &str, path: &str) -> Result<Document> {
        if let Some(cached) = self.get_cached(path) {
            return Ok(cached.clone());
        }

        let mut doc = parse_document(source, path)?;

        let mut content = doc.content.clone();

        content = self.process_shortcodes(&content, path)?;

        doc.content = content;
        let rendered_html = self
            .markdown_renderer
            .render(&doc.content)
            .map_err(|e| crate::types::VutexError::ConfigError { message: format!("Markdown render error: {:?}", e) })?;

        doc.rendered_content = Some(rendered_html);
        self.cache.insert(path.to_string(), doc.clone());

        Ok(doc)
    }

    /// 处理文档中的短代码
    ///
    /// # Arguments
    ///
    /// * `content` - 文档内容
    /// * `path` - 文档路径
    ///
    /// # Returns
    ///
    /// 处理后的内容
    fn process_shortcodes(&self, content: &str, path: &str) -> Result<String> {
        let parser = ShortcodeParser::new();
        let fragments = parser
            .parse_text(content)
            .map_err(|e| crate::types::VutexError::ConfigError { message: format!("Shortcode parse error: {:?}", e) })?;

        let context = ShortcodeContext::new(path.to_string());
        let mut result = String::new();

        for fragment in fragments {
            match fragment {
                TextFragment::Text(text) => {
                    result.push_str(&text);
                }
                TextFragment::Shortcode(shortcode) => match self.shortcode_registry.execute(&shortcode, &context) {
                    Ok(rendered) => {
                        result.push_str(&rendered);
                    }
                    Err(e) => {
                        result.push_str(&format!("[Shortcode error: {:?}]", e));
                    }
                },
            }
        }

        Ok(result)
    }

    /// 批量编译文档
    ///
    /// # Arguments
    ///
    /// * `documents` - 文档映射（路径 -> 内容）
    ///
    /// # Returns
    ///
    /// 编译结果
    pub fn compile_batch(&mut self, documents: &HashMap<String, String>) -> super::CompileResult {
        let start_time = Instant::now();
        let mut compiled_docs = HashMap::new();
        let mut errors = Vec::new();

        for (path, source) in documents {
            match self.compile_document(source, path) {
                Ok(doc) => {
                    compiled_docs.insert(path.to_string(), doc);
                }
                Err(e) => {
                    errors.push(e);
                }
            }
        }

        let compile_time_ms = start_time.elapsed().as_millis() as u64;

        if errors.is_empty() {
            super::CompileResult::success(compiled_docs, compile_time_ms)
        }
        else {
            super::CompileResult::from_errors(errors, compile_time_ms)
        }
    }

    /// 清除编译缓存
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// 从缓存中获取文档
    pub fn get_cached(&self, path: &str) -> Option<&String> {
        self.cache.get(path)
    }
}

impl Default for VutexCompiler {
    fn default() -> Self {
        Self::new()
    }
}
