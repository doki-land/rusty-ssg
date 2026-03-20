//! VuTeX 语言定义

use oak_core::{
    ElementRole, ElementType, Language, LanguageCategory, TokenRole, TokenType, UniversalElementRole, UniversalTokenRole,
};

/// VuTeX 标记类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VutexTokenType {
    /// 结束标记
    EndOfStream,
    /// 文本
    Text,
    /// 注释
    Comment,
    /// 短代码
    Shortcode,
}

impl TokenType for VutexTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::EndOfStream;

    fn role(&self) -> Self::Role {
        match self {
            Self::EndOfStream => UniversalTokenRole::Eof,
            Self::Text => UniversalTokenRole::Literal,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Shortcode => UniversalTokenRole::Name,
        }
    }
}

/// VuTeX 元素类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VutexElementType {
    /// 根元素
    Root,
    /// 文本元素
    Text,
    /// 注释元素
    Comment,
    /// 短代码元素
    Shortcode,
}

impl ElementType for VutexElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::Text => UniversalElementRole::Value,
            Self::Comment => UniversalElementRole::Documentation,
            Self::Shortcode => UniversalElementRole::Call,
        }
    }
}

/// VuTeX 语言定义
#[derive(Debug, Clone, Copy, Default)]
pub struct VutexLanguage;

impl Language for VutexLanguage {
    const NAME: &'static str = "vutex";
    const CATEGORY: LanguageCategory = LanguageCategory::Markup;
    type TokenType = VutexTokenType;
    type ElementType = VutexElementType;
    type TypedRoot = ();
}
