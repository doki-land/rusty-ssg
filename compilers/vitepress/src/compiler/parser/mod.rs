//! 编译器解析器模块
//! 使用 oaks 解析 Markdown 文档

use oak_core::{Builder, parser::session::ParseSession, source::SourceText};
use oak_markdown::{MarkdownBuilder, MarkdownLanguage};
use nargo_types::Document;

use crate::types::Result;

/// Markdown 解析器
/// 使用 oaks 解析 Markdown 文档
pub struct MarkdownParser {
    /// Markdown 语言配置
    lang_config: MarkdownLanguage,
}

impl MarkdownParser {
    /// 创建新的 Markdown 解析器
    pub fn new() -> Self {
        Self {
            lang_config: MarkdownLanguage::default(),
        }
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
            Ok(root) => {
                let frontmatter = self.extract_frontmatter(source)?;
                Ok(Document {
                    path: path.to_string(),
                    frontmatter,
                    content: source.to_string(),
                    rendered_content: None,
                })
            }
            Err(e) => Err(crate::types::VitePressError::ParserError {
                message: format!("Failed to parse Markdown: {}", e),
                path: path.to_string(),
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

        if source.starts_with("---") {
            let lines: Vec<&str> = source.lines().collect();
            if lines.len() > 2 {
                let mut frontmatter_lines = Vec::new();
                for line in &lines[1..] {
                    if line == "---" {
                        break;
                    }
                    frontmatter_lines.push(line);
                }

                let frontmatter_content = frontmatter_lines.join("\n");
                // 解析 YAML frontmatter
                if let Ok(parsed) = oak_yaml::from_str::<serde::Value>(&frontmatter_content) {
                    if let Some(obj) = parsed.as_object() {
                        for (key, value) in obj {
                            match key.as_str() {
                                "title" => if let Some(title) = value.as_str() {
                                    frontmatter.title = Some(title.to_string());
                                },
                                "description" => if let Some(description) = value.as_str() {
                                    frontmatter.description = Some(description.to_string());
                                },
                                "layout" => if let Some(layout) = value.as_str() {
                                    frontmatter.layout = Some(layout.to_string());
                                },
                                "tags" => if let Some(tags) = value.as_array() {
                                    frontmatter.tags = tags
                                        .iter()
                                        .filter_map(|tag| tag.as_str().map(|s| s.to_string()))
                                        .collect();
                                },
                                _ => {
                                    if let Some(value_str) = value.as_str() {
                                        frontmatter.custom.insert(key.to_string(), value_str.to_string());
                                    } else if let Some(value_num) = value.as_f64() {
                                        frontmatter.custom.insert(key.to_string(), value_num.to_string());
                                    } else if let Some(value_bool) = value.as_bool() {
                                        frontmatter.custom.insert(key.to_string(), value_bool.to_string());
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
