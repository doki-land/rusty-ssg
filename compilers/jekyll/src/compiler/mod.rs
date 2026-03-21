//! 编译器模块
//! 提供 VuTeX 文档编译器的核心功能

use crate::types::{
    Result, VutexConfig,
};
use nargo_parser::parse_document;
use nargo_types::Document;
use std::{
    collections::HashMap,
    time::Instant,
};

mod renderer {
    pub mod html_renderer;
    pub use html_renderer::*;
}
pub use renderer::{HtmlRenderer, HtmlRendererConfig};

/// VuTeX 文档编译器
///
/// 负责将 Markdown 文档编译为 VuTeX 文档格式
pub struct VutexCompiler {
    /// 编译器配置
    config: VutexConfig,
    /// HTML 渲染器
    html_renderer: HtmlRenderer,
    /// 编译缓存
    cache: HashMap<String, Document>,
}

impl VutexCompiler {
    /// 创建新的编译器
    pub fn new() -> Self {
        Self { config: VutexConfig::new(), html_renderer: HtmlRenderer::new(), cache: HashMap::new() }
    }

    /// 创建带配置的编译器
    ///
    /// # Arguments
    ///
    /// * `config` - 编译器配置
    pub fn with_config(config: VutexConfig) -> Self {
        Self { config, html_renderer: HtmlRenderer::new(), cache: HashMap::new() }
    }

    /// 创建带配置和 HTML 渲染器配置的编译器
    ///
    /// # Arguments
    ///
    /// * `config` - 编译器配置
    /// * `html_config` - HTML 渲染器配置
    pub fn with_html_config(config: VutexConfig, html_config: HtmlRendererConfig) -> Self {
        Self { config, html_renderer: HtmlRenderer::with_config(html_config), cache: HashMap::new() }
    }

    /// 获取编译器配置
    pub fn config(&self) -> &VutexConfig {
        &self.config
    }

    /// 获取可变的编译器配置
    pub fn config_mut(&mut self) -> &mut VutexConfig {
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

        let rendered_html = self.html_renderer.render(&doc.content);
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
            let document_paths: Vec<String> = compiled_docs.keys().cloned().collect();
            super::CompileResult::success(document_paths, compile_time_ms)
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
    pub fn get_cached(&self, path: &str) -> Option<&Document> {
        self.cache.get(path)
    }
}

impl Default for VutexCompiler {
    fn default() -> Self {
        Self::new()
    }
}
