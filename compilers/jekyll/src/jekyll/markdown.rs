//! Markdown 处理模块
//!
//! 该模块提供多种 Markdown 处理器支持（如 CommonMark、Kramdown），
//! 集成代码高亮功能，并提供可配置的 Markdown 选项。

use nargo_document::generator::markdown::MarkdownRenderer;
use oak_markdown::MarkdownLanguage;
use std::collections::HashMap;

use crate::errors::MarkdownError;

/// Markdown 处理器类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MarkdownProcessor {
    /// CommonMark 标准处理器
    CommonMark,
    /// Kramdown 处理器（模拟 Jekyll 默认行为）
    Kramdown,
    /// GFM (GitHub Flavored Markdown) 处理器
    Gfm,
}

impl MarkdownProcessor {
    /// 获取处理器的名称
    pub fn name(&self) -> &'static str {
        match self {
            MarkdownProcessor::CommonMark => "commonmark",
            MarkdownProcessor::Kramdown => "kramdown",
            MarkdownProcessor::Gfm => "gfm",
        }
    }

    /// 从名称创建 MarkdownProcessor
    pub fn from_name(name: &str) -> Result<Self, MarkdownError> {
        match name.to_lowercase().as_str() {
            "commonmark" | "cmark" => Ok(MarkdownProcessor::CommonMark),
            "kramdown" => Ok(MarkdownProcessor::Kramdown),
            "gfm" | "github" => Ok(MarkdownProcessor::Gfm),
            _ => Err(MarkdownError::InvalidProcessor(name.to_string())),
        }
    }

    /// 获取所有可用的处理器
    pub fn all() -> &'static [MarkdownProcessor] {
        &[MarkdownProcessor::CommonMark, MarkdownProcessor::Kramdown, MarkdownProcessor::Gfm]
    }
}

/// Markdown 选项配置
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkdownOptions {
    /// 启用表格
    pub tables: bool,
    /// 启用脚注
    pub footnotes: bool,
    /// 启用删除线
    pub strikethrough: bool,
    /// 启用任务列表
    pub tasklists: bool,
    /// 启用智能标点
    pub smart_punctuation: bool,
    /// 启用标题属性
    pub heading_attributes: bool,
    /// 启用 YAML 元数据块
    pub yaml_metadata_block: bool,
    /// 启用代码高亮
    pub code_highlighting: bool,
    /// 自定义代码高亮主题
    pub highlight_theme: Option<String>,
    /// 自定义处理器特定选项
    pub custom_options: HashMap<String, String>,
}

impl Default for MarkdownOptions {
    fn default() -> Self {
        Self {
            tables: true,
            footnotes: true,
            strikethrough: true,
            tasklists: true,
            smart_punctuation: true,
            heading_attributes: false,
            yaml_metadata_block: true,
            code_highlighting: true,
            highlight_theme: None,
            custom_options: HashMap::new(),
        }
    }
}

impl MarkdownOptions {
    /// 创建新的 Markdown 选项
    pub fn new() -> Self {
        Self::default()
    }

    /// 启用表格
    pub fn with_tables(mut self, enabled: bool) -> Self {
        self.tables = enabled;
        self
    }

    /// 启用脚注
    pub fn with_footnotes(mut self, enabled: bool) -> Self {
        self.footnotes = enabled;
        self
    }

    /// 启用删除线
    pub fn with_strikethrough(mut self, enabled: bool) -> Self {
        self.strikethrough = enabled;
        self
    }

    /// 启用任务列表
    pub fn with_tasklists(mut self, enabled: bool) -> Self {
        self.tasklists = enabled;
        self
    }

    /// 启用智能标点
    pub fn with_smart_punctuation(mut self, enabled: bool) -> Self {
        self.smart_punctuation = enabled;
        self
    }

    /// 启用代码高亮
    pub fn with_code_highlighting(mut self, enabled: bool) -> Self {
        self.code_highlighting = enabled;
        self
    }

    /// 设置代码高亮主题
    pub fn with_highlight_theme(mut self, theme: String) -> Self {
        self.highlight_theme = Some(theme);
        self
    }

    /// 设置自定义选项
    pub fn with_custom_option(mut self, key: String, value: String) -> Self {
        self.custom_options.insert(key, value);
        self
    }

    /// 获取 oak-markdown 语言配置
    fn get_oak_markdown_language(&self) -> MarkdownLanguage {
        let mut lang = MarkdownLanguage::default();

        lang.allow_tables = self.tables;
        lang.allow_footnotes = self.footnotes;
        lang.allow_strikethrough = self.strikethrough;
        lang.allow_task_lists = self.tasklists;

        lang
    }
}

