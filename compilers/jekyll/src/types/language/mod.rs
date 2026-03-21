//! VuTeX 语言定义

use oak_core::{
    ElementRole, ElementType, Language, LanguageCategory, TokenRole, TokenType, UniversalElementRole, UniversalTokenRole,
};

/// VuTeX 语言定义
#[derive(Debug, Clone, Copy, Default)]
pub struct VutexLanguage;

/// VuTeX 令牌类型
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum VutexTokenType {
    /// 结束标记
    EndOfStream,
    /// 其他标记
    Other,
}

impl TokenType for VutexTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::EndOfStream;
    fn role(&self) -> Self::Role {
        UniversalTokenRole::None
    }
}

/// VuTeX 元素类型
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum VutexElementType {
    /// 根元素
    Root,
    /// 其他元素
    Other,
}

impl ElementType for VutexElementType {
    type Role = UniversalElementRole;
    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            _ => UniversalElementRole::None,
        }
    }
}

impl Language for VutexLanguage {
    const NAME: &'static str = "VuTeX";
    const CATEGORY: LanguageCategory = LanguageCategory::Markup;
    type TokenType = VutexTokenType;
    type ElementType = VutexElementType;
    type TypedRoot = ();
}
