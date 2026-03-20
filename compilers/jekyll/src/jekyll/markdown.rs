//! Markdown 处理模块
//!
//! 该模块提供多种 Markdown 处理器支持（如 CommonMark、Kramdown、GFM），
//! 集成代码高亮功能，并提供可配置的 Markdown 选项。

use nargo_document::generator::markdown::MarkdownRenderer;
use oak_markdown::MarkdownLanguage;
use std::collections::HashMap;

use crate::errors::MarkdownError;

/// Markdown 处理器类型
///
/// 支持三种主要的 Markdown 处理器：
/// - `CommonMark`: 标准 CommonMark 规范
/// - `Kramdown`: 模拟 Jekyll 默认的 Kramdown 行为
/// - `Gfm`: GitHub Flavored Markdown
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
    ///
    /// # Returns
    ///
    /// 返回处理器名称的字符串切片
    pub fn name(&self) -> &'static str {
        match self {
            MarkdownProcessor::CommonMark => "commonmark",
            MarkdownProcessor::Kramdown => "kramdown",
            MarkdownProcessor::Gfm => "gfm",
        }
    }

    /// 从名称创建 MarkdownProcessor
    ///
    /// # Arguments
    ///
    /// * `name` - 处理器名称，支持 "commonmark"、"cmark"、"kramdown"、"gfm"、"github"
    ///
    /// # Returns
    ///
    /// 返回 `Result<MarkdownProcessor, MarkdownError>`，成功时包含处理器实例，
    /// 失败时返回 `MarkdownError::InvalidProcessor`
    pub fn from_name(name: &str) -> Result<Self, MarkdownError> {
        match name.to_lowercase().as_str() {
            "commonmark" | "cmark" => Ok(MarkdownProcessor::CommonMark),
            "kramdown" => Ok(MarkdownProcessor::Kramdown),
            "gfm" | "github" => Ok(MarkdownProcessor::Gfm),
            _ => Err(MarkdownError::InvalidProcessor(name.to_string())),
        }
    }

    /// 获取所有可用的处理器
    ///
    /// # Returns
    ///
    /// 返回包含所有处理器的静态数组切片
    pub fn all() -> &'static [MarkdownProcessor] {
        &[
            MarkdownProcessor::CommonMark,
            MarkdownProcessor::Kramdown,
            MarkdownProcessor::Gfm,
        ]
    }

    /// 获取处理器的默认 Markdown 语言配置
    ///
    /// # Returns
    ///
    /// 返回该处理器对应的默认 `MarkdownLanguage` 配置
    pub fn default_language_config(&self) -> MarkdownLanguage {
        match self {
            MarkdownProcessor::CommonMark => MarkdownLanguage {
                allow_math: false,
                allow_tables: false,
                allow_task_lists: false,
                allow_strikethrough: false,
                allow_footnotes: false,
                allow_front_matter: false,
                allow_definition_lists: false,
                allow_subscript: false,
                allow_autolinks: true,
                allow_abbreviations: false,
                allow_indented_code_blocks: true,
                allow_html: true,
                allow_hard_line_breaks: false,
                allow_gfm_autolinks: false,
                allow_headings: true,
                allow_lists: true,
                allow_blockquotes: true,
                allow_fenced_code_blocks: false,
                allow_horizontal_rules: true,
                allow_setext_headings: true,
                allow_html_tagfilter: false,
                allow_xml: false,
            },
            MarkdownProcessor::Kramdown => MarkdownLanguage {
                allow_math: true,
                allow_tables: true,
                allow_task_lists: false,
                allow_strikethrough: true,
                allow_footnotes: true,
                allow_front_matter: true,
                allow_definition_lists: true,
                allow_subscript: true,
                allow_autolinks: true,
                allow_abbreviations: true,
                allow_indented_code_blocks: true,
                allow_html: true,
                allow_hard_line_breaks: true,
                allow_gfm_autolinks: false,
                allow_headings: true,
                allow_lists: true,
                allow_blockquotes: true,
                allow_fenced_code_blocks: true,
                allow_horizontal_rules: true,
                allow_setext_headings: true,
                allow_html_tagfilter: false,
                allow_xml: false,
            },
            MarkdownProcessor::Gfm => MarkdownLanguage {
                allow_math: false,
                allow_tables: true,
                allow_task_lists: true,
                allow_strikethrough: true,
                allow_footnotes: false,
                allow_front_matter: true,
                allow_definition_lists: false,
                allow_subscript: false,
                allow_autolinks: true,
                allow_abbreviations: false,
                allow_indented_code_blocks: true,
                allow_html: true,
                allow_hard_line_breaks: true,
                allow_gfm_autolinks: true,
                allow_headings: true,
                allow_lists: true,
                allow_blockquotes: true,
                allow_fenced_code_blocks: true,
                allow_horizontal_rules: true,
                allow_setext_headings: true,
                allow_html_tagfilter: true,
                allow_xml: false,
            },
        }
    }
}

