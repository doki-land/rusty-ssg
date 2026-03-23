//! 解析器模块
//! 提供 Markdown 文档解析功能

use oak_core::{Builder, ParseSession};
use oak_markdown::{MarkdownBuilder, MarkdownLanguage, ast::MarkdownRoot};
use nargo_types::Document;
use std::collections::HashMap;

/// 解析器配置
#[derive(Debug, Clone)]
pub struct ParserConfig {
    /// 是否启用 GFM (GitHub Flavored Markdown)
    pub gfm: bool,
    /// 是否启用表格支持
    pub tables: bool,
    /// 是否启用任务列表支持
    pub tasklists: bool,
    /// 是否启用删除线支持
    pub strikethrough: bool,
    /// 是否启用自动链接
    pub autolink: bool,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self { gfm: true, tables: true, tasklists: true, strikethrough: true, autolink: true }
    }
}

impl ParserConfig {
    /// 创建默认的解析器配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置是否启用 GFM
    ///
    /// # Arguments
    ///
    /// * `gfm` - 是否启用 GFM
    pub fn with_gfm(mut self, gfm: bool) -> Self {
        self.gfm = gfm;
        self
    }

    /// 设置是否启用表格支持
    ///
    /// # Arguments
    ///
    /// * `tables` - 是否启用表格支持
    pub fn with_tables(mut self, tables: bool) -> Self {
        self.tables = tables;
        self
    }

    /// 设置是否启用任务列表支持
    ///
    /// # Arguments
    ///
    /// * `tasklists` - 是否启用任务列表支持
    pub fn with_tasklists(mut self, tasklists: bool) -> Self {
        self.tasklists = tasklists;
        self
    }

    /// 设置是否启用删除线支持
    ///
    /// # Arguments
    ///
    /// * `strikethrough` - 是否启用删除线支持
    pub fn with_strikethrough(mut self, strikethrough: bool) -> Self {
        self.strikethrough = strikethrough;
        self
    }

    /// 设置是否启用自动链接
    ///
    /// # Arguments
    ///
    /// * `autolink` - 是否启用自动链接
    pub fn with_autolink(mut self, autolink: bool) -> Self {
        self.autolink = autolink;
        self
    }
}

/// Markdown 解析器
///
/// 负责将 Markdown 内容解析为文档对象
pub struct MarkdownParser {
    /// 解析器配置
    config: ParserConfig,
    /// Markdown 语言配置
    lang_config: MarkdownLanguage,
    /// 自定义解析选项
    options: HashMap<String, String>,
}

impl MarkdownParser {
    /// 创建新的 Markdown 解析器
    pub fn new() -> Self {
        Self::with_config(ParserConfig::default())
    }

    /// 创建带配置的 Markdown 解析器
    ///
    /// # Arguments
    ///
    /// * `config` - 解析器配置
    pub fn with_config(config: ParserConfig) -> Self {
        let mut lang_config = MarkdownLanguage::default();
        lang_config.allow_tables = config.tables;
        lang_config.allow_task_lists = config.tasklists;
        lang_config.allow_strikethrough = config.strikethrough;

        Self { config, lang_config, options: HashMap::new() }
    }

    /// 获取解析器配置
    pub fn config(&self) -> &ParserConfig {
        &self.config
    }

    /// 获取可变的解析器配置
    pub fn config_mut(&mut self) -> &mut ParserConfig {
        &mut self.config
    }

    /// 设置自定义选项
    ///
    /// # Arguments
    ///
    /// * `key` - 选项键
    /// * `value` - 选项值
    pub fn set_option(&mut self, key: String, value: String) {
        self.options.insert(key, value);
    }

    /// 获取自定义选项
    ///
    /// # Arguments
    ///
    /// * `key` - 选项键
    pub fn get_option(&self, key: &str) -> Option<&String> {
        self.options.get(key)
    }

    /// 解析 Markdown 内容为 AST
    ///
    /// # Arguments
    ///
    /// * `content` - Markdown 内容
    ///
    /// # Returns
    ///
    /// 解析后的 Markdown AST
    pub fn parse_ast(&self, content: &str) -> Result<MarkdownRoot, String> {
        let builder = MarkdownBuilder::new(&self.lang_config);
        let mut cache = ParseSession::default();
        let build_output = builder.build(content, &[], &mut cache);

        build_output.result.map_err(|e| format!("Parse error: {:?}", e))
    }

    /// 解析 Markdown 内容为文档对象
    ///
    /// # Arguments
    ///
    /// * `content` - Markdown 内容
    /// * `path` - 文档路径
    ///
    /// # Returns
    ///
    /// 解析后的文档对象
    pub fn parse_document(&self, content: &str, path: &str) -> Result<Document, String> {
        // 解析 frontmatter
        let (frontmatter, body) = self.extract_frontmatter(content);

        // 解析 body 为 AST
        let ast = self.parse_ast(&body)?;

        // 创建文档对象
        let mut doc = Document::new();
        doc.meta.path = path.to_string();

        // 设置 frontmatter
        if let Some(frontmatter_content) = frontmatter {
            doc.frontmatter = self.parse_frontmatter(&frontmatter_content)?;
        }

        // 设置内容
        doc.content = body;

        Ok(doc)
    }

    /// 提取 frontmatter
    ///
    /// # Arguments
    ///
    /// * `content` - Markdown 内容
    ///
    /// # Returns
    ///
    /// (frontmatter 内容, 剩余内容)
    fn extract_frontmatter(&self, content: &str) -> (Option<String>, String) {
        let lines: Vec<&str> = content.lines().collect();

        if lines.len() >= 3 && lines[0] == "---" {
            for (i, line) in lines.iter().enumerate().skip(1) {
                if *line == "---" {
                    let frontmatter = lines[1..i].join("\n");
                    let body = lines[i+1..].join("\n");
                    return (Some(frontmatter), body);
                }
            }
        }

        (None, content.to_string())
    }

    /// 解析 frontmatter
    ///
    /// # Arguments
    ///
    /// * `content` - frontmatter 内容
    ///
    /// # Returns
    ///
    /// 解析后的 frontmatter
    fn parse_frontmatter(&self, content: &str) -> Result<nargo_types::Frontmatter, String> {
        use oak_yaml::language::from_str;

        let frontmatter: nargo_types::Frontmatter = from_str(content)
            .map_err(|e| format!("Frontmatter parse error: {:?}", e))?;

        Ok(frontmatter)
    }
}

impl Default for MarkdownParser {
    fn default() -> Self {
        Self::new()
    }
}

/// 解析器 trait
pub trait Parser {
    /// 解析 Markdown 内容
    ///
    /// # Arguments
    ///
    /// * `content` - Markdown 内容
    /// * `path` - 文档路径
    ///
    /// # Returns
    ///
    /// 解析后的文档对象
    fn parse(&self, content: &str, path: &str) -> Result<Document, String>;
}

impl Parser for MarkdownParser {
    fn parse(&self, content: &str, path: &str) -> Result<Document, String> {
        self.parse_document(content, path)
    }
}
