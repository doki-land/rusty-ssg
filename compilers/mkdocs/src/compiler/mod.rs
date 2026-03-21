//! MkDocs 编译器模块

use crate::types::{MkDocsConfig, Result};
use std::{
    fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub mod html_renderer;
pub use html_renderer::{HtmlRenderer, HtmlRendererConfig};

/// MkDocs 编译器
///
/// 负责编译 MkDocs 文档项目，支持单个文件编译和批量编译。
pub struct MkDocsCompiler {
    /// MkDocs 配置
    config: MkDocsConfig,
    /// HTML 渲染器
    html_renderer: HtmlRenderer,
    /// 源文件目录
    source_dir: PathBuf,
    /// 输出目录
    output_dir: PathBuf,
}

impl MkDocsCompiler {
    /// 创建新的 MkDocs 编译器
    ///
    /// # 参数
    /// * `config` - MkDocs 配置
    /// * `source_dir` - 源文件目录路径
    /// * `output_dir` - 输出目录路径
    ///
    /// # 返回值
    /// 新的 MkDocs 编译器实例
    pub fn new(config: MkDocsConfig, source_dir: impl AsRef<Path>, output_dir: impl AsRef<Path>) -> Self {
        Self {
            config,
            html_renderer: HtmlRenderer::new(),
            source_dir: source_dir.as_ref().to_path_buf(),
            output_dir: output_dir.as_ref().to_path_buf(),
        }
    }

    /// 获取编译器配置
    pub fn config(&self) -> &MkDocsConfig {
        &self.config
    }

    /// 获取 HTML 渲染器
    pub fn html_renderer(&self) -> &HtmlRenderer {
        &self.html_renderer
    }

    /// 获取可变的 HTML 渲染器
    pub fn html_renderer_mut(&mut self) -> &mut HtmlRenderer {
        &mut self.html_renderer
    }

    /// 获取源文件目录
    pub fn source_dir(&self) -> &PathBuf {
        &self.source_dir
    }

    /// 获取输出目录
    pub fn output_dir(&self) -> &PathBuf {
        &self.output_dir
    }

    /// 编译单个 Markdown 文件
    ///
    /// # 参数
    /// * `file_path` - 要编译的 Markdown 文件路径
    ///
    /// # 返回值
    /// 编译结果，包含 HTML 内容
    pub fn compile_file(&self, file_path: impl AsRef<Path>) -> Result<String> {
        let file_path = file_path.as_ref();

        if !file_path.exists() {
            return Err(crate::types::errors::MkDocsError::PathError {
                message: format!("File not found: {}", file_path.display()),
            });
        }

        let content = fs::read_to_string(file_path)?;
        let html = self.html_renderer.render(&content);

        Ok(html)
    }

    /// 编译单个 Markdown 文件并保存到输出目录
    ///
    /// # 参数
    /// * `file_path` - 要编译的 Markdown 文件路径
    ///
    /// # 返回值
    /// 编译时间（毫秒）
    pub fn compile_file_and_save(&self, file_path: impl AsRef<Path>) -> Result<u64> {
        let start_time = std::time::Instant::now();
        let file_path = file_path.as_ref();

        let html = self.compile_file(file_path)?;

        let relative_path = file_path.strip_prefix(&self.source_dir).map_err(|e| {
            crate::types::errors::MkDocsError::PathError { message: format!("Failed to get relative path: {}", e) }
        })?;

        let output_path = self.output_dir.join(relative_path).with_extension("html");

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&output_path, html)?;

        Ok(start_time.elapsed().as_millis() as u64)
    }

    /// 批量编译所有 Markdown 文件
    ///
    /// # 返回值
    /// 编译时间列表（毫秒）
    pub fn compile_all(&self) -> Result<Vec<u64>> {
        let mut results = Vec::new();

        fs::create_dir_all(&self.output_dir)?;

        for entry in WalkDir::new(&self.source_dir) {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "md" || ext == "markdown" {
                        match self.compile_file_and_save(path) {
                            Ok(result) => results.push(result),
                            Err(e) => eprintln!("Error compiling {}: {}", path.display(), e),
                        }
                    }
                }
            }
        }

        Ok(results)
    }

    /// 复制静态资源文件
    ///
    /// 复制非 Markdown 文件到输出目录
    ///
    /// # 返回值
    /// 操作结果
    pub fn copy_static_files(&self) -> Result<()> {
        for entry in WalkDir::new(&self.source_dir) {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                let is_markdown = path.extension().map(|ext| ext == "md" || ext == "markdown").unwrap_or(false);

                if !is_markdown {
                    let relative_path = path.strip_prefix(&self.source_dir).map_err(|e| {
                        crate::types::errors::MkDocsError::PathError { message: format!("Failed to get relative path: {}", e) }
                    })?;

                    let output_path = self.output_dir.join(relative_path);

                    if let Some(parent) = output_path.parent() {
                        fs::create_dir_all(parent)?;
                    }

                    fs::copy(path, output_path)?;
                }
            }
        }

        Ok(())
    }

    /// 完整编译项目
    ///
    /// 编译所有 Markdown 文件并复制静态资源
    ///
    /// # 返回值
    /// 编译时间列表（毫秒）
    pub fn build(&self) -> Result<Vec<u64>> {
        let compile_results = self.compile_all()?;
        self.copy_static_files()?;
        Ok(compile_results)
    }
}