/// Markdown 选项配置
///
/// 提供全面的 Markdown 扩展配置选项，包括表格、脚注、删除线、
/// 任务列表、智能标点、数学公式、定义列表等多种功能。
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
    /// 启用数学公式
    pub math: bool,
    /// 启用定义列表
    pub definition_lists: bool,
    /// 启用上下标
    pub subscript: bool,
    /// 启用自动链接
    pub autolinks: bool,
    /// 启用 GFM 自动链接
    pub gfm_autolinks: bool,
    /// 启用缩写
    pub abbreviations: bool,
    /// 启用缩进代码块
    pub indented_code_blocks: bool,
    /// 启用围栏代码块
    pub fenced_code_blocks: bool,
    /// 启用内联 HTML
    pub allow_html: bool,
    /// 启用 HTML 标签过滤
    pub html_tagfilter: bool,
    /// 启用硬换行
    pub hard_line_breaks: bool,
    /// 启用 ATX 标题
    pub allow_headings: bool,
    /// 启用列表
    pub allow_lists: bool,
    /// 启用块引用
    pub allow_blockquotes: bool,
    /// 启用水平分隔线
    pub allow_horizontal_rules: bool,
    /// 启用 Setext 标题
    pub allow_setext_headings: bool,
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
            math: false,
            definition_lists: false,
            subscript: false,
            autolinks: true,
            gfm_autolinks: true,
            abbreviations: true,
            indented_code_blocks: true,
            fenced_code_blocks: true,
            allow_html: true,
            html_tagfilter: false,
            hard_line_breaks: true,
            allow_headings: true,
            allow_lists: true,
            allow_blockquotes: true,
            allow_horizontal_rules: true,
            allow_setext_headings: true,
            custom_options: HashMap::new(),
        }
    }
}

impl MarkdownOptions {
    /// 创建新的 Markdown 选项
    ///
    /// 使用默认配置创建 Markdown 选项实例
    ///
    /// # Returns
    ///
    /// 返回默认配置的 `MarkdownOptions` 实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 创建适用于 CommonMark 处理器的选项
    ///
    /// # Returns
    ///
    /// 返回适用于 CommonMark 的 `MarkdownOptions` 实例
    pub fn commonmark() -> Self {
        Self {
            tables: false,
            footnotes: false,
            strikethrough: false,
            tasklists: false,
            smart_punctuation: false,
            heading_attributes: false,
            yaml_metadata_block: false,
            code_highlighting: true,
            highlight_theme: None,
            math: false,
            definition_lists: false,
            subscript: false,
            autolinks: true,
            gfm_autolinks: false,
            abbreviations: false,
            indented_code_blocks: true,
            fenced_code_blocks: false,
            allow_html: true,
            html_tagfilter: false,
            hard_line_breaks: false,
            allow_headings: true,
            allow_lists: true,
            allow_blockquotes: true,
            allow_horizontal_rules: true,
            allow_setext_headings: true,
            custom_options: HashMap::new(),
        }
    }

    /// 创建适用于 Kramdown 处理器的选项
    ///
    /// # Returns
    ///
    /// 返回适用于 Kramdown 的 `MarkdownOptions` 实例
    pub fn kramdown() -> Self {
        Self {
            tables: true,
            footnotes: true,
            strikethrough: true,
            tasklists: false,
            smart_punctuation: true,
            heading_attributes: true,
            yaml_metadata_block: true,
            code_highlighting: true,
            highlight_theme: None,
            math: true,
            definition_lists: true,
            subscript: true,
            autolinks: true,
            gfm_autolinks: false,
            abbreviations: true,
            indented_code_blocks: true,
            fenced_code_blocks: true,
            allow_html: true,
            html_tagfilter: false,
            hard_line_breaks: true,
            allow_headings: true,
            allow_lists: true,
            allow_blockquotes: true,
            allow_horizontal_rules: true,
            allow_setext_headings: true,
            custom_options: HashMap::new(),
        }
    }

