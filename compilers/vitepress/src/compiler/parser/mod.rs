//! 编译器解析器模块
//! 使用 oaks 解析 Markdown 文档

use nargo_types::{Document, FrontMatter};
use oak_core::{Builder, parser::session::ParseSession, source::SourceText};
use oak_markdown::{MarkdownBuilder, MarkdownLanguage};
use oak_yaml;
use serde_json::Value;

use crate::types::{Result, VitePressError};

/// Markdown 解析器
/// 使用 oaks 解析 Markdown 文档
pub struct MarkdownParser {
    /// Markdown 语言配置
    lang_config: MarkdownLanguage,
}

impl MarkdownParser {
    /// 创建新的 Markdown 解析器
    pub fn new() -> Self {
        Self { lang_config: MarkdownLanguage::default() }
    }

    /// 创建带配置的 Markdown 解析器
    ///
    /// # Arguments
    ///
    /// * `lang_config` - Markdown 语言配置
    pub fn with_config(lang_config: MarkdownLanguage) -> Self {
        Self { lang_config }
    }

    /// 解析 Markdown 文档
    ///
    /// # Arguments
    ///
    /// * `source` - Markdown 源文件内容
    /// * `path` - 文档路径
    ///
    /// # Returns
    ///
    /// 解析后的文档
    pub fn parse(&self, source: &str, path: &str) -> Result<Document> {
        let source_text = SourceText::new(source);
        let builder = MarkdownBuilder::new(&self.lang_config);
        let mut session = ParseSession::default();

        let output = builder.build(&source_text, &[], &mut session);

        match output.result {
            Ok(_root) => {
                let frontmatter = self.extract_frontmatter(source)?;
                Ok(Document {
                    meta: nargo_types::DocumentMeta {
                        path: path.to_string(),
                        title: frontmatter.title.clone(),
                        lang: None,
                        last_updated: None,
                        extra: std::collections::HashMap::new(),
                    },
                    frontmatter,
                    content: source.to_string(),
                    rendered_content: None,
                    span: nargo_types::Span::unknown(),
                })
            }
            Err(e) => Err(VitePressError::ParseError {
                message: format!("Failed to parse Markdown: {}", e),
                span: nargo_types::Span::unknown(),
                path: Some(path.to_string()),
            }),
        }
    }

    /// 提取文档的 frontmatter
    ///
    /// # Arguments
    ///
    /// * `source` - Markdown 源文件内容
    ///
    /// # Returns
    ///
    /// 提取的 frontmatter
    fn extract_frontmatter(&self, source: &str) -> Result<nargo_types::FrontMatter> {
        // 简单的 frontmatter 提取逻辑
        // 实际实现可能需要更复杂的解析
        let mut frontmatter = nargo_types::FrontMatter::default();

        // 找到第一个非空行
        let mut lines = source.lines().skip_while(|line| line.trim().is_empty());

        // 检查是否以 --- 开头
        if let Some(first_line) = lines.next() {
            if first_line.trim() == "---" {
                let mut frontmatter_lines = Vec::new();

                // 收集 frontmatter 内容，直到遇到 --- 或文件结束
                for line in lines {
                    if line.trim() == "---" {
                        break;
                    }
                    frontmatter_lines.push(line);
                }

                let frontmatter_content = frontmatter_lines.join("\n");
                // 解析 YAML frontmatter
                if let Ok(parsed) = oak_yaml::from_str::<Value>(&frontmatter_content) {
                    if let Some(obj) = parsed.as_object() {
                        for (key, value) in obj {
                            match key.as_str() {
                                "title" => {
                                    if let Some(title) = value.as_str() {
                                        frontmatter.title = Some(title.to_string());
                                    }
                                }
                                "description" => {
                                    if let Some(description) = value.as_str() {
                                        frontmatter.description = Some(description.to_string());
                                    }
                                }
                                "layout" => {
                                    if let Some(layout) = value.as_str() {
                                        frontmatter.layout = Some(layout.to_string());
                                    }
                                }
                                "tags" => {
                                    if let Some(tags) = value.as_array() {
                                        frontmatter.tags =
                                            tags.iter().filter_map(|tag| tag.as_str().map(|s| s.to_string())).collect();
                                    }
                                }
                                _ => {
                                    if let Some(value_str) = value.as_str() {
                                        frontmatter
                                            .custom
                                            .insert(key.to_string(), nargo_types::NargoValue::String(value_str.to_string()));
                                    }
                                    else if let Some(value_num) = value.as_f64() {
                                        frontmatter
                                            .custom
                                            .insert(key.to_string(), nargo_types::NargoValue::String(value_num.to_string()));
                                    }
                                    else if let Some(value_bool) = value.as_bool() {
                                        frontmatter
                                            .custom
                                            .insert(key.to_string(), nargo_types::NargoValue::String(value_bool.to_string()));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(frontmatter)
    }
}

impl Default for MarkdownParser {
    fn default() -> Self {
        Self::new()
    }
}
