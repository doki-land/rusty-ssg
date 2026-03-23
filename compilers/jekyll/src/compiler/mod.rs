//! 编译器模块
//! 提供 Jekyll 文档编译器的核心功能

use crate::{
    jekyll::{JekyllConfig, MarkdownConverter},
    types::{CompileResult, Result},
};
use std::{collections::HashMap, time::Instant};

mod parser;
mod renderer;

pub use parser::JekyllParser;
pub use renderer::JekyllRenderer;

/// Jekyll 文档编译器
///
/// 负责将 Markdown 文档编译为 HTML 格式
pub struct JekyllCompiler {
    /// 编译器配置
    config: JekyllConfig,
    /// 解析器
    parser: JekyllParser,
    /// 渲染器
    renderer: JekyllRenderer,
    /// 编译缓存
    cache: HashMap<String, String>,
}

impl JekyllCompiler {
    /// 创建新的编译器
    pub fn new(config: JekyllConfig) -> Result<Self> {
        Ok(Self { config, parser: JekyllParser::new(), renderer: JekyllRenderer::new(), cache: HashMap::new() })
    }

    /// 获取编译器配置
    pub fn config(&self) -> &JekyllConfig {
        &self.config
    }

    /// 获取可变的编译器配置
    pub fn config_mut(&mut self) -> &mut JekyllConfig {
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
    pub fn compile_document(&mut self, source: &str, path: &str) -> Result<String> {
        if let Some(cached) = self.cache.get(path) {
            return Ok(cached.clone());
        }

        // 解析文档
        let parsed = self.parser.parse(source, path)?;

        // 渲染为 HTML
        let rendered = self.renderer.render(&parsed, &self.config)?;

        self.cache.insert(path.to_string(), rendered.clone());

        Ok(rendered)
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
        let mut compiled_docs = HashMap::new();
        let mut errors = Vec::new();

        for (path, source) in documents {
            match self.compile_document(source, path) {
                Ok(result) => {
                    compiled_docs.insert(path.to_string(), result);
                }
                Err(e) => {
                    errors.push(e);
                }
            }
        }

        let compile_time_ms = start_time.elapsed().as_millis() as u64;

        if errors.is_empty() {
            let document_paths: Vec<String> = compiled_docs.keys().cloned().collect();
            CompileResult::success(document_paths, compile_time_ms)
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
    pub fn get_cached(&self, path: &str) -> Option<&String> {
        self.cache.get(path)
    }
}

impl Default for JekyllCompiler {
    fn default() -> Self {
        Self::new(JekyllConfig::new()).unwrap()
    }
}