    /// 创建适用于 GFM 处理器的选项
    ///
    /// # Returns
    ///
    /// 返回适用于 GFM 的 `MarkdownOptions` 实例
    pub fn gfm() -> Self {
        Self {
            tables: true,
            footnotes: false,
            strikethrough: true,
            tasklists: true,
            smart_punctuation: false,
            heading_attributes: false,
            yaml_metadata_block: true,
            code_highlighting: true,
            highlight_theme: None,
            math: false,
            definition_lists: false,
            subscript: false,
            autolinks: true,
            gfm_autolinks: true,
            abbreviations: false,
            indented_code_blocks: true,
            fenced_code_blocks: true,
            allow_html: true,
            html_tagfilter: true,
            hard_line_breaks: true,
            allow_headings: true,
            allow_lists: true,
            allow_blockquotes: true,
            allow_horizontal_rules: true,
            allow_setext_headings: true,
            custom_options: HashMap::new(),
        }
    }

    /// 启用表格
    ///
    /// # Arguments
    ///
    /// * `enabled` - 是否启用表格功能
    ///
    /// # Returns
    ///
    /// 返回修改后的 `MarkdownOptions` 实例，支持链式调用
    pub fn with_tables(mut self, enabled: bool) -> Self {
        self.tables = enabled;
        self
    }

    /// 启用脚注
    ///
    /// # Arguments
    ///
    /// * `enabled` - 是否启用脚注功能
    ///
    /// # Returns
    ///
    /// 返回修改后的 `MarkdownOptions` 实例，支持链式调用
    pub fn with_footnotes(mut self, enabled: bool) -> Self {
        self.footnotes = enabled;
        self
    }

    /// 启用删除线
    ///
    /// # Arguments
    ///
    /// * `enabled` - 是否启用删除线功能
    ///
    /// # Returns
    ///
    /// 返回修改后的 `MarkdownOptions` 实例，支持链式调用
    pub fn with_strikethrough(mut self, enabled: bool) -> Self {
        self.strikethrough = enabled;
        self
    }

    /// 启用任务列表
    ///
    /// # Arguments
    ///
    /// * `enabled` - 是否启用任务列表功能
    ///
    /// # Returns
    ///
    /// 返回修改后的 `MarkdownOptions` 实例，支持链式调用
    pub fn with_tasklists(mut self, enabled: bool) -> Self {
        self.tasklists = enabled;
        self
    }

    /// 启用智能标点
    ///
    /// # Arguments
    ///
    /// * `enabled` - 是否启用智能标点功能
    ///
    /// # Returns
    ///
    /// 返回修改后的 `MarkdownOptions` 实例，支持链式调用
    pub fn with_smart_punctuation(mut self, enabled: bool) -> Self {
        self.smart_punctuation = enabled;
        self
    }

    /// 启用代码高亮
    ///
    /// # Arguments
    ///
    /// * `enabled` - 是否启用代码高亮功能
    ///
    /// # Returns
    ///
    /// 返回修改后的 `MarkdownOptions` 实例，支持链式调用
    pub fn with_code_highlighting(mut self, enabled: bool) -> Self {
        self.code_highlighting = enabled;
        self
    }

    /// 设置代码高亮主题
    ///
    /// # Arguments
    ///
    /// * `theme` - 代码高亮主题名称
    ///
    /// # Returns
    ///
    /// 返回修改后的 `MarkdownOptions` 实例，支持链式调用
    pub fn with_highlight_theme(mut self, theme: String) -> Self {
        self.highlight_theme = Some(theme);
        self
    }

    /// 启用数学公式
    ///
    /// # Arguments
    ///
    /// * `enabled` - 是否启用数学公式功能
    ///
    /// # Returns
    ///
    /// 返回修改后的 `MarkdownOptions` 实例，支持链式调用
    pub fn with_math(mut self, enabled: bool) -> Self {
        self.math = enabled;
        self
    }

    /// 启用定义列表
    ///
    /// # Arguments
    ///
    /// * `enabled` - 是否启用定义列表功能
    ///
    /// # Returns
    ///
    /// 返回修改后的 `MarkdownOptions` 实例，支持链式调用
    pub fn with_definition_lists(mut self, enabled: bool) -> Self {
        self.definition_lists = enabled;
        self
    }

