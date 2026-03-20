//! VuTeX 语言定义

use oak_core::{Language, LanguageCategory};

/// VuTeX 语言定义
#[derive(Debug, Clone, Copy, Default)]
pub struct VutexLanguage;

impl Language for VutexLanguage {
    const NAME: &'static str = "VuTeX";
    const CATEGORY: LanguageCategory = LanguageCategory::Markup;
    type TokenType = oak_markdown::MarkdownTokenType;
    type ElementType = oak_markdown::MarkdownElementType;
    type TypedRoot = oak_markdown::ast::MarkdownRoot;
}
