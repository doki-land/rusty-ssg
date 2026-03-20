//! VuTeX 语言定义

use oak_core::{Language, LanguageCategory};
use oak_markdown::{MarkdownElementType, MarkdownTokenType};

/// VuTeX 语言定义
#[derive(Debug, Clone, Copy, Default)]
pub struct VutexLanguage;

impl Language for VutexLanguage {
    type TokenType = MarkdownTokenType;
    type ElementType = MarkdownElementType;

    const NAME: &'static str = "VuTeX";
    type TypedRoot = ();
}