    /// 启用上下标
    ///
    /// # Arguments
    ///
    /// * `enabled` - 是否启用上下标功能
    ///
    /// # Returns
    ///
    /// 返回修改后的 `MarkdownOptions` 实例，支持链式调用
    pub fn with_subscript(mut self, enabled: bool) -> Self {
        self.subscript = enabled;
        self
    }

    /// 启用自动链接
    ///
    /// # Arguments
    ///
    /// * `enabled` - 是否启用自动链接功能
    ///
    /// # Returns
    ///
    /// 返回修改后的 `MarkdownOptions` 实例，支持链式调用
    pub fn with_autolinks(mut self, enabled: bool) -> Self {
        self.autolinks = enabled;
        self
    }

    /// 启用 GFM 自动链接
    ///
    /// # Arguments
    ///
    /// * `enabled` - 是否启用 GFM 自动链接功能
    ///
    /// # Returns
    ///
    /// 返回修改后的 `MarkdownOptions` 实例，支持链式调用
    pub fn with_gfm_autolinks(mut self, enabled: bool) -> Self {
        self.gfm_autolinks = enabled;
        self
    }

    /// 启用缩写
    ///
    /// # Arguments
    ///
    /// * `enabled` - 是否启用缩写功能
    ///
    /// # Returns
    ///
    /// 返回修改后的 `MarkdownOptions` 实例，支持链式调用
    pub fn with_abbreviations(mut self, enabled: bool) -> Self {
        self.abbreviations = enabled;
        self
    }

    /// 启用内联 HTML
    ///
    /// # Arguments
    ///
    /// * `enabled` - 是否启用内联 HTML 功能
    ///
    /// # Returns
    ///
    /// 返回修改后的 `MarkdownOptions` 实例，支持链式调用
    pub fn with_allow_html(mut self, enabled: bool) -> Self {
        self.allow_html = enabled;
        self
    }

    /// 启用 HTML 标签过滤
    ///
    /// # Arguments
    ///
    /// * `enabled` - 是否启用 HTML 标签过滤功能
    ///
    /// # Returns
    ///
    /// 返回修改后的 `MarkdownOptions` 实例，支持链式调用
    pub fn with_html_tagfilter(mut self, enabled: bool) -> Self {
        self.html_tagfilter = enabled;
        self
    }

    /// 启用硬换行
    ///
    /// # Arguments
    ///
    /// * `enabled` - 是否启用硬换行功能
    ///
    /// # Returns
    ///
    /// 返回修改后的 `MarkdownOptions` 实例，支持链式调用
    pub fn with_hard_line_breaks(mut self, enabled: bool) -> Self {
        self.hard_line_breaks = enabled;
        self
    }

    /// 设置自定义选项
    ///
    /// # Arguments
    ///
    /// * `key` - 自定义选项键
    /// * `value` - 自定义选项值
    ///
    /// # Returns
    ///
    /// 返回修改后的 `MarkdownOptions` 实例，支持链式调用
    pub fn with_custom_option(mut self, key: String, value: String) -> Self {
        self.custom_options.insert(key, value);
        self
    }

    /// 获取 oak-markdown 语言配置
    ///
    /// 根据当前选项生成对应的 `MarkdownLanguage` 配置
    ///
    /// # Returns
    ///
    /// 返回配置好的 `MarkdownLanguage` 实例
    pub fn get_oak_markdown_language(&self) -> MarkdownLanguage {
        let mut lang = MarkdownLanguage::default();

        lang.allow_tables = self.tables;
        lang.allow_footnotes = self.footnotes;
        lang.allow_strikethrough = self.strikethrough;
        lang.allow_task_lists = self.tasklists;
        lang.allow_math = self.math;
        lang.allow_definition_lists = self.definition_lists;
        lang.allow_subscript = self.subscript;
        lang.allow_autolinks = self.autolinks;
        lang.allow_gfm_autolinks = self.gfm_autolinks;
        lang.allow_abbreviations = self.abbreviations;
        lang.allow_indented_code_blocks = self.indented_code_blocks;
        lang.allow_fenced_code_blocks = self.fenced_code_blocks;
        lang.allow_html = self.allow_html;
        lang.allow_html_tagfilter = self.html_tagfilter;
        lang.allow_hard_line_breaks = self.hard_line_breaks;
        lang.allow_headings = self.allow_headings;
        lang.allow_lists = self.allow_lists;
        lang.allow_blockquotes = self.allow_blockquotes;
        lang.allow_horizontal_rules = self.allow_horizontal_rules;
        lang.allow_setext_headings = self.allow_setext_headings;

        lang
    }
}

