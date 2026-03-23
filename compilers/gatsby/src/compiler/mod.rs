//! 编译器模块
//! 提供 Gatsby 文档编译器的核心功能

use crate::{GatsbyConfig, types::{Result, CompileResult}};
use nargo_document::generator::markdown::MarkdownRenderer;
use nargo_parser::parse_document;
use nargo_types::Document;
use rayon::prelude::*;
use std::{collections::HashMap, sync::Arc, time::Instant};

mod renderer;
pub use renderer::{HtmlRenderer, HtmlRendererConfig};

/// Gatsby 文档编译器
///
/// 负责将 Markdown 文档编译为 Gatsby 文档格式
pub struct GatsbyCompiler {
    /// 编译器配置
    config: GatsbyConfig,
    /// Markdown 渲染器
    markdown_renderer: MarkdownRenderer,
    /// 编译缓存
    cache: HashMap<String, Document>,
}

impl GatsbyCompiler {
    /// 创建新的编译器
    pub fn new() -> Self {
        Self { config: GatsbyConfig::new(), markdown_renderer: MarkdownRenderer::new(), cache: HashMap::new() }
    }

    /// 创建带配置的编译器
    ///
    /// # Arguments
    ///
    /// * `config` - 编译器配置
    pub fn with_config(config: GatsbyConfig) -> Self {
        Self { config, markdown_renderer: MarkdownRenderer::new(), cache: HashMap::new() }
    }

    /// 获取编译器配置
    pub fn config(&self) -> &GatsbyConfig {
        &self.config
    }

    /// 获取可变的编译器配置
    pub fn config_mut(&mut self) -> &mut GatsbyConfig {
        &mut self.config
    }

    /// 将 frontmatter 转换为 HashMap<String, String>
    ///
    /// # Arguments
    ///
    /// * `doc` - 文档对象
    fn convert_frontmatter_to_map(&self, doc: &Document) -> HashMap<String, String> {
        let mut map = HashMap::new();

        if let Some(title) = &doc.frontmatter.title {
            map.insert("title".to_string(), title.clone());
        }

        if let Some(description) = &doc.frontmatter.description {
            map.insert("description".to_string(), description.clone());
        }

        if let Some(layout) = &doc.frontmatter.layout {
            map.insert("layout".to_string(), layout.clone());
        }

        for (key, value) in &doc.frontmatter.custom {
            if let nargo_types::NargoValue::String(s) = value {
                map.insert(key.clone(), s.clone());
            }
        }

        map
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
        if let Some(cached) = self.cache.get(path) {
            return Ok(cached.clone());
        }

        let mut doc = parse_document(source, path)?;

        let content = doc.content.clone();

        let rendered_html = self
            .markdown_renderer
            .render(&content)
            .map_err(|e| crate::types::GatsbyError::ConfigError { message: format!("Markdown render error: {:?}", e) })?;

        doc.rendered_content = Some(rendered_html);
        self.cache.insert(path.to_string(), doc.clone());

        Ok(doc)
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
    pub fn compile_batch(&mut self, documents: &HashMap<String, String>) -> CompileResult {
        let start_time = Instant::now();
        
        // 首先检查缓存，收集需要编译的文档
        let mut to_compile = Vec::new();
        let mut compiled_docs = HashMap::new();
        
        for (path, source) in documents {
            if let Some(cached) = self.cache.get(path) {
                compiled_docs.insert(path.to_string(), cached.clone());
            } else {
                to_compile.push((path.to_string(), source.to_string()));
            }
        }
        
        // 并行编译需要编译的文档
        let markdown_renderer = Arc::new(self.markdown_renderer.clone());
        let compile_results: Vec<_> = to_compile.par_iter().map(|(path, source)| {
            let mut doc = match parse_document(source, path) {
                Ok(doc) => doc,
                Err(e) => return Err((path.clone(), crate::types::GatsbyError::from(e))),
            };
            
            let content = doc.content.clone();
            
            let rendered_html = match markdown_renderer.render(&content) {
                Ok(html) => html,
                Err(e) => return Err((path.clone(), crate::types::GatsbyError::ConfigError { message: format!("Markdown render error: {:?}", e) })),
            };
            
            doc.rendered_content = Some(rendered_html);
            Ok((path.clone(), doc))
        }).collect();
        
        // 处理编译结果
        let mut errors = Vec::new();
        for result in compile_results {
            match result {
                Ok((path, doc)) => {
                    compiled_docs.insert(path.clone(), doc.clone());
                    self.cache.insert(path, doc);
                }
                Err((path, error)) => {
                    errors.push(error);
                }
            }
        }
        
        let compile_time_ms = start_time.elapsed().as_millis() as u64;

        if errors.is_empty() {
            CompileResult::success(compiled_docs, compile_time_ms)
        }
        else {
            CompileResult::from_errors(errors, compile_time_ms)
        }
    }

    /// 清除编译缓存
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// 从缓存中获取文档
    pub fn get_cached(&self, path: &str) -> Option<&Document> {
        self.cache.get(path)
    }
}

impl Default for GatsbyCompiler {
    fn default() -> Self {
        Self::new()
    }
}