/// Markdown 转换器
pub struct MarkdownConverter {
    /// 使用的处理器
    processor: MarkdownProcessor,
    /// Markdown 选项
    options: MarkdownOptions,
    /// nargo-document Markdown 渲染器
    renderer: MarkdownRenderer,
}

impl MarkdownConverter {
    /// 创建新的 Markdown 转换器
    ///
    /// # Arguments
    ///
    /// * `processor` - Markdown 处理器类型
    /// * `options` - Markdown 选项
    pub fn new(processor: MarkdownProcessor, options: MarkdownOptions) -> Self {
        // 创建 nargo-document Markdown 渲染器配置
        let renderer_config = nargo_document::generator::markdown::MarkdownRendererConfig {
            enable_tables: options.tables,
            enable_footnotes: options.footnotes,
            enable_strikethrough: options.strikethrough,
            enable_tasklists: options.tasklists,
            enable_smart_punctuation: options.smart_punctuation,
        };

        // 创建渲染器
        let renderer = MarkdownRenderer::with_config(renderer_config);

        Self { processor, options, renderer }
    }

    /// 创建使用 CommonMark 处理器的转换器
    pub fn commonmark(options: MarkdownOptions) -> Self {
        Self::new(MarkdownProcessor::CommonMark, options)
    }

    /// 创建使用 Kramdown 处理器的转换器
    pub fn kramdown(options: MarkdownOptions) -> Self {
        Self::new(MarkdownProcessor::Kramdown, options)
    }

    /// 创建使用 GFM 处理器的转换器
    pub fn gfm(options: MarkdownOptions) -> Self {
        Self::new(MarkdownProcessor::Gfm, options)
    }

    /// 使用默认选项创建转换器
    pub fn with_defaults(processor: MarkdownProcessor) -> Self {
        Self::new(processor, MarkdownOptions::default())
    }

    /// 获取当前使用的处理器
    pub fn processor(&self) -> MarkdownProcessor {
        self.processor
    }

    /// 获取当前的选项
    pub fn options(&self) -> &MarkdownOptions {
        &self.options
    }

    /// 获取选项的可变引用
    pub fn options_mut(&mut self) -> &mut MarkdownOptions {
        &mut self.options
    }

    /// 转换 Markdown 为 HTML
    ///
    /// # Arguments
    ///
    /// * `markdown` - Markdown 内容
    ///
    /// # Returns
    ///
    /// 返回转换后的 HTML 字符串
    ///
    /// # Errors
    ///
    /// 返回 `MarkdownError` 如果处理失败
    pub fn convert(&self, markdown: &str) -> Result<String, MarkdownError> {
        self.renderer.render(markdown).map_err(|e| MarkdownError::HighlightError(e.to_string()))
    }

    /// 从文件转换 Markdown 为 HTML
    ///
    /// # Arguments
    ///
    /// * `file_path` - Markdown 文件路径
    ///
    /// # Returns
    ///
    /// 返回转换后的 HTML 字符串
    ///
    /// # Errors
    ///
    /// 返回 `std::io::Error` 如果文件读取失败，或 `MarkdownError` 如果处理失败
    pub fn convert_file(&self, file_path: &std::path::Path) -> Result<String, std::io::Error> {
        let markdown = std::fs::read_to_string(file_path)?;
        self.convert(&markdown).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
    }

    /// 从 Jekyll 配置创建 Markdown 转换器
    ///
    /// # Arguments
    ///
    /// * `config` - Jekyll 配置
    ///
    /// # Returns
    ///
    /// 返回创建的 MarkdownConverter
    pub fn from_jekyll_config(config: &crate::jekyll::JekyllConfig) -> Self {
        let processor = match config.markdown.as_deref() {
            Some("kramdown") => MarkdownProcessor::Kramdown,
            Some("gfm") => MarkdownProcessor::Gfm,
            _ => MarkdownProcessor::CommonMark,
        };

        let mut options = MarkdownOptions::default();

        // 从配置中读取 Markdown 相关选项
        if let Some(theme) = config.get_custom("markdown") {
            if let Some(theme_obj) = theme.as_object() {
                if let Some(theme_str) = theme_obj.get("theme") {
                    if let Some(theme_name) = theme_str.as_str() {
                        options.highlight_theme = Some(theme_name.to_string());
                    }
                }
            }
        }

        Self::new(processor, options)
    }
}