/// Markdown 转换器
///
/// 提供将 Markdown 文本转换为 HTML 的功能，支持多种处理器和配置选项。
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
    ///
    /// # Returns
    ///
    /// 返回配置好的 `MarkdownConverter` 实例
    pub fn new(processor: MarkdownProcessor, options: MarkdownOptions) -> Self {
        let renderer_config = nargo_document::generator::markdown::MarkdownRendererConfig {
            enable_tables: options.tables,
            enable_footnotes: options.footnotes,
            enable_strikethrough: options.strikethrough,
            enable_tasklists: options.tasklists,
            enable_smart_punctuation: options.smart_punctuation,
        };

        let renderer = MarkdownRenderer::with_config(renderer_config);

        Self {
            processor,
            options,
            renderer,
        }
    }

    /// 创建使用 CommonMark 处理器的转换器
    ///
    /// # Arguments
    ///
    /// * `options` - Markdown 选项
    ///
    /// # Returns
    ///
    /// 返回配置好的 `MarkdownConverter` 实例
    pub fn commonmark(options: MarkdownOptions) -> Self {
        Self::new(MarkdownProcessor::CommonMark, options)
    }

    /// 创建使用 Kramdown 处理器的转换器
    ///
    /// # Arguments
    ///
    /// * `options` - Markdown 选项
    ///
    /// # Returns
    ///
    /// 返回配置好的 `MarkdownConverter` 实例
    pub fn kramdown(options: MarkdownOptions) -> Self {
        Self::new(MarkdownProcessor::Kramdown, options)
    }

    /// 创建使用 GFM 处理器的转换器
    ///
    /// # Arguments
    ///
    /// * `options` - Markdown 选项
    ///
    /// # Returns
    ///
    /// 返回配置好的 `MarkdownConverter` 实例
    pub fn gfm(options: MarkdownOptions) -> Self {
        Self::new(MarkdownProcessor::Gfm, options)
    }

    /// 使用默认选项创建转换器
    ///
    /// # Arguments
    ///
    /// * `processor` - Markdown 处理器类型
    ///
    /// # Returns
    ///
    /// 返回使用默认配置的 `MarkdownConverter` 实例
    pub fn with_defaults(processor: MarkdownProcessor) -> Self {
        let options = match processor {
            MarkdownProcessor::CommonMark => MarkdownOptions::commonmark(),
            MarkdownProcessor::Kramdown => MarkdownOptions::kramdown(),
            MarkdownProcessor::Gfm => MarkdownOptions::gfm(),
        };
        Self::new(processor, options)
    }

    /// 获取当前使用的处理器
    ///
    /// # Returns
    ///
    /// 返回当前使用的 `MarkdownProcessor`
    pub fn processor(&self) -> MarkdownProcessor {
        self.processor
    }

    /// 获取当前的选项
    ///
    /// # Returns
    ///
    /// 返回当前选项的引用
    pub fn options(&self) -> &MarkdownOptions {
        &self.options
    }

    /// 获取选项的可变引用
    ///
    /// # Returns
    ///
    /// 返回当前选项的可变引用
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
        self.renderer
            .render(markdown)
            .map_err(|e| MarkdownError::HighlightError(e.to_string()))
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
    pub fn convert_file(
        &self,
        file_path: &std::path::Path,
    ) -> Result<String, std::io::Error> {
        let markdown = std::fs::read_to_string(file_path)?;
        self.convert(&markdown)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
    }

    /// 从 Jekyll 配置创建 Markdown 转换器
    ///
    /// # Arguments
    ///
    /// * `config` - Jekyll 配置
    ///
    /// # Returns
    ///
    /// 返回创建的 `MarkdownConverter`
    pub fn from_jekyll_config(config: &crate::jekyll::JekyllConfig) -> Self {
        let processor = match config.markdown.as_deref() {
            Some("kramdown") => MarkdownProcessor::Kramdown,
            Some("gfm") => MarkdownProcessor::Gfm,
            _ => MarkdownProcessor::CommonMark,
        };

        let mut options = match processor {
            MarkdownProcessor::CommonMark => MarkdownOptions::commonmark(),
            MarkdownProcessor::Kramdown => MarkdownOptions::kramdown(),
            MarkdownProcessor::Gfm => MarkdownOptions::gfm(),
        };

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
